import { Channel, invoke } from "@tauri-apps/api/core";

export interface DeviceInfo {
  name: string;
  host: string;
  port: number;
  username: string;
  default: boolean;
  devMode: boolean;
  keyPath: string | null;
  hasPassword: boolean;
}

export type NewDevice =
  | { kind: "devMode"; name: string; host: string; passphrase: string }
  | {
      kind: "rooted";
      name: string;
      host: string;
      port: number;
      username: string;
      password?: string;
      keyPath?: string;
      keyPassphrase?: string;
    };

export interface InstalledApp {
  id: string;
  version: string;
  title: string;
  appType: string;
  vendor?: string;
  visible?: boolean;
}

export interface IpkInfo {
  package: string;
  version: string;
  architecture?: string;
}

export type InstallProgress =
  | { stage: "uploading"; sent: number; total: number }
  | { stage: "verifying" }
  | { stage: "installing"; status: string }
  | { stage: "done"; packageId: string };

export interface SyncConfig {
  folder: string | null;
  devices: string[];
  autoInstall: boolean;
}

export type SyncState =
  | "checking"
  | "upToDate"
  | "missing"
  | "outdated"
  | "tvNewer"
  | "installing"
  | "failed"
  | "offline"
  | "invalid";

export interface SyncEntry {
  file: string;
  device: string;
  package: string | null;
  version: string | null;
  installedVersion: string | null;
  state: SyncState;
  progress: number | null;
  message: string | null;
}

export const SYNC_ENTRIES_EVENT = "sync://entries";

export const api = {
  listDevices: () => invoke<DeviceInfo[]>("list_devices"),
  addDevice: (device: NewDevice) => invoke<DeviceInfo>("add_device", { device }),
  updateDevice: (original: string, device: NewDevice) =>
    invoke<DeviceInfo>("update_device", { original, device }),
  removeDevice: (name: string) => invoke<void>("remove_device", { name }),
  setDefaultDevice: (name: string) => invoke<void>("set_default_device", { name }),
  testDevice: (name: string) => invoke<number>("test_device", { name }),

  listApps: (device: string) => invoke<InstalledApp[]>("list_apps", { device }),
  installApp: (device: string, path: string, onProgress: (p: InstallProgress) => void) => {
    const channel = new Channel<InstallProgress>();
    channel.onmessage = onProgress;
    return invoke<string>("install_app", { device, path, onProgress: channel });
  },
  removeApp: (device: string, id: string) => invoke<void>("remove_app", { device, id }),
  launchApp: (device: string, id: string) => invoke<void>("launch_app", { device, id }),
  readIpk: (path: string) => invoke<IpkInfo>("read_ipk", { path }),

  syncGetConfig: () => invoke<SyncConfig>("sync_get_config"),
  syncSetConfig: (config: SyncConfig) => invoke<void>("sync_set_config", { config }),
  syncEntries: () => invoke<SyncEntry[]>("sync_entries"),
  syncNow: (install: boolean) => invoke<void>("sync_now", { install }),
};

/** Tauri rejects with the Rust error's message as a string. */
export function errorText(e: unknown): string {
  return typeof e === "string" ? e : e instanceof Error ? e.message : JSON.stringify(e);
}
