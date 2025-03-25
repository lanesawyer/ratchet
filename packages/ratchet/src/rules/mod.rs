pub mod regex;
pub mod rule;
pub mod todo;

use regex::RegexRule;
use rule::Rule;
use serde::{Deserialize, Serialize};
use todo::TodoRule;

use crate::ratchet_file::Problem;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // Use a "type" field in the serialized data to distinguish rule types
pub enum RatchetRule {
    Regex(RegexRule),
    Todo(TodoRule),
    // Add other rule types here
    // Example: Custom(CustomRule),
}

impl Rule for RatchetRule {
    fn check(&self, path: &str, content: &str) -> Vec<Problem> {
        match self {
            RatchetRule::Regex(rule) => rule.check(path, content),
            RatchetRule::Todo(rule) => rule.check(path, content),
            // Add other rule implementations here
            // RuleEnum::Custom(rule) => rule.check(path, content),
        }
    }
}
