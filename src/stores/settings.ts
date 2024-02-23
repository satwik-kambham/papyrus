import { defineStore } from "pinia";
import { ref } from "vue";

export enum EOLSequence {
  CRLF = "\r\n",
  LF = "\n",
}

export enum AutoSaveType {
  OFF,
  ON_FOCUS_CHANGE,
  AFTER_DELAY,
}

export const useSettingsStore = defineStore("settings", () => {
  const editorFontSize = ref(26);
  const editorFontFamily = ref(
    "Consolas, ui-monospace, SFMono-Regular, Menlo, Monaco, monospace",
  );
  const editorTheme = ref("");
  const tabSize = ref(4);
  const autoSave = ref(AutoSaveType.OFF);
  const autoSaveDelay = ref(1000);
  const eolSequence = ref(EOLSequence.CRLF);

  return {
    editorFontSize,
    editorFontFamily,
    editorTheme,
    tabSize,
    autoSave,
    autoSaveDelay,
    eolSequence,
  };
});
