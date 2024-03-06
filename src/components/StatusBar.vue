<script setup lang="ts">
import { useEditorStore, EditingMode } from "../stores/editor";
import { useWorkspaceStore } from "../stores/workspace";
import { useSettingsStore, EOLSequence } from "../stores/settings";

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

function changeEOLSequence() {
  settingsStore.eolSequence =
    settingsStore.eolSequence == EOLSequence.LF
      ? EOLSequence.CRLF
      : EOLSequence.LF;
}
</script>

<template>
  <div
    class="bg-atom-bg-dark flex px-1 select-none border-t-[1px] border-atom-black text-xs"
  >
    <div
      class="p-1.5 hover:bg-atom-bg-hover"
      v-if="editorStore.fileEntry != null"
    >
      {{ editorStore.fileEntry.name }}
    </div>
    <div
      class="p-1.5 hover:bg-atom-bg-hover"
      v-if="workspaceStore.currentEditorIndex != -1"
    >
      Ln
      {{ workspaceStore.currentSelection.end.row + 1 }}
      : Col
      {{ workspaceStore.currentSelection.end.column + 1 }}
    </div>
    <div class="">
      <div
        v-if="editorStore.editingMode == EditingMode.NORMAL"
        class="p-1.5 bg-atom-highlight"
      >
        NORMAL
      </div>
      <div
        v-else-if="editorStore.editingMode == EditingMode.INSERT"
        class="p-1.5 bg-atom-highlight-Green text-atom-text-dark"
      >
        INSERT
      </div>
      <div
        v-else-if="editorStore.editingMode == EditingMode.VISUAL"
        class="p-1.5 bg-atom-highlight-Orange text-atom-text-dark"
      >
        VISUAL
      </div>
      <div v-else>UNKNOWN</div>
    </div>
    <div class="grow"></div>
    <div class="px-1.5 flex">
      <div class="px-1 py-1.5">
        Font Size: {{ settingsStore.editorFontSize }}
      </div>
      <div
        class="mx-1 px-1 py-1.5 hover:bg-atom-bg-hover"
        @click="increaseFontSize"
      >
        +
      </div>
      <div
        class="mx-1 px-1 py-1.5 hover:bg-atom-bg-hover"
        @click="decreaseFontSize"
      >
        -
      </div>
    </div>
    <div class="p-1.5 hover:bg-atom-bg-hover" @click="changeTabSpacing">
      {{ settingsStore.tabSize }} spaces
    </div>
    <div class="p-1.5 hover:bg-atom-bg-hover" @click="changeEOLSequence">
      <div v-if="settingsStore.eolSequence == EOLSequence.LF">LF</div>
      <div v-else>CRLF</div>
    </div>
    <div class="p-1.5 hover:bg-atom-bg-hover">{{ editorStore.encoding }}</div>
  </div>
</template>
