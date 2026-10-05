import { ref } from "vue";
import { devicesApi } from "@/lib/api";
import { useDevices } from "@/stores/devices";
import type { DeviceInfo, NewDevice } from "@/types";
import { useConfirm } from "./useConfirm";
import { useNotify } from "./useNotify";

export function useDeviceActions() {
  const devices = useDevices();
  const notify = useNotify();
  const confirm = useConfirm();
  const testing = ref<string | null>(null);

  async function add(device: NewDevice) {
    try {
      await devices.add(device);
      notify.success(`Added ${device.name}`);
    } catch (cause) {
      notify.error(cause);
      throw cause;
    }
  }

  async function update(original: DeviceInfo, device: NewDevice) {
    try {
      await devices.update(original.name, device);
      notify.success(`Saved ${device.name}`);
    } catch (cause) {
      notify.error(cause);
      throw cause;
    }
  }

  async function test(name: string) {
    testing.value = name;
    try {
      const count = await devicesApi.test(name);
      notify.success(`${name}: connected, ${count} apps installed`);
    } catch (cause) {
      notify.error(cause, name);
    } finally {
      testing.value = null;
    }
  }

  async function makeDefault(name: string) {
    try {
      await devices.makeDefault(name);
    } catch (cause) {
      notify.error(cause);
    }
  }

  async function remove(name: string) {
    const ok = await confirm({
      title: "Remove TV",
      content: `Remove ${name} from the list? The key that was fetched from it is deleted too.`,
      confirmText: "Remove",
    });
    if (!ok) return;
    try {
      await devices.remove(name);
    } catch (cause) {
      notify.error(cause);
    }
  }

  return { testing, add, update, test, makeDefault, remove };
}
