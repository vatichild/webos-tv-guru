import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { devicesApi } from "@/lib/api";
import type { DeviceInfo, NewDevice } from "@/types";

export const useDevices = defineStore("devices", () => {
  const devices = ref<DeviceInfo[]>([]);
  const selected = ref<string | null>(null);
  const loading = ref(false);

  const current = computed(() => devices.value.find((d) => d.name === selected.value) ?? null);
  const names = computed(() => devices.value.map((d) => d.name));

  function keepSelectionValid() {
    if (devices.value.some((d) => d.name === selected.value)) return;
    const fallback = devices.value.find((d) => d.default) ?? devices.value[0];
    selected.value = fallback?.name ?? null;
  }

  async function refresh() {
    loading.value = true;
    try {
      devices.value = await devicesApi.list();
      keepSelectionValid();
    } finally {
      loading.value = false;
    }
  }

  async function add(device: NewDevice) {
    const added = await devicesApi.add(device);
    await refresh();
    selected.value = added.name;
  }

  async function update(original: string, device: NewDevice) {
    const updated = await devicesApi.update(original, device);
    if (selected.value === original) selected.value = updated.name;
    await refresh();
  }

  async function remove(name: string) {
    await devicesApi.remove(name);
    await refresh();
  }

  async function makeDefault(name: string) {
    await devicesApi.setDefault(name);
    await refresh();
  }

  return { devices, selected, current, names, loading, refresh, add, update, remove, makeDefault };
});
