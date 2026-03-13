<script lang="ts">
  import { getConfig, setConfig, setDryRun, type UserConfig } from './lib/tauri';

  let { onClose } = $props<{ onClose: () => void }>();
  let config = $state<UserConfig | null>(null);
  let saving = $state(false);
  let saved = $state(false);

  const DAYS = ['So', 'Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa'];

  async function load() {
    config = await getConfig();
  }

  async function save() {
    if (!config) return;
    saving = true;
    try {
      await setConfig(config);
      saved = true;
      setTimeout(() => { saved = false; }, 2000);
    } finally {
      saving = false;
    }
  }

  load();
</script>

{#if config}
<div class="settings">
  <div class="settings-header">
    <h2>Settings</h2>
    <button onclick={onClose}>✕</button>
  </div>

  <section>
    <h3>Schedule</h3>
    <label>
      Start
      <div class="time-input">
        <input type="number" min="0" max="23" bind:value={config.schedule.start_time.hour} />
        :
        <input type="number" min="0" max="59" bind:value={config.schedule.start_time.minute} />
      </div>
    </label>
    <label>
      Work duration (h)
      <input type="number" min="1" max="16" step="0.5" bind:value={config.schedule.work_duration} />
    </label>
    <label>
      Break duration (min)
      <input type="number" min="0" max="120" bind:value={config.schedule.break_duration} />
    </label>
    <label>Workdays
      <div class="days">
        {#each DAYS as day, i}
          <label class="day-toggle">
            <input type="checkbox" bind:checked={config.schedule.workdays[i]} />
            {day}
          </label>
        {/each}
      </div>
    </label>
  </section>

  <section>
    <h3>Notifications</h3>
    <label class="toggle"><input type="checkbox" bind:checked={config.notifications.quiet_mode} /> Quiet Mode</label>
    <label class="toggle"><input type="checkbox" bind:checked={config.notifications.suppress_during_calls} /> Suppress during calls</label>
  </section>

  <section>
    <h3>Developer</h3>
    <label class="toggle dry-run-toggle">
      <input type="checkbox" bind:checked={config.dry_run}
        onchange={() => setDryRun(config!.dry_run)} />
      Dry Run Mode <span class="hint">(no ZeusX calls)</span>
    </label>
  </section>

  <div class="footer">
    <button class="save-btn" onclick={save} disabled={saving}>
      {saving ? 'Saving…' : saved ? '✓ Saved' : 'Save'}
    </button>
  </div>
</div>
{/if}

<style>
  .settings { font-family: system-ui, sans-serif; padding: 16px; background: #1a1a2e; color: #e0e0e0; min-width: 220px; }
  .settings-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
  h2 { margin: 0; font-size: 1rem; }
  h3 { font-size: 0.8rem; color: #888; text-transform: uppercase; letter-spacing: 0.05em; margin: 12px 0 6px; }
  button { background: none; border: none; cursor: pointer; color: #888; font-size: 1rem; }
  section { border-top: 1px solid #2d2d4e; padding-top: 8px; }
  label { display: flex; justify-content: space-between; align-items: center; font-size: 0.85rem; margin-bottom: 8px; }
  input[type=number] { width: 52px; background: #2d2d4e; border: 1px solid #3d3d5e; color: #e0e0e0; padding: 4px 6px; border-radius: 4px; }
  .time-input { display: flex; align-items: center; gap: 4px; }
  .days { display: flex; gap: 4px; }
  .day-toggle { flex-direction: column; font-size: 0.75rem; gap: 2px; justify-content: center; }
  .toggle { gap: 8px; justify-content: flex-start; }
  .dry-run-toggle { color: #f59e0b; }
  .hint { color: #666; font-size: 0.75rem; }
  .footer { margin-top: 16px; }
  .save-btn { width: 100%; padding: 10px; background: #4f46e5; color: #fff; border-radius: 8px; font-size: 0.9rem; transition: background 0.15s; }
  .save-btn:hover:not(:disabled) { background: #6366f1; }
  .save-btn:disabled { opacity: 0.6; cursor: default; }
</style>
