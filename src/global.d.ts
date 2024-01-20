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
