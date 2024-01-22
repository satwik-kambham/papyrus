<script setup lang="ts">
import { computed, ref, nextTick, onUpdated } from "vue";
import { invoke } from "@tauri-apps/api";
import { useWorkspaceStore } from "../stores/workspace";
import { useEditorStore } from "../stores/editor";
import { useSettingsStore } from "../stores/settings";
import { readText, writeText } from "@tauri-apps/api/clipboard";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const settingsStore = useSettingsStore();

const gutterElement = ref<HTMLElement | null>(null);
const editorElement = ref<HTMLElement | null>(null);
const cursorElement = ref<HTMLElement | null>(null);
const dummyElement = ref<HTMLElement | null>(null);
const hiddenInput = ref<HTMLElement | null>(null);

const currentLine = ref<Element | null>(null);
const currentGutterLine = ref<Element | null>(null);
const hOffset = ref(0);
const vOffset = ref(0);
const cursorHOffset = ref(0);
const cursorVOffset = ref(0);
const selectionHighlights = ref<
  {
    vOffset: number;
    startHOffset: number;
    endHOffset: number;
  }[]
>([]);

let selecting = false;

const gutterWidth = ref(0);

onUpdated(() => {
  gutterWidth.value = gutterElement.value?.clientWidth ?? 0;
});

// Setting cursor width based on a dummy element
const cursorWidth = computed(() => {
  return dummyElement.value?.offsetWidth;
});
const cursorHeight = computed(() => {
  return dummyElement.value?.offsetHeight;
});

function resetState() {
  if (currentLine.value != null)
    currentLine.value!.classList.remove("bg-atom-bg-light");
  if (currentGutterLine.value != null) {
    currentGutterLine.value!.classList.remove("bg-atom-bg-light");
    currentGutterLine.value!.classList.remove("text-atom-text-dark");
  }

  currentLine.value = null;
  currentGutterLine.value = null;
  hOffset.value = 0;
  vOffset.value = 0;
  cursorHOffset.value = 0;
  cursorVOffset.value = 0;
  selectionHighlights.value = [];

  selecting = false;

  gutterWidth.value = 0;
}

async function switchBuffer(index: number) {
  await asyncQueue.enqueue(async () => {
    resetState();
    workspaceStore.currentEditorIndex = index;
    invoke<number>("create_buffer_from_file_path", {
      path: workspaceStore.openEditors[index].entry?.path,
    })
      .then((buffer_idx) => {
        editorStore.fileEntry = workspaceStore.openEditors[index].entry!;
        editorStore.encoding = "utf8";
        editorStore.bufferIdx = buffer_idx;
        invoke<IHighlightedText>("get_highlighted_text", {
          bufferIdx: editorStore.bufferIdx,
        }).then((content) => {
          editorStore.highlightedContent = content.text;
        });
        const s = workspaceStore.currentSelection;
        setCursorPosition(s.end.row, s.end.column);
      })
      .catch((error) => {
        editorStore.encoding = "Unknown";
        console.error(error);
      });
  });
}

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

// Clamp value to the range from min to max
function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

// Scroll wheel event handler
function wheel_event(e: WheelEvent) {
  hOffset.value = clamp(
    hOffset.value - e.deltaX * 0.5,
    -e.currentTarget!.clientWidth + e.currentTarget!.parentNode.clientWidth,
    0,
  );

  vOffset.value = clamp(
    vOffset.value - e.deltaY * 0.5,
    -e.currentTarget!.clientHeight + e.currentTarget!.parentNode.clientHeight,
    0,
  );
}

function selection_made() {
  const s = workspaceStore.currentSelection;
  return s.start.row != s.end.row || s.start.column != s.end.column;
}

// Calculate offsets given row and column
async function calculateOffsets(
  row: number,
  column: number,
): Promise<number[]> {
  let tempColumn = column;
  let tokens = editorElement.value!.children[row].firstChild?.childNodes ?? [];
  let currentToken = null;
  for (let i = 0; i < tokens.length; i++) {
    const element = tokens[i];
    if (tempColumn <= (element.textContent?.length ?? 0)) {
      currentToken = element;
      break;
    }
    tempColumn -= element.textContent?.length ?? 0;
  }

  if (currentToken!.firstChild != null) {
    let range = document.createRange();
    range.setStart(currentToken!.firstChild, 0);
    range.setEnd(currentToken!.firstChild, tempColumn);

    const verticalOffset = editorElement.value!.children[row].offsetTop;
    const horizontalOffset =
      range.getBoundingClientRect().right -
      cursorElement.value?.parentNode!.offsetLeft -
      hOffset.value;
    return [verticalOffset, horizontalOffset];
  } else {
    const verticalOffset = editorElement.value!.children[row].offsetTop;
    const horizontalOffset = 0;
    return [verticalOffset, horizontalOffset];
  }
}

// Set cursor position at given row and column
async function setCursorPosition(row: number, column: number) {
  await nextTick();

  const cursorOffsets = await calculateOffsets(row, column);
  cursorVOffset.value = cursorOffsets[0];
  cursorHOffset.value = cursorOffsets[1];

  if (currentLine.value != null) {
    currentLine.value.classList.remove("bg-atom-bg-light");
    currentGutterLine.value!.classList.remove("bg-atom-bg-light");
    currentGutterLine.value!.classList.remove("text-atom-text-dark");
  }
  currentLine.value = editorElement.value!.children[row];
  currentGutterLine.value = gutterElement.value!.children[row];

  let highlights = [];
  if (selection_made()) {
    const s = workspaceStore.currentSelection;
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

    const startOffsets = await calculateOffsets(start.row, start.column);
    const endOffsets = await calculateOffsets(end.row, end.column);

    if (start.row == end.row) {
      highlights.push({
        vOffset: startOffsets[0],
        startHOffset: startOffsets[1],
        endHOffset: endOffsets[1],
      });
    } else {
      let offsets = await calculateOffsets(
        start.row,
        await get_row_length(start.row),
      );
      highlights.push({
        vOffset: startOffsets[0],
        startHOffset: startOffsets[1],
        endHOffset: offsets[1],
      });
      for (let i = start.row + 1; i < end.row; i++) {
        let startHOffset = (await calculateOffsets(i, 0))[1];
        offsets = await calculateOffsets(i, await get_row_length(i));
        highlights.push({
          vOffset: offsets[0],
          startHOffset: startHOffset,
          endHOffset: offsets[1],
        });
      }
      offsets = await calculateOffsets(end.row, 0);
      highlights.push({
        vOffset: endOffsets[0],
        startHOffset: offsets[1],
        endHOffset: endOffsets[1],
      });
    }
  } else {
    currentLine.value.classList.add("bg-atom-bg-light");
    currentGutterLine.value.classList.add("bg-atom-bg-light");
    currentGutterLine.value.classList.add("text-atom-text-dark");
  }
  selectionHighlights.value = highlights;

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
      e.target!.parentNode.offsetTop / e.target!.parentNode.clientHeight,
    );

    let children = Array.from(
      editorElement.value!.children[row].firstChild!.children,
    );
    let current = children.indexOf(textNode.parentNode);
    column = offset;

    children.slice(0, current).forEach((child) => {
      column += child.textContent.length;
    });
  } else if (textNode?.nodeType === 1) {
    // If user clicked outside the text
    // Calculating row
    row = Math.floor(e.target!.offsetTop / e.target!.clientHeight);
    if (offset != 0) {
      // If user did not click on the outer editor div
      // Setting column to end of line
      column = textNode.textContent.length;
    }
  }

  return {
    row: row,
    column: column,
  };
}

// Insert character after cursor
async function insert_character(character: string) {
  const s = workspaceStore.currentSelection;
  if (selection_made()) {
    await remove_character();
  }
  let update = await invoke("insert_text", {
    bufferIdx: editorStore.bufferIdx,
    text: character,
    cursor: {
      row: s.end.row,
      column: s.end.column,
    },
  });
  editorStore.highlightedContent = update[0].text;
  workspaceStore.updateSelection(
    update[1].row,
    update[1].column,
    update[1].row,
    update[1].column,
  );
  await setCursorPosition(update[1].row, update[1].column);
}

// Remove character before cursor
async function remove_character() {
  const sel = workspaceStore.currentSelection;
  if (sel.end.row != 0 || sel.end.column != 0) {
    let s;
    if (selection_made()) {
      let start = {
        row: sel.start.row,
        column: sel.start.column,
      };
      let end = {
        row: sel.end.row,
        column: sel.end.column,
      };

      if (
        start.row > end.row ||
        (start.row == end.row && start.column > end.column)
      ) {
        let buf = start;
        start = end;
        end = buf;
      }

      s = {
        start: start,
        end: end,
      };
    } else {
      if (sel.end.column != 0) {
        s = {
          start: {
            row: sel.end.row,
            column: sel.end.column - 1,
          },
          end: {
            row: sel.end.row,
            column: sel.end.column,
          },
        };
      } else {
        let prev_row_length = await get_row_length(sel.end.row - 1);
        s = {
          start: {
            row: sel.end.row - 1,
            column: prev_row_length,
          },
          end: {
            row: sel.end.row,
            column: 0,
          },
        };
      }
    }
    let update = await invoke("remove_text", {
      bufferIdx: editorStore.bufferIdx,
      selection: s,
    });
    editorStore.highlightedContent = update[0].text;
    let removed_text = update[1];
    workspaceStore.updateSelection(
      update[2].row,
      update[2].column,
      update[2].row,
      update[2].column,
    );
    await setCursorPosition(update[2].row, update[2].column);
    return removed_text;
  }
}

// Undo last action
async function undo() {
  let update = await invoke("undo", {
    bufferIdx: editorStore.bufferIdx,
  });
  if (update != null) {
    editorStore.highlightedContent = update[0].text;
    workspaceStore.updateSelection(
      update[1].row,
      update[1].column,
      update[1].row,
      update[1].column,
    );
    await setCursorPosition(update[1].row, update[1].column);
  }
}

// Redo last action
async function redo() {
  let update = await invoke("redo", {
    bufferIdx: editorStore.bufferIdx,
  });
  if (update != null) {
    editorStore.highlightedContent = update[0].text;
    workspaceStore.updateSelection(
      update[1].row,
      update[1].column,
      update[1].row,
      update[1].column,
    );
    await setCursorPosition(update[1].row, update[1].column);
  }
}

// Remove character before cursor
async function get_selected_text() {
  if (selection_made()) {
    let sel = workspaceStore.currentSelection;
    let start = sel.start;
    let end = sel.end;

    if (
      start.row > end.row ||
      (start.row == end.row && start.column > end.column)
    ) {
      let buf = start;
      start = end;
      end = buf;
    }

    let s = {
      start: start,
      end: end,
    };

    let selected_text = await invoke<string>("get_selected_text", {
      bufferIdx: editorStore.bufferIdx,
      selection: s,
    });
    return selected_text;
  }
  return "";
}

// Get lines length (total rows)
async function get_lines_length() {
  let lines_length = await invoke<number>("get_lines_length", {
    bufferIdx: editorStore.bufferIdx,
  });
  return lines_length;
}

// Get row length
async function get_row_length(row_number: number) {
  let row_length = await invoke<number>("get_row_length", {
    bufferIdx: editorStore.bufferIdx,
    row: row_number,
  });
  return row_length;
}

// Move cursor up
async function move_cursor_up() {
  const s = workspaceStore.currentSelection;
  if (s.end.row == 0) {
    await move_cursor_line_start();
  } else {
    let column = Math.min(s.end.column, await get_row_length(s.end.row - 1));
    workspaceStore.updateSelection(
      s.end.row - 1,
      column,
      s.end.row - 1,
      column,
    );
    await setCursorPosition(s.end.row - 1, column);
  }
}

// Move cursor down
async function move_cursor_down() {
  const s = workspaceStore.currentSelection;
  if (s.end.row == (await get_lines_length()) - 1) {
    await move_cursor_line_end();
  } else {
    let column = Math.min(s.end.column, await get_row_length(s.end.row + 1));
    workspaceStore.updateSelection(
      s.end.row + 1,
      column,
      s.end.row + 1,
      column,
    );
    await setCursorPosition(s.end.row + 1, column);
  }
}

// Move cursor left
async function move_cursor_left() {
  const s = workspaceStore.currentSelection;
  if (selection_made()) {
    workspaceStore.updateSelection(
      s.end.row,
      s.end.column,
      s.end.row,
      s.end.column,
    );
    await setCursorPosition(s.end.row, s.end.column);
  } else {
    if (s.end.column == 0) {
      // Move to end of previous line
      if (s.end.row != 0) {
        const column = await get_row_length(s.end.row - 1);
        workspaceStore.updateSelection(
          s.end.row - 1,
          column,
          s.end.row - 1,
          column,
        );
        await setCursorPosition(s.end.row - 1, column);
      }
    } else {
      workspaceStore.updateSelection(
        s.end.row,
        s.end.column - 1,
        s.end.row,
        s.end.column - 1,
      );
      await setCursorPosition(s.end.row, s.end.column - 1);
    }
  }
}

// Move cursor right
async function move_cursor_right() {
  const s = workspaceStore.currentSelection;
  if (selection_made()) {
    workspaceStore.updateSelection(
      s.end.row,
      s.end.column,
      s.end.row,
      s.end.column,
    );
    await setCursorPosition(s.end.row, s.end.column);
  } else {
    if (s.end.column == (await get_row_length(s.end.row))) {
      // Move to start of next line
      if (s.end.row != (await get_lines_length()) - 1) {
        const column = 0;
        workspaceStore.updateSelection(
          s.end.row + 1,
          column,
          s.end.row + 1,
          column,
        );
        await setCursorPosition(s.end.row + 1, column);
      }
    } else {
      workspaceStore.updateSelection(
        s.end.row,
        s.end.column + 1,
        s.end.row,
        s.end.column + 1,
      );
      await setCursorPosition(s.end.row, s.end.column + 1);
    }
  }
}

// Move cursor to start of line
async function move_cursor_line_start() {
  const s = workspaceStore.currentSelection;
  workspaceStore.updateSelection(s.end.row, 0, s.end.row, 0);
  await setCursorPosition(s.end.row, 0);
}

// Move cursor to end of line
async function move_cursor_line_end() {
  const s = workspaceStore.currentSelection;
  let column = await get_row_length(s.end.row);
  workspaceStore.updateSelection(s.end.row, column, s.end.row, column);
  await setCursorPosition(s.end.row, column);
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
    await setCursorPosition(position.row, position.column);
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
      await setCursorPosition(position.row, position.column);
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
    await setCursorPosition(position.row, position.column);
  });
}

// Keyboard event handler
async function key_event(e: KeyboardEvent) {
  e.preventDefault();

  await asyncQueue.enqueue(async () => {
    if (e.ctrlKey) {
      if (e.key === "c") {
        let selected_text = await get_selected_text();
        await writeText(selected_text);
      } else if (e.key === "v") {
        const t = await readText();
        await insert_character(t ?? "");
      } else if (e.key === "x") {
        let removed_text = await remove_character();
        await writeText(removed_text);
      } else if (e.key === "z") {
        await undo();
      } else if (e.key === "y") {
        await redo();
      }
    } else if (e.key.length == 1 || e.key === "Enter") {
      let key = e.key;
      if (key === "Enter") key = "\n";
      await insert_character(key);
    } else if (e.key === "Backspace") {
      await remove_character();
    } else if (e.key === "ArrowUp") {
      await move_cursor_up();
    } else if (e.key === "ArrowLeft") {
      await move_cursor_left();
    } else if (e.key === "ArrowDown") {
      await move_cursor_down();
    } else if (e.key === "ArrowRight") {
      await move_cursor_right();
    } else if (e.key === "Home") {
      await move_cursor_line_start();
    } else if (e.key === "End") {
      await move_cursor_line_end();
    }
  });
}
</script>
<template>
  <div class="flex flex-col h-full">
    <div class="flex overflow-auto custom-scrollbar z-30 bg-atom-bg">
      <div
        class="px-2 py-1 border-x-2 border-atom-bg-light whitespace-nowrap cursor-pointer select-none"
        v-for="(editor, index) in workspaceStore.openEditors"
        :key="index"
        @click="async () => await switchBuffer(index)"
        :class="{
          'bg-atom-bg-light': index == workspaceStore.currentEditorIndex,
        }"
      >
        {{ editor.entry?.name }}
      </div>
    </div>
    <div class="flex h-full">
      <div
        class="h-full relative bg-atom-bg z-20 w-0"
        :style="{ width: gutterWidth + 'px' }"
      >
        <div
          ref="gutterElement"
          class="min-h-full font-code antialiased text-xl overflow-visible h-fit pointer-events-none select-none absolute w-fit text-atom-text-light"
          :style="{ top: vOffset + 'px' }"
        >
          <div
            class="px-5"
            v-for="(_, index) in editorStore.highlightedContent"
            :key="index"
          >
            <span class="inline-block">
              {{ index + 1 }}
            </span>
          </div>
        </div>
      </div>
      <div class="flex-1 h-full relative">
        <div
          ref="editorElement"
          class="bg-transparent min-h-full font-code antialiased text-xl absolute min-w-full overflow-visible h-fit w-fit cursor-text z-10 select-none"
          @wheel="wheel_event"
          @mousedown="mouse_down"
          @mousemove="mouse_move"
          @mouseup="mouse_up"
          :style="{ top: vOffset + 'px', left: hOffset + 'px' }"
        >
          <div
            class=""
            v-for="(line, index) in editorStore.highlightedContent"
            :key="index"
          >
            <span class="inline-block">
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
          class="absolute font-code text-xl antialiased border-atom-primary border-l-2 pointer-events-none select-none animate-blink z-20"
          :style="{
            top: vOffset + cursorVOffset + 'px',
            left: hOffset + cursorHOffset + 'px',
            width: cursorWidth + 'px',
            height: cursorHeight + 'px',
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
          class="absolute font-code text-xl antialiased pointer-events-none select-none h-full w-full bg-atom-bg"
        >
          <div
            class="absolute bg-atom-highlight z-0"
            :style="{
              top: vOffset + highlight.vOffset + 'px',
              left: hOffset + highlight.startHOffset + 'px',
              width: highlight.endHOffset - highlight.startHOffset + 'px',
              height: cursorHeight + 'px',
            }"
            v-for="(highlight, index) in selectionHighlights"
            :key="index"
          ></div>
        </div>
        <div
          class="absolute invisible whitespace-pre text-xl text-atom-highlight-None text-atom-highlight-White text-atom-highlight-Red text-atom-highlight-Orange text-atom-highlight-Blue text-atom-highlight-Green text-atom-highlight-Purple text-atom-highlight-Yellow text-atom-highlight-Gray text-atom-highlight-Turquoise"
          ref="dummyElement"
        >
          <span>x</span>
        </div>
      </div>
    </div>
  </div>
</template>
