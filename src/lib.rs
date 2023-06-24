//! # ansi-style
//!
//! ## Usage
//!
//! ```rust
//! use ansi_style::{Color, Style};
//!
//! // You can either color the text directly with the Color enumeration
//! println!(
//!     "{}Cyan colored \"Hello World!\"{}",
//!     Color::Cyan.open(),
//!     Color::Cyan.close()
//! );
//!
//! // or you can use the builder function from within the Style stuct
//! // to create a style that can be used for more than one instance of
//! // a string and you wouldn't need to have an open and close function
//! // prepended and appended to every text you type like the above example
//!
//! let style = Style::builder().red().strikethrough().build();
//!
//! println!(
//!     "{}",
//!     style.stylize("Hello World in red with strikethrough")
//! )
//! ```

#![no_std]

extern crate alloc;

use core::fmt::Display;

use alloc::{format, string::String};

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
    /// Creates a new [`StyleBuilder`] struct for creating styles
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

    /// Sets the `bold` property to true
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Sets the `dim` property to true
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Sets the `italic` property to true
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Sets the `underline` property to true
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Sets the `blink` property to true
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    /// Sets the `inverse` property to true
    pub fn inverse(mut self) -> Self {
        self.inverse = true;
        self
    }

    /// Sets the `hidden` property to true
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Sets the `strikethrough` property to true
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Sets the `overline` property to true
    pub fn overline(mut self) -> Self {
        self.overline = true;
        self
    }

    /// Sets the text color to black
    pub fn black(mut self) -> Self {
        self.color = Some(Color::Black);
        self
    }

    /// Sets the text color to red
    pub fn red(mut self) -> Self {
        self.color = Some(Color::Red);
        self
    }

    /// Sets the text color to green
    pub fn green(mut self) -> Self {
        self.color = Some(Color::Green);
        self
    }

    /// Sets the text color to yellow
    pub fn yellow(mut self) -> Self {
        self.color = Some(Color::Yellow);
        self
    }

    /// Sets the text color to blue
    pub fn blue(mut self) -> Self {
        self.color = Some(Color::Blue);
        self
    }

    /// Sets the text color to magenta
    pub fn magenta(mut self) -> Self {
        self.color = Some(Color::Magenta);
        self
    }

    /// Sets the text color to cyan
    pub fn cyan(mut self) -> Self {
        self.color = Some(Color::Cyan);
        self
    }

    /// Sets the text color to white
    pub fn white(mut self) -> Self {
        self.color = Some(Color::White);
        self
    }

    /// Sets the text color to a bright black (gray)
    pub fn black_bright(mut self) -> Self {
        self.color = Some(Color::BlackBright);
        self
    }

    /// Sets the text color to a bright red
    pub fn red_bright(mut self) -> Self {
        self.color = Some(Color::RedBright);
        self
    }

    /// Sets the text color to a bright green
    pub fn green_bright(mut self) -> Self {
        self.color = Some(Color::GreenBright);
        self
    }

    /// Sets the text color to a bright yellow
    pub fn yellow_bright(mut self) -> Self {
        self.color = Some(Color::YellowBright);
        self
    }

    /// Sets the text color to a bright blue
    pub fn blue_bright(mut self) -> Self {
        self.color = Some(Color::BlueBright);
        self
    }

    /// Sets the text color to a bright magenta
    pub fn magenta_bright(mut self) -> Self {
        self.color = Some(Color::MagentaBright);
        self
    }

    /// Sets the text color to a bright cyan
    pub fn cyan_bright(mut self) -> Self {
        self.color = Some(Color::CyanBright);
        self
    }

    /// Sets the text color to a bright white
    pub fn white_bright(mut self) -> Self {
        self.color = Some(Color::WhiteBright);
        self
    }

    /// Sets the text's bckground color to black
    pub fn bg_black(mut self) -> Self {
        self.bg_color = Some(BGColor::Black);
        self
    }

    /// Sets the text's bckground color to red
    pub fn bg_red(mut self) -> Self {
        self.bg_color = Some(BGColor::Red);
        self
    }

    /// Sets the text's bckground color to green
    pub fn bg_green(mut self) -> Self {
        self.bg_color = Some(BGColor::Green);
        self
    }

    /// Sets the text's bckground color to yellow
    pub fn bg_yellow(mut self) -> Self {
        self.bg_color = Some(BGColor::Yellow);
        self
    }

    /// Sets the text's bckground color to blue
    pub fn bg_blue(mut self) -> Self {
        self.bg_color = Some(BGColor::Blue);
        self
    }

    /// Sets the text's bckground color to magenta
    pub fn bg_magenta(mut self) -> Self {
        self.bg_color = Some(BGColor::Magenta);
        self
    }

    /// Sets the text's bckground color to cyan
    pub fn bg_cyan(mut self) -> Self {
        self.bg_color = Some(BGColor::Cyan);
        self
    }

    /// Sets the text's bckground color to white
    pub fn bg_white(mut self) -> Self {
        self.bg_color = Some(BGColor::White);
        self
    }

    /// Sets the text's bckground color to a bright black (gray)
    pub fn bg_black_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::BlackBright);
        self
    }

    /// Sets the text's bckground color to a bright red
    pub fn bg_red_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::RedBright);
        self
    }

    /// Sets the text's bckground color to a bright green
    pub fn bg_green_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::GreenBright);
        self
    }

    /// Sets the text's bckground color to a bright yellow
    pub fn bg_yellow_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::YellowBright);
        self
    }

    /// Sets the text's bckground color to a bright blue
    pub fn bg_blue_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::BlueBright);
        self
    }

    /// Sets the text's bckground color to a bright magenta
    pub fn bg_magenta_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::MagentaBright);
        self
    }

    /// Sets the text's bckground color to a bright cyan
    pub fn bg_cyan_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::CyanBright);
        self
    }

    /// Sets the text's bckground color to a bright white
    pub fn bg_white_bright(mut self) -> Self {
        self.bg_color = Some(BGColor::WhiteBright);
        self
    }

    /// Builds a [`Style`] struct from a the current instance of
    /// the [`StyleBuilder`] struct
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
    /// Creates a new [`Style`] struct
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

    /// Creates a new [`StyleBuilder`] struct
    pub fn builder() -> StyleBuilder {
        StyleBuilder::new()
    }

    /// Checks whether or not the text's style is bold
    pub fn bold(&self) -> bool {
        self.bold
    }

    /// Checks whether or not the text's style is dim
    pub fn dim(&self) -> bool {
        self.dim
    }

    /// Checks whether or not the text's style is italic
    pub fn italic(&self) -> bool {
        self.italic
    }

    /// Checks whether or not the text's style is underline
    pub fn underline(&self) -> bool {
        self.underline
    }

    /// Checks whether or not the text's style is blink
    pub fn blink(&self) -> bool {
        self.blink
    }

    /// Checks whether or not the text's style is inverse
    pub fn inverse(&self) -> bool {
        self.inverse
    }

    /// Checks whether or not the text's style is hidden
    pub fn hidden(&self) -> bool {
        self.hidden
    }

    /// Checks whether or not the text's style is strikethrough
    pub fn strikethrough(&self) -> bool {
        self.strikethrough
    }

    /// Checks whether or not the text's style is overline
    pub fn overline(&self) -> bool {
        self.overline
    }

    /// Checks whether oor not the current style has a text color if no color is
    /// provided a default color [`Color::Any`] will be provided
    pub fn color(&self) -> Color {
        match self.color {
            Some(color) => color,
            None => Color::Any,
        }
    }

    /// Checks whether oor not the current style has a text background color if no color is
    /// provided a default color [`BGColor::Any`] will be provided
    pub fn bg_color(&self) -> BGColor {
        match self.bg_color {
            Some(color) => color,
            None => BGColor::Any,
        }
    }

    /// Converts three values (red, green, blue) to an ansi 256-color lookup (8-bit)
    ///
    /// # Arguments
    ///
    /// * `red` - a number representation for red
    /// * `green` - a number representation for green
    /// * `blue` - a number representation for blue
    pub fn rgb_to_ansi256(red: u8, green: u8, blue: u8) -> u8 {
        if red == green && green == blue {
            if red < 8 {
                return 16;
            }

            if red > 248 {
                return 231;
            }

            return ((((red as f32 - 8.) / 247.) * 24.) + 232.) as u8;
        }

        (16. + (36. * (red as f32 / 255. * 5.))
            + (6. * (green as f32 / 255. * 5.))
            + (blue as f32 / 255. * 5.)) as u8
    }

    /// Adds style to a given text
    ///
    /// * `text` - the string of characters to add style to  
    pub fn stylize(&self, text: &str) -> String {
        format!("{self}{text}{}", Self::reset())
    }

    fn is_default(&self) -> bool {
        *self == Self::default()
    }

    fn reset() -> &'static str {
        "\x1B[0m"
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_default() {
            return Ok(());
        }

        write!(f, "\x1B[")?;

        let mut written = false;

        {
            let mut write_char = |c| {
                if written {
                    write!(f, ";")?;
                }
                written = true;
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
            if written {
                write!(f, ";{}", color)?;
            } else {
                write!(f, "{}", color)?;
                written = true;
            }
        }

        if let Some(color) = self.color {
            if written {
                write!(f, ";{}", color)?;
            } else {
                write!(f, "{}", color)?;
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

/// The [`Color`] enumeration can be used to directly add color in ANSI
/// compatible environments without the use of the [`Style`] struct.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    /// A generic color
    Any,
    /// A black text color
    Black,
    /// A red text color
    Red,
    /// A green text color
    Green,
    /// A yellow text color
    Yellow,
    /// A blue text color
    Blue,
    /// A magenta text color
    Magenta,
    /// A cyan text color
    Cyan,
    /// A white text color
    White,

    /// A bright black (gray) text color
    BlackBright,
    /// A bright red text color
    RedBright,
    /// A bright green text color
    GreenBright,
    /// A bright yellow text color
    YellowBright,
    /// A bright blue text color
    BlueBright,
    /// A bright magenta text color
    MagentaBright,
    /// A bright cyan text color
    CyanBright,
    /// A bright white text color
    WhiteBright,

    /// An 8-bit color from  0 to 255
    Ansi256(u8),
    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

impl Color {
    /// Closes the terminal from adding selected color to text
    pub fn close(&self) -> &'static str {
        "\x1B[39m"
    }

    /// Opens the terminal for adding selected color to text
    pub fn open(&self) -> String {
        format!("\x1B[{}m", self)
    }

    /// Adds color to a given text
    ///
    /// * `text` - the string of characters to add background color to  
    pub fn paint(&self, text: &str) -> String {
        format!("{}{text}{}", self.open(), self.close())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

/// The [`BGColor`] enumeration can be used to directly add color in ANSI
/// compatible environments without the use of the [`Style`] struct.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BGColor {
    /// A generic color
    Any,
    /// A black background color
    Black,
    /// A red background color
    Red,
    /// A green background color
    Green,
    /// A yellow background color
    Yellow,
    /// A blue background color
    Blue,
    /// A magenta background color
    Magenta,
    /// A cyan background color
    Cyan,
    /// A white background color
    White,

    /// A bright black (gray) background color
    BlackBright,
    /// A bright red background color
    RedBright,
    /// A bright green background color
    GreenBright,
    /// A bright yellow background color
    YellowBright,
    /// A bright blue background color
    BlueBright,
    /// A bright magenta background color
    MagentaBright,
    /// A bright cyan background color
    CyanBright,
    /// A bright white background color
    WhiteBright,

    /// An 8-bit color from  0 to 255
    Ansi256(u8),
    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

impl BGColor {
    /// Closes the terminal from adding selected color to text's background
    pub fn close(&self) -> &'static str {
        "\x1B[49m"
    }

    /// Opens the terminal to adding selected color to text's background
    pub fn open(&self) -> String {
        format!("\x1B[{}m", self)
    }

    /// Adds background color to a given text
    ///
    /// * `text` - the string of characters to add color to  
    pub fn paint(&self, text: &str) -> String {
        format!("{}{text}{}", self.open(), self.close())
    }
}

impl Display for BGColor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
