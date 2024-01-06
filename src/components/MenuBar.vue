<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
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
</script>

<template>
  <div class="bg-atom-bg-dark p-1">
    <button type="button" @click="open_file()">Open File</button>
  </div>
</template>
