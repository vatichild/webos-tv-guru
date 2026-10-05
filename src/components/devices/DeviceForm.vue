<script setup lang="ts">
import { ref } from "vue";
import { NButton, NForm, NFormItem, NInput, NInputNumber, NRadioButton, NRadioGroup, NSpace } from "naive-ui";
import PathPicker from "@/components/common/PathPicker.vue";
import { useDeviceForm } from "@/composables/useDeviceForm";
import type { DeviceInfo, NewDevice } from "@/types";
import DevModeHelp from "./DevModeHelp.vue";

const props = defineProps<{
  device?: DeviceInfo;
  submit: (device: NewDevice) => Promise<void>;
}>();
const emit = defineEmits<{ cancel: [] }>();

const { form, editing, hasDevModeKey, canSubmit, hints, toDevice, reset } = useDeviceForm(props.device);
const busy = ref(false);

async function onSubmit() {
  busy.value = true;
  try {
    await props.submit(toDevice());
    if (!editing) reset();
  } catch {
    return;
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
    <DevModeHelp v-if="form.kind === 'devMode'" :has-key="hasDevModeKey" class="help" />
    <n-form-item label="Name">
      <n-input v-model:value="form.name" placeholder="living-room" />
    </n-form-item>
    <n-form-item label="IP address">
      <n-input v-model:value="form.host" placeholder="192.168.1.50" />
    </n-form-item>
    <n-form-item v-if="form.kind === 'devMode'" label="Passphrase">
      <n-input v-model:value="form.passphrase" :placeholder="hints.passphrase" />
    </n-form-item>
    <template v-else>
      <n-form-item label="Port">
        <n-input-number v-model:value="form.port" :min="1" :max="65535" />
      </n-form-item>
      <n-form-item label="User">
        <n-input v-model:value="form.username" />
      </n-form-item>
      <n-form-item label="Password">
        <n-input v-model:value="form.password" type="password" show-password-on="click" :placeholder="hints.password" />
      </n-form-item>
      <n-form-item label="Private key">
        <PathPicker v-model="form.keyPath" placeholder="Optional path to an OpenSSH key" clearable />
      </n-form-item>
      <n-form-item v-if="form.keyPath" label="Key passphrase">
        <n-input
          v-model:value="form.keyPassphrase"
          type="password"
          show-password-on="click"
          :placeholder="hints.keyPassphrase"
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
.help {
  margin-bottom: 16px;
}
</style>
