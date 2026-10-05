<script setup lang="ts">
import { computed } from "vue";
import { NModal } from "naive-ui";
import type { DeviceInfo, NewDevice } from "@/types";
import DeviceForm from "./DeviceForm.vue";

const props = defineProps<{ submit: (original: DeviceInfo, device: NewDevice) => Promise<void> }>();
const device = defineModel<DeviceInfo | null>({ required: true });

const open = computed({
  get: () => device.value !== null,
  set: (value: boolean) => {
    if (!value) device.value = null;
  },
});
const title = computed(() => `Edit ${device.value?.name ?? ""}`);

async function save(edited: NewDevice) {
  if (!device.value) return;
  await props.submit(device.value, edited);
  device.value = null;
}
</script>

<template>
  <n-modal v-model:show="open" preset="card" :title="title" :mask-closable="false" :style="{ width: 'min(600px, 92vw)' }">
    <DeviceForm v-if="device" :key="device.name" :device="device" :submit="save" @cancel="open = false" />
  </n-modal>
</template>

