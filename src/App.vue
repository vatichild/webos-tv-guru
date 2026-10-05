<script setup lang="ts">
import { computed, onMounted } from "vue";
import {
  NConfigProvider,
  NDialogProvider,
  NMessageProvider,
  NTabPane,
  NTabs,
  darkTheme,
  useOsTheme,
} from "naive-ui";
import AppLayout from "@/layouts/AppLayout.vue";
import AppsPage from "@/pages/AppsPage.vue";
import DevicesPage from "@/pages/DevicesPage.vue";
import SyncPage from "@/pages/SyncPage.vue";
import { useDevices } from "@/stores/devices";

const osTheme = useOsTheme();
const theme = computed(() => (osTheme.value === "dark" ? darkTheme : null));
const devices = useDevices();

onMounted(() => devices.refresh());
</script>

<template>
  <n-config-provider :theme="theme">
    <n-message-provider>
      <n-dialog-provider>
        <AppLayout>
          <n-tabs type="line" default-value="sync" animated>
            <n-tab-pane name="sync" tab="Folder sync"><SyncPage /></n-tab-pane>
            <n-tab-pane name="apps" tab="Apps"><AppsPage /></n-tab-pane>
            <n-tab-pane name="devices" tab="TVs"><DevicesPage /></n-tab-pane>
          </n-tabs>
        </AppLayout>
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
</style>
