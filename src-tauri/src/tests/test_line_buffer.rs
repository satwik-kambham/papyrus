use crate::editor::text_buffer::LineTextBuffer;

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
