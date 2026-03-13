/**
 * zeus-sidecar — Playwright automation sidecar for HermesX
 * Login flow and selectors ported directly from zeus-punch/src/
 */

import { chromium, Browser, BrowserContext, Page } from "playwright";
import * as readline from "readline";

// ── Selectors (from zeus-punch/src/selectors.ts) ─────────────────────────────
const BUTTONS: Record<string, { id: string }> = {
  "in-out":               { id: "#TerminalButton0" },
  "in":                   { id: "#TerminalButton1" },
  "out":                  { id: "#TerminalButton2" },
  "pause":                { id: "#TerminalButton3" },
  "mobiles-arbeiten-start": { id: "#TerminalButton4" },
  "mobiles-arbeiten-end":   { id: "#TerminalButton5" },
  "pause-mobil":            { id: "#TerminalButton6" },
  "bereitschaft-start":   { id: "#TerminalButton7" },
  "bereitschaft-stop":    { id: "#TerminalButton8" },
  "dienstgang":           { id: "#TerminalButton9" },
};

// From zeus-punch/src/config.ts
const BASE_URL = "https://zeusx.intersport.de/ZEUSX/Environment/Account/LogOn.aspx";
const TIMEOUT  = 30_000;

interface Credentials { username: string; password: string; }
interface Request {
  id: string;
  action: string;
  credentials?: Credentials;
  dry_run?: boolean;
}
interface Response {
  id: string; ok: boolean; result?: string; error?: string;
}

let browser: Browser | null = null;
let context: BrowserContext | null = null;
let page: Page | null = null;

async function ensureBrowser(): Promise<Page> {
  if (!browser) {
    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    page = await context.newPage();
  }
  return page!;
}

// Ported from zeus-punch/src/login.ts
async function login(p: Page, creds: Credentials): Promise<void> {
  await p.goto(BASE_URL, { waitUntil: "networkidle", timeout: TIMEOUT });

  // Step 1: Username — DevExpress widget, use JS click like zeus-punch does
  await p.waitForSelector("#uiUserName_I", { timeout: TIMEOUT });
  await p.fill("#uiUserName_I", creds.username);
  await p.evaluate(() => (document.getElementById("uiNextButton") as HTMLElement)?.click());

  // Step 2: Password — ASP.NET postback, wait for field
  await p.waitForTimeout(2000);
  await p.waitForSelector("#uiPassword_I", { state: "visible", timeout: TIMEOUT });
  await p.fill("#uiPassword_I", creds.password);
  await p.evaluate(() => (document.getElementById("uiLogOnButton") as HTMLElement)?.click());

  // Wait for dashboard
  await p.waitForURL("**/workspace.aspx**", { timeout: TIMEOUT });
}

async function ensureLoggedIn(p: Page, creds: Credentials): Promise<void> {
  // Check if already on terminal (buttons visible)
  const hasButtons = await p.locator("#TerminalButton4").count().catch(() => 0);
  if (hasButtons > 0) return;

  // Not logged in or not on terminal — login first
  await login(p, creds);
}

// Ported from zeus-punch/src/punch.ts — JS click to avoid DevExpress overlay
async function clickButton(p: Page, key: string): Promise<string> {
  const btn = BUTTONS[key];
  if (!btn) throw new Error(`unknown action key: ${key}`);

  await p.waitForSelector(btn.id, { state: "visible", timeout: TIMEOUT });
  const buttonId = btn.id.replace("#", "");
  await p.evaluate((id) => (document.getElementById(id) as HTMLElement)?.click(), buttonId);

  // Wait for server response
  await p.waitForTimeout(3000);

  // Try to read confirmation
  try {
    const alert = await p.locator('[role="alert"], .alert, .buchungsnachricht, .status-message').first();
    const text = await alert.innerText({ timeout: 3000 });
    return text.trim() || `${key} executed`;
  } catch {
    return `${key} executed`;
  }
}

async function handleRequest(req: Request): Promise<Response> {
  if (req.action === "ping") return { id: req.id, ok: true, result: "pong" };

  if (req.dry_run) {
    return { id: req.id, ok: true, result: `[dry-run] would execute: ${req.action}` };
  }

  if (!req.credentials) return { id: req.id, ok: false, error: "credentials required" };

  try {
    const p = await ensureBrowser();
    await ensureLoggedIn(p, req.credentials);
    const result = await clickButton(p, req.action);
    return { id: req.id, ok: true, result };
  } catch (err) {
    return { id: req.id, ok: false, error: String(err) };
  }
}

// ── Pending counter: don't exit before async handlers finish ─────────────────
const rl = readline.createInterface({ input: process.stdin, terminal: false });
let pending = 0;
let closing = false;

async function shutdown(): Promise<void> {
  if (browser) await browser.close();
  process.exit(0);
}

rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;

  pending++;
  let req: Request;
  try {
    req = JSON.parse(trimmed);
  } catch {
    process.stdout.write(JSON.stringify({ id: "parse_error", ok: false, error: "invalid JSON" }) + "\n");
    pending--;
    if (closing && pending === 0) await shutdown();
    return;
  }

  const resp = await handleRequest(req);
  process.stdout.write(JSON.stringify(resp) + "\n");
  pending--;
  if (closing && pending === 0) await shutdown();
});

rl.on("close", async () => {
  closing = true;
  if (pending === 0) await shutdown();
});

process.on("SIGTERM", async () => {
  await shutdown();
});
