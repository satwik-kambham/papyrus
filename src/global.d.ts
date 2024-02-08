declare interface IFileEntry {
  path: string;
  is_dir: boolean;
  name: string;
  extension: string;
  entries: Array<IFileEntry> | null;
}

declare interface IHighlightedText {
  text: Array<Array<string>>;
}

declare interface ITerminalPayload {
  output: string;
}

declare interface ISelection {
  start: {
    row: number;
    column: number;
  };
  end: {
    row: number;
    column: number;
  };
}

declare interface OpenEditor {
  entry?: IFileEntry;
  unsavedChanges: boolean;
  content?: string;
  selection: {
    start: {
      row: number;
      column: number;
    };
    end: {
      row: number;
      column: number;
    };
  };
  scroll: {
    hOffset: number;
    vOffset: number;
  };
}
