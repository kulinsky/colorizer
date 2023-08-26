use crate::{colorizer::Colorizer, finder::Finder};

pub struct TextProcessor<C, F>
where
    C: Colorizer,
    F: Finder,
{
    colorizer: C,
    finder: F,
}

impl<C, F> TextProcessor<C, F>
where
    C: Colorizer,
    F: Finder,
{
    pub fn new(c: C, f: F) -> Self {
        Self {
            colorizer: c,
            finder: f,
        }
    }

    pub fn process_line(&self, s: &str) -> String {
        let mut result = s.to_string();

        let matches = self.finder.find(s);

        for m in matches.iter() {
            result = result.replace(m, &self.colorizer.paint(m))
        }

        result
    }
}

mod tests {
    use super::*;
    use crate::{colorizer::ConsoleColorizer, finder::RegexFinder, processor::TextProcessor};

    #[test]
    fn happy_path() {
        // Arrange
        let c = ConsoleColorizer::default();
        let f = RegexFinder::new(vec!["foo".to_string(), "bar".to_string()]);
        let processor = TextProcessor::new(c, f);
        let input = "foo bar baz";

        // Act
        let actual = processor.process_line(input);

        // Assert
        let expected = "\x1b[31mfoo\x1b[0m \x1b[31mbar\x1b[0m baz";
        assert_eq!(actual, expected);
    }

    #[test]
    fn paint_to_blue() {
        // Arrange
        let c = ConsoleColorizer::new(Some("blue".to_string()));
        let f = RegexFinder::new(vec!["foo".to_string(), "bar".to_string()]);
        let processor = TextProcessor::new(c, f);
        let input = "foo bar baz";

        // Act
        let actual = processor.process_line(input);

        // Assert
        let expected = "\x1b[34mfoo\x1b[0m \x1b[34mbar\x1b[0m baz";
        assert_eq!(actual, expected);
    }
}
