<script setup lang="ts">
import { NButton, NCard, NProgress, NSpace } from "naive-ui";
import type { InstallTask } from "@/types";

defineProps<{ device: string | null; task: InstallTask | null; dragging: boolean }>();
const emit = defineEmits<{ pick: [] }>();
</script>

<template>
  <n-card size="small" class="drop-zone" :class="{ active: dragging }">
    <n-space align="center" justify="space-between">
      <span v-if="task"><b>{{ task.file }}</b>: {{ task.status }}</span>
      <span v-else-if="device">Drop <code>.ipk</code> files anywhere in this window to install them on <b>{{ device }}</b></span>
      <span v-else>Select a TV to install apps</span>
      <n-button type="primary" :disabled="!device || !!task" @click="emit('pick')">Install .ipk</n-button>
    </n-space>
    <n-progress
      v-if="task"
      type="line"
      :percentage="task.percent"
      :processing="task.percent === 100"
      class="progress"
    />
  </n-card>
</template>

<style scoped>
.drop-zone {
  border-style: dashed;
}
.drop-zone.active {
  border-color: #18a058;
}
.progress {
  margin-top: 8px;
}
</style>
