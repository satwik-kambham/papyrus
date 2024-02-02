<script setup lang="ts">
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import { openFileDialog, openFolder, saveCurrent, saveAs } from "../io.ts";
import { minimize, maximize, quit } from "../window.ts";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();
</script>

<template>
  <div data-tauri-drag-region class="bg-atom-bg-dark p-1 flex select-none">
    <div
      class="pr-1 cursor-pointer hover:bg-atom-bg"
      @click="openFileDialog(editorStore, workspaceStore)"
    >
      Open File
    </div>
    <div
      class="pr-1 cursor-pointer hover:bg-atom-bg"
      @click="openFolder(editorStore, workspaceStore)"
    >
      Open Folder
    </div>
    <div
      class="px-1 cursor-pointer hover:bg-atom-bg"
      @click="saveCurrent(editorStore, workspaceStore, settingsStore)"
    >
      Save
    </div>
    <div
      class="px-1 cursor-pointer hover:bg-atom-bg"
      @click="saveAs(editorStore, workspaceStore, settingsStore)"
    >
      Save as
    </div>
    <div data-tauri-drag-region class="flex-1"></div>
    <div class="px-2 cursor-pointer hover:bg-atom-bg" @click="minimize()">
      -
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-bg"
      @click="maximize(workspaceStore)"
    >
      o
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-highlight-Red hover:text-black"
      @click="quit()"
    >
      x
    </div>
  </div>
</template>
