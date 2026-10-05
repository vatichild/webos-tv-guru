import { Channel, invoke } from "@tauri-apps/api/core";
import type {
  DeviceInfo,
  InstalledApp,
  InstallProgress,
  NewDevice,
  SyncConfig,
  SyncEntry,
} from "@/types";

export const SYNC_ENTRIES_EVENT = "sync://entries";

export const devicesApi = {
  list: () => invoke<DeviceInfo[]>("list_devices"),
  add: (device: NewDevice) => invoke<DeviceInfo>("add_device", { device }),
  update: (original: string, device: NewDevice) =>
    invoke<DeviceInfo>("update_device", { original, device }),
  remove: (name: string) => invoke<void>("remove_device", { name }),
  setDefault: (name: string) => invoke<void>("set_default_device", { name }),
  test: (name: string) => invoke<number>("test_device", { name }),
};

export const appsApi = {
  list: (device: string) => invoke<InstalledApp[]>("list_apps", { device }),
  install: (device: string, path: string, onProgress: (progress: InstallProgress) => void) => {
    const channel = new Channel<InstallProgress>();
    channel.onmessage = onProgress;
    return invoke<string>("install_app", { device, path, onProgress: channel });
  },
  remove: (device: string, id: string) => invoke<void>("remove_app", { device, id }),
  launch: (device: string, id: string) => invoke<void>("launch_app", { device, id }),
};

export const syncApi = {
  getConfig: () => invoke<SyncConfig>("sync_get_config"),
  setConfig: (config: SyncConfig) => invoke<void>("sync_set_config", { config }),
  entries: () => invoke<SyncEntry[]>("sync_entries"),
  run: (install: boolean) => invoke<void>("sync_now", { install }),
};
