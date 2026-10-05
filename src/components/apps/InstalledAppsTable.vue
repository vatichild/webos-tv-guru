<script setup lang="ts">
import { h } from "vue";
import { NButton, NDataTable, NSpace, type DataTableColumns } from "naive-ui";
import type { InstalledApp } from "@/types";

defineProps<{ apps: InstalledApp[]; loading: boolean }>();
const emit = defineEmits<{ launch: [app: InstalledApp]; remove: [app: InstalledApp] }>();

const rowKey = (app: InstalledApp) => app.id;

function actions(app: InstalledApp) {
  return h(NSpace, { size: "small", wrap: false }, () => [
    h(NButton, { size: "small", onClick: () => emit("launch", app) }, () => "Launch"),
    h(NButton, { size: "small", type: "error", ghost: true, onClick: () => emit("remove", app) }, () => "Uninstall"),
  ]);
}

const columns: DataTableColumns<InstalledApp> = [
  { title: "Title", key: "title", ellipsis: { tooltip: true } },
  { title: "ID", key: "id", ellipsis: { tooltip: true } },
  { title: "Version", key: "version", width: 110 },
  { title: "", key: "actions", width: 190, render: actions },
];
</script>

<template>
  <n-data-table
    :columns="columns"
    :data="apps"
    :loading="loading"
    :row-key="rowKey"
    size="small"
    max-height="calc(100vh - 330px)"
  />
</template>
