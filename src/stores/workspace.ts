import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useWorkspaceStore = defineStore("workspace", () => {
  // Window
  const maximized = ref(false);
  const windowDimensions = ref({ width: 1280, height: 720 });

  function resized() {}

  // Workspace
  const workspaceFolder = ref<string | null>(null);
  const folderEntries = ref<Array<IFileEntry>>([]);

  // Open editors
  const currentEditorIndex = ref(-1);
  const openEditors = ref<Array<OpenEditor>>([]);

  function switchEditor(index: number) {
    currentEditorIndex.value = index;
  }

  const currentSelection = computed(
    () => openEditors.value[currentEditorIndex.value].selection,
  );

  function updateSelection(
    start_row: number,
    start_column: number,
    end_row: number,
    end_column: number,
  ) {
    openEditors.value[currentEditorIndex.value].selection = {
      start: { row: start_row, column: start_column },
      end: { row: end_row, column: end_column },
    };
  }

  return {
    maximized,
    windowDimensions,
    resized,
    workspaceFolder,
    folderEntries,
    currentEditorIndex,
    openEditors,
    switchEditor,
    currentSelection,
    updateSelection,
  };
});
