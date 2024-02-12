<script setup lang="ts">
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import { invoke } from "@tauri-apps/api";
import FileIO from "../io.ts";
import { PropType } from "vue";
import {
  FolderIcon,
  FolderOpenIcon,
  DocumentTextIcon,
} from "@heroicons/vue/24/solid";

const props = defineProps({
  entries: {
    type: Array<IFileEntry>,
    required: true,
  },
});

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const fileIO = new FileIO(editorStore, settingsStore, workspaceStore);

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
    class="flex flex-col"
    v-for="(entry, index) in props.entries"
    :key="index"
  >
    <div
      class="flex hover:bg-atom-bg-light cursor-pointer pl-2 py-0.5"
      @click="clickItem(index, props.entries)"
    >
      <div class="pl-1 pr-2">
        <FolderIcon
          v-if="entry.is_dir && entry.entries == null"
          class="h-full w-4"
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
</template>
