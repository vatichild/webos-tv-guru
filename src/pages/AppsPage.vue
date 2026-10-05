<script setup lang="ts">
import { storeToRefs } from "pinia";
import { NCard, NEmpty, NSpace } from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";
import AppsToolbar from "@/components/apps/AppsToolbar.vue";
import InstallDropZone from "@/components/apps/InstallDropZone.vue";
import InstalledAppsTable from "@/components/apps/InstalledAppsTable.vue";
import { useConfirm } from "@/composables/useConfirm";
import { useFileDrop } from "@/composables/useFileDrop";
import { useInstalledApps } from "@/composables/useInstalledApps";
import { useInstaller } from "@/composables/useInstaller";
import { useNotify } from "@/composables/useNotify";
import { useDevices } from "@/stores/devices";
import type { InstalledApp } from "@/types";

const { selected } = storeToRefs(useDevices());
const notify = useNotify();
const confirm = useConfirm();
const installed = useInstalledApps(selected);
const { visibleApps, loading, filter, showHidden } = installed;
const installer = useInstaller();
const { task } = installer;

async function install(paths: string[]) {
  if (!selected.value) {
    notify.warning("Select a TV first");
    return;
  }
  if (task.value) return;
  await installer.install(selected.value, paths);
  await installed.refresh();
}

async function pick() {
  const picked = await open({ multiple: true, filters: [{ name: "webOS app", extensions: ["ipk"] }] });
  if (picked) await install(Array.isArray(picked) ? picked : [picked]);
}

async function uninstall(app: InstalledApp) {
  const ok = await confirm({
    title: "Uninstall app",
    content: `Uninstall ${app.title} (${app.id}) from ${selected.value}?`,
    confirmText: "Uninstall",
  });
  if (ok) await installed.remove(app);
}

const { dragging } = useFileDrop(install);
</script>

<template>
  <n-space vertical size="large">
    <InstallDropZone :device="selected" :task="task" :dragging="dragging" @pick="pick" />
    <n-empty v-if="!selected" description="Add a TV on the TVs tab first." />
    <n-card v-else size="small" title="Installed apps">
      <template #header-extra>
        <AppsToolbar
          v-model:filter="filter"
          v-model:show-hidden="showHidden"
          :loading="loading"
          @refresh="installed.refresh"
        />
      </template>
      <InstalledAppsTable :apps="visibleApps" :loading="loading" @launch="installed.launch" @remove="uninstall" />
    </n-card>
  </n-space>
</template>
