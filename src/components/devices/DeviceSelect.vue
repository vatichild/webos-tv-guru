<script setup lang="ts">
import { computed } from "vue";
import { NSelect } from "naive-ui";
import type { DeviceInfo } from "@/types";

const props = defineProps<{ devices: DeviceInfo[] }>();
const selected = defineModel<string | null>({ required: true });

const options = computed(() =>
  props.devices.map((device) => ({ label: `${device.name} (${device.host})`, value: device.name })),
);
</script>

<template>
  <n-select v-model:value="selected" :options="options" placeholder="No TV selected" size="small" />
</template>
