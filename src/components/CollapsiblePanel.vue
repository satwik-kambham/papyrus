<script setup lang="ts">
import { ref, onBeforeUnmount } from "vue";
import { useWorkspaceStore } from "../stores/workspace";

const props = defineProps({
  horizontal: Boolean,
});

const workspaceStore = useWorkspaceStore();

const isDragging = ref(false);
const start = ref(0);
const panelSize = ref(200);

const handleMouseDown = (event) => {
  isDragging.value = true;
  start.value = event.clientX;
  if (props.horizontal) {
    start.value = event.clientY;
  }

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
};

const handleMouseMove = (event) => {
  if (isDragging.value) {
    if (props.horizontal) {
      panelSize.value += -(event.clientY - start.value);
    } else {
      panelSize.value += event.clientX - start.value;
    }
  }

  start.value = event.clientX;
  if (props.horizontal) {
    start.value = event.clientY;
  }
};

const handleMouseUp = () => {
  isDragging.value = false;

  document.removeEventListener("mousemove", handleMouseMove);
  document.removeEventListener("mouseup", handleMouseUp);

  workspaceStore.resized();
};

onBeforeUnmount(() => {
  document.removeEventListener("mousemove", handleMouseMove);
  document.removeEventListener("mouseup", handleMouseUp);
});
</script>

<template>
  <div
    class="flex relative"
    :class="{
      'h-full flex-row': !props.horizontal,
      'w-full flex-col-reverse': props.horizontal,
    }"
    :style="{
      width: props.horizontal ? '100%' : panelSize + 'px',
      height: props.horizontal ? panelSize + 'px' : '100%',
    }"
  >
    <div
      class="overflow-auto custom-scrollbar"
      :class="{
        'h-full': props.horizontal,
        'w-full': !props.horizontal,
      }"
    >
      <slot></slot>
    </div>
    <div
      class="hover:bg-atom-primary transition-colors delay-200 duration-500 absolute z-50"
      :class="{
        'h-full w-1 top-0 -right-1 cursor-col-resize': !props.horizontal,
        'w-full h-1 -top-1 right-0 cursor-row-resize': props.horizontal,
      }"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
    ></div>
  </div>
</template>
