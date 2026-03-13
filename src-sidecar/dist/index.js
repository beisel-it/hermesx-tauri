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
  "in-out": { id: "#TerminalButton0" },
  "in": { id: "#TerminalButton1" },
  "out": { id: "#TerminalButton2" },
  "pause": { id: "#TerminalButton3" },
  "mobiles-arbeiten-start": { id: "#TerminalButton4" },
  "mobiles-arbeiten-end": { id: "#TerminalButton5" },
  "pause-mobil": { id: "#TerminalButton6" },
  "bereitschaft-start": { id: "#TerminalButton7" },
  "bereitschaft-stop": { id: "#TerminalButton8" },
  "dienstgang": { id: "#TerminalButton9" }
};
var BASE_URL = "https://zeusx.intersport.de/ZEUSX/Environment/Account/LogOn.aspx";
var TIMEOUT = 3e4;
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
async function login(p, creds) {
  await p.goto(BASE_URL, { waitUntil: "networkidle", timeout: TIMEOUT });
  await p.waitForSelector("#uiUserName_I", { timeout: TIMEOUT });
  await p.fill("#uiUserName_I", creds.username);
  await p.evaluate(() => document.getElementById("uiNextButton")?.click());
  await p.waitForTimeout(2e3);
  await p.waitForSelector("#uiPassword_I", { state: "visible", timeout: TIMEOUT });
  await p.fill("#uiPassword_I", creds.password);
  await p.evaluate(() => document.getElementById("uiLogOnButton")?.click());
  await p.waitForURL("**/workspace.aspx**", { timeout: TIMEOUT });
}
async function ensureLoggedIn(p, creds) {
  const hasButtons = await p.locator("#TerminalButton4").count().catch(() => 0);
  if (hasButtons > 0) return;
  await login(p, creds);
}
async function clickButton(p, key) {
  const btn = BUTTONS[key];
  if (!btn) throw new Error(`unknown action key: ${key}`);
  await p.waitForSelector(btn.id, { state: "visible", timeout: TIMEOUT });
  const buttonId = btn.id.replace("#", "");
  await p.evaluate((id) => document.getElementById(id)?.click(), buttonId);
  await p.waitForTimeout(3e3);
  try {
    const alert = await p.locator('[role="alert"], .alert, .buchungsnachricht, .status-message').first();
    const text = await alert.innerText({ timeout: 3e3 });
    return text.trim() || `${key} executed`;
  } catch {
    return `${key} executed`;
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
var pending = 0;
var closing = false;
async function shutdown() {
  if (browser) await browser.close();
  process.exit(0);
}
rl.on("line", async (line) => {
  const trimmed = line.trim();
  if (!trimmed) return;
  pending++;
  let req;
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
