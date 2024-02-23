<script setup lang="ts">
import { ref, nextTick } from "vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import { invoke } from "@tauri-apps/api";
import FileIO from "../io.ts";
import Modals from "../modal.ts";
import {
  FolderIcon,
  FolderOpenIcon,
  DocumentTextIcon,
} from "@heroicons/vue/24/solid";
import ContextMenu from "./ContextMenu.vue";
import ContextMenuItem from "./ContextMenuItem.vue";

const props = defineProps({
  entries: {
    type: Array<IFileEntry>,
    required: true,
  },
});

const contextEntry = ref<IFileEntry | null>(null);
const contextMenuElement = ref<ContextMent | null>(null);
const contextMenuVisible = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const fileIO = new FileIO(editorStore, settingsStore, workspaceStore);
const modals = new Modals(editorStore, settingsStore, workspaceStore);

async function openContextMenu(
  e: PointerEvent,
  index: number,
  entries: Array<IFileEntry>,
) {
  contextEntry.value = entries[index];
  contextMenuX.value = e.clientX;
  contextMenuY.value = e.clientY;
  contextMenuVisible.value = true;
  await nextTick();
  contextMenuElement.value.contextMenuElement.focus();
}

function clickItem(index: number, entries: Array<IFileEntry>) {
  const entry = entries[index];
  if (!entry.is_dir) {
    fileIO.openFile(entry.path);
  } else {
    if (entry.entries == null) {
      invoke<Array<IFileEntry>>("get_folder_content", {
        path: entry.path,
      })
        .then((entries) => {
          entry.entries = entries;
        })
        .catch((error) => {
          console.error(error);
        });
    } else {
      entry.entries = null;
    }
  }
}
</script>

<template>
  <div
    class="flex flex-col text-sm leading-7"
    v-for="(entry, index) in props.entries"
    :key="index"
  >
    <div
      class="flex hover:bg-atom-bg-hover cursor-pointer pl-2"
      @click="clickItem(index, props.entries)"
      @contextmenu.prevent="openContextMenu($event, index, props.entries)"
    >
      <div class="pl-1 pr-2">
        <FolderIcon
          v-if="entry.is_dir && entry.entries == null"
          class="h-full w-4 text-atom-bg-icon"
        />
        <FolderOpenIcon
          v-else-if="entry.is_dir && entry.entries != null"
          class="text-atom-highlight h-full w-4"
        />
        <DocumentTextIcon v-else class="text-atom-highlight h-full w-4" />
      </div>
      <div class="truncate">{{ entry.name }}</div>
    </div>
    <div v-if="entry.entries != null" class="pl-4">
      <TreeView :entries="entry.entries"></TreeView>
    </div>
  </div>
  <div class="relative">
    <ContextMenu
      class="fixed z-50"
      v-show="contextMenuVisible"
      :style="{ top: contextMenuY + 'px', left: contextMenuX + 'px' }"
      ref="contextMenuElement"
      @context-blur="contextMenuVisible = false"
    >
      <ContextMenuItem
        @click="
          async () => {
            modals.promptUser(
              'New File',
              'Enter file path',
              (result, context) => fileIO.createNewFile(result, context),
              contextEntry,
              await fileIO.relative(
                workspaceStore.workspaceFolder,
                contextEntry.is_dir
                  ? contextEntry.path
                  : await fileIO.parent(contextEntry.path),
              ),
            );
          }
        "
        >New File</ContextMenuItem
      >
      <ContextMenuItem
        @click="
          async () => {
            modals.promptUser(
              'New Folder',
              'Enter folder path',
              (result, context) => fileIO.createNewFolder(result, context),
              contextEntry,
              await fileIO.relative(
                workspaceStore.workspaceFolder,
                contextEntry.is_dir
                  ? contextEntry.path
                  : await fileIO.parent(contextEntry.path),
              ),
            );
          }
        "
        >New Folder</ContextMenuItem
      >
      <ContextMenuItem />
      <ContextMenuItem>Rename</ContextMenuItem>
      <ContextMenuItem>Delete</ContextMenuItem>
    </ContextMenu>
  </div>
</template>
