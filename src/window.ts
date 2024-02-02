import { appWindow } from "@tauri-apps/api/window";

export async function minimize() {
  await appWindow.minimize();
}

export async function maximize(workspaceStore) {
  await appWindow.toggleMaximize();
  workspaceStore.resized();
  workspaceStore.maximized = await appWindow.isMaximized();
}

export async function quit() {
  await appWindow.close();
}
