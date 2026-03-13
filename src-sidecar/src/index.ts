/**
 * zeus-sidecar — Playwright automation sidecar for HermesX
 *
 * Protocol (stdin/stdout, one JSON object per line):
 *   IN:  { "id": string, "action": "start_work"|"end_work"|"start_break"|"end_break"|"ping", "credentials"?: {...} }
 *   OUT: { "id": string, "ok": boolean, "result"?: string, "error"?: string }
 *
 * Managed by Tauri sidecar spawn. Exits cleanly on stdin close.
 */

import { chromium, Browser, BrowserContext, Page } from "playwright";
import * as readline from "readline";

// ── Selectors (mirrors zeusX/selectors.rs) ───────────────────────────────────
const SEL = {
  START_WORK:   "input[value='Mobiles Arbeiten'], button:has-text('Mobiles Arbeiten')",
  END_WORK:     "input[value='Ende MA'], button:has-text('Ende MA')",
  START_BREAK:  "input[value='Pause'], button:has-text('Pause')",
  END_BREAK:    "input[value='Ende Pause'], button:has-text('Ende Pause')",
  BOOKING_OK:   ".buchungsnachricht, .success, [class*='success']",
  TERMINAL_URL: "https://zeusX.intersport.de/terminal",
} as const;

// ── Types ─────────────────────────────────────────────────────────────────────
type Action = "start_work" | "end_work" | "start_break" | "end_break" | "ping";

interface Credentials {
  username: string;
  password: string;
}

interface Request {
  id:          string;
  action:      Action;
  credentials?: Credentials;
  dry_run?:    boolean;
}

interface Response {
  id:     string;
  ok:     boolean;
  result?: string;
  error?: string;
}

// ── State ─────────────────────────────────────────────────────────────────────
let browser: Browser | null = null;
let context: BrowserContext | null = null;
let page:    Page | null    = null;

async function ensureBrowser(): Promise<Page> {
  if (!browser) {
    browser = await chromium.launch({ headless: true });
    context = await browser.newContext();
    page    = await context.newPage();
  }
  return page!;
}

async function ensureLoggedIn(p: Page, creds: Credentials): Promise<void> {
  await p.goto(SEL.TERMINAL_URL, { waitUntil: "domcontentloaded" });

  // If already on terminal, skip login
  const onTerminal = await p.locator(SEL.START_WORK).count();
  if (onTerminal > 0) return;

  // Login form
  await p.fill("input[name='username'], input[type='text']", creds.username);
  await p.fill("input[name='password'], input[type='password']", creds.password);
  await p.click("input[type='submit'], button[type='submit']");
  await p.waitForLoadState("domcontentloaded");
}

async function clickAction(p: Page, selector: string): Promise<string> {
  await p.waitForSelector(selector, { timeout: 10_000 });
  await p.click(selector);
  // Wait for booking confirmation
  try {
    await p.waitForSelector(SEL.BOOKING_OK, { timeout: 8_000 });
    const msg = await p.locator(SEL.BOOKING_OK).first().innerText();
    return msg.trim();
  } catch {
    return "action clicked, no confirmation element found";
  }
}

async function handleRequest(req: Request): Promise<Response> {
  if (req.action === "ping") {
    return { id: req.id, ok: true, result: "pong" };
  }

  if (req.dry_run) {
    return { id: req.id, ok: true, result: `[dry-run] would execute: ${req.action}` };
  }

  if (!req.credentials) {
    return { id: req.id, ok: false, error: "credentials required" };
  }

  try {
    const p = await ensureBrowser();
    await ensureLoggedIn(p, req.credentials);

    let selector: string;
    switch (req.action) {
      case "start_work":  selector = SEL.START_WORK;  break;
      case "end_work":    selector = SEL.END_WORK;    break;
      case "start_break": selector = SEL.START_BREAK; break;
      case "end_break":   selector = SEL.END_BREAK;   break;
      default:
        return { id: req.id, ok: false, error: `unknown action: ${req.action}` };
    }

    const result = await clickAction(p, selector);
    return { id: req.id, ok: true, result };
  } catch (err) {
    return { id: req.id, ok: false, error: String(err) };
  }
}

// ── Main: stdin/stdout JSON protocol ─────────────────────────────────────────
const rl = readline.createInterface({ input: process.stdin, terminal: false });

rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;

  let req: Request;
  try {
    req = JSON.parse(trimmed);
  } catch {
    const resp: Response = { id: "parse_error", ok: false, error: "invalid JSON" };
    process.stdout.write(JSON.stringify(resp) + "\n");
    return;
  }

  const resp = await handleRequest(req);
  process.stdout.write(JSON.stringify(resp) + "\n");
});

rl.on("close", async () => {
  if (browser) await browser.close();
  process.exit(0);
});

process.on("SIGTERM", async () => {
  if (browser) await browser.close();
  process.exit(0);
});
