<script setup lang="ts">
import { NAlert, NCard, NEmpty, NSpace } from "naive-ui";
import SyncActions from "@/components/sync/SyncActions.vue";
import SyncMatrix from "@/components/sync/SyncMatrix.vue";
import SyncSettings from "@/components/sync/SyncSettings.vue";
import { useSync } from "@/composables/useSync";
import { useDevices } from "@/stores/devices";

const devices = useDevices();
const { config, rows, pendingCount, busy, ready, update, rescan, installPending } = useSync();
</script>

<template>
  <n-space vertical size="large">
    <SyncSettings :config="config" :devices="devices.names" @update="update" />
    <n-card size="small" title="Status">
      <template #header-extra>
        <SyncActions :busy="busy" :pending-count="pendingCount" @rescan="rescan" @install-pending="installPending" />
      </template>
      <n-alert v-if="!ready" type="default" :bordered="false">
        Choose a folder and at least one TV to start syncing.
      </n-alert>
      <n-empty v-else-if="!rows.length" description="There are no .ipk files in this folder yet." />
      <SyncMatrix v-else :rows="rows" :devices="config.devices" />
    </n-card>
  </n-space>
</template>
