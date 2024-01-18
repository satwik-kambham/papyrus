<script setup lang="ts">
import { useWorkspaceStore } from "../stores/workspace";
import { useStatusStore } from "../stores/status";
import { useEditorStore } from "../stores/editor";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

const workspaceStore = useWorkspaceStore();
const editorStore = useEditorStore();
const statusStore = useStatusStore();

const terminalElement = ref(null);

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

const terminal = new Terminal();
const fitAddon = new FitAddon();

onMounted(async () => {
  await asyncQueue.enqueue(async () => {
    await invoke("init_pty");
  });
  terminal.loadAddon(fitAddon);
  terminal.open(terminalElement.value);
  terminal.onData(async (data) => {
    await asyncQueue.enqueue(async () => {
      await invoke("send_to_pty", {
        input: data,
      });
    });
  });
  terminal.onResize(async (size) => {
    await asyncQueue.enqueue(async () => {
      await invoke("resize_pty", {
        cols: size.cols,
        rows: size.rows,
      });
    });
  });
  fitAddon.fit();
  await asyncQueue.enqueue(async () => {
    await invoke("resize_pty", {
      cols: terminal.cols,
      rows: terminal.rows,
    });
  });
  terminal.write("Welcome to Payrus!\r\n");
});

window.onresize = () => {
  fitAddon.fit();
};

const unlisten = await appWindow.listen(
  "terminal_output",
  async ({ payload }) => {
    terminal.write(payload.output);
  },
);

workspaceStore.$onAction((context) => {
  context.after(() => {
    fitAddon.fit();
  });
});

onUnmounted(() => {
  unlisten();
});
</script>

<template>
  <div class="w-full h-full bg-atom-bg-light overflow-hidden">
    <div class="w-full h-full custom-scrollbar" ref="terminalElement"></div>
  </div>
</template>
