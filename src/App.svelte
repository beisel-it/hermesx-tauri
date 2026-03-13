<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
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
      await performAction(label);
      await refresh();
    } catch (e) {
      error = String(e);
    }
  }

  function formatWorkedTime(ms: number): string {
    const s = Math.floor(ms / 1000);
    const h = Math.floor(s / 3600).toString().padStart(2, '0');
    const m = Math.floor((s % 3600) / 60).toString().padStart(2, '0');
    const sec = (s % 60).toString().padStart(2, '0');
    return `${h}:${m}:${sec}`;
  }

  $effect(() => {
    if (status?.state === 'Working' && status.start_time_ms) {
      const tick = () => {
        const elapsed = Date.now() - (status!.start_time_ms ?? Date.now());
        workedTime = formatWorkedTime((status!.total_worked_ms ?? 0) + elapsed);
      };
      tick();
      interval = setInterval(tick, 1000);
      return () => clearInterval(interval);
    } else if (status) {
      workedTime = formatWorkedTime(status.total_worked_ms);
    }
  });

  onMount(refresh);
  onDestroy(() => clearInterval(interval));
</script>

<main>
  {#if showSettings}
    <Settings onClose={() => { showSettings = false; refresh(); }} />
  {:else if status}
    <div class="header">
      <span class="emoji">{status.emoji}</span>
      <div class="state-info">
        <span class="state-label">{status.label}</span>
        {#if status.dry_run}
          <span class="dry-run-badge">DRY RUN</span>
        {/if}
      </div>
      <button class="settings-btn" onclick={() => showSettings = true}>⚙</button>
    </div>

    <div class="worked-time">{workedTime}</div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <div class="actions">
      {#each status.available_actions as action}
        <button class="action-btn" onclick={() => handleAction(action.label)}>
          {action.label}
        </button>
      {/each}
    </div>
  {:else}
    <div class="loading">Loading…</div>
  {/if}
</main>

<style>
  main { font-family: system-ui, sans-serif; padding: 16px; min-width: 220px; background: #1a1a2e; color: #e0e0e0; }
  .header { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
  .emoji { font-size: 2rem; }
  .state-info { flex: 1; }
  .state-label { font-size: 1rem; font-weight: 600; display: block; }
  .dry-run-badge { font-size: 0.65rem; background: #f59e0b; color: #000; padding: 1px 6px; border-radius: 4px; }
  .settings-btn { background: none; border: none; font-size: 1.2rem; cursor: pointer; color: #888; }
  .settings-btn:hover { color: #fff; }
  .worked-time { font-size: 1.6rem; font-variant-numeric: tabular-nums; text-align: center; margin: 12px 0; color: #a78bfa; }
  .actions { display: flex; flex-direction: column; gap: 8px; }
  .action-btn { padding: 10px; border: none; border-radius: 8px; background: #4f46e5; color: #fff; font-size: 0.9rem; cursor: pointer; transition: background 0.15s; }
  .action-btn:hover { background: #6366f1; }
  .error { color: #f87171; font-size: 0.8rem; margin: 8px 0; }
  .loading { text-align: center; padding: 24px; color: #666; }
</style>
