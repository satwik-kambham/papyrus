<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

import { useWorkspaceStore } from "../stores/workspace";
import { useStatusStore } from "../stores/status";
import { useEditorStore } from "../stores/editor";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const statusStore = useStatusStore();

const selectedIndex = ref(null);

function clickItem(index) {
  selectedIndex.value = index;
  const entry = workspaceStore.entries[index];
  console.log(entry);
  if (!entry.isDir) {
    invoke("create_buffer_from_file_path", { path: entry.path })
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
  <div class="bg-atom-bg-dark h-full">
    <div class="border-atom-text-dark border-b-2 text-justify p-1">Project</div>
    <div class="text-sm">
      <div
        class="py-0.5 px-1 cursor-pointer hover:bg-atom-bg-light"
        v-for="(entry, index) in workspaceStore.entries"
        :key="index"
        @click="clickItem(index)"
        :class="{
          'bg-atom-bg-light': selectedIndex === index,
        }"
      >
        {{ entry.name }}
      </div>
    </div>
  </div>
</template>
