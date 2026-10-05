import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { api, type DeviceInfo, type NewDevice } from "../api";

export const useDevices = defineStore("devices", () => {
  const devices = ref<DeviceInfo[]>([]);
  const selected = ref<string | null>(null);
  const loading = ref(false);

  const current = computed(() => devices.value.find((d) => d.name === selected.value) ?? null);

  async function refresh() {
    loading.value = true;
    try {
      devices.value = await api.listDevices();
      if (!devices.value.some((d) => d.name === selected.value)) {
        selected.value = (devices.value.find((d) => d.default) ?? devices.value[0])?.name ?? null;
      }
    } finally {
      loading.value = false;
    }
  }

  async function add(device: NewDevice) {
    const added = await api.addDevice(device);
    await refresh();
    selected.value = added.name;
  }

  async function update(original: string, device: NewDevice) {
    const updated = await api.updateDevice(original, device);
    if (selected.value === original) selected.value = updated.name;
    await refresh();
  }

  async function remove(name: string) {
    await api.removeDevice(name);
    await refresh();
  }

  async function makeDefault(name: string) {
    await api.setDefaultDevice(name);
    await refresh();
  }

  return { devices, selected, current, loading, refresh, add, update, remove, makeDefault };
});
