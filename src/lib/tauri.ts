import { invoke } from '@tauri-apps/api/core';

export interface AppStatus {
  state: string;
  emoji: string;
  label: string;
  start_time_ms: number | null;
  total_worked_ms: number;
  finished_for_today: boolean;
  dry_run: boolean;
  available_actions: Array<{ label: string; zeusX_action?: string }>;
}

export interface ActionResult {
  transition?: unknown;
  zeusX?: {
    id?: string;
    ok?: boolean;
    result?: string;
    error?: string;
  } | null;
}

export interface ManualResult {
  action: string;
  dry_run: boolean;
  result: { id?: string; ok?: boolean; result?: string; error?: string };
}

export const getStatus = () => invoke<AppStatus>('get_status');

export const performAction = (label: string) =>
  invoke<ActionResult>('perform_action', { actionLabel: label });

export const performManualAction = (actionKey: string) =>
  invoke<ManualResult>('perform_manual_action', { actionKey });
