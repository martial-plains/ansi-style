use ansi_style::{BGColor, Color, Style};

#[test]
fn test_color() {
    assert_eq!(Color::Green.open(), "\x1B[32m");
    assert_eq!(BGColor::Green.open(), "\x1B[42m");
    assert_eq!(Color::Green.close(), "\x1B[39m");
    assert_eq!(Color::BlackBright.open(), Color::BlackBright.open());
}

#[test]
fn test_convertion_to_ansi256() {
    assert_eq!(
        Color::Ansi256(Style::rgb_to_ansi256(255, 255, 255)).open(),
        "\x1B[38;5;231m"
    )
}
