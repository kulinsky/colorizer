#[derive(PartialEq, Debug, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        match value.as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "purple" => Color::Purple,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            _ => Color::Red,
        }
    }
}

impl From<Color> for ansi_term::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => ansi_term::Color::Black,
            Color::Red => ansi_term::Color::Red,
            Color::Green => ansi_term::Color::Green,
            Color::Yellow => ansi_term::Color::Yellow,
            Color::Blue => ansi_term::Color::Blue,
            Color::Purple => ansi_term::Color::Purple,
            Color::Cyan => ansi_term::Color::Cyan,
            Color::White => ansi_term::Color::White,
        }
    }
}
