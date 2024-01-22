<script setup lang="ts">
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { appWindow } from "@tauri-apps/api/window";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();

async function open_file() {
  const selected = await open({
    multiple: false,
  });
  if (selected !== null && !Array.isArray(selected)) {
    // user selected a single file
    invoke<number>("create_buffer_from_file_path", {
      path: selected,
    })
      .then(async (buffer_idx) => {
        editorStore.bufferIdx = buffer_idx;
        editorStore.encoding = "utf-8";
        const fileEntry = await invoke<IFileEntry>("get_file_info", {
          path: selected,
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
  }
}

async function open_folder() {
  const selected = await open({
    directory: true,
    multiple: false,
    recursive: true,
  });
  if (selected !== null && !Array.isArray(selected)) {
    // user selected a single folder
    invoke<Array<IFileEntry>>("get_folder_content", {
      path: selected,
    })
      .then((entries) => {
        workspaceStore.workspaceFolder = selected;
        workspaceStore.folderEntries = entries;
      })
      .catch((error) => {
        console.error(error);
      });
  }
}

async function save_current() {
  invoke<string>("save_buffer", {
    bufferIdx: editorStore.bufferIdx,
  })
    .then(() => {
      console.log("File saved successfully");
    })
    .catch((error) => {
      console.error(error);
    });
}

async function save_as() {
  const selected = await save();
  if (selected !== null) {
    // user selected a single file
    invoke<string>("save_buffer_to_new_file", {
      bufferIdx: editorStore.bufferIdx,
      path: selected,
    })
      .then(() => {
        console.log("File saved successfully");
      })
      .catch((error) => {
        console.error(error);
      });
  }
}

async function minimize() {
  await appWindow.minimize();
}

async function maximize() {
  await appWindow.toggleMaximize();
  workspaceStore.resized();
  workspaceStore.maximized = await appWindow.isMaximized();
}

async function quit() {
  await appWindow.close();
}
</script>

<template>
  <div data-tauri-drag-region class="bg-atom-bg-dark p-1 flex select-none">
    <div class="pr-1 cursor-pointer" @click="open_file()">Open File</div>
    <div class="pr-1 cursor-pointer" @click="open_folder()">Open Folder</div>
    <div class="px-1 cursor-pointer" @click="save_current()">Save</div>
    <div class="px-1 cursor-pointer" @click="save_as()">Save as</div>
    <div data-tauri-drag-region class="flex-1"></div>
    <div class="px-1 cursor-pointer" @click="minimize()">Minimize</div>
    <div class="px-1 cursor-pointer" @click="maximize()">Maximize</div>
    <div class="px-1 cursor-pointer" @click="quit()">Quit</div>
  </div>
</template>
