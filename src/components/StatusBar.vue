<script setup lang="ts">
import { useEditorStore } from "../stores/editor";
import { useWorkspaceStore } from "../stores/workspace";

const editorStore = useEditorStore();
const workspaceStore = useWorkspaceStore();
</script>

<template>
  <div
    class="bg-atom-bg-dark flex p-1 text-sm select-none border-t-[1px] border-atom-black"
  >
    <div v-if="editorStore.fileEntry != null">
      {{ editorStore.fileEntry.name }}
    </div>
    <div class="grow"></div>
    <div class="px-1" v-if="workspaceStore.currentEditorIndex != -1">
      Ln
      {{ workspaceStore.currentSelection.end.row + 1 }}
      : Col
      {{ workspaceStore.currentSelection.end.column + 1 }}
    </div>
    <div class="px-1">{{ editorStore.encoding }}</div>
  </div>
</template>
