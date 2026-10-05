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

export interface DevModeDevice {
  kind: "devMode";
  name: string;
  host: string;
  passphrase: string;
}

export interface RootedDevice {
  kind: "rooted";
  name: string;
  host: string;
  port: number;
  username: string;
  password?: string;
  keyPath?: string;
  keyPassphrase?: string;
}

export type NewDevice = DevModeDevice | RootedDevice;

export type DeviceKind = NewDevice["kind"];

export interface InstalledApp {
  id: string;
  version: string;
  title: string;
  appType: string;
  vendor?: string;
  visible?: boolean;
}

export type InstallProgress =
  | { stage: "uploading"; sent: number; total: number }
  | { stage: "verifying" }
  | { stage: "installing"; status: string }
  | { stage: "done"; packageId: string };

export interface InstallTask {
  file: string;
  percent: number;
  status: string;
}

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

export interface SyncRow {
  file: string;
  package: string | null;
  version: string | null;
  cells: Record<string, SyncEntry | undefined>;
}
