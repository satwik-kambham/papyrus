<script setup lang="ts">
import { useEditorStore } from "../stores/editor";
import { useWorkspaceStore } from "../stores/workspace";
import { useSettingsStore } from "../stores/settings";

const editorStore = useEditorStore();
const workspaceStore = useWorkspaceStore();
const settingsStore = useSettingsStore();

function changeTabSpacing() {
  settingsStore.tabSize = settingsStore.tabSize == 2 ? 4 : 2;
}

function increaseFontSize() {
  settingsStore.editorFontSize += 1;
}

function decreaseFontSize() {
  settingsStore.editorFontSize -= 1;
}
</script>

<template>
  <div
    class="bg-atom-bg-dark flex p-1 text-sm select-none border-t-[1px] border-atom-black"
  >
    <div v-if="editorStore.fileEntry != null">
      {{ editorStore.fileEntry.name }}
    </div>
    <div class="grow"></div>
    <div class="px-1.5 flex">
      Font Size: {{ settingsStore.editorFontSize }}
      <div class="px-1 hover:bg-atom-bg" @click="increaseFontSize">+</div>
      <div class="px-1 hover:bg-atom-bg" @click="decreaseFontSize">-</div>
    </div>
    <div
      class="px-1.5 hover:bg-atom-bg"
      v-if="workspaceStore.currentEditorIndex != -1"
    >
      Ln
      {{ workspaceStore.currentSelection.end.row + 1 }}
      : Col
      {{ workspaceStore.currentSelection.end.column + 1 }}
    </div>
    <div class="px-1.5 hover:bg-atom-bg" @click="changeTabSpacing">
      {{ settingsStore.tabSize }} spaces
    </div>
    <div class="px-1.5 hover:bg-atom-bg">{{ editorStore.encoding }}</div>
  </div>
</template>
