<script setup lang="ts">
import { NEmpty, NList } from "naive-ui";
import type { DeviceInfo } from "@/types";
import DeviceListItem from "./DeviceListItem.vue";

defineProps<{ devices: DeviceInfo[]; testing: string | null }>();
const emit = defineEmits<{
  test: [name: string];
  edit: [device: DeviceInfo];
  makeDefault: [name: string];
  remove: [name: string];
}>();
</script>

<template>
  <n-empty v-if="!devices.length" description="No TVs yet. Add one below." />
  <n-list v-else hoverable>
    <DeviceListItem
      v-for="device in devices"
      :key="device.name"
      :device="device"
      :testing="testing === device.name"
      @test="emit('test', $event)"
      @edit="emit('edit', $event)"
      @make-default="emit('makeDefault', $event)"
      @remove="emit('remove', $event)"
    />
  </n-list>
</template>
