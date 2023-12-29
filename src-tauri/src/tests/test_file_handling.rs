use crate::editor_io::file_handling::read_file_content;

#[test]
fn test_file_exists() {
    let content = read_file_content("./src/tests/example.txt".into()).unwrap();

    assert_eq!(content, "This\nis\na simple\nfile\n".to_string());
}

#[test]
fn test_non_utf8_encoding() {
    let content = read_file_content("./src/tests/non_standard_encoding.txt".into());

    assert!(content.is_err());
}
