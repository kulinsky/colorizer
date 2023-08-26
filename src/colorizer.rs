const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub trait Colorizer {
    fn paint(&self, s: &str) -> String;
    fn red(&self, s: &str) -> String;
    fn blue(&self, s: &str) -> String;
    fn green(&self, s: &str) -> String;
}

#[derive(PartialEq, Debug)]
pub enum Color {
    RED,
    GREEN,
    BLUE,
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        match value.as_str() {
            "red" => Color::RED,
            "green" => Color::GREEN,
            "blue" => Color::BLUE,
            _ => Color::RED,
        }
    }
}

pub struct ConsoleColorizer {
    default_color: Color,
}

impl ConsoleColorizer {
    pub fn new(default_color: Option<String>) -> Self {
        match default_color {
            Some(v) => Self {
                default_color: v.into(),
            },
            None => Self {
                default_color: Color::RED,
            },
        }
    }
}

impl Default for ConsoleColorizer {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Colorizer for ConsoleColorizer {
    fn paint(&self, s: &str) -> String {
        match self.default_color {
            Color::RED => self.red(s),
            Color::GREEN => self.green(s),
            Color::BLUE => self.blue(s),
        }
    }

    fn red(&self, s: &str) -> String {
        format!("{}{}{}", RED, s, RESET)
    }

    fn blue(&self, s: &str) -> String {
        format!("{}{}{}", BLUE, s, RESET)
    }

    fn green(&self, s: &str) -> String {
        format!("{}{}{}", GREEN, s, RESET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_red() {
        // Arrange
        let input = "Hello, username!";

        let c = ConsoleColorizer::default();

        // Act
        let actual = c.red(input);

        // Assert
        let expected = "\x1b[31mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_blue() {
        // Arrange
        let input = "Hello, username!";

        let c = ConsoleColorizer::default();

        // Act
        let actual = c.blue(input);

        // Assert
        let expected = "\x1b[34mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_green() {
        // Arrange
        let input = "Hello, username!";

        let c = ConsoleColorizer::default();

        // Act
        let actual = c.green(input);

        // Assert
        let expected = "\x1b[32mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_colorizer() {
        // Arrange
        let input = Some("blue".to_string());

        // Act
        let actual = ConsoleColorizer::new(input);

        // Assert
        assert_eq!(actual.default_color, Color::BLUE);
    }

    #[test]
    fn paint_with_default_color() {
        // Arrange
        let input = "Hello, username!";
        let c = ConsoleColorizer::new(Some("blue".to_string()));

        // Act
        let actual = c.paint(input);

        // Assert
        let expected = "\x1b[34mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }

    #[test]
    fn must_paint_to_green() {
        // Arrange
        let input = "Hello, username!";
        let c = ConsoleColorizer::new(Some("green".to_string()));

        // Act
        let actual = c.paint(input);

        // Assert
        let expected = "\x1b[32mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }
}
