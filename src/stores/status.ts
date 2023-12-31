import { defineStore } from "pinia";
import { ref } from "vue";

export const useStatusStore = defineStore("status", () => {
  const encoding = ref("Unknown");
  const cursorRow = ref(0);
  const cursorColumn = ref(0);

  return { encoding, cursorRow, cursorColumn };
});
