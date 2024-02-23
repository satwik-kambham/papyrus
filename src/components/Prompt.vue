<script setup lang="ts">
import { useEditorStore } from "../stores/editor";
import {
  Dialog,
  DialogPanel,
  DialogTitle,
  DialogDescription,
} from "@headlessui/vue";

const editorStore = useEditorStore();

function canceled() {
  editorStore.promptOpen = false;
}

async function submitted() {
  editorStore.promptOpen = false;
  await editorStore.promptCallback(
    editorStore.promptResponse,
    editorStore.promptContext,
  );

  editorStore.promptTitle = "";
  editorStore.promptDescription = "";
  editorStore.promptCallback = null;
}
</script>

<template>
  <Dialog
    :open="editorStore.promptOpen"
    @close="canceled"
    class="relative z-50"
  >
    <div
      class="fixed inset-0 flex flex-col bg-black/20 items-center text-atom-text min-h-full w-screen"
    >
      <DialogPanel
        class="w-3/4 bg-atom-bg rounded-2xl py-2 px-4 shadow-md shadow-atom-bg mt-10"
      >
        <DialogTitle class="text-md">{{ editorStore.promptTitle }}</DialogTitle>
        <DialogDescription class="text-sm italic mb-2">{{
          editorStore.promptDescription
        }}</DialogDescription>
        <input
          class="border bg-atom-bg-dark border-atom-highlight rounded px-4 py-2 focus:outline-none focus:border-atom-primary w-full"
          v-model="editorStore.promptResponse"
          @keydown.enter="submitted"
        />
      </DialogPanel>
    </div>
  </Dialog>
</template>
