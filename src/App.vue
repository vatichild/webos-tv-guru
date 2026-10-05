<script setup lang="ts">
import { computed, onMounted } from "vue";
import {
  NConfigProvider,
  NDialogProvider,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NMessageProvider,
  NSelect,
  NTabPane,
  NTabs,
  darkTheme,
  useOsTheme,
} from "naive-ui";
import DevicesView from "./components/DevicesView.vue";
import AppsView from "./components/AppsView.vue";
import SyncView from "./components/SyncView.vue";
import { useDevices } from "./stores/devices";

const osTheme = useOsTheme();
const theme = computed(() => (osTheme.value === "dark" ? darkTheme : null));
const devices = useDevices();
const deviceOptions = computed(() =>
  devices.devices.map((d) => ({ label: `${d.name} (${d.host})`, value: d.name })),
);

onMounted(() => devices.refresh());
</script>

<template>
  <n-config-provider :theme="theme">
    <n-message-provider>
      <n-dialog-provider>
        <n-layout class="shell">
          <n-layout-header bordered class="header">
            <strong>WebOS TV Guru</strong>
            <n-select
              v-model:value="devices.selected"
              :options="deviceOptions"
              placeholder="No TV selected"
              size="small"
              class="device-select"
            />
          </n-layout-header>
          <n-layout-content content-style="padding: 0 16px 16px">
            <n-tabs type="line" default-value="sync" animated>
              <n-tab-pane name="sync" tab="Folder sync"><SyncView /></n-tab-pane>
              <n-tab-pane name="apps" tab="Apps"><AppsView /></n-tab-pane>
              <n-tab-pane name="devices" tab="TVs"><DevicesView /></n-tab-pane>
            </n-tabs>
          </n-layout-content>
        </n-layout>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
html,
body,
#app {
  margin: 0;
  height: 100%;
}
.shell {
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
