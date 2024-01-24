import { defineStore } from "pinia";
import { ref } from "vue";

enum EOLSequence {
  CRLF = "\r\n",
  LF = "\n",
}

enum AutoSaveType {
  OFF,
  ON_FOCUS_CHANGE,
  AFTER_DELAY,
}

export const useSettingsStore = defineStore("settings", () => {
  const editorFontSize = ref(16);
  const editorFontFamily = ref(
    "Consolas, ui-monospace, SFMono-Regular, Menlo, Monaco, monospace",
  );
  const editorTheme = ref("");
  const tabSize = ref(2);
  const autoSave = ref(AutoSaveType.OFF);
  const autoSaveDelay = ref(1000);
  const eolSequence = ref(EOLSequence.LF);

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
