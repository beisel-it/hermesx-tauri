"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// src/index.ts
var import_playwright = require("playwright");
var readline = __toESM(require("readline"));
var BUTTONS = {
  "in-out": { id: "#TerminalButton0", text: "IN / OUT" },
  "in": { id: "#TerminalButton1", text: "IN" },
  "out": { id: "#TerminalButton2", text: "OUT" },
  "pause": { id: "#TerminalButton3", text: "Pause" },
  "mobiles-arbeiten-start": { id: "#TerminalButton4", text: "Mobiles Arbeiten beg" },
  "mobiles-arbeiten-end": { id: "#TerminalButton5", text: "Mobiles Arbeiten end" },
  "pause-mobil": { id: "#TerminalButton6", text: "Pause mob. Arbeiten" },
  "bereitschaft-start": { id: "#TerminalButton7", text: "Bereitschaft START" },
  "bereitschaft-stop": { id: "#TerminalButton8", text: "Bereitschaft STOP" },
  "dienstgang": { id: "#TerminalButton9", text: "Dienstgang" }
};
var TERMINAL_URL = "https://isg.intersport.de/terminal";
var SEL_BOOKING_OK = ".buchungsnachricht, .success, [class*='erfolgreich']";
var browser = null;
var context = null;
var page = null;
async function ensureBrowser() {
  if (!browser) {
    browser = await import_playwright.chromium.launch({ headless: true });
    context = await browser.newContext();
    page = await context.newPage();
  }
  return page;
}
async function ensureLoggedIn(p, creds) {
  await p.goto(TERMINAL_URL, { waitUntil: "domcontentloaded" });
  const onTerminal = await p.locator("#TerminalButton4").count();
  if (onTerminal > 0) return;
  await p.fill("#uiUserName_I", creds.username);
  await p.fill("#uiPassword_I", creds.password);
  await p.click("#uiNextButton");
  await p.waitForLoadState("domcontentloaded");
  try {
    await p.click("#uiLogOnButton", { timeout: 3e3 });
    await p.waitForLoadState("domcontentloaded");
  } catch {
  }
}
async function clickButton(p, key) {
  const btn = BUTTONS[key];
  if (!btn) throw new Error(`unknown action key: ${key}`);
  try {
    await p.waitForSelector(btn.id, { timeout: 8e3 });
    await p.click(btn.id);
  } catch {
    await p.click(`button:has-text("${btn.text}")`);
  }
  try {
    await p.waitForSelector(SEL_BOOKING_OK, { timeout: 6e3 });
    return (await p.locator(SEL_BOOKING_OK).first().innerText()).trim();
  } catch {
    return `${key} clicked`;
  }
}
async function handleRequest(req) {
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
var rl = readline.createInterface({ input: process.stdin, terminal: false });
rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;
  let req;
  try {
    req = JSON.parse(trimmed);
  } catch {
    process.stdout.write(JSON.stringify({ id: "parse_error", ok: false, error: "invalid JSON" }) + "\n");
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
