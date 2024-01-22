<script setup lang="ts">
import { invoke } from "@tauri-apps/api";

import TreeView from "./TreeView.vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();

function clickItem(index: number, entries: Array<IFileEntry>) {
  const entry = entries[index];
  if (!entry.is_dir) {
    invoke<number>("create_buffer_from_file_path", { path: entry.path })
      .then(async (buffer_idx) => {
        editorStore.bufferIdx = buffer_idx;
        editorStore.encoding = "utf-8";
        const fileEntry = await invoke<IFileEntry>("get_file_info", {
          path: entry.path,
        });

        let entryExists = false;
        workspaceStore.openEditors.forEach((openEditor, index) => {
          if (openEditor.entry?.path == fileEntry.path) {
            entryExists = true;
            workspaceStore.currentEditorIndex = index;
          }
        });
        if (!entryExists) {
          workspaceStore.openEditors.push({
            entry: fileEntry,
            unsavedChanges: false,
            selection: {
              start: {
                row: 0,
                column: 0,
              },
              end: {
                row: 0,
                column: 0,
              },
            },
            scroll: {
              hOffset: 0,
              vOffset: 0,
            },
          });
          workspaceStore.currentEditorIndex =
            workspaceStore.openEditors.length - 1;
        }

        invoke<IHighlightedText>("get_highlighted_text", {
          bufferIdx: editorStore.bufferIdx,
        }).then((content) => {
          editorStore.highlightedContent = content.text;
        });
      })
      .catch((error) => {
        editorStore.encoding = "Unknown";
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
        :entries="workspaceStore.folderEntries"
        :click-handler="clickItem"
      ></TreeView>
    </div>
  </div>
</template>
