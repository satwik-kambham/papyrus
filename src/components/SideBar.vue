<script setup lang="ts">
import TreeView from "./TreeView.vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { invoke } from "@tauri-apps/api";
import { openFile } from "../io.ts";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();

function clickItem(index: number, entries: Array<IFileEntry>) {
  const entry = entries[index];
  if (!entry.is_dir) {
    openFile(entry.path, editorStore, workspaceStore);
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
    class="bg-atom-bg-dark h-full select-none flex flex-col border-r-[1px] border-atom-black"
  >
    <div class="border-atom-bg border-b-[0.5px] text-center p-1">Project</div>
    <div class="grow overflow-auto custom-scrollbar">
      <TreeView
        :entries="workspaceStore.folderEntries"
        :click-handler="clickItem"
      ></TreeView>
    </div>
  </div>
</template>
