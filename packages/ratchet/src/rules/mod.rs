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
    // Add other rule types here, make sure to add to the macro call below
}

/// Macro to call the inner type of each enum value to power the Rule trait functionality
macro_rules! impl_functions_for_rule_types {
    ($($variant:ident),*) => {
        impl Rule for RatchetRule {
            fn check(&self, path: &str, content: &str) -> Vec<Problem> {
                match self {
                    $(Self::$variant(rule) => rule.check(path, content),)*
                }
            }

            fn include(&self) -> Option<Vec<::regex::Regex>> {
                match self {
                    $(Self::$variant(rule) => rule.include(),)*
                }
            }

            fn exclude(&self) -> Option<Vec<::regex::Regex>> {
                match self {
                    $(Self::$variant(rule) => rule.exclude(),)*
                }
            }
        }
    };
}

impl_functions_for_rule_types!(Regex, Todo);
