export type ShortcutConfig = {
  modifiers: string[];
  key: string;
};

export interface AppSettings {
  autoCleanDays: number;
  maxHistorySize: number;
  ignoredApps: string[];
  isNewUser: boolean;
  isAutostartEnabled: boolean;
  quickPickerShortcut: ShortcutConfig;
}

export interface Clip {
  id: number;
  window_title: string;
  app_name: string;
  content: string;
  auto_tags: string;
  is_pinned: boolean;
  created_at: string;
  updated_at: string;
}
