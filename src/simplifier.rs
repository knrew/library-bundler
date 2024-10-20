use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use lazy_static::lazy_static;
use regex::Regex;

pub fn simplify(file: impl AsRef<Path>, num_indent: usize) -> String {
    let reader =
        BufReader::new(File::open(file).unwrap_or_else(|_| panic!("could not open file.")));

    let mut is_in_comment = false;

    let mut str = String::new();

    for line in reader.lines() {
        let mut line = line.unwrap_or_else(|e| panic!("{e}"));
        if line.is_empty() {
            continue;
        }

        if is_in_comment && line.find("*/").is_some() {
            is_in_comment = false;
            continue;
        }

        if is_in_comment {
            continue;
        }

        if is_test_attribute(&line) {
            break;
        }

        if line.find("/*").is_some() {
            is_in_comment = true;
            continue;
        }

        if let Some(pos) = line.find("//") {
            line = line[..pos].to_string();
        }

        if line.trim().is_empty() {
            continue;
        }

        for _ in 0..num_indent {
            str += "    ";
        }
        str += &line.replace("crate", "super");
        str += "\n";
    }

    str
}

fn is_test_attribute(line: &str) -> bool {
    lazy_static! {
        static ref re: Regex = Regex::new(r#"^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*"#).unwrap();
    }
    re.is_match(line)
}
