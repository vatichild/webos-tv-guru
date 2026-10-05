import { computed, onMounted, onUnmounted, ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { SYNC_ENTRIES_EVENT, syncApi } from "@/lib/api";
import type { SyncConfig, SyncEntry, SyncRow } from "@/types";
import { useNotify } from "./useNotify";

export function useSync() {
  const notify = useNotify();
  const config = ref<SyncConfig>({ folder: null, devices: [], autoInstall: false });
  const entries = ref<SyncEntry[]>([]);

  const rows = computed<SyncRow[]>(() => {
    const byFile = new Map<string, SyncRow>();
    for (const entry of entries.value) {
      const row = byFile.get(entry.file) ?? {
        file: entry.file,
        package: entry.package,
        version: entry.version,
        cells: {},
      };
      row.cells[entry.device] = entry;
      byFile.set(entry.file, row);
    }
    return [...byFile.values()];
  });

  const pendingCount = computed(
    () => entries.value.filter((e) => e.state === "missing" || e.state === "outdated").length,
  );
  const busy = computed(() => entries.value.some((e) => e.state === "checking" || e.state === "installing"));
  const ready = computed(() => !!config.value.folder && config.value.devices.length > 0);

  async function update(patch: Partial<SyncConfig>) {
    const next = { ...config.value, ...patch };
    try {
      await syncApi.setConfig(next);
      config.value = next;
    } catch (cause) {
      notify.error(cause);
    }
  }

  const rescan = () => syncApi.run(false);
  const installPending = () => syncApi.run(true);

  let stop: UnlistenFn | undefined;
  onMounted(async () => {
    stop = await listen<SyncEntry[]>(SYNC_ENTRIES_EVENT, (event) => (entries.value = event.payload));
    config.value = await syncApi.getConfig();
    entries.value = await syncApi.entries();
  });
  onUnmounted(() => stop?.());

  return { config, rows, pendingCount, busy, ready, update, rescan, installPending };
}
