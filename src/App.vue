<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue";
import SideBar from "./components/SideBar.vue";
import StatusBar from "./components/StatusBar.vue";
import TerminalComponent from "./components/TerminalComponent.vue";
import TreeView from "./components/TreeView.vue";
import { useWorkspaceStore } from "./stores/workspace";
import { appWindow } from "@tauri-apps/api/window";
import CollapsiblePanel from "./components/CollapsiblePanel.vue";
import TabsComponent from "./components/TabsComponent.vue";

const workspaceStore = useWorkspaceStore();

appWindow.onResized(async () => {
  workspaceStore.maximized = await appWindow.isMaximized();
});
</script>

<template>
  <div
    class="bg-atom-bg-dark text-atom-text h-screen flex flex-col border-2 border-atom-bg-light overflow-y-hidden w-full text-sm"
    :class="workspaceStore.maximized ? 'rounded-none' : 'rounded-xl'"
  >
    <div data-tauri-drag-region class="w-full">
      <MenuBar />
    </div>
    <div class="flex flex-1 overflow-hidden w-full">
      <div class="flex-none">
        <CollapsiblePanel
          ><SideBar>
            <TreeView :entries="workspaceStore.folderEntries" /> </SideBar
        ></CollapsiblePanel>
      </div>
      <div class="flex flex-col shrink grow min-w-0">
        <div class="flex-1 w-full">
          <div class="overflow-hidden h-full w-full block">
            <TabsComponent />
          </div>
        </div>
        <CollapsiblePanel horizontal
          ><Suspense><TerminalComponent /></Suspense
        ></CollapsiblePanel>
      </div>
    </div>
    <div class="w-full">
      <StatusBar />
    </div>
  </div>
</template>
