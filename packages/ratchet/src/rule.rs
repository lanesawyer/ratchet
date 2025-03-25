use regex::Regex;

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

    // todo: what's this look like
    fn check(&self, path: &str, content: &str) -> Vec<(usize, usize, String, String)>;

    fn include(&self) -> Option<Vec<Regex>>;
    fn exclude(&self) -> Option<Vec<Regex>>;
}

pub struct RegexRule {
    pub regex: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl Rule for RegexRule {
    fn check(&self, path: &str, content: &str) -> Vec<(usize, usize, String, String)> {
        let mut problems: Vec<(usize, usize, String, String)> = Vec::new();

        let rule_regex = Regex::new(&self.regex).expect("Failed to compile regex");
        let matches: Vec<_> = rule_regex.find_iter(content).collect();
        println!("Found {} matches for {}", matches.len(), path);
        for found in matches {
            let value = (
                found.start(),
                found.end(),
                rule_regex.to_string(),
                seahash::hash(rule_regex.as_str().as_bytes()).to_string(),
            );
            problems.push(value);
        }

        problems
    }

    fn include(&self) -> Option<Vec<Regex>> {
        self.include.as_ref().map(|include| {
            include
                .iter()
                .map(|i| Regex::new(i).expect("Failed to compile include regex"))
                .collect()
        })
    }

    fn exclude(&self) -> Option<Vec<Regex>> {
        self.exclude.as_ref().map(|exclude| {
            exclude
                .iter()
                .map(|e| Regex::new(e).expect("Failed to compile include regex"))
                .collect()
        })
    }
}
