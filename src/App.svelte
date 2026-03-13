<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getStatus, performAction, performManualAction, type AppStatus } from './lib/tauri';
  import Settings from './Settings.svelte';

  let status    = $state<AppStatus | null>(null);
  let config    = $state<Record<string, unknown> | null>(null);
  let showSettings = $state(false);
  let error     = $state<string | null>(null);
  let toast     = $state<string | null>(null);
  let workedTime = $state('00:00:00');
  let interval: ReturnType<typeof setInterval>;

  // Manual booking keys — keys match sidecar BUTTONS in src-sidecar/src/index.ts
  const MANUAL_ACTIONS = [
    { key: 'mobiles-arbeiten-start', label: '▶ MA Start' },
    { key: 'mobiles-arbeiten-end',   label: '⏹ MA Ende' },
    { key: 'pause-mobil',            label: '☕ Pause' },
    { key: 'in',                     label: '🟢 IN' },
    { key: 'out',                    label: '🔴 OUT' },
  ];

  async function refresh() {
    try {
      [status, config] = await Promise.all([
        getStatus(),
        invoke<Record<string, any>>('get_config'),
      ]);
      error = null;
    } catch (e) {
      error = String(e);
    }
  }

  function showToast(msg: string, duration = 4000) {
    toast = msg;
    setTimeout(() => (toast = null), duration);
  }

  async function handleAction(label: string) {
    try {
      error = null;
      const result = await performAction(label);
      if (result?.zeusX) {
        const z = result.zeusX;
        if (z?.result) showToast(`🔖 ${z.result}`);
        else if (z?.error) showToast(`⚠️ ZeusX: ${z.error}`);
      }
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  async function handleManual(key: string) {
    try {
      error = null;
      const result = await performManualAction(key);
      if (result.dry_run) {
        showToast(`🧪 Dry Run — hätte ausgeführt: ${result.action}`);
      } else if (result.result?.ok) {
        showToast(`✅ ${result.result.result ?? result.action}`);
      } else if (result.result?.error) {
        showToast(`⚠️ ${result.result.error}`);
      }
    } catch (e) {
      error = String(e);
    }
  }

  async function close() {
    await invoke('hide_window');
  }

  function formatTime(ms: number): string {
    const s = Math.floor(ms / 1000);
    const h = Math.floor(s / 3600).toString().padStart(2, '0');
    const m = Math.floor((s % 3600) / 60).toString().padStart(2, '0');
    const sec = (s % 60).toString().padStart(2, '0');
    return `${h}:${m}:${sec}`;
  }

  function stateColor(state?: string): string {
    switch (state) {
      case 'Working':  return '#22c55e';
      case 'Paused':   return '#f59e0b';
      case 'Finished': return '#6366f1';
      default:         return '#52525b';
    }
  }

  $effect(() => {
    if (status?.state === 'Working' && status.start_time_ms) {
      const tick = () => {
        const elapsed = Date.now() - (status!.start_time_ms ?? Date.now());
        workedTime = formatTime((status!.total_worked_ms ?? 0) + elapsed);
      };
      tick();
      interval = setInterval(tick, 1000);
      return () => clearInterval(interval);
    } else if (status) {
      workedTime = formatTime(status.total_worked_ms);
    }
  });

  onMount(refresh);
  onDestroy(() => clearInterval(interval));
</script>

<div class="app">
  <!-- Title bar -->
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-left">
      <span class="app-icon">🪶</span>
      <span class="app-name">HermesX</span>
      {#if status?.dry_run}
        <span class="badge dry-run">DRY RUN</span>
      {/if}
    </div>
    <div class="titlebar-right">
      <button class="icon-btn" title="Einstellungen" onclick={() => { showSettings = !showSettings; }}>⚙️</button>
      <button class="icon-btn close-btn" title="In Tray minimieren" onclick={close}>✕</button>
    </div>
  </div>

  <!-- Toast -->
  {#if toast}
    <div class="toast">{toast}</div>
  {/if}

  <div class="content">
    {#if showSettings}
      <Settings
        config={config}
        onSave={async (cfg) => {
          try {
            await invoke('set_config', { config: cfg });
            showSettings = false;
            await refresh();
          } catch (e) { error = String(e); }
        }}
      />
    {:else if status}
      <!-- Status card -->
      <div class="status-card" style="--state-color: {stateColor(status.state)}">
        <div class="status-emoji">{status.emoji}</div>
        <div class="status-label">{status.label}</div>
        <div class="status-time">{workedTime}</div>
      </div>

      <!-- State machine actions -->
      {#if status.available_actions?.length > 0}
        <div class="actions">
          {#each status.available_actions as action}
            <button
              class="action-btn"
              class:primary={action.label.toLowerCase().includes('start')}
              onclick={() => handleAction(action.label)}
            >
              {action.label}
            </button>
          {/each}
        </div>
      {/if}

      <!-- Manual booking (hidden unless manual_mode) -->
      {#if config?.manual_mode}
        <div class="divider">
          <span>Manuelle Buchungen</span>
        </div>
        <div class="actions manual">
          {#each MANUAL_ACTIONS as a}
            <button class="action-btn manual-btn" onclick={() => handleManual(a.key)}>
              {a.label}
            </button>
          {/each}
        </div>
      {/if}

      {#if error}
        <div class="error-banner">⚠️ {error}</div>
      {/if}
    {:else}
      <div class="loading">
        <div class="spinner"></div>
        <span>Verbinde…</span>
      </div>
    {/if}
  </div>
</div>

<style>
  :global(*, *::before, *::after) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(html, body) { height: 100%; overflow: hidden; background: transparent; }

  .app {
    display: flex; flex-direction: column; height: 100vh;
    background: #18181b; color: #f4f4f5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
    border-radius: 12px; overflow: hidden; border: 1px solid #3f3f46;
    position: relative;
  }

  .titlebar {
    display: flex; align-items: center; justify-content: space-between;
    padding: 10px 12px; background: #09090b; border-bottom: 1px solid #27272a;
    user-select: none; -webkit-user-select: none; flex-shrink: 0;
  }
  .titlebar-left  { display: flex; align-items: center; gap: 8px; }
  .titlebar-right { display: flex; align-items: center; gap: 4px; }
  .app-icon { font-size: 1.1rem; }
  .app-name { font-size: 0.85rem; font-weight: 600; color: #a1a1aa; }
  .icon-btn { background: none; border: none; cursor: pointer; padding: 4px 6px; border-radius: 6px; color: #71717a; font-size: 0.9rem; transition: background 0.1s, color 0.1s; }
  .icon-btn:hover { background: #27272a; color: #f4f4f5; }
  .close-btn:hover { background: #7f1d1d; color: #fca5a5; }

  .badge { font-size: 0.6rem; font-weight: 700; padding: 2px 6px; border-radius: 4px; text-transform: uppercase; letter-spacing: 0.05em; }
  .dry-run { background: #78350f; color: #fcd34d; }

  /* Toast */
  .toast {
    position: absolute; top: 48px; left: 12px; right: 12px;
    background: #1e293b; border: 1px solid #334155; border-radius: 8px;
    padding: 8px 12px; font-size: 0.82rem; color: #cbd5e1;
    z-index: 100; animation: slide-in 0.15s ease-out;
  }
  @keyframes slide-in { from { opacity: 0; transform: translateY(-4px); } to { opacity: 1; transform: translateY(0); } }

  .content { flex: 1; display: flex; flex-direction: column; padding: 16px; overflow-y: auto; gap: 12px; }

  .status-card {
    display: flex; flex-direction: column; align-items: center; gap: 4px;
    padding: 20px 16px; background: #09090b; border-radius: 10px;
    border: 1px solid #27272a; border-top: 3px solid var(--state-color, #52525b);
    flex-shrink: 0;
  }
  .status-emoji { font-size: 2.2rem; }
  .status-label { font-size: 0.95rem; font-weight: 600; color: var(--state-color, #f4f4f5); }
  .status-time  { font-size: 1.8rem; font-variant-numeric: tabular-nums; font-weight: 300; color: #a1a1aa; letter-spacing: -0.02em; }

  .actions { display: flex; flex-direction: column; gap: 6px; }
  .action-btn {
    padding: 10px 14px; border: 1px solid #3f3f46; border-radius: 8px;
    background: #27272a; color: #f4f4f5; font-size: 0.88rem; font-weight: 500;
    cursor: pointer; transition: background 0.1s; text-align: left;
  }
  .action-btn:hover { background: #3f3f46; }
  .action-btn.primary { background: #4f46e5; border-color: #4f46e5; }
  .action-btn.primary:hover { background: #6366f1; }

  .divider { display: flex; align-items: center; gap: 8px; color: #52525b; font-size: 0.72rem; text-transform: uppercase; letter-spacing: 0.08em; }
  .divider::before, .divider::after { content: ''; flex: 1; border-top: 1px solid #27272a; }

  .actions.manual { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }
  .manual-btn { font-size: 0.78rem; padding: 8px 10px; background: #1c1c1e; border-color: #2d2d30; color: #a1a1aa; }
  .manual-btn:hover { background: #27272a; color: #f4f4f5; }

  .error-banner { background: #450a0a; border: 1px solid #7f1d1d; border-radius: 8px; padding: 10px 12px; font-size: 0.8rem; color: #fca5a5; }
  .loading { display: flex; flex-direction: column; align-items: center; justify-content: center; flex: 1; gap: 10px; color: #52525b; font-size: 0.9rem; }
  .spinner { width: 22px; height: 22px; border: 2px solid #27272a; border-top-color: #4f46e5; border-radius: 50%; animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
