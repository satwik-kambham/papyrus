<script setup lang="ts">
import { ref, onBeforeUnmount, nextTick } from "vue";
import { useWorkspaceStore } from "../stores/workspace";

const props = defineProps({
  horizontal: Boolean,
  inverse: Boolean,
});

const workspaceStore = useWorkspaceStore();

const isDragging = ref(false);
const start = ref(0);
const panelSize = ref(250);

let prevPanelSize = 250;

function handleMouseDown(event: MouseEvent) {
  isDragging.value = true;
  start.value = event.clientX;
  if (props.horizontal) {
    start.value = event.clientY;
  }

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
}

function handleMouseMove(event: MouseEvent) {
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
}

async function handleMouseUp() {
  isDragging.value = false;

  document.removeEventListener("mousemove", handleMouseMove);
  document.removeEventListener("mouseup", handleMouseUp);

  await nextTick();
  workspaceStore.resized();
}

async function toggleMinimizer() {
  if (panelSize.value === 0) {
    panelSize.value = prevPanelSize;
  } else {
    prevPanelSize = panelSize.value;
    panelSize.value = 0;
  }
  await nextTick();
  workspaceStore.resized();
}

onBeforeUnmount(() => {
  document.removeEventListener("mousemove", handleMouseMove);
  document.removeEventListener("mouseup", handleMouseUp);
});
</script>

<template>
  <div
    class="flex relative"
    :class="{
      'flex-row': !props.horizontal,
      'flex-col-reverse': props.horizontal,
    }"
    :style="{
      width: props.horizontal ? '100%' : panelSize + 'px',
      height: props.horizontal ? panelSize + 'px' : '100%',
    }"
  >
    <div
      class="overflow-auto custom-scrollbar z-40"
      :class="{
        'h-full': props.horizontal,
        'w-full': !props.horizontal,
      }"
    >
      <slot></slot>
    </div>
    <div
      class="hover:bg-atom-primary transition-colors delay-200 duration-500 absolute z-40"
      :class="{
        'h-full w-1 top-0 -right-1 cursor-col-resize':
          !props.horizontal && !props.inverse,
        'w-full h-1 -top-1 right-0 cursor-row-resize':
          props.horizontal && !props.inverse,
        'h-full w-1 bottom-0 -left-1 cursor-col-resize':
          !props.horizontal && props.inverse,
        'w-full h-1 -bottom-1 left-0 cursor-row-resize':
          props.horizontal && props.inverse,
      }"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
    ></div>
    <div
      class="bg-atom-highlight cursor-pointer select-none transition-all duration-200 transform p-2 absolute w-8 h-8 text-center text-xl z-30 opacity-0 hover:opacity-100"
      :class="{
        'rotate-180 -top-4 right-1/2 hover:-translate-y-4 rounded-b-full':
          props.horizontal && !props.inverse && panelSize !== 0,
        '-rotate-90 top-1/2 -right-4 hover:translate-x-4 rounded-b-full':
          !props.horizontal && !props.inverse && panelSize !== 0,
        'rotate-90 top-1/2 -right-4 hover:translate-x-4 rounded-t-full':
          !props.horizontal && !props.inverse && panelSize === 0,
        'rotate-0 -top-4 right-1/2 hover:-translate-y-4 rounded-t-full':
          props.horizontal && !props.inverse && panelSize === 0,
        'rotate-180 -bottom-4 left-1/2 hover:translate-y-4 rounded-b-full':
          props.horizontal && props.inverse && panelSize !== 0,
        'rotate-90 bottom-1/2 -left-4 hover:-translate-x-4 rounded-b-full':
          !props.horizontal && props.inverse && panelSize !== 0,
        '-rotate-90 bottom-1/2 -left-4 hover:-translate-x-4 rounded-t-full':
          !props.horizontal && props.inverse && panelSize === 0,
        'rotate-0 -bottom-4 left-1/2 hover:translate-y-4 rounded-t-full':
          props.horizontal && props.inverse && panelSize === 0,
      }"
      @mousedown="toggleMinimizer"
    >
      ^
    </div>
  </div>
</template>
