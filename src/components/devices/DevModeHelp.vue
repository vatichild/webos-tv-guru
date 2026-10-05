<script setup lang="ts">
import { NAlert } from "naive-ui";

defineProps<{ hasKey: boolean }>();
</script>

<template>
  <n-alert v-if="hasKey" type="default" :bordered="false">
    Enter the passphrase only if the TV's key changed, for example after you reinstall the Developer Mode app.
    Key Server must be on to fetch the key again.
  </n-alert>
  <n-alert v-else type="info" :bordered="false">
    On the TV, open the Developer Mode app and turn on <b>Dev Mode Status</b> and <b>Key Server</b>.
    Then enter the TV's IP address and the passphrase shown in the app.
  </n-alert>
</template>
