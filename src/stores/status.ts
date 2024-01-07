import { defineStore } from "pinia";
import { ref } from "vue";

export const useStatusStore = defineStore("status", () => {
  const encoding = ref("Unknown");
  const startCursorRow = ref(0);
  const startCursorColumn = ref(0);
  const cursorRow = ref(0);
  const cursorColumn = ref(0);

  return {
    encoding,
    startCursorRow,
    startCursorColumn,
    cursorRow,
    cursorColumn,
  };
});
