<script setup lang="ts">
import { invoke } from "@tauri-apps/api";

import TreeView from "./TreeView.vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useStatusStore } from "../stores/status";
import { useEditorStore } from "../stores/editor";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const statusStore = useStatusStore();

function clickItem(index: number, entries: Array<IFileEntry>) {
  const entry = entries[index];
  if (!entry.is_dir) {
    invoke<number>("create_buffer_from_file_path", { path: entry.path })
      .then(async (buffer_idx) => {
        const fileInfo = await invoke<IFileEntry>("get_file_info", {
          path: entry.path,
        });
        let entryExists = false;
        workspaceStore.openEditors.forEach((entry) => {
          if (entry.path == fileInfo.path) entryExists = true;
        });
        if (!entryExists) {
          workspaceStore.openEditors.push(fileInfo);
        }
        workspaceStore.selectedEntry = entry.path;
        statusStore.encoding = "utf8";
        editorStore.bufferIdx = buffer_idx;
        invoke<IHighlightedText>("get_highlighted_text", {
          bufferIdx: editorStore.bufferIdx,
        }).then((content) => {
          editorStore.content = content.text;
        });
      })
      .catch((error) => {
        statusStore.encoding = "Unknown";
        console.error(error);
      });
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
  <div class="bg-atom-bg-dark h-full select-none">
    <div class="border-atom-text-dark border-b-2 text-justify p-1">Project</div>
    <div class="text-sm">
      <TreeView
        :entries="workspaceStore.entries"
        :click-handler="clickItem"
      ></TreeView>
    </div>
  </div>
</template>
