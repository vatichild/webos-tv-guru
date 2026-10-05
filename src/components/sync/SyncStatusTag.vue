<script setup lang="ts">
import { computed } from "vue";
import { NTag, NTooltip } from "naive-ui";
import type { SyncEntry, SyncState } from "@/types";

type TagType = "default" | "success" | "info" | "warning" | "error";

const STATES: Record<SyncState, { label: string; type: TagType }> = {
  checking: { label: "Checking", type: "default" },
  upToDate: { label: "Up to date", type: "success" },
  missing: { label: "Not installed", type: "warning" },
  outdated: { label: "Update", type: "info" },
  tvNewer: { label: "TV is newer", type: "default" },
  installing: { label: "Installing", type: "info" },
  failed: { label: "Failed", type: "error" },
  offline: { label: "Offline", type: "error" },
  invalid: { label: "Bad file", type: "error" },
};

const props = defineProps<{ entry: SyncEntry }>();

const type = computed(() => STATES[props.entry.state].type);
const label = computed(() => {
  const { state, progress, installedVersion } = props.entry;
  if (state === "installing" && progress !== null) return `Installing ${progress}%`;
  if (state === "outdated" && installedVersion) return `Update from ${installedVersion}`;
  return STATES[state].label;
});
</script>

<template>
  <n-tooltip :disabled="!entry.message">
    <template #trigger>
      <n-tag size="small" :type="type">{{ label }}</n-tag>
    </template>
    {{ entry.message }}
  </n-tooltip>
</template>
