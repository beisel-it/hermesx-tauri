<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { getStatus, performAction, type AppStatus } from './lib/tauri';
  import Settings from './Settings.svelte';

  let status = $state<AppStatus | null>(null);
  let showSettings = $state(false);
  let error = $state<string | null>(null);
  let workedTime = $state('00:00:00');
  let interval: ReturnType<typeof setInterval>;

  async function refresh() {
    try {
      status = await getStatus();
      error = null;
    } catch (e) {
      error = String(e);
    }
  }

  async function handleAction(label: string) {
    try {
      error = null;
      await performAction(label);
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  async function close() {
    await getCurrentWindow().hide();
  }

  function formatTime(ms: number): string {
    const s = Math.floor(ms / 1000);
    const h = Math.floor(s / 3600).toString().padStart(2, '0');
    const m = Math.floor((s % 3600) / 60).toString().padStart(2, '0');
    const sec = (s % 60).toString().padStart(2, '0');
    return `${h}:${m}:${sec}`;
  }

  // State → color mapping
  function stateColor(state: string | undefined): string {
    switch (state) {
      case 'Working':  return '#22c55e';
      case 'Paused':   return '#f59e0b';
      case 'Finished': return '#6366f1';
      default:         return '#6b7280';
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
  <!-- Custom title bar (draggable) -->
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-left">
      <span class="app-icon">🪶</span>
      <span class="app-name">HermesX</span>
      {#if status?.dry_run}
        <span class="badge dry-run">DRY RUN</span>
      {/if}
    </div>
    <div class="titlebar-right">
      <button class="icon-btn" title="Einstellungen" onclick={() => showSettings = !showSettings}>⚙️</button>
      <button class="icon-btn close-btn" title="Schließen" onclick={close}>✕</button>
    </div>
  </div>

  <div class="content">
    {#if showSettings}
      <!-- Settings View -->
      <Settings
        config={status}
        onSave={async (cfg) => {
          try {
            const { invoke } = await import('@tauri-apps/api/core');
            await invoke('set_config', { config: cfg });
            showSettings = false;
            await refresh();
          } catch (e) { error = String(e); }
        }}
      />
    {:else if status}
      <!-- Status View -->
      <div class="status-card" style="--state-color: {stateColor(status.state)}">
        <div class="status-emoji">{status.emoji}</div>
        <div class="status-label">{status.label}</div>
        <div class="status-time">{workedTime}</div>
      </div>

      {#if status.available_actions?.length > 0}
        <div class="actions">
          {#each status.available_actions as action}
            <button
              class="action-btn"
              class:primary={action.label.includes('Start') || action.label.includes('Beenden')}
              onclick={() => handleAction(action.label)}
            >
              {action.label}
            </button>
          {/each}
        </div>
      {:else}
        <p class="no-actions">Keine Aktionen verfügbar.</p>
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
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #18181b;
    color: #f4f4f5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
    border-radius: 12px;
    overflow: hidden;
    border: 1px solid #3f3f46;
  }

  /* Title bar */
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    background: #09090b;
    border-bottom: 1px solid #27272a;
    user-select: none;
    -webkit-user-select: none;
  }
  .titlebar-left  { display: flex; align-items: center; gap: 8px; }
  .titlebar-right { display: flex; align-items: center; gap: 4px; }
  .app-icon  { font-size: 1.1rem; }
  .app-name  { font-size: 0.85rem; font-weight: 600; color: #a1a1aa; }
  .icon-btn  { background: none; border: none; cursor: pointer; padding: 4px 6px; border-radius: 6px; color: #71717a; font-size: 0.9rem; transition: background 0.1s, color 0.1s; }
  .icon-btn:hover { background: #27272a; color: #f4f4f5; }
  .close-btn:hover { background: #7f1d1d; color: #fca5a5; }

  /* Badges */
  .badge { font-size: 0.6rem; font-weight: 700; padding: 2px 6px; border-radius: 4px; text-transform: uppercase; letter-spacing: 0.05em; }
  .dry-run { background: #78350f; color: #fcd34d; }

  /* Content */
  .content { flex: 1; display: flex; flex-direction: column; padding: 20px 16px 16px; overflow-y: auto; gap: 16px; }

  /* Status card */
  .status-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 24px 16px;
    background: #09090b;
    border-radius: 10px;
    border: 1px solid #27272a;
    border-top: 3px solid var(--state-color, #6b7280);
  }
  .status-emoji { font-size: 2.5rem; }
  .status-label { font-size: 1rem; font-weight: 600; color: var(--state-color, #f4f4f5); }
  .status-time  { font-size: 2rem; font-variant-numeric: tabular-nums; font-weight: 300; color: #a1a1aa; letter-spacing: -0.02em; }

  /* Actions */
  .actions { display: flex; flex-direction: column; gap: 8px; }
  .action-btn {
    padding: 11px 16px;
    border: 1px solid #3f3f46;
    border-radius: 8px;
    background: #27272a;
    color: #f4f4f5;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.12s, border-color 0.12s;
    text-align: left;
  }
  .action-btn:hover { background: #3f3f46; border-color: #52525b; }
  .action-btn.primary { background: #4f46e5; border-color: #4f46e5; }
  .action-btn.primary:hover { background: #6366f1; border-color: #6366f1; }

  /* Error */
  .error-banner { background: #450a0a; border: 1px solid #7f1d1d; border-radius: 8px; padding: 10px 12px; font-size: 0.8rem; color: #fca5a5; }

  /* Loading */
  .loading { display: flex; flex-direction: column; align-items: center; justify-content: center; flex: 1; gap: 12px; color: #52525b; font-size: 0.9rem; }
  .spinner { width: 24px; height: 24px; border: 2px solid #27272a; border-top-color: #4f46e5; border-radius: 50%; animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }

  .no-actions { color: #52525b; font-size: 0.85rem; text-align: center; padding: 8px; }
</style>
