<script setup lang="ts">
import { computed, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api";
import { useEditorStore } from "../stores/editor";
import { useStatusStore } from "../stores/status";

const statusStore = useStatusStore();
const editorStore = useEditorStore();

const editorElement = ref(null);
const cursorElement = ref(null);
const dummyElement = ref(null);
const hiddenInput = ref(null);
const currentLine = ref(null);
const hOffset = ref(0);
const vOffset = ref(0);
const cursorHOffset = ref(0);
const cursorVOffset = ref(0);
const selectionHighlights = ref([]);

let selecting = false;

// Setting cursor width based on a dummy element
const cursorWidth = computed(() => {
  return dummyElement.value?.offsetWidth;
});
const cursorHeight = computed(() => {
  return dummyElement.value?.offsetHeight;
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

// Clamp value to the range from min to max
function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

// Scroll wheel event handler
function wheel_event(e) {
  hOffset.value = clamp(
    hOffset.value - e.deltaX * 0.5,
    -e.currentTarget.clientWidth + e.currentTarget.parentNode.clientWidth,
    0
  );

  vOffset.value = clamp(
    vOffset.value - e.deltaY * 0.5,
    -e.currentTarget.clientHeight + e.currentTarget.parentNode.clientHeight,
    0
  );
}

function selection_made() {
  return (
    statusStore.startCursorRow != statusStore.cursorRow ||
    statusStore.startCursorColumn != statusStore.cursorColumn
  );
}

// Calculate offsets given row and column
async function calculateOffsets(row, column) {
  let tempColumn = column;
  let tokens = editorElement.value.children[row].firstChild.children;
  let currentToken = null;
  for (let i = 0; i < tokens.length; i++) {
    const element = tokens[i];
    if (tempColumn <= element.textContent.length) {
      currentToken = element;
      break;
    }
    tempColumn -= element.textContent.length;
  }

  if (currentToken.firstChild != null) {
    let range = document.createRange();
    range.setStart(currentToken.firstChild, 0);
    range.setEnd(currentToken.firstChild, tempColumn);

    const verticalOffset = editorElement.value.children[row].offsetTop;
    const horizontalOffset =
      range.getBoundingClientRect().right -
      cursorElement.value?.parentNode.offsetLeft -
      hOffset.value;
    return [verticalOffset, horizontalOffset];
  } else {
    const verticalOffset = editorElement.value.children[row].offsetTop;
    const horizontalOffset = 0;
    return [verticalOffset, horizontalOffset];
  }
}

// Set cursor position at given row and column
async function setCursorPosition(row, column) {
  await nextTick();

  const cursorOffsets = await calculateOffsets(row, column);
  cursorVOffset.value = cursorOffsets[0];
  cursorHOffset.value = cursorOffsets[1];

  if (currentLine.value != null) {
    currentLine.value.classList.remove("bg-atom-bg-light");
  }
  currentLine.value = editorElement.value.children[row];

  let highlights = [];
  if (selection_made()) {
    let start = {
      row: statusStore.startCursorRow,
      column: statusStore.startCursorColumn,
    };
    let end = {
      row: statusStore.cursorRow,
      column: statusStore.cursorColumn,
    };

    if (!(start.row < end.row || start.column < end.column)) {
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
        await get_row_length(start.row)
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
    selectionHighlights.value = highlights;
  } else {
    currentLine.value.classList.add("bg-atom-bg-light");
  }

  hiddenInput.value?.focus();
}

function get_mouse_position(e) {
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
      e.target.parentNode.offsetTop / e.target.parentNode.clientHeight
    );

    let children = Array.from(
      editorElement.value.children[row].firstChild.children
    );
    let current = children.indexOf(textNode.parentNode);
    column = offset;

    children.slice(0, current).forEach((child) => {
      column += child.textContent.length;
    });
  } else if (textNode?.nodeType === 1) {
    // If user clicked outside the text
    // Calculating row
    row = Math.floor(e.target.offsetTop / e.target.clientHeight);
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
async function insert_character(character) {
  let update = await invoke("insert_text", {
    text: character,
    cursor: {
      row: statusStore.cursorRow,
      column: statusStore.cursorColumn,
    },
  });
  editorStore.content = update[0].text;
  statusStore.cursorRow = update[1].row;
  statusStore.cursorColumn = update[1].column;
  await setCursorPosition(update[1].row, update[1].column);
}

// Remove character before cursor
async function remove_character() {
  if (statusStore.cursorRow != 0 || statusStore.cursorColumn != 0) {
    let s;
    if (statusStore.cursorColumn != 0) {
      s = {
        start: {
          row: statusStore.cursorRow,
          column: statusStore.cursorColumn - 1,
        },
        end: {
          row: statusStore.cursorRow,
          column: statusStore.cursorColumn,
        },
      };
    } else {
      let prev_row_length = await get_row_length(statusStore.cursorRow - 1);
      s = {
        start: {
          row: statusStore.cursorRow - 1,
          column: prev_row_length,
        },
        end: {
          row: statusStore.cursorRow,
          column: 0,
        },
      };
    }
    let update = await invoke("remove_text", {
      selection: s,
    });
    editorStore.content = update[0].text;
    let removed_text = update[1];
    statusStore.cursorRow = update[2].row;
    statusStore.cursorColumn = update[2].column;
    await setCursorPosition(update[2].row, update[2].column);
  }
}

// Get lines length (total rows)
async function get_lines_length() {
  let lines_length = await invoke("get_lines_length");
  return lines_length;
}

// Get row length
async function get_row_length(row_number) {
  let row_length = await await invoke("get_row_length", {
    row: row_number,
  });
  return row_length;
}

// Move cursor up
async function move_cursor_up() {
  if (statusStore.cursorRow == 0) {
    await move_cursor_line_start();
  } else {
    let column = Math.min(
      statusStore.cursorColumn,
      await get_row_length(statusStore.cursorRow - 1)
    );
    statusStore.cursorRow -= 1;
    statusStore.cursorColumn = column;
    await setCursorPosition(statusStore.cursorRow - 1, column);
  }
}

// Move cursor down
async function move_cursor_down() {
  if (statusStore.cursorRow == (await get_lines_length()) - 1) {
    await move_cursor_line_end();
  } else {
    let column = Math.min(
      statusStore.cursorColumn,
      await get_row_length(statusStore.cursorRow + 1)
    );
    statusStore.cursorRow += 1;
    statusStore.cursorColumn = column;
    await setCursorPosition(statusStore.cursorRow + 1, column);
  }
}

// Move cursor left
async function move_cursor_left() {
  if (statusStore.cursorColumn == 0) {
    // Move to end of previous line
    if (statusStore.cursorRow != 0) {
      let column = await get_row_length(statusStore.cursorRow - 1);
      statusStore.cursorRow -= 1;
      statusStore.cursorColumn = column;
      await setCursorPosition(statusStore.cursorRow - 1, column);
    }
  } else {
    statusStore.cursorColumn -= 1;
    await setCursorPosition(
      statusStore.cursorRow,
      statusStore.cursorColumn - 1
    );
  }
}

// Move cursor right
async function move_cursor_right() {
  if (
    statusStore.cursorColumn == (await get_row_length(statusStore.cursorRow))
  ) {
    // Move to start of next line
    if (statusStore.cursorRow != (await get_lines_length()) - 1) {
      let column = 0;
      statusStore.cursorRow += 1;
      statusStore.cursorColumn = column;
      await setCursorPosition(statusStore.cursorRow + 1, column);
    }
  } else {
    statusStore.cursorColumn += 1;
    await setCursorPosition(
      statusStore.cursorRow,
      statusStore.cursorColumn + 1
    );
  }
}

// Move cursor to start of line
async function move_cursor_line_start() {
  statusStore.cursorColumn = 0;
  await setCursorPosition(statusStore.cursorRow, 0);
}

// Move cursor to end of line
async function move_cursor_line_end() {
  let column = await get_row_length(statusStore.cursorRow);
  statusStore.cursorColumn = column;
  await setCursorPosition(statusStore.cursorRow, column);
}

// Mouse click event handler
async function mouse_down(e) {
  e.preventDefault();
  selecting = true;

  await asyncQueue.enqueue(async () => {
    let position = get_mouse_position(e);

    await setCursorPosition(position.row, position.column);
    statusStore.startCursorRow = position.row;
    statusStore.startCursorColumn = position.column;
    statusStore.cursorRow = position.row;
    statusStore.cursorColumn = position.column;
  });
}

// Mouse click event handler
async function mouse_move(e) {
  e.preventDefault();

  if (selecting) {
    await asyncQueue.enqueue(async () => {
      let position = get_mouse_position(e);

      statusStore.cursorRow = position.row;
      statusStore.cursorColumn = position.column;
      await setCursorPosition(position.row, position.column);
    });
  }
}

// Mouse click event handler
async function mouse_up(e) {
  e.preventDefault();
  selecting = false;

  await asyncQueue.enqueue(async () => {
    let position = get_mouse_position(e);

    statusStore.cursorRow = position.row;
    statusStore.cursorColumn = position.column;
    await setCursorPosition(position.row, position.column);
  });
}

// Keyboard event handler
async function key_event(e) {
  e.preventDefault();

  await asyncQueue.enqueue(async () => {
    if (e.key.length == 1 || e.key === "Enter") {
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
  <div
    ref="editorElement"
    class="bg-transparent min-h-full font-code antialiased text-xl absolute min-w-full overflow-visible h-fit w-fit cursor-text z-10"
    @wheel="wheel_event"
    @mousedown="mouse_down"
    @mousemove="mouse_move"
    @mouseup="mouse_up"
    :style="{ top: vOffset + 'px', left: hOffset + 'px' }"
  >
    <div class="" v-for="line in editorStore.content">
      <span class="inline-block">
        <span
          :class="['whitespace-pre', 'text-atom-highlight-' + token[0]]"
          v-for="token in line"
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
      v-for="highlight in selectionHighlights"
    ></div>
  </div>
  <div
    class="absolute invisible whitespace-pre text-xl text-atom-highlight-None text-atom-highlight-White text-atom-highlight-Red text-atom-highlight-Orange text-atom-highlight-Blue text-atom-highlight-Green text-atom-highlight-Purple text-atom-highlight-Yellow text-atom-highlight-Gray text-atom-highlight-Turquoise"
    ref="dummyElement"
  >
    <span>x</span>
  </div>
</template>
