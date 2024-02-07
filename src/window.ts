import { appWindow } from "@tauri-apps/api/window";
import { useWorkspaceStore } from "./stores/workspace";

export default class Window {
  constructor(public workspaceStore: ReturnType<typeof useWorkspaceStore>) {}

  async close() {
    await appWindow.close();
  }

  async maximize() {
    await appWindow.toggleMaximize();
    this.workspaceStore.resized();
    this.workspaceStore.maximized = await appWindow.isMaximized();
  }

  async minimize() {
    await appWindow.minimize();
  }
}
