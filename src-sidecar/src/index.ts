/**
 * zeus-sidecar — Playwright automation sidecar for HermesX
 * Keys mirror BUTTONS map in this file (used by Rust zeus_x/mod.rs)
 */

import { chromium, Browser, BrowserContext, Page } from "playwright";
import * as readline from "readline";

const BUTTONS: Record<string, { id: string; text: string }> = {
  "in-out":               { id: "#TerminalButton0", text: "IN / OUT" },
  "in":                   { id: "#TerminalButton1", text: "IN" },
  "out":                  { id: "#TerminalButton2", text: "OUT" },
  "pause":                { id: "#TerminalButton3", text: "Pause" },
  "mobiles-arbeiten-start": { id: "#TerminalButton4", text: "Mobiles Arbeiten beg" },
  "mobiles-arbeiten-end":   { id: "#TerminalButton5", text: "Mobiles Arbeiten end" },
  "pause-mobil":            { id: "#TerminalButton6", text: "Pause mob. Arbeiten" },
  "bereitschaft-start":   { id: "#TerminalButton7", text: "Bereitschaft START" },
  "bereitschaft-stop":    { id: "#TerminalButton8", text: "Bereitschaft STOP" },
  "dienstgang":           { id: "#TerminalButton9", text: "Dienstgang" },
};

const TERMINAL_URL = "https://isg.intersport.de/terminal";
const SEL_BOOKING_OK = ".buchungsnachricht, .success, [class*='erfolgreich']";

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

async function ensureLoggedIn(p: Page, creds: Credentials): Promise<void> {
  await p.goto(TERMINAL_URL, { waitUntil: "domcontentloaded" });
  const onTerminal = await p.locator("#TerminalButton4").count();
  if (onTerminal > 0) return;
  await p.fill("#uiUserName_I", creds.username);
  await p.fill("#uiPassword_I", creds.password);
  await p.click("#uiNextButton");
  await p.waitForLoadState("domcontentloaded");
  try {
    await p.click("#uiLogOnButton", { timeout: 3000 });
    await p.waitForLoadState("domcontentloaded");
  } catch { /* already logged in */ }
}

async function clickButton(p: Page, key: string): Promise<string> {
  const btn = BUTTONS[key];
  if (!btn) throw new Error(`unknown action key: ${key}`);
  try {
    await p.waitForSelector(btn.id, { timeout: 8000 });
    await p.click(btn.id);
  } catch {
    await p.click(`button:has-text("${btn.text}")`);
  }
  try {
    await p.waitForSelector(SEL_BOOKING_OK, { timeout: 6000 });
    return (await p.locator(SEL_BOOKING_OK).first().innerText()).trim();
  } catch {
    return `${key} clicked`;
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
  // else: last line handler calls shutdown() when pending reaches 0
});

process.on("SIGTERM", async () => {
  await shutdown();
});
