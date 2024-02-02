import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

export async function openFileDialog(editorStore, workspaceStore) {
  const selected = await open({
    multiple: false,
  });

  if (selected !== null && !Array.isArray(selected)) {
    openFile(selected, editorStore, workspaceStore);
  }
}

export async function openFile(path, editorStore, workspaceStore) {
  invoke<number>("create_buffer_from_file_path", {
    path: path,
  })
    .then(async (buffer_idx) => {
      editorStore.bufferIdx = buffer_idx;
      editorStore.encoding = "utf-8";
      const fileEntry = await invoke<IFileEntry>("get_file_info", {
        path: path,
      });

      let entryExists = false;
      workspaceStore.openEditors.forEach((openEditor, index) => {
        if (openEditor.entry?.path == fileEntry.path) {
          entryExists = true;
          workspaceStore.switchEditor(index);
        }
      });
      if (!entryExists) {
        workspaceStore.openEditors.push({
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
        workspaceStore.switchEditor(workspaceStore.openEditors.length - 1);
      }

      invoke<IHighlightedText>("get_highlighted_text", {
        bufferIdx: editorStore.bufferIdx,
      }).then((content) => {
        editorStore.highlightedContent = content.text;
      });
    })
    .catch((error) => {
      editorStore.encoding = "Unknown";
      console.error(error);
    });
}

export async function openFolder(editorStore, workspaceStore) {
  const selected = await open({
    directory: true,
    multiple: false,
    recursive: true,
  });
  if (selected !== null && !Array.isArray(selected)) {
    // user selected a single folder
    invoke<Array<IFileEntry>>("get_folder_content", {
      path: selected,
    })
      .then((entries) => {
        workspaceStore.workspaceFolder = selected;
        workspaceStore.folderEntries = entries;
      })
      .catch((error) => {
        console.error(error);
      });
  }
}

export async function saveCurrent(editorStore, workspaceStore, settingsStore) {
  invoke<string>("save_buffer", {
    bufferIdx: editorStore.bufferIdx,
    eolSequence: settingsStore.eolSequence,
  })
    .then(() => {
      workspaceStore.openEditors[
        workspaceStore.currentEditorIndex
      ].unsavedChanges = false;
      console.log("File saved successfully");
    })
    .catch((error) => {
      console.error(error);
    });
}

export async function saveAs(editorStore, workspaceStore, settingsStore) {
  const selected = await save();
  if (selected !== null) {
    // user selected a single file
    invoke<string>("save_buffer_to_new_file", {
      bufferIdx: editorStore.bufferIdx,
      path: selected,
      eolSequence: settingsStore.eolSequence,
    })
      .then(() => {
        console.log("File saved successfully");
      })
      .catch((error) => {
        console.error(error);
      });
  }
}
