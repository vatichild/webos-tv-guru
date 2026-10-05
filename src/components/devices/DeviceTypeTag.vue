<script setup lang="ts">
import { NTag } from "naive-ui";

defineProps<{ devMode: boolean }>();
</script>

<template>
  <n-tag size="small" :type="devMode ? 'info' : 'warning'">
    {{ devMode ? "Dev Mode" : "Rooted / SSH" }}
  </n-tag>
</template>
