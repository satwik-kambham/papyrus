import { defineStore } from "pinia";
import { ref } from "vue";

export const useWorkspaceStore = defineStore("workspace", () => {
  const folder = ref("");
  const entries = ref([]);

  const maximized = ref(false);

  const openEditors = ref([]);
  const selectedEntry = ref(null);

  return { folder, entries, maximized, openEditors, selectedEntry };
});
