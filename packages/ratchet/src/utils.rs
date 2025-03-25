use std::path::Path;

use regex::Regex;

/// Normalizes the path between operating systems
/// by replacing any backslashes with forward slashes.
pub fn to_normalized_path(path: &Path) -> String {
    path.to_str().unwrap().replace("\\", "/")
}

/// Normalizes the file contents between operating systems
/// by replacing any carriage returns with newlines.
pub fn to_normalized_file_contents(contents: &str) -> String {
    let re = Regex::new(r"\r\n|\r").expect("Failed to compile regex");
    re.replace_all(contents, "\n").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_normalized_path() {
        let path = Path::new("foo\\bar");
        assert_eq!(to_normalized_path(path), "foo/bar");
    }

    #[test]
    fn test_to_normalized_file_contents() {
        let contents = "foo\r\nbar\r";
        assert_eq!(to_normalized_file_contents(contents), "foo\nbar\n");
    }
}
