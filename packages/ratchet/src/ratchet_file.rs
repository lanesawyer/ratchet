use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::{File, read_to_string},
    io::Write,
};

/// Current version of the ratchet file format, past versions may not be compatible
pub const RATCHET_FILE_VERSION: u8 = 1;
/// Default file name for the ratchet file
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

pub type Problem = (Start, End, MessageText, MessageHash);

/// Start and end are the character positions in the file
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

        ron::de::from_str(&contents.unwrap())
            .expect("Failed to deserialize, RON file may be corrupt or an old version")
    }

    pub fn save(&self, file: &String) {
        let pretty_config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let ron = ron::ser::to_string_pretty(self, pretty_config).expect("Serialization failed");

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

            let new_rule_items = new_ratchet.rules.get(rule);

            if let Some(new_rule_items) = new_rule_items {
                let (new_count, old_count) = compare_rule_maps(previous_rule_items, new_rule_items);
                previous_rule_count = old_count;
                new_rule_count = new_count;
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
}

fn compare_rule_maps(old_map: &RuleMap, new_map: &RuleMap) -> (usize, usize) {
    let mut total_old_issues = 0;
    let mut total_new_issues = 0;

    // Step 1: Iterate through the old map
    for (key, old_problems) in old_map {
        if let Some(new_problems) = new_map.get(key) {
            // Key exists in both maps, count issues from both
            total_old_issues += old_problems.len();
            total_new_issues += new_problems.len();
        } else {
            // Key exists only in the old map
            total_old_issues += old_problems.len();
        }
    }

    // Step 2: Iterate through the new map
    for (key, new_problems) in new_map {
        if !old_map.contains_key(key) {
            // Key exists only in the new map
            total_new_issues += new_problems.len();
        }
    }

    // Step 3: Return totals so we can log properly and report if it got worse
    (total_new_issues, total_old_issues)
}

#[cfg(test)]
mod test {
    const TEST_RULE_ONE: &str = "test_rule1";

    #[test]
    fn new_violation_returns_worse() {
        let mut previous_rule_issues = super::RuleMap::new();
        previous_rule_issues.insert(
            ("file1".into(), 1234),
            vec![(1, 2, "message1".into(), "hash1".into())],
        );

        let mut previous_file = super::RatchetFile::new();
        previous_file
            .rules
            .insert(TEST_RULE_ONE.into(), previous_rule_issues);

        let mut new_rule_issues = super::RuleMap::new();
        new_rule_issues.insert(
            // Note the fake hash, it's different because the file "changed"
            ("file1".into(), 4321),
            vec![
                (1, 2, "message1".into(), "hash1".into()),
                (3, 4, "message2".into(), "hash2".into()),
            ],
        );
        let mut new_file = super::RatchetFile::new();
        new_file.rules.insert(TEST_RULE_ONE.into(), new_rule_issues);

        assert!(previous_file.compare(&new_file));
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
            ("file1".into(), 4321),
            vec![(1, 2, "message1".into(), "hash".into())],
        );
        let mut new_file = super::RatchetFile::new();
        new_file.rules.insert(TEST_RULE_ONE.into(), new_rule_issues);

        assert!(!previous_file.compare(&new_file));
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
