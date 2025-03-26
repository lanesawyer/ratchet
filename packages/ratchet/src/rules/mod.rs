pub mod regex;
pub mod rule;
pub mod todo;

use regex::RegexRule;
use rule::Rule;
use serde::{Deserialize, Serialize};
use todo::TodoRule;

use crate::ratchet_file::Problem;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // Use a "type" field in the serialized data to distinguish rule types
pub enum RatchetRule {
    Regex(RegexRule),
    Todo(TodoRule),
    // Add other rule types here
}

impl Rule for RatchetRule {
    fn check(&self, path: &str, content: &str) -> Vec<Problem> {
        match self {
            RatchetRule::Regex(rule) => rule.check(path, content),
            RatchetRule::Todo(rule) => rule.check(path, content),
        }
    }
}
