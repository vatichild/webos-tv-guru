<script setup lang="ts">
import { NButton, NInput, NSpace, NSwitch } from "naive-ui";

defineProps<{ loading: boolean }>();
const emit = defineEmits<{ refresh: [] }>();
const filter = defineModel<string>("filter", { required: true });
const showHidden = defineModel<boolean>("showHidden", { required: true });
</script>

<template>
  <n-space align="center" size="small">
    <n-input v-model:value="filter" size="small" placeholder="Filter" clearable />
    <n-switch v-model:value="showHidden" size="small" />
    <span class="label">Hidden apps</span>
    <n-button size="small" :loading="loading" @click="emit('refresh')">Refresh</n-button>
  </n-space>
</template>

<style scoped>
.label {
  font-size: 12px;
  opacity: 0.7;
}
</style>
