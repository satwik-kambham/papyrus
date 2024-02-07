<script setup lang="ts">
import { PropType } from "vue";

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
      <img
        class="pl-1 pr-2"
        :src="
          !entry.is_dir
            ? '/file.svg'
            : entry.entries == null
              ? '/folder.svg'
              : '/folder-open.svg'
        "
        alt="File Icon"
      />
      <div class="truncate">{{ entry.name }}</div>
    </div>
    <div v-if="entry.entries != null" class="pl-4">
      <TreeView :entries="entry.entries" :click-handler="clickItem"></TreeView>
    </div>
  </div>
</template>
