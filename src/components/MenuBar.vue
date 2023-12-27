<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { useStatusStore } from "../stores/status";

const statusStore = useStatusStore();

async function open_file() {
  const selected = await open();
  if (selected !== null) {
    // user selected a single file
    console.log(selected);
    invoke("read_file_content", { path: selected })
      .then((content) => {
        statusStore.encoding = "utf8";
        console.log(content);
      })
      .catch((error) => {
        statusStore.encoding = "Unknown";
        console.error(error);
      });
  }
}
</script>

<template>
  <div class="bg-slate-900 p-1">
    MenuBar
    <button type="button" @click="open_file()">Open File</button>
  </div>
</template>
