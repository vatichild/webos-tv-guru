<script setup lang="ts">
import { computed } from "vue";
import { NCard, NSpace, NSwitch } from "naive-ui";
import PathPicker from "@/components/common/PathPicker.vue";
import type { SyncConfig } from "@/types";
import SyncDevicePicker from "./SyncDevicePicker.vue";

const props = defineProps<{ config: SyncConfig; devices: string[] }>();
const emit = defineEmits<{ update: [patch: Partial<SyncConfig>] }>();

const folder = computed({
  get: () => props.config.folder ?? "",
  set: (value: string) => emit("update", { folder: value || null }),
});
const targets = computed({
  get: () => props.config.devices,
  set: (value: string[]) => emit("update", { devices: value }),
});
const autoInstall = computed({
  get: () => props.config.autoInstall,
  set: (value: boolean) => emit("update", { autoInstall: value }),
});
</script>

<template>
  <n-card size="small" title="Sync settings">
    <n-space vertical>
      <PathPicker v-model="folder" directory readonly placeholder="Choose a folder with .ipk files" />
      <div>
        <div class="label">Sync to</div>
        <SyncDevicePicker v-model="targets" :devices="devices" />
      </div>
      <n-space align="center">
        <n-switch v-model:value="autoInstall" />
        <span>Install new and updated apps by itself when files change</span>
      </n-space>
    </n-space>
  </n-card>
</template>

<style scoped>
.label {
  font-size: 13px;
  margin-bottom: 6px;
  opacity: 0.8;
}
</style>
