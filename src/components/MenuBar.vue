<script setup lang="ts">
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import MenuItem from "./MenuItem.vue";
import SubMenu from "./SubMenu.vue";
import SubMenuItem from "./SubMenuItem.vue";
import {
  MinusIcon,
  StopIcon,
  Square2StackIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";
import FileIO from "../io.ts";
import Window from "../window";
import Editor from "../editor";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const fileIO = new FileIO(editorStore, settingsStore, workspaceStore);
const window = new Window(workspaceStore);
const editor = new Editor(editorStore, settingsStore, workspaceStore);
</script>

<template>
  <div data-tauri-drag-region class="bg-atom-bg-dark p-1 flex select-none">
    <MenuItem name="File">
      <SubMenu>
        <SubMenuItem @click="fileIO.openFileDialog()">Open File</SubMenuItem>
        <SubMenuItem @click="fileIO.openFolder()">Open Folder</SubMenuItem>
        <SubMenuItem />
        <SubMenuItem @click="fileIO.saveCurrent()">Save</SubMenuItem>
        <SubMenuItem @click="fileIO.saveAs()">Save as</SubMenuItem>
        <SubMenuItem />
        <SubMenuItem @click="window.close()">Quit</SubMenuItem>
      </SubMenu>
    </MenuItem>
    <MenuItem name="Edit">
      <SubMenu>
        <SubMenuItem @click="editor.undo()">Undo</SubMenuItem>
        <SubMenuItem @click="editor.redo()">Redo</SubMenuItem>
        <SubMenuItem />
        <SubMenuItem @click="editor.add_indentation()">Add Indent</SubMenuItem>
        <SubMenuItem @click="editor.remove_indentation()"
          >Remove Indent</SubMenuItem
        >
      </SubMenu>
    </MenuItem>
    <div data-tauri-drag-region class="flex-1"></div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-bg"
      @click="window.minimize()"
    >
      <MinusIcon class="h-full w-4" />
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-bg"
      @click="window.maximize()"
    >
      <StopIcon v-if="!workspaceStore.maximized" class="h-full w-4" />
      <Square2StackIcon v-else class="h-full w-4" />
    </div>
    <div
      class="px-2 cursor-pointer hover:bg-atom-highlight-Red hover:text-black"
      @click="window.close()"
    >
      <XMarkIcon class="h-full w-4" />
    </div>
  </div>
</template>
