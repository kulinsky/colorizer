use regex::Regex;

pub trait Finder {
    fn find(&self, s: &str) -> Vec<String>;
}

pub struct RegexFinder {
    patterns: Vec<Regex>,
}

impl RegexFinder {
    pub fn new(patterns: Vec<String>) -> Self {
        Self {
            patterns: patterns.iter().map(|p| Regex::new(p).unwrap()).collect(),
        }
    }
}

impl Finder for RegexFinder {
    fn find(&self, s: &str) -> Vec<String> {
        let mut results = vec![];

        self.patterns
            .iter()
            .for_each(|r| results.extend(r.find_iter(s).map(|m| m.as_str().to_string())));

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::RegexFinder;

    #[test]
    fn happy_path() {
        // Arrange
        let patterns = vec!["foo".to_string(), "bar".to_string()];
        let finder = RegexFinder::new(patterns);
        let input = "foo bar baz";

        // Act
        let actual = finder.find(input);

        // Assert
        let expected = vec!["foo".to_string(), "bar".to_string()];
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_email() {
        // Arrange
        let patterns = vec!["[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z0-9]+".to_string()];
        let finder = RegexFinder::new(patterns);
        let input = "hello test@mail.com world";

        // Act
        let actual = finder.find(input);

        // Assert
        let expected = vec!["test@mail.com".to_string()];
        assert_eq!(actual, expected);
    }
}
