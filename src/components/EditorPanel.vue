<script setup lang="ts">
import { computed, ref } from "vue";
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

// Setting cursor width based on a dummy element
const cursorWidth = computed(() => {
  return dummyElement.value?.offsetWidth;
});
const cursorHeight = computed(() => {
  return dummyElement.value?.offsetHeight;
});

// Clamp value to the range from min to max
function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

// Scroll wheel event handler
function wheel_event(e) {
  hOffset.value = clamp(
    hOffset.value - e.deltaX * 0.3,
    -e.currentTarget.clientWidth + e.currentTarget.parentNode.clientWidth,
    0
  );

  vOffset.value = clamp(
    vOffset.value - e.deltaY * 0.3,
    -e.currentTarget.clientHeight + e.currentTarget.parentNode.clientHeight,
    0
  );
}

// Set cursor position at given row and column
function setCursorPosition(row, column) {
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

    cursorVOffset.value = editorElement.value.children[row].offsetTop;
    cursorHOffset.value =
      range.getBoundingClientRect().right -
      cursorElement.value?.parentNode.offsetLeft -
      hOffset.value;
  } else {
    cursorVOffset.value = editorElement.value.children[row].offsetTop;
    cursorHOffset.value = 0;
  }

  if (currentLine.value != null) {
    currentLine.value.classList.remove("bg-atom-bg-light");
  }
  currentLine.value = editorElement.value.children[row];
  currentLine.value.classList.add("bg-atom-bg-light");

  hiddenInput.value?.focus();
}

// Mouse click event handler
function click_event(e) {
  e.preventDefault();

  let range;
  let textNode;
  let offset;
  let boundingRect;
  let row = 0;
  let column = 0;

  // Try to find text node and exact position in the text node where the user clicked
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

    console.log("Row: " + row + " Column: " + column);
    statusStore.cursorRow = row;
    statusStore.cursorColumn = column;
  } else if (textNode?.nodeType === 1) {
    // If user clicked outside the text
    // Calculating row
    row = Math.floor(e.target.offsetTop / e.target.clientHeight);
    if (offset != 0) {
      // If user did not click on the outer editor div
      // Setting column to end of line
      column = textNode.textContent.length;

      console.log("Outside bounds -> Row: " + row + " Column: " + column);
      statusStore.cursorRow = row;
      statusStore.cursorColumn = column;
    }
  }

  setCursorPosition(row, column);
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
  setCursorPosition(update[1].row, update[1].column);
  statusStore.cursorRow = update[1].row;
  statusStore.cursorColumn = update[1].column;
}

// Remove character before cursor
async function remove_character() {
  let prev_row_length = await get_row_length(statusStore.cursorRow - 1);

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
    setCursorPosition(update[2].row, update[2].column);
    statusStore.cursorRow = update[2].row;
    statusStore.cursorColumn = update[2].column;
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
    setCursorPosition(statusStore.cursorRow - 1, column);
    statusStore.cursorRow -= 1;
    statusStore.cursorColumn = column;
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
    setCursorPosition(statusStore.cursorRow + 1, column);
    statusStore.cursorRow += 1;
    statusStore.cursorColumn = column;
  }
}

// Move cursor left
async function move_cursor_left() {
  if (statusStore.cursorColumn == 0) {
    // Move to end of previous line
    if (statusStore.cursorRow != 0) {
      let column = await get_row_length(statusStore.cursorRow - 1);
      setCursorPosition(statusStore.cursorRow - 1, column);
      statusStore.cursorRow -= 1;
      statusStore.cursorColumn = column;
    }
  } else {
    setCursorPosition(statusStore.cursorRow, statusStore.cursorColumn - 1);
    statusStore.cursorColumn -= 1;
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
      setCursorPosition(statusStore.cursorRow + 1, column);
      statusStore.cursorRow += 1;
      statusStore.cursorColumn = column;
    }
  } else {
    setCursorPosition(statusStore.cursorRow, statusStore.cursorColumn + 1);
    statusStore.cursorColumn += 1;
  }
}

// Move cursor to start of line
async function move_cursor_line_start() {
  setCursorPosition(statusStore.cursorRow, 0);
  statusStore.cursorColumn = 0;
}

// Move cursor to end of line
async function move_cursor_line_end() {
  let column = await get_row_length(statusStore.cursorRow);
  setCursorPosition(statusStore.cursorRow, column);
  statusStore.cursorColumn = column;
}

// Keyboard event handler
async function key_event(e) {
  e.preventDefault();

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
}
</script>
<template>
  <div
    ref="editorElement"
    class="bg-atom-bg min-h-full font-code antialiased text-lg absolute min-w-full overflow-visible h-fit w-fit cursor-text"
    @wheel="wheel_event"
    @mousedown="click_event"
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
    class="absolute font-code text-lg antialiased border-atom-primary border-l-2 pointer-events-none select-none animate-blink"
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
  <div class="absolute invisible whitespace-pre text-lg" ref="dummyElement">
    <span>x</span>
  </div>
</template>
