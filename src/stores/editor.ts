import { defineStore } from "pinia";
import { ref } from "vue";

export const useEditorStore = defineStore("editor", () => {
  const fileEntry = ref<IFileEntry | null>(null);
  const highlightedContent = ref<Array<Array<string>>>([]);
  const bufferIdx = ref(-1);
  const language = ref("Unknown");
  const encoding = ref("utf-8");

  // Prompt
  const promptOpen = ref(false);
  const promptTitle = ref("Title");
  const promptDescription = ref("Description");
  const promptResponse = ref("");
  const promptCallback = ref(null);
  const promptContext = ref(null);

  return {
    fileEntry,
    highlightedContent,
    bufferIdx,
    language,
    encoding,
    promptOpen,
    promptTitle,
    promptDescription,
    promptResponse,
    promptCallback,
    promptContext,
  };
});
