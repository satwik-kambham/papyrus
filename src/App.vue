<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue";
import EditorPanel from "./components/EditorPanel.vue";
import SideBar from "./components/SideBar.vue";
import StatusBar from "./components/StatusBar.vue";
import TerminalComponent from "./components/TerminalComponent.vue";

import { useWorkspaceStore } from "./stores/workspace";
import { appWindow } from "@tauri-apps/api/window";
import CollapsiblePanel from "./components/CollapsiblePanel.vue";

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
      <CollapsiblePanel><SideBar></SideBar></CollapsiblePanel>
      <div class="flex-1 flex flex-col">
        <div class="flex-1">
          <div class="overflow-hidden h-full">
            <EditorPanel />
          </div>
        </div>
        <CollapsiblePanel horizontal
          ><Suspense><TerminalComponent /></Suspense
        ></CollapsiblePanel>
      </div>
    </div>
    <div class="">
      <StatusBar />
    </div>
  </div>
</template>
