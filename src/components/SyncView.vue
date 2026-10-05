<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import {
  NAlert,
  NButton,
  NCard,
  NCheckbox,
  NCheckboxGroup,
  NEmpty,
  NInput,
  NSpace,
  NSwitch,
  NTag,
  NTooltip,
  useMessage,
} from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { api, errorText, SYNC_ENTRIES_EVENT, type SyncConfig, type SyncEntry, type SyncState } from "../api";
import { useDevices } from "../stores/devices";

const devices = useDevices();
const message = useMessage();

const config = ref<SyncConfig>({ folder: null, devices: [], autoInstall: false });
const entries = ref<SyncEntry[]>([]);

const files = computed(() => [...new Set(entries.value.map((e) => e.file))]);
const cell = (file: string, device: string) =>
  entries.value.find((e) => e.file === file && e.device === device);
const meta = (file: string) => entries.value.find((e) => e.file === file);
const pendingCount = computed(
  () => entries.value.filter((e) => e.state === "missing" || e.state === "outdated").length,
);
const busy = computed(() => entries.value.some((e) => e.state === "checking" || e.state === "installing"));

const STATE: Record<SyncState, { label: string; type: "default" | "success" | "info" | "warning" | "error" }> = {
  checking: { label: "Checking…", type: "default" },
  upToDate: { label: "Up to date", type: "success" },
  missing: { label: "Not installed", type: "warning" },
  outdated: { label: "Update", type: "info" },
  tvNewer: { label: "TV newer", type: "default" },
  installing: { label: "Installing", type: "info" },
  failed: { label: "Failed", type: "error" },
  offline: { label: "Offline", type: "error" },
  invalid: { label: "Bad file", type: "error" },
};

function label(e: SyncEntry) {
  const base = STATE[e.state].label;
  if (e.state === "installing" && e.progress != null) return `${base} ${e.progress}%`;
  if (e.state === "outdated" && e.installedVersion) return `Update from ${e.installedVersion}`;
  return base;
}

async function save(patch: Partial<SyncConfig>) {
  const next = { ...config.value, ...patch };
  try {
    await api.syncSetConfig(next);
    config.value = next;
  } catch (e) {
    message.error(errorText(e));
  }
}

async function pickFolder() {
  const folder = await open({ directory: true, multiple: false });
  if (typeof folder === "string") await save({ folder });
}

let unlisten: UnlistenFn | undefined;
onMounted(async () => {
  unlisten = await listen<SyncEntry[]>(SYNC_ENTRIES_EVENT, (e) => (entries.value = e.payload));
  config.value = await api.syncGetConfig();
  entries.value = await api.syncEntries();
});
onUnmounted(() => unlisten?.());
</script>

<template>
  <n-space vertical size="large">
    <n-card size="small" title="Sync settings">
      <n-space vertical>
        <n-space :wrap="false" align="center">
          <n-input :value="config.folder ?? ''" readonly placeholder="Choose a folder with .ipk files" style="min-width: 320px" />
          <n-button @click="pickFolder">Choose folder…</n-button>
        </n-space>
        <div>
          <div class="label">Sync to</div>
          <n-empty v-if="!devices.devices.length" size="small" description="Add a TV on the TVs tab first." />
          <n-checkbox-group v-else :value="config.devices" @update:value="(v) => save({ devices: v as string[] })">
            <n-space>
              <n-checkbox v-for="d in devices.devices" :key="d.name" :value="d.name" :label="d.name" />
            </n-space>
          </n-checkbox-group>
        </div>
        <n-space align="center">
          <n-switch :value="config.autoInstall" @update:value="(v: boolean) => save({ autoInstall: v })" />
          <span>Install new and updated apps automatically when files change</span>
        </n-space>
      </n-space>
    </n-card>

    <n-card size="small" title="Status">
      <template #header-extra>
        <n-space size="small">
          <n-button size="small" :disabled="busy" @click="api.syncNow(false)">Rescan</n-button>
          <n-button size="small" type="primary" :disabled="busy || !pendingCount" @click="api.syncNow(true)">
            Install {{ pendingCount || "" }} pending
          </n-button>
        </n-space>
      </template>
      <n-alert v-if="!config.folder || !config.devices.length" type="default" :bordered="false">
        Choose a folder and at least one TV to start syncing.
      </n-alert>
      <n-empty v-else-if="!files.length" description="No .ipk files in this folder yet." />
      <table v-else class="matrix">
        <thead>
          <tr>
            <th>Package</th>
            <th v-for="d in config.devices" :key="d">{{ d }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="f in files" :key="f">
            <td>
              <div class="file">{{ f }}</div>
              <div class="muted">{{ meta(f)?.package ?? "?" }} {{ meta(f)?.version ?? "" }}</div>
            </td>
            <td v-for="d in config.devices" :key="d">
              <template v-if="cell(f, d)">
                <n-tooltip :disabled="!cell(f, d)!.message">
                  <template #trigger>
                    <n-tag size="small" :type="STATE[cell(f, d)!.state].type">{{ label(cell(f, d)!) }}</n-tag>
                  </template>
                  {{ cell(f, d)!.message }}
                </n-tooltip>
              </template>
            </td>
          </tr>
        </tbody>
      </table>
    </n-card>
  </n-space>
</template>

<style scoped>
.label {
  font-size: 13px;
  margin-bottom: 6px;
  opacity: 0.8;
}
.matrix {
  width: 100%;
  border-collapse: collapse;
}
.matrix th,
.matrix td {
  text-align: left;
  padding: 6px 8px;
  border-bottom: 1px solid rgba(128, 128, 128, 0.2);
}
.file {
  font-weight: 500;
  word-break: break-all;
}
.muted {
  font-size: 12px;
  opacity: 0.65;
}
</style>
