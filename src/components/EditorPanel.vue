<script setup lang="ts">
import { ref } from "vue";
import { useEditorStore } from "../stores/editor";

const editorStore = useEditorStore();

const vOffset = ref(0);

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function wheel_event(e) {
  vOffset.value = clamp(
    vOffset.value - e.deltaY * 0.3,
    -e.currentTarget.clientHeight + e.currentTarget.parentNode.clientHeight,
    0
  );
}
</script>
<template>
  <div
    class="bg-gray-800 p-1 whitespace-pre-wrap min-h-full font-code antialiased text-lg absolute w-full overflow-visible h-fit"
    @wheel="wheel_event"
    :style="{ top: vOffset + 'px' }"
  >
    <div class="whitespace-pre-wrap" v-for="line in editorStore.content">
      <span class="whitespace-pre-wrap inline-block" v-for="token in line">
        {{ token[1] }}
      </span>
    </div>
  </div>
</template>
