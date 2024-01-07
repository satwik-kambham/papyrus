<script setup lang="ts">
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { useStatusStore } from "../stores/status";
import { useEditorStore } from "../stores/editor";

const editorStore = useEditorStore();
const statusStore = useStatusStore();

async function open_file() {
  const selected = await open();
  if (selected !== null) {
    // user selected a single file
    invoke("create_buffer_from_file_path", { path: selected })
      .then((success) => {
        statusStore.encoding = "utf8";
        editorStore.buffer_idx = 0;
        invoke("get_highlighted_text").then((content) => {
          editorStore.content = content.text;
        });
      })
      .catch((error) => {
        statusStore.encoding = "Unknown";
        console.error(error);
      });
  }
}

async function save_current() {
  invoke("save_buffer")
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
    invoke("save_buffer_to_new_file", { path: selected })
      .then((success) => {
        console.log("File saved successfully");
      })
      .catch((error) => {
        console.error(error);
      });
  }
}
</script>

<template>
  <div class="bg-atom-bg-dark p-1">
    <button class="pr-1" type="button" @click="open_file()">Open File</button>
    <button class="px-1" type="button" @click="save_current()">Save</button>
    <button class="px-1" type="button" @click="save_as()">Save as</button>
  </div>
</template>
