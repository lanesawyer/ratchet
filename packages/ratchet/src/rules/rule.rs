use regex::Regex;

use crate::ratchet_file::Problem;

pub trait Rule {
    /// Default implementation to determine if a file should be analyzed
    fn analyze_file(&self, path: &str) -> bool {
        if self.include().is_some() {
            let include = self.include().unwrap();
            if !include.iter().any(|r| r.is_match(path)) {
                return false;
            }
        }

        if self.exclude().is_some() {
            let exclude = self.exclude().unwrap();
            if exclude.iter().any(|r| r.is_match(path)) {
                return false;
            }
        }

        true
    }

    fn include(&self) -> Option<Vec<Regex>> {
        None
    }

    fn exclude(&self) -> Option<Vec<Regex>> {
        None
    }

    /// Check is the main function that will be called to determine if a file has any problems
    /// and every type of rule will need it's own implementation
    fn check(&self, path: &str, content: &str) -> Vec<Problem>;
}
