<script setup lang="ts">
import type { SyncRow } from "@/types";
import SyncStatusTag from "./SyncStatusTag.vue";

defineProps<{ rows: SyncRow[]; devices: string[] }>();
</script>

<template>
  <table class="matrix">
    <thead>
      <tr>
        <th>Package</th>
        <th v-for="device in devices" :key="device">{{ device }}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="row in rows" :key="row.file">
        <td>
          <div class="file">{{ row.file }}</div>
          <div class="meta">{{ row.package ?? "?" }} {{ row.version ?? "" }}</div>
        </td>
        <td v-for="device in devices" :key="device">
          <SyncStatusTag v-if="row.cells[device]" :entry="row.cells[device]" />
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
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
.meta {
  font-size: 12px;
  opacity: 0.65;
}
</style>
