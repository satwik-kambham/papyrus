<script setup lang="ts">
import { ref } from "vue";

const contextMenuElement = ref<HTMLElement | null>(null);

defineExpose({ contextMenuElement });
</script>

<template>
  <div
    class="bg-atom-bg-dark p-2 border-[1px] border-atom-bg shadow-lg shadow-atom-bg"
    tabindex="-1"
    ref="contextMenuElement"
    @blur="$emit('context-blur')"
  >
    <slot></slot>
  </div>
</template>
