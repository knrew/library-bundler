use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

pub fn simplify(file: impl AsRef<Path>) -> String {
    let reader =
        BufReader::new(File::open(file).unwrap_or_else(|_| panic!("could not open file.")));

    let checker = LineChecker::new();

    let mut is_in_comment = false;

    let mut str = String::new();

    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| panic!("{e}"));
        if line.is_empty() {
        } else if checker.is_comment(&line) {
        } else if checker.is_block_comment_start(&line) {
            is_in_comment = true;
        } else if checker.is_block_comment_end(&line) {
            is_in_comment = false;
        } else if checker.parse_cfg_test(&line) {
            break;
        } else if !is_in_comment {
            str += "    ";
            str += &line.replace("crate", "super");
            str += "\n";
        } else {
        }
    }

    str
}

struct LineChecker {
    re_cfg_test: Regex,
}

impl LineChecker {
    fn new() -> Self {
        Self {
            re_cfg_test: Regex::new(r#"^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*"#).unwrap(),
        }
    }

    fn is_comment(&self, line: &str) -> bool {
        line.trim().starts_with("//")
    }

    fn is_block_comment_start(&self, line: &str) -> bool {
        line.trim().starts_with("/*") || line.trim().starts_with("/*")
    }

    fn is_block_comment_end(&self, line: &str) -> bool {
        line.trim().ends_with("*/")
    }

    // #[cfg(test)]
    pub fn parse_cfg_test(&self, line: &str) -> bool {
        self.re_cfg_test.is_match(line)
    }
}
