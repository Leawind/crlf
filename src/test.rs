use crate::{CRLF, LineEnding};

#[test]
fn test_convert_line_endings() {
    assert_eq!(
        "\r\nHello\r\nWorld\r\n".crlf(LineEnding::LF),
        "\nHello\nWorld\n"
    );
    assert_eq!("Hello\r\nWorld\r\n".crlf(LineEnding::LF), "Hello\nWorld\n");
    assert_eq!("\r\nHello\r\nWorld".crlf(LineEnding::LF), "\nHello\nWorld");
    assert_eq!("Hello\r\nWorld".crlf(LineEnding::LF), "Hello\nWorld");
    assert_eq!(
        "\r\n\r\nHello\r\n\r\nWorld\r\n\r\n".crlf(LineEnding::LF),
        "\n\nHello\n\nWorld\n\n"
    );

    assert_eq!(
        "\nHello\nWorld\n".crlf(LineEnding::CRLF),
        "\r\nHello\r\nWorld\r\n"
    );

    assert_eq!("".crlf(LineEnding::LF), "");
    assert_eq!("".crlf(LineEnding::CRLF), "");

    assert_eq!("\r\n".crlf(LineEnding::LF), "\n");
    assert_eq!("\n".crlf(LineEnding::CRLF), "\r\n");

    assert_eq!("\r\n\r\n\r\n".crlf(LineEnding::LF), "\n\n\n");
    assert_eq!("\n\n\n".crlf(LineEnding::CRLF), "\r\n\r\n\r\n");

    assert_eq!("Hello World".crlf(LineEnding::LF), "Hello World");
    assert_eq!("Hello World".crlf(LineEnding::CRLF), "Hello World");

    assert_eq!(
        "Hello\nWorld\r\nTest".crlf(LineEnding::CRLF),
        "Hello\r\nWorld\r\nTest"
    );
    assert_eq!(
        "Hello\r\nWorld\nTest".crlf(LineEnding::LF),
        "Hello\nWorld\nTest"
    );

    assert_eq!("\n".crlf(LineEnding::LF), "\n");
    assert_eq!("\r\n".crlf(LineEnding::CRLF), "\r\n");
    assert_eq!("\r".crlf(LineEnding::CR), "\r");

    assert_eq!(
        "C:\\Windows\\System32".crlf(LineEnding::LF),
        "C:\\Windows\\System32"
    );
    assert_eq!(
        "C:\\Windows\\System32".crlf(LineEnding::CRLF),
        "C:\\Windows\\System32"
    );
}
