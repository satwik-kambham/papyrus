import { useEditorStore } from "./stores/editor";
import { useSettingsStore } from "./stores/settings";
import { useWorkspaceStore } from "./stores/workspace";

export default class Modals {
  constructor(
    public editorStore: ReturnType<typeof useEditorStore>,
    public settingsStore: ReturnType<typeof useSettingsStore>,
    public workspaceStore: ReturnType<typeof useWorkspaceStore>,
  ) {}

  promptUser(title, description, handler, context, defaultPrompt) {
    this.editorStore.promptTitle = title;
    this.editorStore.promptDescription = description;
    this.editorStore.promptCallback = handler;
    this.editorStore.promptResponse = defaultPrompt;
    this.editorStore.promptContext = context;

    this.editorStore.promptOpen = true;
  }
}
