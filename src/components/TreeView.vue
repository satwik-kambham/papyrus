<script setup lang="ts">
import { PropType } from "vue";
import {
  FolderIcon,
  FolderOpenIcon,
  DocumentTextIcon,
} from "@heroicons/vue/24/solid";

const props = defineProps({
  entries: {
    type: Array<IFileEntry>,
    required: true,
  },
  clickHandler: {
    type: Function as PropType<
      (index: number, entries: Array<IFileEntry>) => void
    >,
    required: true,
  },
});

function clickItem(index: number, entries: Array<IFileEntry>) {
  props.clickHandler(index, entries);
}
</script>

<template>
  <div
    class="flex flex-col"
    v-for="(entry, index) in props.entries"
    :key="index"
  >
    <div
      class="flex hover:bg-atom-bg-light cursor-pointer pl-2 py-0.5"
      @click="clickItem(index, props.entries)"
    >
      <div class="pl-1 pr-2">
        <FolderIcon
          v-if="entry.is_dir && entry.entries == null"
          class="h-full w-4"
        />
        <FolderOpenIcon
          v-else-if="entry.is_dir && entry.entries != null"
          class="text-atom-highlight h-full w-4"
        />
        <DocumentTextIcon v-else class="text-atom-highlight h-full w-4" />
      </div>
      <div class="truncate">{{ entry.name }}</div>
    </div>
    <div v-if="entry.entries != null" class="pl-4">
      <TreeView :entries="entry.entries" :click-handler="clickItem"></TreeView>
    </div>
  </div>
</template>
