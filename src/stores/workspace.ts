import { defineStore } from "pinia";
import { ref } from "vue";

export const useWorkspaceStore = defineStore("workspace", () => {
  const folder = ref("");
  const entries = ref<Array<IFileEntry>>([]);

  const maximized = ref(false);
  function resized() {}

  const openEditors = ref<Array<IFileEntry>>([]);
  const selectedEntry = ref<string | null>(null);

  return { folder, entries, maximized, resized, openEditors, selectedEntry };
});
