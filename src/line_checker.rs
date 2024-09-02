use regex::Regex;

pub struct LineChecker {
    re_cfg_test: Regex,
}

impl LineChecker {
    pub fn new() -> Self {
        Self {
            re_cfg_test: Regex::new(r#"^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*"#).unwrap(),
        }
    }

    pub fn is_comment(&self, line: &str) -> bool {
        line.trim().starts_with("//")
    }

    pub fn is_block_comment_start(&self, line: &str) -> bool {
        line.trim().starts_with("/*") || line.trim().starts_with("/*")
    }

    pub fn is_block_comment_end(&self, line: &str) -> bool {
        line.trim().ends_with("*/")
    }

    // #[cfg(test)]
    pub fn parse_cfg_test(&self, line: &str) -> bool {
        self.re_cfg_test.is_match(line)
    }
}
