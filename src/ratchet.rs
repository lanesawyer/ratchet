use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub items: HashMap<(FileName, FileHash), RatchetItem>,
}

pub type RatchetItem = (FileName, FileHash, Problems);

type FileName = String;
type FileHash = String;
type Problems = Vec<Problem>;

type Problem = (Line, Column, Message);

type Line = u32;
type Column = u32;
type Message = String;
