<script setup lang="ts">
import { computed } from "vue";
import { NButton, NSpace } from "naive-ui";

const props = defineProps<{ busy: boolean; pendingCount: number }>();
const emit = defineEmits<{ rescan: []; installPending: [] }>();

const installLabel = computed(() =>
  props.pendingCount ? `Install ${props.pendingCount} pending` : "Nothing to install",
);
</script>

<template>
  <n-space size="small">
    <n-button size="small" :disabled="busy" @click="emit('rescan')">Rescan</n-button>
    <n-button size="small" type="primary" :disabled="busy || !pendingCount" @click="emit('installPending')">
      {{ installLabel }}
    </n-button>
  </n-space>
</template>
