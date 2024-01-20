import { defineStore } from "pinia";
import { ref } from "vue";

export const useEditorStore = defineStore("editor", () => {
  const content = ref<Array<Array<string>>>([]);
  const bufferIdx = ref(-1);

  return { content, bufferIdx };
});
