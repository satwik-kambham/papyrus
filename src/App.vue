<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue";
import EditorPanel from "./components/EditorPanel.vue";
import SidePanel from "./components/SidePanel.vue";
import StatusBar from "./components/StatusBar.vue";

import { useWorkspaceStore } from "./stores/workspace";
import { appWindow } from "@tauri-apps/api/window";

const workspaceStore = useWorkspaceStore();

appWindow.onResized(async (e) => {
  workspaceStore.maximized = await appWindow.isMaximized();
});
</script>

<template>
  <div
    class="bg-atom-bg-dark text-atom-text h-screen flex flex-col border-2 border-atom-bg-light overflow-hidden"
    :class="workspaceStore.maximized ? 'rounded-none' : 'rounded-xl'"
  >
    <div data-tauri-drag-region class="">
      <MenuBar />
    </div>
    <div class="flex flex-1 overflow-auto">
      <div class="w-1/6 overflow-auto h-full custom-scrollbar">
        <SidePanel />
      </div>
      <div class="flex-1 overflow-hidden">
        <EditorPanel />
      </div>
    </div>
    <div class="">
      <StatusBar />
    </div>
  </div>
</template>
