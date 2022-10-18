use std::fmt::Display;

/// A helper struct for creating the [`Style`] object
pub struct StyleBuilder {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    inverse: bool,
    hidden: bool,
    strikethrough: bool,
    overline: bool,
    color: Option<Color>,
    bg_color: Option<BGColor>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            inverse: false,
            hidden: false,
            strikethrough: false,
            overline: false,
            color: None,
            bg_color: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    pub fn inverse(mut self) -> Self {
        self.inverse = true;
        self
    }

    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn overline(mut self) -> Self {
        self.overline = true;
        self
    }

    pub fn black(mut self) -> Self {
        self.color = Some(Color::Black);
        self
    }

    pub fn red(mut self) -> Self {
        self.color = Some(Color::Red);
        self
    }

    pub fn green(mut self) -> Self {
        self.color = Some(Color::Green);
        self
    }

    pub fn yellow(mut self) -> Self {
        self.color = Some(Color::Yellow);
        self
    }

    pub fn blue(mut self) -> Self {
        self.color = Some(Color::Blue);
        self
    }

    pub fn magenta(mut self) -> Self {
        self.color = Some(Color::Magenta);
        self
    }

    pub fn cyan(mut self) -> Self {
        self.color = Some(Color::Cyan);
        self
    }

    pub fn white(mut self) -> Self {
        self.color = Some(Color::White);
        self
    }

    pub fn black_bright(mut self) -> Self {
        self.color = Some(Color::BlackBright);
        self
    }

    pub fn red_bright(mut self) -> Self {
        self.color = Some(Color::RedBright);
        self
    }

    pub fn green_bright(mut self) -> Self {
        self.color = Some(Color::GreenBright);
        self
    }

    pub fn yellow_bright(mut self) -> Self {
        self.color = Some(Color::YellowBright);
        self
    }

    pub fn blue_bright(mut self) -> Self {
        self.color = Some(Color::BlueBright);
        self
    }

    pub fn magenta_bright(mut self) -> Self {
        self.color = Some(Color::MagentaBright);
        self
    }

    pub fn cyan_bright(mut self) -> Self {
        self.color = Some(Color::CyanBright);
        self
    }

    pub fn white_bright(mut self) -> Self {
        self.color = Some(Color::WhiteBright);
        self
    }

    pub fn bg_black(mut self) -> Self {
        self.bg_color = Some(BGColor::Black);
        self
    }

    pub fn bg_red(mut self) -> Self {
        self.bg_color = Some(BGColor::Red);
        self
    }

    pub fn bg_green(mut self) -> Self {
        self.bg_color = Some(BGColor::Green);
        self
    }

    pub fn bg_yellow(mut self) -> Self {
        self.bg_color = Some(BGColor::Yellow);
        self
    }

    pub fn bg_blue(mut self) -> Self {
        self.bg_color = Some(BGColor::Blue);
        self
    }

    pub fn bg_magenta(mut self) -> Self {
        self.bg_color = Some(BGColor::Magenta);
        self
    }

    pub fn bg_cyan(mut self) -> Self {
        self.bg_color = Some(BGColor::Cyan);
        self
    }

    pub fn bg_white(mut self) -> Self {
        self.bg_color = Some(BGColor::White);
        self
    }

    pub fn bg_black_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::BlackBright);
        self
    }

    pub fn bg_red_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::RedBright);
        self
    }

    pub fn bg_green_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::GreenBright);
        self
    }

    pub fn bg_yellow_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::YellowBright);
        self
    }

    pub fn bg_blue_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::BlueBright);
        self
    }

    pub fn bg_magenta_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::MagentaBright);
        self
    }

    pub fn bg_cyan_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::CyanBright);
        self
    }

    pub fn bg_white_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::WhiteBright);
        self
    }

    pub fn build(self) -> Style {
        let StyleBuilder {
            bold,
            dim,
            italic,
            underline,
            blink,
            inverse,
            hidden,
            strikethrough,
            overline,
            color,
            bg_color,
        } = self;

        Style {
            bold,
            dim,
            italic,
            underline,
            blink,
            inverse,
            hidden,
            strikethrough,
            overline,
            color,
            bg_color,
        }
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A  collection of properties that can be used to format a string using ANSI escape codes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Style {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    inverse: bool,
    hidden: bool,
    strikethrough: bool,
    overline: bool,
    color: Option<Color>,
    bg_color: Option<BGColor>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            inverse: false,
            hidden: false,
            strikethrough: false,
            overline: false,
            color: None,
            bg_color: None,
        }
    }

    pub fn builder() -> StyleBuilder {
        StyleBuilder::new()
    }

    pub fn bold(&self) -> bool {
        self.bold
    }

    pub fn dim(&self) -> bool {
        self.dim
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn underline(&self) -> bool {
        self.underline
    }

    pub fn blink(&self) -> bool {
        self.blink
    }

    pub fn inverse(&self) -> bool {
        self.inverse
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn strikethrough(&self) -> bool {
        self.strikethrough
    }

    pub fn overline(&self) -> bool {
        self.overline
    }

    pub fn color(&self) -> Color {
        match self.color {
            Some(color) => color,
            None => Color::Any,
        }
    }

    pub fn bg_color(&self) -> BGColor {
        match self.bg_color {
            Some(color) => color,
            None => BGColor::Any,
        }
    }

    pub fn rgb_to_ansi256(red: u8, green: u8, blue: u8) -> u8 {
        if red == green && green == blue {
            if red < 8 {
                return 16;
            }

            if red > 248 {
                return 231;
            }

            return ((((red as f32 - 8.) / 247.) * 24.) + 232.).round() as u8;
        }

        (16. + (36. * (red as f32 / 255. * 5.))
            + (6. * (green as f32 / 255. * 5.))
            + (blue as f32 / 255. * 5.))
            .round() as u8
    }

    pub fn stylize(&self, text: &str) -> String {
        format!("{self}{text};{}", Self::reset())
    }

    fn is_default(&self) -> bool {
        *self == Self::default()
    }

    fn reset() -> &'static str {
        "\x1B[0m"
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_default() {
            return Ok(());
        }

        write!(f, "\x1B[")?;

        let mut written_anything = false;

        {
            let mut write_char = |c| {
                if written_anything {
                    write!(f, ";")?;
                }
                written_anything = true;
                write!(f, "{}", c)?;
                Ok(())
            };

            if self.bold {
                write_char('1')?
            }
            if self.dim {
                write_char('2')?
            }
            if self.italic {
                write_char('3')?
            }
            if self.underline {
                write_char('4')?
            }
            if self.blink {
                write_char('5')?
            }
            if self.inverse {
                write_char('7')?
            }
            if self.hidden {
                write_char('8')?
            }
            if self.strikethrough {
                write_char('9')?
            }

            if self.overline {
                write!(f, "53")?
            }
        }

        if let Some(color) = self.bg_color {
            if written_anything {
                write!(f, ";{}", color)?;
            }
            written_anything = true;
        }

        if let Some(color) = self.color {
            if written_anything {
                write!(f, ";{}", color)?;
            }
        }

        write!(f, "m")
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// The [`Color`] enumeration can be used to directly add color to ANSI
/// compatible text without the use of the [`Style`] struct.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    Any,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,

    Ansi256(u8),
    RGB(u8, u8, u8),
}

impl Color {
    pub fn close(&self) -> &'static str {
        "\x1B[39m"
    }

    pub fn open(&self) -> String {
        format!("\x1B[{}m", self)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Any => write!(f, ""),
            Color::Black => write!(f, "30"),
            Color::Red => write!(f, "31"),
            Color::Green => write!(f, "32"),
            Color::Yellow => write!(f, "33"),
            Color::Blue => write!(f, "34"),
            Color::Magenta => write!(f, "35"),
            Color::Cyan => write!(f, "36"),
            Color::White => write!(f, "37"),
            Color::BlackBright => write!(f, "90"),
            Color::RedBright => write!(f, "91"),
            Color::GreenBright => write!(f, "92"),
            Color::YellowBright => write!(f, "93"),
            Color::BlueBright => write!(f, "94"),
            Color::MagentaBright => write!(f, "95"),
            Color::CyanBright => write!(f, "96"),
            Color::WhiteBright => write!(f, "97"),
            Color::Ansi256(num) => write!(f, "38;5;{}", &num),
            Color::RGB(r, g, b) => write!(f, "38;2;{};{};{}", &r, &g, &b),
        }
    }
}

/// The [`BGColor`] enumeration can be used to directly add color to ANSI
/// compatible background without the use of the [`Style`] struct.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BGColor {
    Any,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,

    Ansi256(u8),
    RGB(u8, u8, u8),
}

impl BGColor {
    pub fn close(&self) -> &'static str {
        "\x1B[49m"
    }

    pub fn open(&self) -> String {
        format!("\x1B[{}m", self)
    }
}

impl Display for BGColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BGColor::Any => write!(f, ""),
            BGColor::Black => write!(f, "40"),
            BGColor::Red => write!(f, "41"),
            BGColor::Green => write!(f, "42"),
            BGColor::Yellow => write!(f, "43"),
            BGColor::Blue => write!(f, "44"),
            BGColor::Magenta => write!(f, "45"),
            BGColor::Cyan => write!(f, "46"),
            BGColor::White => write!(f, "47"),
            BGColor::BlackBright => write!(f, "100"),
            BGColor::RedBright => write!(f, "101"),
            BGColor::GreenBright => write!(f, "102"),
            BGColor::YellowBright => write!(f, "103"),
            BGColor::BlueBright => write!(f, "104"),
            BGColor::MagentaBright => write!(f, "105"),
            BGColor::CyanBright => write!(f, "106"),
            BGColor::WhiteBright => write!(f, "107"),
            BGColor::Ansi256(num) => write!(f, "48;5;{}", &num),
            BGColor::RGB(r, g, b) => write!(f, "48;2;{};{};{}", &r, &g, &b),
        }
    }
}
