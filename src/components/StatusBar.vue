<script setup lang="ts">
import { useStatusStore } from "../stores/status";

const statusStore = useStatusStore();
</script>

<template>
  <div class="bg-neutral-600 flex p-1">
    <div>Status Bar</div>
    <div class="grow"></div>
    <div>{{ statusStore.encoding }}</div>
  </div>
</template>
