<script setup lang="ts">
import { ref } from "vue";
import { useEditorStore } from "../stores/editor";
import { useStatusStore } from "../stores/status";

const statusStore = useStatusStore();
const editorStore = useEditorStore();

const cursorElement = ref(null);
const hOffset = ref(0);
const vOffset = ref(0);
const cursorHOffset = ref(0);
const cursorVOffset = ref(0);

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function wheel_event(e) {
  hOffset.value = clamp(
    hOffset.value - e.deltaX * 0.3,
    -e.currentTarget.clientWidth + e.currentTarget.parentNode.clientWidth - 100,
    0
  );

  vOffset.value = clamp(
    vOffset.value - e.deltaY * 0.3,
    -e.currentTarget.clientHeight +
      e.currentTarget.parentNode.clientHeight -
      100,
    0
  );
}

function click_event(e) {
  let range;
  let textNode;
  let offset;
  let boundingRect;
  if (document.caretPositionFromPoint) {
    range = document.caretPositionFromPoint(e.clientX, e.clientY);
    textNode = range.offsetNode;
    offset = range.offset;
    boundingRect = range.getBoundingClientRect();
  } else if (document.caretRangeFromPoint) {
    // Use WebKit-proprietary fallback method
    range = document.caretRangeFromPoint(e.clientX, e.clientY);
    textNode = range?.startContainer;
    offset = range?.startOffset;
    boundingRect = range?.getBoundingClientRect();
  } else {
    console.error("Cannot handle click as caret APIs not supported");
    return;
  }
  if (textNode?.nodeType === 3) {
    let children = Array.from(textNode.parentNode.parentNode.children);
    let current = children.indexOf(textNode.parentNode);
    let length = offset;
    let row = Math.floor(
      e.target.parentNode.offsetTop / e.target.parentNode.clientHeight
    );

    children.slice(0, current).forEach((child) => {
      length += child.textContent.length;
    });
    console.log("Row: " + row + " Column: " + length);
    statusStore.cursorRow = row;
    statusStore.cursorColumn = length;
    cursorVOffset.value = e.target.parentNode.offsetTop;
    cursorHOffset.value =
      boundingRect.right - cursorElement.value?.parentNode.offsetLeft;
  } else if (textNode?.nodeType === 1) {
    let length;
    let row = Math.floor(e.target.offsetTop / e.target.clientHeight);
    if (offset != 0) {
      length = textNode.textContent.length;
      statusStore.cursorRow = row;
      statusStore.cursorColumn = length;
      console.log("Outside bounds -> Row: " + row + " Column: " + length);
      cursorVOffset.value = e.target.offsetTop;
      cursorHOffset.value = textNode.clientWidth;
    }
  }
}
</script>
<template>
  <div
    class="bg-atom-bg min-h-full font-code antialiased text-lg absolute min-w-full overflow-visible h-fit w-fit"
    @wheel="wheel_event"
    @mousedown="click_event"
    @focusin=""
    @focusout=""
    :style="{ top: vOffset + 'px', left: hOffset + 'px' }"
  >
    <div class="" v-for="line in editorStore.content">
      <span class="inline-block">
        <span class="whitespace-pre" v-for="token in line">
          {{ token[1] }}
        </span>
      </span>
    </div>
  </div>
  <div
    ref="cursorElement"
    class="absolute font-code text-lg antialiased text-atom-primary bg-atom-text"
    :style="{
      top: vOffset + cursorVOffset + 'px',
      left: hOffset + cursorHOffset + 'px',
    }"
  >
    C
  </div>
</template>
