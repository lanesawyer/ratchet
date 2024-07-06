use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::{read_to_string, File},
    io::Write,
};

pub const RATCHET_FILE_VERSION: u8 = 1;
pub const RATCHET_FILE: &str = "ratchet.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub version: u8,
    pub rules: BTreeMap<RuleName, RuleMap>,
}

pub type RuleName = String;
// TODO: Probably don't need file name and hash as the key
pub type RuleMap = BTreeMap<(FileName, FileHash), Problems>;

type FileName = String;
type FileHash = u64;
type Problems = Vec<Problem>;

type Problem = (Start, End, MessageText, MessageHash);

type Start = usize;
type End = usize;
// TODO: The next two could be optional, rules like regex won't have a unique message
type MessageText = String;
type MessageHash = String;

impl RatchetFile {
    pub fn new() -> Self {
        RatchetFile {
            version: RATCHET_FILE_VERSION,
            rules: BTreeMap::new(),
        }
    }

    pub fn load(file: &String) -> Self {
        let contents = read_to_string(file);

        if contents.is_err() {
            println!("No ratchet results file found, evaluating initial baseline");
            return RatchetFile::new();
        }

        ron::de::from_str(&contents.unwrap()).expect("Failed to deserialize")
    }

    pub fn save(&self, file: &String) {
        let pretty_config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let ron = ron::ser::to_string_pretty(self, pretty_config).expect("Serialization failed");
        let ron = format!("{}\n", ron);

        let mut file = File::create(file).expect("Failed to create file");
        file.write_all(ron.as_bytes())
            .expect("Failed to write to file");
    }

    /// Compare the current ratchet file to a new one
    /// Returns true if the new ratchet file is worse
    pub fn compare(&self, new_ratchet: &Self) -> bool {
        let mut got_worse = false;

        // for each rule, see if it got better or worse than the previous
        for (rule, previous_rule_items) in &self.rules {
            let mut previous_rule_count = 0;
            let mut new_rule_count = 0;

            match new_ratchet.rules.get(rule) {
                Some(new_rule) => {
                    for (key, value) in previous_rule_items {
                        match new_rule.get(key) {
                            Some(new_value) => {
                                previous_rule_count += value.len();
                                new_rule_count += new_value.len();
                            }
                            None => println!("Key: {:?} does not exist in the current file", key),
                        }
                    }
                }
                // TODO: If a rule wasn't found, it could be because the hash changed!
                // That means we should ditch the count from the old rule and add the new rule
                // ... but we can't, because we don't know the new one. So we need to go through
                // any new ones that weren't in the old one, and add up those! Shoot.
                None => println!("Rule {} does not exist in the current file", rule),
            }

            got_worse = new_rule_count > previous_rule_count;
            match new_rule_count.cmp(&previous_rule_count) {
                Ordering::Greater => {
                    println!(
                        "❌ Rule {} got worse ({} new issues out of {} total)",
                        rule,
                        new_rule_count - previous_rule_count,
                        new_rule_count
                    );
                }
                Ordering::Less => {
                    println!(
                        "🛠️ Rule {} improved ({} issues fixed out of {} total)",
                        rule,
                        previous_rule_count - new_rule_count,
                        new_rule_count
                    );
                }
                Ordering::Equal => {
                    println!("✔️ Rule {} did not change ({} total)", rule, new_rule_count);
                }
            }
        }
        got_worse
    }

    pub fn compare_new(&self, new_ratchet: &Self) -> bool {
        let mut total_old_issues = 0;
        let mut total_new_issues = 0;

        // Step 2: Iterate through old rules
        for (rule, previous_rule_items) in &self.rules {
            let mut rule_old_count = 0;
            let mut rule_new_count = 0;

            if let Some(new_rule) = new_ratchet.rules.get(rule) {
                // Rule exists in both old and new sets
                for (key, value) in previous_rule_items {
                    rule_old_count += value.len();
                    if let Some(new_value) = new_rule.get(key) {
                        rule_new_count += new_value.len();
                    }
                }
            } else {
                // Rule does not exist in new set, count all issues from old
                for value in previous_rule_items.values() {
                    rule_old_count += value.len();
                }
            }

            total_old_issues += rule_old_count;
            total_new_issues += rule_new_count;
        }

        // Step 3: Iterate through new rules that weren't in old rules
        for (rule, new_rule_items) in &new_ratchet.rules {
            if !&self.rules.contains_key(rule) {
                // This is a new rule, count its issues
                for value in new_rule_items.values() {
                    total_new_issues += value.len();
                }
            }
        }

        total_new_issues > total_old_issues
    }
}

#[cfg(test)]
mod test {
    const TEST_RULE_ONE: &str = "test_rule1";

    #[test]
    fn new_violation_returns_worse() {
        let mut previous_rule_issues = super::RuleMap::new();
        previous_rule_issues.insert(
            ("file1".into(), 1234),
            vec![(1, 2, "message".into(), "hash".into())],
        );

        let mut previous_file = super::RatchetFile::new();
        previous_file
            .rules
            .insert("test_rule".into(), previous_rule_issues);

        let mut new_rule_issues = super::RuleMap::new();
        new_rule_issues.insert(
            // TODO: Change the hash once we have file hashing
            ("file1".into(), 1234),
            vec![
                (1, 2, "message1".into(), "hash".into()),
                (3, 4, "message2".into(), "hash".into()),
            ],
        );
        let mut new_file = super::RatchetFile::new();
        new_file.rules.insert(TEST_RULE_ONE.into(), new_rule_issues);

        assert!(previous_file.compare_new(&new_file));
    }

    #[test]
    fn new_improvement_returns_better() {
        let mut previous_rule_issues = super::RuleMap::new();
        previous_rule_issues.insert(
            ("file1".into(), 1234),
            vec![
                (1, 2, "message".into(), "hash".into()),
                (3, 4, "message2".into(), "hash".into()),
            ],
        );

        let mut previous_file = super::RatchetFile::new();
        previous_file
            .rules
            .insert(TEST_RULE_ONE.into(), previous_rule_issues);

        let mut new_rule_issues = super::RuleMap::new();
        new_rule_issues.insert(
            // TODO: Change the hash once we have file hashing
            ("file1".into(), 1234),
            vec![(1, 2, "message1".into(), "hash".into())],
        );
        let mut new_file = super::RatchetFile::new();
        new_file.rules.insert(TEST_RULE_ONE.into(), new_rule_issues);

        assert!(!previous_file.compare_new(&new_file));
    }

    #[test]
    fn file_rename_no_changes_returns_same() {
        let mut previous_rule_issues = super::RuleMap::new();
        previous_rule_issues.insert(
            ("file1".into(), 1234),
            vec![(1, 2, "message".into(), "hash".into())],
        );

        let mut previous_file = super::RatchetFile::new();
        previous_file
            .rules
            .insert(TEST_RULE_ONE.into(), previous_rule_issues);

        let mut new_rule_issues = super::RuleMap::new();
        new_rule_issues.insert(
            ("file1_renamed".into(), 1234),
            vec![(1, 2, "message".into(), "hash".into())],
        );
        let mut new_file = super::RatchetFile::new();
        new_file.rules.insert(TEST_RULE_ONE.into(), new_rule_issues);

        assert!(!previous_file.compare(&new_file));
    }
}
