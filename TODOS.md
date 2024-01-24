- Delete buffer when closed
- Order folder items (Folders come first, then files)
- Settings
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

- Order of operations changes when inserting text after a selection was made
- Highlighting string content with escape sequences
- Use different carriage return for windows (LF, CRLF)
- Stop selecting when mouse goes out of editor (editor needs to be focused first before any interaction can be done)
