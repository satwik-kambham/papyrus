import { useEditorStore, EditingMode } from "./stores/editor";
import { useSettingsStore } from "./stores/settings";
import { useWorkspaceStore } from "./stores/workspace";
import { invoke } from "@tauri-apps/api";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import FileIO from "./io";

export default class Editor {
  fileIO: FileIO;

  constructor(
    public editorStore: ReturnType<typeof useEditorStore>,
    public settingsStore: ReturnType<typeof useSettingsStore>,
    public workspaceStore: ReturnType<typeof useWorkspaceStore>,
  ) {
    this.fileIO = new FileIO(editorStore, settingsStore, workspaceStore);
  }

  async normalModeMapping(e: KeyboardEvent) {
    if (e.ctrlKey) {
      if (e.key === "s") {
        await this.fileIO.saveCurrent();
      }
    } else {
      // Basic Navigation
      if (e.key === "ArrowLeft" || e.key === "h") {
        await this.move_cursor_left();
      } else if (e.key === "ArrowRight" || e.key === "l") {
        await this.move_cursor_right();
      } else if (e.key === "ArrowUp" || e.key === "k") {
        await this.move_cursor_up();
      } else if (e.key === "ArrowDown" || e.key === "j") {
        await this.move_cursor_down();
      }

      // Advanced navigation
      else if (e.key === "Home" || e.key === "0") {
        await this.move_cursor_line_start();
      } else if (e.key === "End" || e.key === "$") {
        await this.move_cursor_line_end();
      } else if (e.key === "g") {
        await this.move_cursor_document_start();
      } else if (e.key === "G") {
        await this.move_cursor_document_end();
      }

      // Editing
      else if (e.key === "o") {
        await this.move_cursor_line_end();
        const indentSize = await this.get_indent_size();
        await this.insert_character("\n");
        await this.add_indentation(indentSize);
        this.editorStore.editingMode = EditingMode.INSERT;
      } else if (e.key === "a") {
        await this.move_cursor_right();
        this.editorStore.editingMode = EditingMode.INSERT;
      }
    }
  }

  async insertModeMapping(e: KeyboardEvent) {
    if (e.ctrlKey) {
      if (e.key === "c") {
        const selected_text = await this.get_selected_text();
        await writeText(selected_text);
      } else if (e.key === "v") {
        const t = await readText();
        await this.insert_character(t ?? "");
      } else if (e.key === "x") {
        const removed_text = await this.remove_character();
        await writeText(removed_text);
      } else if (e.key === "z") {
        await this.undo();
      } else if (e.key === "y") {
        await this.redo();
      } else if (e.key === "s") {
        await this.fileIO.saveCurrent();
      } else if (e.key === "[") {
        await this.remove_indentation();
      } else if (e.key === "]") {
        await this.add_indentation();
      }
    } else {
      // Basic Navigation
      if (e.key === "ArrowUp") {
        await this.move_cursor_up();
      } else if (e.key === "ArrowLeft") {
        await this.move_cursor_left();
      } else if (e.key === "ArrowDown") {
        await this.move_cursor_down();
      } else if (e.key === "ArrowRight") {
        await this.move_cursor_right();
      }

      // Advanced navigation
      else if (e.key === "Home") {
        await this.move_cursor_line_start();
      } else if (e.key === "End") {
        await this.move_cursor_line_end();
      }

      // Editing
      else if (e.key.length == 1 || e.key === "Enter") {
        let key = e.key;
        let indentSize = 0;
        if (key === "Enter") {
          key = "\n";
          indentSize = await this.get_indent_size();
        }
        await this.insert_character(key);
        if (e.key === "Enter") {
          await this.add_indentation(indentSize);
        }
      } else if (e.key === "Tab") {
        const spacing = " ".repeat(this.settingsStore.tabSize);
        await this.insert_character(spacing);
      } else if (e.key === "Backspace") {
        await this.remove_character();
      } else if (e.key === "Delete") {
        await this.remove_character(true);
      }
    }
  }

  async visualModeMapping(e: KeyboardEvent) {}

  async closeBuffer(index: number) {
    await invoke("delete_buffer", {
      bufferIdx: this.editorStore.bufferIdx,
    });
    this.workspaceStore.openEditors.splice(index, 1);
    if (this.workspaceStore.openEditors.length == 0) {
      this.workspaceStore.currentEditorIndex = -1;
      this.editorStore.bufferIdx = -1;
      this.editorStore.fileEntry = null;
      this.editorStore.encoding = "Unknown";
      this.editorStore.language = "Unknown";
      this.editorStore.highlightedContent = [];
    } else {
      this.workspaceStore.switchEditor(0);
    }
  }

  selection_made() {
    const s = this.workspaceStore.currentSelection;
    return s.start.row != s.end.row || s.start.column != s.end.column;
  }

  // Get lines length (total rows)
  async get_lines_length() {
    const lines_length = await invoke<number>("get_lines_length", {
      bufferIdx: this.editorStore.bufferIdx,
    });
    return lines_length;
  }

  // Get row length
  async get_row_length(row_number: number) {
    const row_length = await invoke<number>("get_row_length", {
      bufferIdx: this.editorStore.bufferIdx,
      row: row_number,
    });
    return row_length;
  }

  // Get selected text
  async get_selected_text() {
    if (this.selection_made()) {
      const sel = this.workspaceStore.currentSelection;
      let start = sel.start;
      let end = sel.end;

      if (
        start.row > end.row ||
        (start.row == end.row && start.column > end.column)
      ) {
        const buf = start;
        start = end;
        end = buf;
      }

      const s = {
        start: start,
        end: end,
      };

      const selected_text = await invoke<string>("get_selected_text", {
        bufferIdx: this.editorStore.bufferIdx,
        selection: s,
      });
      return selected_text;
    }
    return "";
  }

  // Get selected text
  async get_token_under_cursor() {
    const s = this.workspaceStore.currentSelection;
    const selection = await invoke<ISelection>("select_token_under_cursor", {
      bufferIdx: this.editorStore.bufferIdx,
      cursor: {
        row: s.end.row,
        column: s.end.column,
      },
    });
    this.workspaceStore.updateSelection(
      selection.start.row,
      selection.start.column,
      selection.end.row,
      selection.end.column,
    );
  }

  // Insert character after cursor
  async insert_character(character: string) {
    if (this.selection_made()) {
      await this.remove_character();
    }
    const s = this.workspaceStore.currentSelection;
    const update = await invoke("insert_text", {
      bufferIdx: this.editorStore.bufferIdx,
      text: character,
      cursor: {
        row: s.end.row,
        column: s.end.column,
      },
    });
    this.editorStore.highlightedContent = update[0].text;
    this.workspaceStore.updateSelection(
      update[1].row,
      update[1].column,
      update[1].row,
      update[1].column,
    );
    this.workspaceStore.openEditors[
      this.workspaceStore.currentEditorIndex
    ].unsavedChanges = true;
  }

  // Remove character before cursor
  async remove_character(backwards = false) {
    const sel = this.workspaceStore.currentSelection;
    if (
      (!backwards && (sel.end.row != 0 || sel.end.column != 0)) ||
      (backwards &&
        (sel.end.row != (await this.get_lines_length()) - 1 ||
          sel.end.column != (await this.get_row_length(sel.end.row))))
    ) {
      let s;
      if (this.selection_made()) {
        let start = {
          row: sel.start.row,
          column: sel.start.column,
        };
        let end = {
          row: sel.end.row,
          column: sel.end.column,
        };

        if (
          start.row > end.row ||
          (start.row == end.row && start.column > end.column)
        ) {
          const buf = start;
          start = end;
          end = buf;
        }

        s = {
          start: start,
          end: end,
        };
      } else {
        if (backwards) {
          if (sel.end.column != (await this.get_row_length(sel.end.row))) {
            s = {
              start: {
                row: sel.end.row,
                column: sel.end.column,
              },
              end: {
                row: sel.end.row,
                column: sel.end.column + 1,
              },
            };
          } else {
            s = {
              start: {
                row: sel.end.row,
                column: sel.end.column,
              },
              end: {
                row: sel.end.row + 1,
                column: 0,
              },
            };
          }
        } else {
          if (sel.end.column != 0) {
            s = {
              start: {
                row: sel.end.row,
                column: sel.end.column - 1,
              },
              end: {
                row: sel.end.row,
                column: sel.end.column,
              },
            };
          } else {
            const prev_row_length = await this.get_row_length(sel.end.row - 1);
            s = {
              start: {
                row: sel.end.row - 1,
                column: prev_row_length,
              },
              end: {
                row: sel.end.row,
                column: 0,
              },
            };
          }
        }
      }
      const update = await invoke("remove_text", {
        bufferIdx: this.editorStore.bufferIdx,
        selection: s,
      });
      this.editorStore.highlightedContent = update[0].text;
      const removed_text = update[1];
      this.workspaceStore.updateSelection(
        update[2].row,
        update[2].column,
        update[2].row,
        update[2].column,
      );
      this.workspaceStore.openEditors[
        this.workspaceStore.currentEditorIndex
      ].unsavedChanges = true;
      return removed_text;
    }
  }

  // Move cursor up
  async move_cursor_up() {
    const s = this.workspaceStore.currentSelection;
    if (s.end.row == 0) {
      await this.move_cursor_line_start();
    } else {
      const column = Math.min(
        s.end.column,
        await this.get_row_length(s.end.row - 1),
      );
      this.workspaceStore.updateSelection(
        s.end.row - 1,
        column,
        s.end.row - 1,
        column,
      );
    }
  }

  // Move cursor down
  async move_cursor_down() {
    const s = this.workspaceStore.currentSelection;
    if (s.end.row == (await this.get_lines_length()) - 1) {
      await this.move_cursor_line_end();
    } else {
      const column = Math.min(
        s.end.column,
        await this.get_row_length(s.end.row + 1),
      );
      this.workspaceStore.updateSelection(
        s.end.row + 1,
        column,
        s.end.row + 1,
        column,
      );
    }
  }

  // Move cursor left
  async move_cursor_left() {
    const s = this.workspaceStore.currentSelection;
    if (this.selection_made()) {
      this.workspaceStore.updateSelection(
        s.end.row,
        s.end.column,
        s.end.row,
        s.end.column,
      );
    } else {
      if (s.end.column == 0) {
        // Move to end of previous line
        if (s.end.row != 0) {
          const column = await this.get_row_length(s.end.row - 1);
          this.workspaceStore.updateSelection(
            s.end.row - 1,
            column,
            s.end.row - 1,
            column,
          );
        }
      } else {
        this.workspaceStore.updateSelection(
          s.end.row,
          s.end.column - 1,
          s.end.row,
          s.end.column - 1,
        );
      }
    }
  }

  // Move cursor right
  async move_cursor_right() {
    const s = this.workspaceStore.currentSelection;
    if (this.selection_made()) {
      this.workspaceStore.updateSelection(
        s.end.row,
        s.end.column,
        s.end.row,
        s.end.column,
      );
    } else {
      if (s.end.column == (await this.get_row_length(s.end.row))) {
        // Move to start of next line
        if (s.end.row != (await this.get_lines_length()) - 1) {
          const column = 0;
          this.workspaceStore.updateSelection(
            s.end.row + 1,
            column,
            s.end.row + 1,
            column,
          );
        }
      } else {
        this.workspaceStore.updateSelection(
          s.end.row,
          s.end.column + 1,
          s.end.row,
          s.end.column + 1,
        );
      }
    }
  }

  // Move cursor to start of line
  async move_cursor_line_start() {
    const s = this.workspaceStore.currentSelection;
    this.workspaceStore.updateSelection(s.end.row, 0, s.end.row, 0);
  }

  // Move cursor to end of line
  async move_cursor_line_end() {
    const s = this.workspaceStore.currentSelection;
    const column = await this.get_row_length(s.end.row);
    this.workspaceStore.updateSelection(s.end.row, column, s.end.row, column);
  }
  
  // Move cursor to document start
  async move_cursor_document_start() {
    this.workspaceStore.updateSelection(0, 0, 0, 0);
  }
  
  // Move cursor to document end
  async move_cursor_document_end() {
    const totalLines = await this.get_lines_length();
    this.workspaceStore.updateSelection(totalLines - 1, 0, totalLines - 1, 0);
  }

  // Undo last action
  async undo() {
    const update = await invoke("undo", {
      bufferIdx: this.editorStore.bufferIdx,
    });
    if (update != null) {
      this.editorStore.highlightedContent = update[0].text;
      this.workspaceStore.updateSelection(
        update[1].row,
        update[1].column,
        update[1].row,
        update[1].column,
      );
    }
    this.workspaceStore.openEditors[
      this.workspaceStore.currentEditorIndex
    ].unsavedChanges = true;
  }

  // Redo last action
  async redo() {
    const update = await invoke("redo", {
      bufferIdx: this.editorStore.bufferIdx,
    });
    if (update != null) {
      this.editorStore.highlightedContent = update[0].text;
      this.workspaceStore.updateSelection(
        update[1].row,
        update[1].column,
        update[1].row,
        update[1].column,
      );
    }
    this.workspaceStore.openEditors[
      this.workspaceStore.currentEditorIndex
    ].unsavedChanges = true;
  }

  // Add indentation
  async add_indentation(tabSize?: number) {
    const s = this.workspaceStore.currentSelection;
    const update = await invoke("add_indentation", {
      bufferIdx: this.editorStore.bufferIdx,
      selection: s,
      tabSize: tabSize ?? this.settingsStore.tabSize,
    });
    this.editorStore.highlightedContent = update[0].text;
    this.workspaceStore.updateSelection(
      update[1].start.row,
      update[1].start.column,
      update[1].end.row,
      update[1].end.column,
    );
    this.workspaceStore.openEditors[
      this.workspaceStore.currentEditorIndex
    ].unsavedChanges = true;
  }

  // Remove indentation
  async remove_indentation() {
    const s = this.workspaceStore.currentSelection;
    const update = await invoke("remove_indentation", {
      bufferIdx: this.editorStore.bufferIdx,
      selection: s,
      tabSize: this.settingsStore.tabSize,
    });
    this.editorStore.highlightedContent = update[0].text;
    this.workspaceStore.updateSelection(
      update[1].start.row,
      update[1].start.column,
      update[1].end.row,
      update[1].end.column,
    );
    this.workspaceStore.openEditors[
      this.workspaceStore.currentEditorIndex
    ].unsavedChanges = true;
  }

  // Get indent size
  async get_indent_size() {
    const indent_size = await invoke<number>("get_indent_size", {
      bufferIdx: this.editorStore.bufferIdx,
      row: this.workspaceStore.currentSelection.start.row,
    });
    return indent_size;
  }
}
