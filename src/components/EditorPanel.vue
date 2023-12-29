<script setup lang="ts">
import { useEditorStore } from "../stores/editor";

const editorStore = useEditorStore();
</script>
<template>
  <div class="bg-gray-800 p-1 whitespace-pre-wrap h-full overflow-auto">
    <div class="whitespace-pre-wrap" v-for="line in editorStore.content">
      <span class="whitespace-pre-wrap inline-block" v-for="token in line">
        {{ token[1] }}
      </span>
    </div>
  </div>
</template>
