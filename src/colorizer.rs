const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub trait Colorizer {
    fn red(&self, s: &str) -> String;
    fn blue(&self, s: &str) -> String;
    fn green(&self, s: &str) -> String;
}

pub struct ConsoleColorizer {}

impl ConsoleColorizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConsoleColorizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Colorizer for ConsoleColorizer {
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

        let c = ConsoleColorizer::new();

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

        let c = ConsoleColorizer::new();

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

        let c = ConsoleColorizer::new();

        // Act
        let actual = c.green(input);

        // Assert
        let expected = "\x1b[32mHello, username!\x1b[0m";
        assert_eq!(actual, expected);
    }
}
