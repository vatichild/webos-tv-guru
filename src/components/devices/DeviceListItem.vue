<script setup lang="ts">
import { computed } from "vue";
import { NButton, NListItem, NSpace, NTag, NThing } from "naive-ui";
import type { DeviceInfo } from "@/types";
import DeviceTypeTag from "./DeviceTypeTag.vue";

const props = defineProps<{ device: DeviceInfo; testing: boolean }>();
const emit = defineEmits<{
  test: [name: string];
  edit: [device: DeviceInfo];
  makeDefault: [name: string];
  remove: [name: string];
}>();

const address = computed(() => `${props.device.username}@${props.device.host}:${props.device.port}`);
</script>

<template>
  <n-list-item>
    <n-thing :title="device.name" :description="address">
      <template #header-extra>
        <n-space size="small">
          <DeviceTypeTag :dev-mode="device.devMode" />
          <n-tag v-if="device.default" size="small" type="success">Default</n-tag>
        </n-space>
      </template>
    </n-thing>
    <template #suffix>
      <n-space size="small" :wrap="false">
        <n-button size="small" :loading="testing" @click="emit('test', device.name)">Test</n-button>
        <n-button size="small" @click="emit('edit', device)">Edit</n-button>
        <n-button v-if="!device.default" size="small" @click="emit('makeDefault', device.name)">Make default</n-button>
        <n-button size="small" type="error" ghost @click="emit('remove', device.name)">Remove</n-button>
      </n-space>
    </template>
  </n-list-item>
</template>
