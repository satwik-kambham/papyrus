<script setup lang="ts">
import { ref } from "vue";

const selectedIndex = ref(null);

const props = defineProps(["clickHandler", "entries"]);

function clickItem(index, entries) {
  selectedIndex.value = index;
  props.clickHandler(index, entries);
}
</script>

<template>
  <div
    class="py-0.5 px-1 flex flex-col"
    v-for="(entry, index) in props.entries"
    :key="index"
  >
    <div
      class="flex hover:bg-atom-bg-light cursor-pointer"
      @click="clickItem(index, props.entries)"
      :class="{
        'bg-atom-bg-light': selectedIndex === index,
      }"
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
    <div v-if="entry.entries != null" class="">
      <TreeView :entries="entry.entries" :click-handler="clickItem"></TreeView>
    </div>
  </div>
</template>
