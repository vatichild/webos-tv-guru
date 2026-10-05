<script setup lang="ts">
import { ref } from "vue";
import {
  NButton,
  NCard,
  NEmpty,
  NList,
  NListItem,
  NModal,
  NSpace,
  NTag,
  NThing,
  useDialog,
  useMessage,
} from "naive-ui";
import { api, errorText, type DeviceInfo, type NewDevice } from "../api";
import { useDevices } from "../stores/devices";
import DeviceForm from "./DeviceForm.vue";

const devices = useDevices();
const message = useMessage();
const dialog = useDialog();

const testing = ref<string | null>(null);
const editing = ref<DeviceInfo | null>(null);

async function add(device: NewDevice) {
  try {
    await devices.add(device);
    message.success(`Added ${device.name}`);
  } catch (e) {
    message.error(errorText(e), { duration: 8000 });
    throw e;
  }
}

async function save(device: NewDevice) {
  if (!editing.value) return;
  try {
    await devices.update(editing.value.name, device);
    message.success(`Saved ${device.name}`);
    editing.value = null;
  } catch (e) {
    message.error(errorText(e), { duration: 8000 });
  }
}

async function test(name: string) {
  testing.value = name;
  try {
    const count = await api.testDevice(name);
    message.success(`${name}: connected, ${count} apps installed`);
  } catch (e) {
    message.error(`${name}: ${errorText(e)}`, { duration: 8000 });
  } finally {
    testing.value = null;
  }
}

function remove(name: string) {
  dialog.warning({
    title: "Remove TV",
    content: `Remove ${name} from the device list? Its fetched key is deleted too.`,
    positiveText: "Remove",
    negativeText: "Cancel",
    onPositiveClick: async () => {
      try {
        await devices.remove(name);
      } catch (e) {
        message.error(errorText(e));
      }
    },
  });
}
</script>

<template>
  <n-space vertical size="large">
    <n-card title="TVs" size="small">
      <n-empty v-if="!devices.devices.length" description="No TVs yet. Add one below." />
      <n-list v-else hoverable>
        <n-list-item v-for="d in devices.devices" :key="d.name">
          <n-thing :title="d.name" :description="`${d.username}@${d.host}:${d.port}`">
            <template #header-extra>
              <n-space size="small">
                <n-tag size="small" :type="d.devMode ? 'info' : 'warning'">
                  {{ d.devMode ? "Dev Mode" : "Rooted / SSH" }}
                </n-tag>
                <n-tag v-if="d.default" size="small" type="success">Default</n-tag>
              </n-space>
            </template>
          </n-thing>
          <template #suffix>
            <n-space size="small" :wrap="false">
              <n-button size="small" :loading="testing === d.name" @click="test(d.name)">Test</n-button>
              <n-button size="small" @click="editing = d">Edit</n-button>
              <n-button v-if="!d.default" size="small" @click="devices.makeDefault(d.name)">Make default</n-button>
              <n-button size="small" type="error" ghost @click="remove(d.name)">Remove</n-button>
            </n-space>
          </template>
        </n-list-item>
      </n-list>
      <p class="hint">Stored in the webOS SDK device list (<code>%APPDATA%\.webos\ose\novacom-devices.json</code>).</p>
    </n-card>

    <n-card title="Add a TV" size="small">
      <DeviceForm :submit="add" />
    </n-card>

    <n-modal
      :show="!!editing"
      preset="card"
      :title="`Edit ${editing?.name ?? ''}`"
      style="width: min(600px, 92vw)"
      :mask-closable="false"
      @update:show="(v: boolean) => !v && (editing = null)"
    >
      <DeviceForm v-if="editing" :key="editing.name" :device="editing" :submit="save" @cancel="editing = null" />
    </n-modal>
  </n-space>
</template>

<style scoped>
.hint {
  font-size: 12px;
  opacity: 0.7;
  margin: 8px 0 0;
}
</style>
