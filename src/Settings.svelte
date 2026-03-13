<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let { config = $bindable(), onSave } = $props<{
    config: any;
    onSave: (c: any) => void;
  }>();

  // Credentials
  let credStatus = $state<{ stored: boolean; username?: string } | null>(null);
  let credUsername = $state("");
  let credPassword = $state("");
  let credMessage  = $state("");

  $effect(() => {
    invoke<{ stored: boolean; username?: string }>("load_credentials_status")
      .then(s => { credStatus = s; });
  });

  async function saveCreds() {
    await invoke("save_credentials", { username: credUsername, password: credPassword });
    credStatus = await invoke("load_credentials_status");
    credMessage = "✅ Gespeichert";
    credUsername = "";
    credPassword = "";
    setTimeout(() => (credMessage = ""), 3000);
  }

  async function deleteCreds() {
    await invoke("delete_credentials");
    credStatus = { stored: false };
    credMessage = "🗑 Gelöscht";
    setTimeout(() => (credMessage = ""), 3000);
  }
</script>

<div class="settings">
  <h2>⚙️ Einstellungen</h2>

  <!-- Schedule -->
  <section>
    <h3>📅 Arbeitszeit</h3>
    <label>
      Arbeitsbeginn
      <input type="time"
        value={`${String(config?.schedule?.start_time?.hour ?? 8).padStart(2,'0')}:${String(config?.schedule?.start_time?.minute ?? 30).padStart(2,'0')}`}
        onchange={(e) => {
          const [h, m] = e.currentTarget.value.split(":").map(Number);
          config = { ...config, schedule: { ...config.schedule, start_time: { hour: h, minute: m } } };
        }} />
    </label>
    <label>
      Arbeitszeit (Stunden)
      <input type="number" min="1" max="12" step="0.5"
        value={config?.schedule?.work_duration ?? 8}
        onchange={(e) => {
          config = { ...config, schedule: { ...config.schedule, work_duration: parseFloat(e.currentTarget.value) } };
        }} />
    </label>
    <label>
      Pausendauer (Minuten)
      <input type="number" min="15" max="120"
        value={config?.schedule?.break_duration ?? 30}
        onchange={(e) => {
          config = { ...config, schedule: { ...config.schedule, break_duration: parseInt(e.currentTarget.value) } };
        }} />
    </label>
  </section>

  <!-- Notifications -->
  <section>
    <h3>🔔 Benachrichtigungen</h3>
    <label>
      <input type="checkbox"
        checked={config?.notifications?.quiet_mode ?? false}
        onchange={(e) => {
          config = { ...config, notifications: { ...config.notifications, quiet_mode: e.currentTarget.checked } };
        }} />
      Ruhemodus (keine Benachrichtigungen)
    </label>
  </section>

  <!-- ZeusX Credentials -->
  <section>
    <h3>🔐 ZeusX Zugangsdaten</h3>
    {#if credStatus?.stored}
      <p class="stored">✅ Gespeichert als <strong>{credStatus.username}</strong></p>
      <button class="danger" onclick={deleteCreds}>🗑 Löschen</button>
    {:else}
      <p class="hint">Noch keine Zugangsdaten gespeichert. Werden sicher im OS-Schlüsselbund abgelegt.</p>
      <label>
        Benutzername
        <input type="text" bind:value={credUsername} placeholder="INTERSPORT Kürzel" />
      </label>
      <label>
        Passwort
        <input type="password" bind:value={credPassword} />
      </label>
      <button onclick={saveCreds} disabled={!credUsername || !credPassword}>
        🔒 Speichern
      </button>
    {/if}
    {#if credMessage}<p class="msg">{credMessage}</p>{/if}
  </section>

  <!-- Debug -->
  <section>
    <h3>🧪 Debug</h3>
    <label>
      <input type="checkbox"
        checked={config?.dry_run ?? false}
        onchange={(e) => {
          config = { ...config, dry_run: e.currentTarget.checked };
        }} />
      Dry Run (kein ZeusX-Zugriff)
    </label>
    <label>
      <input type="checkbox"
        checked={config?.manual_mode ?? false}
        onchange={(e) => {
          config = { ...config, manual_mode: e.currentTarget.checked };
        }} />
      Manuelle Buchungen anzeigen
    </label>
  </section>

  <button class="primary" onclick={() => onSave(config)}>Speichern</button>
</div>

<style>
  .settings { padding: 1rem; font-family: system-ui; }
  h2 { margin: 0 0 1rem; font-size: 1.1rem; }
  h3 { font-size: 0.85rem; color: #6b7280; margin: 1rem 0 0.5rem; }
  section { margin-bottom: 1rem; }
  label { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.85rem; margin-bottom: 0.5rem; }
  input[type="text"], input[type="password"], input[type="number"], input[type="time"] {
    padding: 0.35rem 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem; font-size: 0.85rem;
  }
  input[type="checkbox"] { width: 1rem; height: 1rem; margin-right: 0.5rem; }
  label:has(input[type="checkbox"]) { flex-direction: row; align-items: center; }
  button { padding: 0.4rem 0.9rem; border-radius: 0.375rem; border: none; cursor: pointer; font-size: 0.85rem; }
  button.primary { background: #4f46e5; color: white; margin-top: 0.5rem; }
  button.danger  { background: #ef4444; color: white; }
  button:disabled { opacity: 0.5; cursor: not-allowed; }
  .stored { color: #16a34a; font-size: 0.85rem; }
  .hint   { color: #6b7280; font-size: 0.8rem; }
  .msg    { font-size: 0.85rem; margin-top: 0.25rem; }
</style>
