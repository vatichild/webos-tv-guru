import { computed, reactive } from "vue";
import type { DeviceInfo, DeviceKind, NewDevice } from "@/types";

/**
 * Form state for adding or editing a TV. When editing, blank secrets keep the saved ones,
 * so the form never needs to show a stored passphrase or password.
 */
export function useDeviceForm(device?: DeviceInfo) {
  const editing = !!device;
  const hasDevModeKey = !!device?.devMode;
  const rooted = device && !device.devMode ? device : undefined;

  function initial() {
    return {
      kind: (rooted ? "rooted" : "devMode") as DeviceKind,
      name: device?.name ?? "",
      host: device?.host ?? "",
      passphrase: "",
      port: rooted?.port ?? 22,
      username: rooted?.username ?? "root",
      password: "",
      keyPath: device?.keyPath ?? "",
      keyPassphrase: "",
    };
  }

  const form = reactive(initial());

  const canSubmit = computed(() => {
    if (!form.name.trim() || !form.host.trim()) return false;
    if (form.kind === "rooted") return true;
    return !!form.passphrase.trim() || hasDevModeKey;
  });

  const hints = computed(() => ({
    passphrase: hasDevModeKey ? "Leave empty to keep the current key" : "ABC123",
    password: device?.hasPassword ? "Leave empty to keep the current password" : "Leave empty to use a key",
    keyPassphrase: editing ? "Leave empty to keep the current one" : "",
  }));

  function toDevice(): NewDevice {
    const name = form.name.trim();
    const host = form.host.trim();
    if (form.kind === "devMode") {
      return { kind: "devMode", name, host, passphrase: form.passphrase.trim() };
    }
    return {
      kind: "rooted",
      name,
      host,
      port: form.port,
      username: form.username.trim(),
      password: form.password || undefined,
      keyPath: form.keyPath || undefined,
      keyPassphrase: form.keyPassphrase || undefined,
    };
  }

  function reset() {
    Object.assign(form, initial());
  }

  return { form, editing, hasDevModeKey, canSubmit, hints, toDevice, reset };
}
