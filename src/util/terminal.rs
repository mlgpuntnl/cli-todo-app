#![allow(dead_code)]


use std::fmt;
use std::fmt::Display;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

pub enum TextStyle {
    Normal,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Reverse,
    Hidden,
    Strikethrough,
}

pub enum EraseMode {
    CursorToEnd,
    CursorToBegin,
    EntireScreen,
    SavedLines,
    CursorToEOL,
    SOLToCursor,
    EntireLine,
}

const RESET_CODE: &str = "\x1B[0m";

pub struct ColoredText<T: Display> {
    text: T,
    foreground_color: Color,
    background_color: Color,
    text_style: TextStyle,
}

impl<T> ColoredText<T>
where
    T: Display,
{
    pub fn from(text: T) -> ColoredText<T> {
        ColoredText {
            text,
            foreground_color: Color::Default,
            background_color: Color::Default,
            text_style: TextStyle::Normal,
        }
    }

    pub fn foreground_color(&mut self, color: Color) -> &mut Self {
        self.foreground_color = color;
        self
    }

    pub fn background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = color;
        self
    }

    pub fn text_style(&mut self, text_style: TextStyle) -> &mut Self {
        self.text_style = text_style;
        self
    }

    fn as_string(&self) -> String {
        let text_style = match self.text_style {
            TextStyle::Normal => "22",
            TextStyle::Bold => "1",
            TextStyle::Dim => "2",
            TextStyle::Italic => "3",
            TextStyle::Underline => "4",
            TextStyle::Blinking => "5",
            TextStyle::Reverse => "7",
            TextStyle::Hidden => "8",
            TextStyle::Strikethrough => "9",
        };
        let foreground_color = match self.foreground_color {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::Default => "39",
        };
        let background_color = match self.background_color {
            Color::Black => "40",
            Color::Red => "41",
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Blue => "44",
            Color::Magenta => "45",
            Color::Cyan => "46",
            Color::White => "47",
            Color::Default => "49",
        };
        format!(
            "\x1B[{text_style};{foreground};{background}m{text}{reset}",
            text_style = text_style,
            foreground = foreground_color,
            background = background_color,
            text = self.text,
            reset = RESET_CODE
        )
    }
}

impl<T> Display for ColoredText<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub struct Erase {
    erase_mode: EraseMode,
    reset_cursor: bool,
}

impl Erase {
    pub fn new() -> Erase {
        Erase {
            erase_mode: EraseMode::EntireScreen,
            reset_cursor: true,
        }
    }

    pub fn mode(&mut self, mode: EraseMode) -> &mut Self {
        self.erase_mode = mode;
        self
    }

    pub fn reset_cursor(&mut self, reset: bool) -> &mut Self {
        self.reset_cursor = reset;
        self
    }

    fn as_string(&self) -> String {
        let erase_mode = match self.erase_mode {
            EraseMode::CursorToEnd => "0J",
            EraseMode::CursorToBegin => "1J",
            EraseMode::EntireScreen => "2J",
            EraseMode::SavedLines => "3J",
            EraseMode::CursorToEOL => "0K",
            EraseMode::SOLToCursor => "1K",
            EraseMode::EntireLine => "2K",
        };
        let mut reset = "";
        if self.reset_cursor {
            reset = "\r";
        }
        format!("\x1B[{}{}", erase_mode, reset)
    }
}

impl Display for Erase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub fn clear_screen() {
    println!("{}", Erase::new());
}

pub fn clear_line() {
    println!("\x1B[1f{}", Erase::new().mode(EraseMode::EntireLine));
}
