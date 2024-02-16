<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUpdated } from "vue";
import { invoke } from "@tauri-apps/api";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import Editor from "../editor";
import FileIO from "../io";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const editor = new Editor(editorStore, settingsStore, workspaceStore);
const fileIO = new FileIO(editorStore, settingsStore, workspaceStore);

const EditorPanelElement = ref<HTMLElement | null>(null);
const gutterElement = ref<HTMLElement | null>(null);
const editorElement = ref<HTMLElement | null>(null);
const cursorElement = ref<HTMLElement | null>(null);
const dummyElement = ref<HTMLElement | null>(null);
const hiddenInput = ref<HTMLElement | null>(null);

const hOffset = ref(0);
const vOffset = ref(0);
const cursorHOffset = ref(0);
const cursorVOffset = ref(0);
const selectionHighlights = ref<
  {
    vOffset: number;
    startHOffset: number;
    endHOffset?: number;
  }[]
>([]);
const gutterWidth = ref(0);
const cursorWidth = ref(0);
const cursorHeight = ref(0);
const visibleHeight = ref(
  EditorPanelElement.value?.getBoundingClientRect().height ?? 0,
);
const visibleContent = ref<string[][]>([]);
const visibleHOffset = ref(0);
const visibleVOffset = ref(0);
const maxVOffset = ref(0);
const startLine = ref(0);
const endLine = ref(0);

const selectionMade = computed(() => {
  return editor.selection_made();
});

let selecting = false;

onUpdated(() => {
  gutterWidth.value = gutterElement.value?.getBoundingClientRect().width ?? 0;
});

settingsStore.$subscribe(async () => {
  await nextTick();
  cursorWidth.value = dummyElement.value?.getBoundingClientRect().width ?? 0;
  cursorHeight.value = dummyElement.value?.getBoundingClientRect().height ?? 0;
  await updateVisibleContent();
});

workspaceStore.$subscribe(async () => {
  await nextTick();
  visibleHeight.value =
    EditorPanelElement.value?.getBoundingClientRect().height ?? 0;
  await updateVisibleContent();
});

editorStore.$subscribe(async () => {
  await nextTick();
  await updateVisibleContent();
});

onMounted(async () => {
  if (workspaceStore.currentEditorIndex != -1) {
    await switchBuffer(workspaceStore.currentEditorIndex);
    await nextTick();
    await updateVisibleContent();
  }
});

workspaceStore.$onAction((context) => {
  context.after(async () => {
    if (context.name === "switchEditor") {
      await switchBuffer(workspaceStore.currentEditorIndex);
    }
  });
});

class AsyncQueue {
  constructor() {
    this.queue = [];
    this.running = false;
  }

  enqueue(asyncFunction) {
    return new Promise((resolve, reject) => {
      this.queue.push({ asyncFunction, resolve, reject });
      this.run();
    });
  }

  async run() {
    if (this.running) return;

    this.running = true;

    while (this.queue.length > 0) {
      const { asyncFunction, resolve, reject } = this.queue.shift();

      try {
        const result = await asyncFunction();
        resolve(result);
      } catch (error) {
        reject(error);
      }
    }

    this.running = false;
  }
}

const asyncQueue = new AsyncQueue();

async function updateVisibleContent() {
  startLine.value = Math.floor(vOffset.value / cursorHeight.value);
  endLine.value =
    Math.ceil((vOffset.value + visibleHeight.value) / cursorHeight.value) - 1;
  visibleContent.value = editorStore.highlightedContent.slice(
    startLine.value,
    endLine.value + 1,
  );
  visibleVOffset.value = vOffset.value - startLine.value * cursorHeight.value;
  visibleHOffset.value = hOffset.value;
  maxVOffset.value =
    editorStore.highlightedContent.length * cursorHeight.value -
    visibleHeight.value +
    50;
  await nextTick();
  setCursorPosition();
}

async function switchBuffer(index: number) {
  await asyncQueue.enqueue(async () => {
    invoke<number>("create_buffer_from_file_path", {
      path: workspaceStore.openEditors[index].entry?.path,
    })
      .then((buffer_idx) => {
        editorStore.fileEntry = workspaceStore.openEditors[index].entry!;
        editorStore.encoding = "utf8";
        editorStore.bufferIdx = buffer_idx;
        invoke<IHighlightedText>("get_highlighted_text", {
          bufferIdx: editorStore.bufferIdx,
        }).then(async (content) => {
          editorStore.highlightedContent = content.text;
          const scroll = workspaceStore.openEditors[index].scroll;
          hOffset.value = scroll.hOffset;
          vOffset.value = scroll.vOffset;
          cursorHeight.value =
            dummyElement.value?.getBoundingClientRect().height ?? 0;
          cursorHeight.value =
            dummyElement.value?.getBoundingClientRect().height ?? 0;
          selecting = false;
          await nextTick();
          await updateVisibleContent();
        });
      })
      .catch((error) => {
        editorStore.encoding = "Unknown";
        console.error(error);
      });
  });
}

// Clamp value to the range from min to max
function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

// Scroll wheel event handler
function wheel_event(e: WheelEvent) {
  hOffset.value = clamp(
    hOffset.value - e.deltaX * 0.5,
    -e.currentTarget!.getBoundingClientRect().width +
      e.currentTarget!.parentNode.getBoundingClientRect().width,
    0,
  );
  vOffset.value = clamp(vOffset.value + e.deltaY * 0.5, 0, maxVOffset.value);
  workspaceStore.openEditors[workspaceStore.currentEditorIndex].scroll = {
    hOffset: hOffset.value,
    vOffset: vOffset.value,
  };
}

// Calculate offsets given row and column
// Expects row to be within startLine and endLine
function calculateOffsets(row: number, column: number) {
  let tempColumn = column;
  let tokens =
    editorElement.value!.children[row - startLine.value].firstChild
      ?.childNodes ?? [];
  let currentToken = null;
  for (let i = 0; i < tokens.length; i++) {
    const element = tokens[i];
    if (tempColumn <= (element.textContent?.length ?? 0)) {
      currentToken = element;
      break;
    }
    tempColumn -= element.textContent?.length ?? 0;
  }

  const verticalOffset =
    editorElement.value!.children[row - startLine.value].offsetTop +
    vOffset.value -
    visibleVOffset.value;
  if (currentToken!.firstChild != null) {
    let range = document.createRange();
    range.setStart(currentToken!.firstChild, 0);
    range.setEnd(currentToken!.firstChild, tempColumn);

    const horizontalOffset =
      range.getBoundingClientRect().right -
      cursorElement.value?.parentNode!.getBoundingClientRect().left -
      hOffset.value;
    return [verticalOffset, horizontalOffset];
  } else {
    const horizontalOffset = 0;
    return [verticalOffset, horizontalOffset];
  }
}

// Set cursor position at given row and column
function setCursorPosition() {
  const s = workspaceStore.currentSelection;

  // Setting cursor position
  if (s.end.row >= startLine.value && s.end.row <= endLine.value) {
    const cursorOffsets = calculateOffsets(s.end.row, s.end.column);
    cursorVOffset.value = cursorOffsets[0];
    cursorHOffset.value = cursorOffsets[1];
  }

  // Setting selection highlights
  if (editor.selection_made()) {
    let highlights = [];
    let start = s.start;
    let end = s.end;

    if (
      start.row > end.row ||
      (start.row == end.row && start.column > end.column)
    ) {
      let buf = start;
      start = end;
      end = buf;
    }

    if (!(end.row < startLine.value || start.row > endLine.value)) {
      let start_row = start.row;
      let end_row = end.row;
      let start_column = start.column;
      let end_column = end.column;
      if (start.row < startLine.value) {
        start_row = startLine.value;
        start_column = 0;
      }
      if (end.row > endLine.value) {
        end_row = endLine.value;
        end_column = -1;
      }

      const startOffsets = calculateOffsets(start_row, start_column);
      const endOffsets = calculateOffsets(
        end_row,
        end_column != -1 ? end_column : 0,
      );

      if (start_row == end_row) {
        highlights.push({
          vOffset: startOffsets[0],
          startHOffset: startOffsets[1],
          endHOffset: endOffsets[1],
        });
      } else {
        let offsets = calculateOffsets(start_row, 0);
        highlights.push({
          vOffset: startOffsets[0],
          startHOffset: startOffsets[1],
        });
        for (let i = start_row + 1; i < end_row; i++) {
          let startHOffset = calculateOffsets(i, 0)[1];
          offsets = calculateOffsets(i, 0);
          highlights.push({
            vOffset: offsets[0],
            startHOffset: startHOffset,
          });
        }
        if (end_column == -1) {
          offsets = calculateOffsets(end_row, 0);
          highlights.push({
            vOffset: endOffsets[0],
            startHOffset: offsets[1],
          });
        } else {
          offsets = calculateOffsets(end_row, 0);
          highlights.push({
            vOffset: endOffsets[0],
            startHOffset: offsets[1],
            endHOffset: endOffsets[1],
          });
        }
      }
    }

    selectionHighlights.value = highlights;
  } else {
    selectionHighlights.value = [];
  }

  hiddenInput.value?.focus();
}

function get_mouse_position(e: MouseEvent) {
  let range;
  let textNode;
  let offset;
  let row = 0;
  let column = 0;

  // Try to find text node and exact position in the text node where the user clicked
  if (document.caretPositionFromPoint) {
    range = document.caretPositionFromPoint(e.clientX, e.clientY);
    textNode = range.offsetNode;
    offset = range.offset;
  } else if (document.caretRangeFromPoint) {
    // Use WebKit-proprietary fallback method
    range = document.caretRangeFromPoint(e.clientX, e.clientY);
    textNode = range?.startContainer;
    offset = range?.startOffset;
  } else {
    console.error("Cannot handle click as caret APIs not supported");
    return {
      row: row,
      column: column,
    };
  }

  if (textNode?.nodeType === 3) {
    // If user clicked on text
    // Calculating row and column
    row = Math.floor(
      e.target!.parentNode.offsetTop /
        e.target!.parentNode.getBoundingClientRect().height,
    );

    let children = Array.from(
      editorElement.value!.children[row].firstChild!.children,
    );
    let current = children.indexOf(textNode.parentNode);
    column = offset;

    children.slice(0, current).forEach((child) => {
      column += child.textContent.length;
    });
    row += startLine.value;
  } else if (textNode?.nodeType === 1) {
    // If user clicked outside the text
    // Calculating row
    row = Math.floor(
      e.target!.offsetTop / e.target!.getBoundingClientRect().height,
    );
    if (offset != 0) {
      // If user did not click on the outer editor div
      // Setting column to end of line
      column = textNode.textContent.length;
    }
    row += startLine.value;
  }

  return {
    row: row,
    column: column,
  };
}

// Mouse click event handler
async function mouse_down(e: MouseEvent) {
  e.preventDefault();
  selecting = true;

  await asyncQueue.enqueue(async () => {
    let position = get_mouse_position(e);
    workspaceStore.updateSelection(
      position.row,
      position.column,
      position.row,
      position.column,
    );
  });
}

// Double click event handler
async function dblclick(e: MouseEvent) {
  e.preventDefault();

  await asyncQueue.enqueue(async () => {
    let position = get_mouse_position(e);
    workspaceStore.updateSelection(
      position.row,
      position.column,
      position.row,
      position.column,
    );
    await editor.get_token_under_cursor();
  });
}

// Mouse click event handler
async function mouse_move(e: MouseEvent) {
  e.preventDefault();

  if (selecting) {
    await asyncQueue.enqueue(async () => {
      let position = get_mouse_position(e);
      const s = workspaceStore.currentSelection;
      workspaceStore.updateSelection(
        s.start.row,
        s.start.column,
        position.row,
        position.column,
      );
    });
  }
}

// Mouse click event handler
async function mouse_up(e: MouseEvent) {
  e.preventDefault();
  selecting = false;

  await asyncQueue.enqueue(async () => {
    let position = get_mouse_position(e);
    const s = workspaceStore.currentSelection;
    workspaceStore.updateSelection(
      s.start.row,
      s.start.column,
      position.row,
      position.column,
    );
  });
}

// Keyboard event handler
async function key_event(e: KeyboardEvent) {
  e.preventDefault();

  await asyncQueue.enqueue(async () => {
    if (e.ctrlKey) {
      if (e.key === "c") {
        let selected_text = await editor.get_selected_text();
        await writeText(selected_text);
      } else if (e.key === "v") {
        const t = await readText();
        await editor.insert_character(t ?? "");
      } else if (e.key === "x") {
        let removed_text = await editor.remove_character();
        await writeText(removed_text);
      } else if (e.key === "z") {
        await editor.undo();
      } else if (e.key === "y") {
        await editor.redo();
      } else if (e.key === "s") {
        await fileIO.saveCurrent();
      } else if (e.key === "[") {
        await editor.remove_indentation();
      } else if (e.key === "]") {
        await editor.add_indentation();
      }
    } else if (e.key.length == 1 || e.key === "Enter") {
      let key = e.key;
      let indentSize = 0;
      if (key === "Enter") {
        key = "\n";
        indentSize = await editor.get_indent_size();
      }
      await editor.insert_character(key);
      if (e.key === "Enter") {
        await editor.add_indentation(indentSize);
      }
    } else if (e.key === "Tab") {
      const spacing = " ".repeat(settingsStore.tabSize);
      await editor.insert_character(spacing);
    } else if (e.key === "Backspace") {
      await editor.remove_character();
    } else if (e.key === "Delete") {
      await editor.remove_character(true);
    } else if (e.key === "ArrowUp") {
      await editor.move_cursor_up();
    } else if (e.key === "ArrowLeft") {
      await editor.move_cursor_left();
    } else if (e.key === "ArrowDown") {
      await editor.move_cursor_down();
    } else if (e.key === "ArrowRight") {
      await editor.move_cursor_right();
    } else if (e.key === "Home") {
      await editor.move_cursor_line_start();
    } else if (e.key === "End") {
      await editor.move_cursor_line_end();
    }
  });
}
</script>
<template>
  <div class="h-full" v-if="workspaceStore.currentEditorIndex != -1">
    <div ref="EditorPanelElement" class="flex h-full">
      <div
        class="h-full relative bg-atom-bg z-20 w-0"
        :style="{ width: gutterWidth + 'px' }"
      >
        <div
          ref="gutterElement"
          class="min-h-full font-code antialiased leading-normal overflow-visible h-fit pointer-events-none select-none absolute w-fit text-atom-text-light"
          :style="{
            top: -visibleVOffset + 'px',
            'font-size': settingsStore.editorFontSize + 'px',
          }"
        >
          <div
            class="px-5"
            v-for="(_, index) in visibleContent"
            :key="index"
            :class="{
              'text-atom-text-dark bg-atom-bg-light':
                startLine + index ==
                  workspaceStore.currentSelection.start.row && !selectionMade,
            }"
          >
            <span class="inline-block">
              {{ startLine + index + 1 }}
            </span>
          </div>
          <div class="px-5 opacity-0">
            <span class="inline-block">
              {{ editorStore.highlightedContent.length + 1 }}
            </span>
          </div>
        </div>
      </div>
      <div class="flex-1 h-full relative">
        <div
          ref="editorElement"
          class="bg-transparent min-h-full font-code antialiased leading-normal absolute min-w-full overflow-visible h-fit w-fit cursor-text z-10 select-none"
          @wheel="wheel_event"
          @mousedown="mouse_down"
          @dblclick="dblclick"
          @mousemove="mouse_move"
          @mouseup="mouse_up"
          :style="{
            top: -visibleVOffset + 'px',
            left: visibleHOffset + 'px',
            'font-size': settingsStore.editorFontSize + 'px',
          }"
        >
          <div
            class=""
            v-for="(line, index) in visibleContent"
            :key="index"
            :class="{
              'text-atom-text-dark bg-atom-bg-light':
                startLine + index ==
                  workspaceStore.currentSelection.start.row && !selectionMade,
            }"
          >
            <span class="inline-block whitespace-pre">
              <span
                :class="['whitespace-pre', 'text-atom-highlight-' + token[0]]"
                v-for="(token, index) in line"
                :key="index"
              >
                {{ token[1] }}
              </span>
            </span>
          </div>
        </div>
        <div
          ref="cursorElement"
          class="absolute font-code antialiased leading-normal border-atom-primary border-l-2 pointer-events-none select-none animate-blink z-20"
          :style="{
            top: cursorVOffset - vOffset + 'px',
            left: visibleHOffset + cursorHOffset + 'px',
            width: cursorWidth + 'px',
            height: cursorHeight + 'px',
            'font-size': settingsStore.editorFontSize + 'px',
          }"
        >
          <input
            ref="hiddenInput"
            class="opacity-0"
            tabindex="-1"
            type="text"
            @keydown="key_event"
            :style="{
              width: 1 + 'px',
              height: cursorHeight + 'px',
            }"
          />
        </div>
        <div
          ref="highlightElement"
          class="absolute font-code antialiased leading-normal pointer-events-none select-none h-full w-full bg-atom-bg"
          :style="{
            'font-size': settingsStore.editorFontSize + 'px',
          }"
        >
          <div
            class="absolute bg-atom-highlight z-0"
            :style="{
              top: -vOffset + highlight.vOffset + 'px',
              left: visibleHOffset + highlight.startHOffset + 'px',
              height: cursorHeight + 'px',
              width: highlight.endHOffset
                ? highlight.endHOffset - highlight.startHOffset + 'px'
                : '100%',
            }"
            v-for="(highlight, index) in selectionHighlights"
            :key="index"
          ></div>
        </div>
        <div
          class="absolute invisible whitespace-pre leading-normal text-atom-highlight-None text-atom-highlight-White text-atom-highlight-Red text-atom-highlight-Orange text-atom-highlight-Blue text-atom-highlight-Green text-atom-highlight-Purple text-atom-highlight-Yellow text-atom-highlight-Gray text-atom-highlight-Turquoise"
          ref="dummyElement"
          :style="{
            'font-size': settingsStore.editorFontSize + 'px',
          }"
        >
          <span>x</span>
        </div>
      </div>
    </div>
  </div>
</template>
