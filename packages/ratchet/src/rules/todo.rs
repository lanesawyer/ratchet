use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::ratchet_file::Problem;

const TODO_REGEX: &str = "TODO";

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoRule {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl TodoRule {
    pub fn check(&self, path: &str, content: &str) -> Vec<Problem> {
        let mut problems: Vec<Problem> = Vec::new();

        let rule_regex = Regex::new(TODO_REGEX).expect("Failed to compile TODO regex");
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
}
