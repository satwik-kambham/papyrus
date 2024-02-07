<script setup lang="ts">
import { ref } from "vue";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import Editor from "../editor";
import EditorPanel from "./EditorPanel.vue";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const editor = new Editor(editorStore, settingsStore, workspaceStore);

const hoverTabIndex = ref(-1);
</script>

<template>
  <div class="flex flex-col h-full">
    <div
      class="flex overflow-x-auto custom-scrollbar z-30 bg-atom-bg-dark border-b-[1px] border-atom-black"
    >
      <div
        class="px-2 py-1 border-r-[1px] min-w-64 justify-center flex-shrink-0 border-r-atom-black whitespace-nowrap cursor-pointer select-none flex"
        v-for="(openEditor, index) in workspaceStore.openEditors"
        :key="index"
        @mouseover="hoverTabIndex = index"
        @mouseleave="hoverTabIndex = -1"
        @click="() => workspaceStore.switchEditor(index)"
        :class="{
          'bg-atom-bg text-atom-text border-l-2 border-atom-primary':
            index == workspaceStore.currentEditorIndex,
          'text-atom-text-light': index != workspaceStore.currentEditorIndex,
        }"
      >
        <div class="grow"></div>
        {{ openEditor.entry?.name }}
        <div class="grow"></div>

        <div
          class="px-1 text-atom-text hover:text-black hover:bg-atom-primary mx-1 rounded-sm"
          @click="
            async (e) => {
              e.stopPropagation();
              await editor.closeBuffer(index);
            }
          "
        >
          <div
            class="text-atom-primary w-2 mb-1.5"
            v-if="
              workspaceStore.openEditors[index].unsavedChanges &&
              hoverTabIndex != index
            "
          >
            U
          </div>
          <div
            v-else
            class="transition duration-500 mb-1.5 w-2 text-center"
            :class="{
              'opacity-100 scale-100': hoverTabIndex == index,
              'opacity-0 scale-0': hoverTabIndex != index,
            }"
          >
            x
          </div>
        </div>
      </div>
    </div>
    <div class="h-full">
      <component :is="EditorPanel" />
    </div>
  </div>
</template>
