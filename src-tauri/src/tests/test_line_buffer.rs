use crate::editor::text_buffer::{Cursor, LineTextBuffer, Selection};

#[test]
fn line_buffer_from_file_content() {
    let initial_code = "This\nis\na simple\nfile\n".into();
    let buffer = LineTextBuffer::new(initial_code);
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a simple"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
}

#[test]
fn line_buffer_from_file_content_with_blank_lines() {
    let initial_code = "This\nis\n\na\tsimple\nfile\n".into();
    let buffer = LineTextBuffer::new(initial_code);
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from(""),
        String::from("a\tsimple"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
}

#[test]
fn line_buffer_from_file_content_with_crlf() {
    let initial_code = "This\r\nis\r\na simple\r\nfile\r\n".into();

    let buffer = LineTextBuffer::new(initial_code);
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a simple"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
}

#[test]
fn line_buffer_from_file_content_with_escape_sequences() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let buffer = LineTextBuffer::new(initial_code);
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a\\nsimple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
}

#[test]
fn get_row_length() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let buffer = LineTextBuffer::new(initial_code);
    let row_length = buffer.get_row_length(Cursor { row: 3, column: 0 });
    assert_eq!(row_length, 6);
}

#[test]
fn insert_character_at_start() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("3".into(), Cursor { row: 0, column: 0 });
    let lines = vec![
        String::from("3This"),
        String::from("is"),
        String::from("a\\nsimple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 0);
    assert_eq!(new_pos.column, 1);
}

#[test]
fn insert_character_at_end() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("3".into(), Cursor { row: 3, column: 6 });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a\\nsimple"),
        String::from("file\\n3"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 3);
    assert_eq!(new_pos.column, 7);
}

#[test]
fn insert_character_in_between() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("3".into(), Cursor { row: 0, column: 2 });
    let lines = vec![
        String::from("Th3is"),
        String::from("is"),
        String::from("a\\nsimple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 0);
    assert_eq!(new_pos.column, 3);
}

#[test]
fn insert_new_line_character_in_between() {
    let initial_code = "This\nis\na simple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("\n".into(), Cursor { row: 2, column: 1 });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a"),
        String::from(" simple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 3);
    assert_eq!(new_pos.column, 0);
}

#[test]
fn insert_new_line_character_at_line_end() {
    let initial_code = "This\nis\na simple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("\n".into(), Cursor { row: 1, column: 2 });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from(""),
        String::from("a simple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 2);
    assert_eq!(new_pos.column, 0);
}

#[test]
fn insert_string_in_between() {
    let initial_code = "This\nis\na\\nsimple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("hello".into(), Cursor { row: 0, column: 2 });
    let lines = vec![
        String::from("Thhellois"),
        String::from("is"),
        String::from("a\\nsimple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 0);
    assert_eq!(new_pos.column, 7);
}

#[test]
fn insert_string_with_new_lines_in_between() {
    let initial_code = "This\nis\na simple\nfile\\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let new_pos = buffer.insert_text("\nmany\nnew\nlines\n".into(), Cursor { row: 2, column: 1 });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("a"),
        String::from("many"),
        String::from("new"),
        String::from("lines"),
        String::from(" simple"),
        String::from("file\\n"),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(new_pos.row, 6);
    assert_eq!(new_pos.column, 0);
}

#[test]
fn remove_character_in_between() {
    let initial_code = "This\nis\na simple\nfile\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let (text, updated_cursor) = buffer.remove_text(Selection {
        start: Cursor { row: 2, column: 0 },
        end: Cursor { row: 2, column: 1 },
    });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from(" simple"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(text, String::from("a"));
    assert_eq!(updated_cursor.row, 2);
    assert_eq!(updated_cursor.column, 0);
}

#[test]
fn remove_new_line_character_at_line_start() {
    let initial_code = "This\nis\na simple\nfile\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let (text, updated_cursor) = buffer.remove_text(Selection {
        start: Cursor { row: 1, column: 2 },
        end: Cursor { row: 2, column: 0 },
    });
    let lines = vec![
        String::from("This"),
        String::from("isa simple"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(text, String::from("\n"));
    assert_eq!(updated_cursor.row, 1);
    assert_eq!(updated_cursor.column, 2);
}

#[test]
fn remove_string_in_between() {
    let initial_code = "This\nis\na simple\nfile\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let (text, updated_cursor) = buffer.remove_text(Selection {
        start: Cursor { row: 2, column: 1 },
        end: Cursor { row: 2, column: 7 },
    });
    let lines = vec![
        String::from("This"),
        String::from("is"),
        String::from("ae"),
        String::from("file"),
        String::from(""),
    ];

    assert_eq!(buffer.lines, lines);
    assert_eq!(text, String::from(" simpl"));
    assert_eq!(updated_cursor.row, 2);
    assert_eq!(updated_cursor.column, 1);
}

#[test]
fn remove_multi_line_in_between() {
    let initial_code = "This\nis\na\nsimple\nfile\n".into();

    let mut buffer = LineTextBuffer::new(initial_code);
    let (text, updated_cursor) = buffer.remove_text(Selection {
        start: Cursor { row: 1, column: 1 },
        end: Cursor { row: 4, column: 1 },
    });
    let lines = vec![String::from("This"), String::from("iile"), String::from("")];

    assert_eq!(buffer.lines, lines);
    assert_eq!(text, String::from("s\na\nsimple\nf"));
    assert_eq!(updated_cursor.row, 1);
    assert_eq!(updated_cursor.column, 1);
}
