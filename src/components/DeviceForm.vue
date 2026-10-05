<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import {
  NAlert,
  NButton,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NRadioButton,
  NRadioGroup,
  NSpace,
} from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";
import type { DeviceInfo, NewDevice } from "../api";

const props = defineProps<{
  device?: DeviceInfo;
  submit: (device: NewDevice) => Promise<void>;
}>();
const emit = defineEmits<{ cancel: [] }>();

const editing = computed(() => !!props.device);
const wasDevMode = props.device?.devMode ?? false;

function initial() {
  const d = props.device;
  return {
    kind: (d && !d.devMode ? "rooted" : "devMode") as "devMode" | "rooted",
    name: d?.name ?? "",
    host: d?.host ?? "",
    passphrase: "",
    port: d && !d.devMode ? d.port : 22,
    username: d && !d.devMode ? d.username : "root",
    password: "",
    keyPath: d?.keyPath ?? "",
    keyPassphrase: "",
  };
}
const form = reactive(initial());
const busy = ref(false);

const passphraseHint = computed(() =>
  editing.value && wasDevMode ? "Leave empty to keep the current key" : "ABC123",
);
const passwordHint = computed(() =>
  editing.value && props.device?.hasPassword ? "Leave empty to keep the current password" : "Leave empty to use a key",
);
const canSubmit = computed(
  () =>
    !!form.name &&
    !!form.host &&
    (form.kind === "rooted" || !!form.passphrase.trim() || (editing.value && wasDevMode)),
);

async function pickKey() {
  const path = await open({ multiple: false, directory: false });
  if (typeof path === "string") form.keyPath = path;
}

async function onSubmit() {
  const device: NewDevice =
    form.kind === "devMode"
      ? { kind: "devMode", name: form.name.trim(), host: form.host.trim(), passphrase: form.passphrase.trim() }
      : {
          kind: "rooted",
          name: form.name.trim(),
          host: form.host.trim(),
          port: form.port,
          username: form.username.trim(),
          password: form.password || undefined,
          keyPath: form.keyPath || undefined,
          keyPassphrase: form.keyPassphrase || undefined,
        };
  busy.value = true;
  try {
    await props.submit(device);
    if (!editing.value) Object.assign(form, initial());
  } catch {
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <n-form label-placement="left" label-width="120" @submit.prevent="onSubmit">
    <n-form-item label="Type">
      <n-radio-group v-model:value="form.kind">
        <n-radio-button value="devMode">Developer Mode</n-radio-button>
        <n-radio-button value="rooted">Rooted</n-radio-button>
      </n-radio-group>
    </n-form-item>
    <n-alert v-if="form.kind === 'devMode' && !(editing && wasDevMode)" type="info" :bordered="false" class="tip">
      On the TV open the Developer Mode app, turn on <b>Dev Mode Status</b> and <b>Key Server</b>,
      then enter the TV's IP and the passphrase shown in the app.
    </n-alert>
    <n-alert v-else-if="form.kind === 'devMode'" type="default" :bordered="false" class="tip">
      Enter the passphrase only if the TV's key changed (for example after reinstalling the Developer Mode app).
      Key Server must be on to fetch it again.
    </n-alert>
    <n-form-item label="Name">
      <n-input v-model:value="form.name" placeholder="living-room" />
    </n-form-item>
    <n-form-item label="IP address">
      <n-input v-model:value="form.host" placeholder="192.168.1.50" />
    </n-form-item>
    <template v-if="form.kind === 'devMode'">
      <n-form-item label="Passphrase">
        <n-input v-model:value="form.passphrase" :placeholder="passphraseHint" />
      </n-form-item>
    </template>
    <template v-else>
      <n-form-item label="Port">
        <n-input-number v-model:value="form.port" :min="1" :max="65535" />
      </n-form-item>
      <n-form-item label="User">
        <n-input v-model:value="form.username" />
      </n-form-item>
      <n-form-item label="Password">
        <n-input v-model:value="form.password" type="password" show-password-on="click" :placeholder="passwordHint" />
      </n-form-item>
      <n-form-item label="Private key">
        <n-space :wrap="false" style="width: 100%">
          <n-input v-model:value="form.keyPath" placeholder="Optional path to an OpenSSH key" clearable />
          <n-button @click="pickKey">Browse</n-button>
        </n-space>
      </n-form-item>
      <n-form-item v-if="form.keyPath" label="Key passphrase">
        <n-input
          v-model:value="form.keyPassphrase"
          type="password"
          show-password-on="click"
          :placeholder="editing ? 'Leave empty to keep the current one' : ''"
        />
      </n-form-item>
    </template>
    <n-space>
      <n-button type="primary" attr-type="submit" :loading="busy" :disabled="!canSubmit">
        {{ editing ? "Save" : "Add TV" }}
      </n-button>
      <n-button v-if="editing" @click="emit('cancel')">Cancel</n-button>
    </n-space>
  </n-form>
</template>

<style scoped>
.tip {
  margin-bottom: 16px;
}
</style>
