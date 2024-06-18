use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub version: u8,
    pub rules: HashMap<RuleName, RuleMap>,
}

pub type RuleName = String;
// TODO: Probably don't need file name and hash as the key
pub type RuleMap = HashMap<(FileName, FileHash), Problems>;

type FileName = String;
type FileHash = String;
type Problems = Vec<Problem>;

type Problem = (Start, End, MessageText, MessageHash);

type Start = usize;
type End = usize;
// TODO: The next two could be optional, rules like regex won't have a unique message
type MessageText = String;
type MessageHash = String;
