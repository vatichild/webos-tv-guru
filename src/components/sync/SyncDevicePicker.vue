<script setup lang="ts">
import { NCheckbox, NCheckboxGroup, NEmpty, NSpace } from "naive-ui";

defineProps<{ devices: string[] }>();
const selected = defineModel<string[]>({ required: true });
</script>

<template>
  <n-empty v-if="!devices.length" size="small" description="Add a TV on the TVs tab first." />
  <n-checkbox-group v-else v-model:value="selected">
    <n-space>
      <n-checkbox v-for="device in devices" :key="device" :value="device" :label="device" />
    </n-space>
  </n-checkbox-group>
</template>
