<script setup lang="ts">
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import FileIO from "../io.ts";
import Window from "../window";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const fileIO = new FileIO(editorStore, settingsStore, workspaceStore);
const window = new Window(workspaceStore);
</script>

<template>
  <div data-tauri-drag-region class="bg-atom-bg-dark p-1 flex select-none">
    <div
      class="pr-1 cursor-pointer hover:bg-atom-bg"
      @click="fileIO.openFileDialog()"
    >
      Open File
    </div>
    <div
      class="pr-1 cursor-pointer hover:bg-atom-bg"
      @click="fileIO.openFolder()"
    >
      Open Folder
    </div>
    <div
      class="px-1 cursor-pointer hover:bg-atom-bg"
      @click="fileIO.saveCurrent()"
    >
      Save
    </div>
    <div class="px-1 cursor-pointer hover:bg-atom-bg" @click="fileIO.saveAs()">
      Save as
    </div>
    <div data-tauri-drag-region class="flex-1"></div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-bg"
      @click="window.minimize()"
    >
      -
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-bg"
      @click="window.maximize()"
    >
      o
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-highlight-Red hover:text-black"
      @click="window.close()"
    >
      x
    </div>
  </div>
</template>
