<script setup lang="ts">
import { useStatusStore } from "../stores/status";

const statusStore = useStatusStore();
</script>

<template>
  <div class="bg-atom-bg-dark flex p-1 text-sm select-none">
    <div>Status Bar</div>
    <div class="grow"></div>
    <div class="px-1">
      Ln {{ statusStore.cursorRow + 1 }} : Col
      {{ statusStore.cursorColumn + 1 }}
    </div>
    <div class="px-1">{{ statusStore.encoding }}</div>
  </div>
</template>
