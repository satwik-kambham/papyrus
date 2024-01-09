<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue";
import EditorPanel from "./components/EditorPanel.vue";
import SidePanel from "./components/SidePanel.vue";
import StatusBar from "./components/StatusBar.vue";
</script>

<template>
  <div class="bg-atom-bg-dark text-atom-text h-screen flex flex-col">
    <div class="">
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
