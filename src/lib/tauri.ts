import { invoke } from '@tauri-apps/api/core';

export interface WorkAction {
  label: string;
  next_state: string;
  zeusX_action: string | null;
}

export interface AppStatus {
  state: string;
  emoji: string;
  label: string;
  start_time_ms: number | null;
  total_worked_ms: number;
  finished_for_today: boolean;
  dry_run: boolean;
  available_actions: WorkAction[];
}

export interface UserConfig {
  schedule: {
    start_time: { hour: number; minute: number };
    work_duration: number;
    break_duration: number;
    is_flexible: boolean;
    workdays: boolean[];
  };
  notifications: {
    quiet_mode: boolean;
    smart_flexibility: boolean;
    work_mode_detection: boolean;
    suppress_during_calls: boolean;
    suppress_during_gaming: boolean;
  };
  inactivity: {
    short_break_reminder: number;
    auto_break_suggestion: number;
  };
  debug: boolean;
  dry_run: boolean;
}

export const getStatus   = ()                         => invoke<AppStatus>('get_status');
export const getConfig   = ()                         => invoke<UserConfig>('get_config');
export const setConfig   = (config: UserConfig)       => invoke<void>('set_config', { config });
export const setDryRun   = (enabled: boolean)         => invoke<{ dry_run: boolean }>('set_dry_run', { enabled });
export const performAction = (actionLabel: string)    => invoke<unknown>('perform_action', { actionLabel });
