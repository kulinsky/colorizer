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

        for r in self.patterns.iter() {
            for c in r.captures_iter(s) {
                results.push(c[0].to_string());
            }
        }

        results
    }
}

mod tests {
    

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
