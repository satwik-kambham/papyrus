- Refactoring
- Order folder items (Folders come first, then files)
- Settings
- Tab spacing
- Extra padding
- Double click to make selection
- Update grouping
- Scroll bars
- Scroll to cursor
- Handle changes made externally to files or open folders
- Efficient rendering and editing
- Better buffer data structure
- LSP
- Language selection
- Formatter
- Throttle mousemove updates
- Text Wrapping
- Update scroll position on updates
- Scroll when selecting
- Scroll to zoom
- Save current status to file
- Remote development
- jupyter notebook

Bugs:

- Cannot open empty file
- Highlighting string content with escape sequences
- Use different carriage return for windows (LF, CRLF)
- Reset cursor position when new file is opened from the tree view
- Stop selecting when mouse goes out of editor (editor needs to be focused first before
  any interaction can be done)

Stores:

1. Settings

- Editor font size
- Editor font family
- Editor theme
- Tab Size
- Auto save
- EOL sequence - LF or CRLF

2. Workspace

- Window
  - Window size
  - Maximized
- Workspace folders
  - Folder path
  - Folder entries
- Open editors
  - File path (optional, if saved to disk)
  - Unsaved changes present
  - Editor content (optional, if not saved to disk)
  - Cursor location
  - Current selection
  - Index (order in which the editor tabs appear)
  - Scroll offsets
- Index of current open editor

3. Editor (Current open editor state) (not persisted)

- File path
- Highlighted content
- Buffer index (-1 if not yet created)
- Language
- Encoding
