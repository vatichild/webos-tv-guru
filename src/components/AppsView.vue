<script setup lang="ts">
import { computed, h, onMounted, onUnmounted, ref, watch } from "vue";
import {
  NButton,
  NCard,
  NDataTable,
  NEmpty,
  NInput,
  NProgress,
  NSpace,
  NSwitch,
  useDialog,
  useMessage,
  type DataTableColumns,
} from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { api, errorText, type InstalledApp, type InstallProgress } from "../api";
import { useDevices } from "../stores/devices";

const devices = useDevices();
const message = useMessage();
const dialog = useDialog();

const apps = ref<InstalledApp[]>([]);
const loading = ref(false);
const filter = ref("");
const showSystem = ref(false);
const install = ref<{ file: string; percent: number; status: string } | null>(null);
const dragging = ref(false);

const visibleApps = computed(() => {
  const q = filter.value.toLowerCase();
  return apps.value
    .filter((a) => showSystem.value || a.visible !== false)
    .filter((a) => !q || a.id.toLowerCase().includes(q) || a.title.toLowerCase().includes(q))
    .sort((a, b) => a.title.localeCompare(b.title));
});

async function refresh() {
  const device = devices.selected;
  apps.value = [];
  if (!device) return;
  loading.value = true;
  try {
    apps.value = await api.listApps(device);
  } catch (e) {
    message.error(errorText(e), { duration: 8000 });
  } finally {
    loading.value = false;
  }
}

async function installFiles(paths: string[]) {
  const device = devices.selected;
  if (!device) return message.warning("Select a TV first");
  for (const path of paths.filter((p) => p.toLowerCase().endsWith(".ipk"))) {
    const file = path.split(/[\\/]/).pop() ?? path;
    install.value = { file, percent: 0, status: "Uploading" };
    try {
      const id = await api.installApp(device, path, (p: InstallProgress) => {
        if (!install.value) return;
        if (p.stage === "uploading") install.value.percent = p.total ? Math.round((p.sent / p.total) * 100) : 0;
        else if (p.stage === "verifying") install.value.status = "Verifying";
        else if (p.stage === "installing") install.value.status = `Installing ${p.status}`;
      });
      message.success(`Installed ${id || file}`);
    } catch (e) {
      message.error(`${file}: ${errorText(e)}`, { duration: 10000 });
    }
  }
  install.value = null;
  await refresh();
}

async function pickAndInstall() {
  const picked = await open({ multiple: true, filters: [{ name: "webOS package", extensions: ["ipk"] }] });
  if (picked) await installFiles(Array.isArray(picked) ? picked : [picked]);
}

function confirmRemove(app: InstalledApp) {
  dialog.warning({
    title: "Uninstall app",
    content: `Uninstall ${app.title} (${app.id}) from ${devices.selected}?`,
    positiveText: "Uninstall",
    negativeText: "Cancel",
    onPositiveClick: async () => {
      try {
        await api.removeApp(devices.selected!, app.id);
        message.success(`Removed ${app.title}`);
        await refresh();
      } catch (e) {
        message.error(errorText(e), { duration: 8000 });
      }
    },
  });
}

async function launch(app: InstalledApp) {
  try {
    await api.launchApp(devices.selected!, app.id);
  } catch (e) {
    message.error(errorText(e));
  }
}

const columns: DataTableColumns<InstalledApp> = [
  { title: "Title", key: "title", ellipsis: { tooltip: true } },
  { title: "ID", key: "id", ellipsis: { tooltip: true } },
  { title: "Version", key: "version", width: 110 },
  {
    title: "",
    key: "actions",
    width: 190,
    render: (app) =>
      h(NSpace, { size: "small", wrap: false }, () => [
        h(NButton, { size: "small", onClick: () => launch(app) }, () => "Launch"),
        h(NButton, { size: "small", type: "error", ghost: true, onClick: () => confirmRemove(app) }, () => "Uninstall"),
      ]),
  },
];

let unlistenDrop: (() => void) | undefined;
onMounted(async () => {
  unlistenDrop = await getCurrentWebview().onDragDropEvent((event) => {
    const p = event.payload;
    if (p.type === "enter" || p.type === "over") dragging.value = true;
    else if (p.type === "leave") dragging.value = false;
    else if (p.type === "drop") {
      dragging.value = false;
      if (!install.value) installFiles(p.paths);
    }
  });
});
onUnmounted(() => unlistenDrop?.());
watch(() => devices.selected, refresh, { immediate: true });
</script>

<template>
  <n-space vertical size="large">
    <n-card size="small" :class="{ drop: true, active: dragging }">
      <n-space align="center" justify="space-between">
        <span v-if="install">
          <b>{{ install.file }}</b> · {{ install.status }}
        </span>
        <span v-else>Drop <code>.ipk</code> files anywhere in this window to install them on <b>{{ devices.selected ?? "…" }}</b></span>
        <n-button type="primary" :disabled="!devices.selected || !!install" @click="pickAndInstall">Install .ipk…</n-button>
      </n-space>
      <n-progress v-if="install" type="line" :percentage="install.percent" :processing="install.percent === 100" style="margin-top: 8px" />
    </n-card>

    <n-empty v-if="!devices.selected" description="Add a TV on the TVs tab first." />
    <n-card v-else size="small" title="Installed apps">
      <template #header-extra>
        <n-space align="center" size="small">
          <n-input v-model:value="filter" size="small" placeholder="Filter" clearable />
          <n-switch v-model:value="showSystem" size="small" />
          <span class="muted">hidden apps</span>
          <n-button size="small" :loading="loading" @click="refresh">Refresh</n-button>
        </n-space>
      </template>
      <n-data-table :columns="columns" :data="visibleApps" :loading="loading" :row-key="(a: InstalledApp) => a.id" size="small" max-height="calc(100vh - 330px)" />
    </n-card>
  </n-space>
</template>

<style scoped>
.drop {
  border-style: dashed;
}
.drop.active {
  border-color: #18a058;
}
.muted {
  font-size: 12px;
  opacity: 0.7;
}
</style>
