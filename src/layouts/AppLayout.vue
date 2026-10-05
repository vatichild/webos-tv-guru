<script setup lang="ts">
import { storeToRefs } from "pinia";
import { NLayout, NLayoutContent, NLayoutHeader } from "naive-ui";
import DeviceSelect from "@/components/devices/DeviceSelect.vue";
import { useDevices } from "@/stores/devices";

const store = useDevices();
const { devices, selected } = storeToRefs(store);
</script>

<template>
  <n-layout class="layout">
    <n-layout-header bordered class="header">
      <strong>WebOS TV Guru</strong>
      <DeviceSelect v-model="selected" :devices="devices" class="device-select" />
    </n-layout-header>
    <n-layout-content content-style="padding: 0 16px 16px">
      <slot />
    </n-layout-content>
  </n-layout>
</template>

<style scoped>
.layout {
  height: 100vh;
}
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 16px;
}
.device-select {
  max-width: 280px;
}
</style>
