<script setup lang="ts">
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { useWorkspaceStore } from "../stores/workspace";
import { useStatusStore } from "../stores/status";
import { useEditorStore } from "../stores/editor";
import { appWindow } from "@tauri-apps/api/window";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const statusStore = useStatusStore();

async function open_file() {
  const selected = await open({
    multiple: false,
  });
  if (selected !== null) {
    // user selected a single file
    invoke("create_buffer_from_file_path", {
      path: selected,
    })
      .then(async (buffer_idx) => {
        const fileInfo = await invoke("get_file_info", { path: selected });
        let entryExists = false;
        workspaceStore.openEditors.forEach((entry) => {
          if (entry.path == fileInfo.path) entryExists = true;
        });
        if (!entryExists) {
          workspaceStore.openEditors.push(fileInfo);
        }
        workspaceStore.selectedEntry = selected;
        statusStore.encoding = "utf8";
        editorStore.bufferIdx = buffer_idx;
        invoke("get_highlighted_text", {
          bufferIdx: editorStore.bufferIdx,
        }).then((content) => {
          editorStore.content = content.text;
        });
      })
      .catch((error) => {
        statusStore.encoding = "Unknown";
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
  if (selected !== null) {
    // user selected a single folder
    invoke("get_folder_content", {
      path: selected,
    })
      .then((entries) => {
        console.log(entries);
        workspaceStore.folder = selected;
        workspaceStore.entries = entries;
      })
      .catch((error) => {
        console.error(error);
      });
  }
}

async function save_current() {
  invoke("save_buffer", {
    bufferIdx: editorStore.bufferIdx,
  })
    .then((success) => {
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
    invoke("save_buffer_to_new_file", {
      bufferIdx: editorStore.bufferIdx,
      path: selected,
    })
      .then((success) => {
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
  workspaceStore.maximized = await appWindow.isMaximized();
}

async function quit() {
  await appWindow.close();
}
</script>

<template>
  <div data-tauri-drag-region class="bg-atom-bg-dark p-1 flex">
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
