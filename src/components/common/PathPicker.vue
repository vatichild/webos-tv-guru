<script setup lang="ts">
import { NButton, NInput, NSpace } from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";

const props = withDefaults(
  defineProps<{
    directory?: boolean;
    placeholder?: string;
    extensions?: string[];
    clearable?: boolean;
    readonly?: boolean;
  }>(),
  { directory: false, placeholder: "", extensions: undefined, clearable: false, readonly: false },
);
const path = defineModel<string>({ default: "" });

async function browse() {
  const picked = await open({
    directory: props.directory,
    multiple: false,
    filters: props.extensions ? [{ name: "Files", extensions: props.extensions }] : undefined,
  });
  if (typeof picked === "string") path.value = picked;
}
</script>

<template>
  <n-space :wrap="false" class="path-picker">
    <n-input v-model:value="path" :placeholder="placeholder" :clearable="clearable" :readonly="readonly" />
    <n-button @click="browse">Browse</n-button>
  </n-space>
</template>

<style scoped>
.path-picker {
  width: 100%;
}
.path-picker :deep(.n-input) {
  min-width: 280px;
}
</style>
