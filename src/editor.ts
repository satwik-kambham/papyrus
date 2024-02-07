import { useEditorStore } from "./stores/editor";
import { useSettingsStore } from "./stores/settings";
import { useWorkspaceStore } from "./stores/workspace";
import { invoke } from "@tauri-apps/api";

export default class Editor {
  constructor(
    public editorStore: ReturnType<typeof useEditorStore>,
    public settingsStore: ReturnType<typeof useSettingsStore>,
    public workspaceStore: ReturnType<typeof useWorkspaceStore>,
  ) {}

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
  async remove_character() {
    const sel = this.workspaceStore.currentSelection;
    if (sel.end.row != 0 || sel.end.column != 0) {
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
}
