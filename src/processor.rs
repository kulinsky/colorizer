use ansi_term::Color;
use regex::Regex;

pub struct TextProcessor {
    patterns: Vec<(Color, Regex)>,
}

impl TextProcessor {
    pub fn new(patterns: Vec<(Color, Regex)>) -> Self {
        Self { patterns }
    }

    pub fn process_line(&self, mut s: String) -> String {
        for (color, regex) in &self.patterns {
            s = regex
                .replace_all(
                    &s,
                    format!("{}$0{}", color.prefix(), color.suffix()).as_str(),
                )
                .to_string();
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        // Arrange
        let patterns = vec![(Color::Red, Regex::new("foo").unwrap())];
        let processor = TextProcessor::new(patterns);
        let input = "foo bar baz".to_string();

        // Act
        let actual = processor.process_line(input);

        // Assert
        let expected = "\x1b[31mfoo\x1b[0m bar baz";
        assert_eq!(actual, expected);
    }
}
