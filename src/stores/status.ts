import { defineStore } from "pinia";
import { ref } from "vue";

export const useStatusStore = defineStore("status", () => {
  const encoding = ref("Unknown");

  return { encoding };
});
