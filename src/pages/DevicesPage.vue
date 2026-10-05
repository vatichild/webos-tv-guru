<script setup lang="ts">
import { ref } from "vue";
import { NCard, NSpace } from "naive-ui";
import DeviceEditModal from "@/components/devices/DeviceEditModal.vue";
import DeviceForm from "@/components/devices/DeviceForm.vue";
import DeviceList from "@/components/devices/DeviceList.vue";
import { useDeviceActions } from "@/composables/useDeviceActions";
import { useDevices } from "@/stores/devices";
import type { DeviceInfo } from "@/types";

const devices = useDevices();
const actions = useDeviceActions();
const { testing } = actions;
const editing = ref<DeviceInfo | null>(null);
</script>

<template>
  <n-space vertical size="large">
    <n-card title="TVs" size="small">
      <DeviceList
        :devices="devices.devices"
        :testing="testing"
        @test="actions.test"
        @edit="editing = $event"
        @make-default="actions.makeDefault"
        @remove="actions.remove"
      />
      <p class="hint">Saved in the webOS SDK device list (<code>%APPDATA%\.webos\ose\novacom-devices.json</code>).</p>
    </n-card>
    <n-card title="Add a TV" size="small">
      <DeviceForm :submit="actions.add" />
    </n-card>
    <DeviceEditModal v-model="editing" :submit="actions.update" />
  </n-space>
</template>

<style scoped>
.hint {
  font-size: 12px;
  opacity: 0.7;
  margin: 8px 0 0;
}
</style>
