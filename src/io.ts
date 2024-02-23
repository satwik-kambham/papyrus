import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { useEditorStore } from "./stores/editor";
import { useSettingsStore } from "./stores/settings";
import { useWorkspaceStore } from "./stores/workspace";

export default class FileIO {
  constructor(
    public editorStore: ReturnType<typeof useEditorStore>,
    public settingsStore: ReturnType<typeof useSettingsStore>,
    public workspaceStore: ReturnType<typeof useWorkspaceStore>,
  ) {}

  async openFileDialog() {
    const selected = await open({
      multiple: false,
    });

    if (selected !== null && !Array.isArray(selected)) {
      this.openFile(selected);
    }
  }

  async openFile(path: string) {
    invoke<number>("create_buffer_from_file_path", {
      path: path,
    })
      .then(async (buffer_idx) => {
        this.editorStore.bufferIdx = buffer_idx;
        this.editorStore.encoding = "utf-8";
        const fileEntry = await invoke<IFileEntry>("get_file_info", {
          path: path,
        });

        let entryExists = false;
        this.workspaceStore.openEditors.forEach((openEditor, index) => {
          if (openEditor.entry?.path == fileEntry.path) {
            entryExists = true;
            this.workspaceStore.switchEditor(index);
          }
        });
        if (!entryExists) {
          this.workspaceStore.openEditors.push({
            entry: fileEntry,
            unsavedChanges: false,
            selection: {
              start: {
                row: 0,
                column: 0,
              },
              end: {
                row: 0,
                column: 0,
              },
            },
            scroll: {
              hOffset: 0,
              vOffset: 0,
            },
          });
          this.workspaceStore.switchEditor(
            this.workspaceStore.openEditors.length - 1,
          );
        }

        invoke<IHighlightedText>("get_highlighted_text", {
          bufferIdx: this.editorStore.bufferIdx,
        }).then((content) => {
          this.editorStore.highlightedContent = content.text;
        });
      })
      .catch((error) => {
        this.editorStore.encoding = "Unknown";
        console.error(error);
      });
  }

  async setFileTree(workspaceFolder) {
    invoke<Array<IFileEntry>>("get_folder_content", {
      path: workspaceFolder,
    })
      .then((entries) => {
        this.workspaceStore.workspaceFolder = workspaceFolder;
        this.workspaceStore.folderEntries = entries;
      })
      .catch((error) => {
        console.error(error);
      });
  }

  async openFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      recursive: true,
    });
    if (selected !== null && !Array.isArray(selected)) {
      // user selected a single folder
      this.setFileTree(selected);
    }
  }

  async saveCurrent() {
    invoke<string>("save_buffer", {
      bufferIdx: this.editorStore.bufferIdx,
      eolSequence: this.settingsStore.eolSequence,
    })
      .then(() => {
        this.workspaceStore.openEditors[
          this.workspaceStore.currentEditorIndex
        ].unsavedChanges = false;
        console.log("File saved successfully");
      })
      .catch((error) => {
        console.error(error);
      });
  }

  async saveAs() {
    const selected = await save();
    if (selected !== null) {
      // user selected a single file
      invoke<string>("save_buffer_to_new_file", {
        bufferIdx: this.editorStore.bufferIdx,
        path: selected,
        eolSequence: this.settingsStore.eolSequence,
      })
        .then(() => {
          console.log("File saved successfully");
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }

  async relative(from, to) {
    const path = await invoke<string>("get_relative_path", {
      from: from,
      to: to,
    });
    return path;
  }

  async parent(path) {
    const parentPath = await invoke<string>("get_parent", {
      path: path,
    });
    return parentPath;
  }

  async join(start, end) {
    const path = await invoke<string>("join_paths", {
      start: start,
      end: end,
    });
    return path;
  }

  async createNewFile(path, context) {
    const absolutePath = await this.join(
      this.workspaceStore.workspaceFolder,
      path,
    );
    await invoke("create_file", {
      path: absolutePath,
    });
    this.setFileTree(this.workspaceStore.workspaceFolder);
  }

  async createNewFolder(path, context) {
    const absolutePath = await this.join(
      this.workspaceStore.workspaceFolder,
      path,
    );
    await invoke("create_folder", {
      path: absolutePath,
    });
    this.setFileTree(this.workspaceStore.workspaceFolder);
  }
}
