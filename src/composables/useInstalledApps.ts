import { computed, ref, watch, type Ref } from "vue";
import { appsApi } from "@/lib/api";
import type { InstalledApp } from "@/types";
import { useNotify } from "./useNotify";

export function useInstalledApps(device: Ref<string | null>) {
  const notify = useNotify();
  const apps = ref<InstalledApp[]>([]);
  const loading = ref(false);
  const filter = ref("");
  const showHidden = ref(false);

  const visibleApps = computed(() => {
    const query = filter.value.toLowerCase();
    return apps.value
      .filter((app) => showHidden.value || app.visible !== false)
      .filter((app) => !query || app.id.toLowerCase().includes(query) || app.title.toLowerCase().includes(query))
      .sort((a, b) => a.title.localeCompare(b.title));
  });

  async function refresh() {
    apps.value = [];
    if (!device.value) return;
    loading.value = true;
    try {
      apps.value = await appsApi.list(device.value);
    } catch (cause) {
      notify.error(cause);
    } finally {
      loading.value = false;
    }
  }

  async function remove(app: InstalledApp) {
    if (!device.value) return;
    try {
      await appsApi.remove(device.value, app.id);
      notify.success(`Removed ${app.title}`);
      await refresh();
    } catch (cause) {
      notify.error(cause);
    }
  }

  async function launch(app: InstalledApp) {
    if (!device.value) return;
    try {
      await appsApi.launch(device.value, app.id);
    } catch (cause) {
      notify.error(cause);
    }
  }

  watch(device, refresh, { immediate: true });

  return { apps, visibleApps, loading, filter, showHidden, refresh, remove, launch };
}
