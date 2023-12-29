import { defineStore } from "pinia";
import { ref } from "vue";

export const useEditorStore = defineStore("editor", () => {
  const content = ref([]);
  const buffer_idx = ref(-1);

  return { content, buffer_idx };
});
