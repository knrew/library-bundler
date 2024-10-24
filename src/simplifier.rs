use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn simplify(file: impl AsRef<Path>, num_indent: usize) -> String {
    let reader =
        BufReader::new(File::open(file).unwrap_or_else(|_| panic!("could not open file.")));

    let mut lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    {
        let mut is_in_comment = false;
        for line in &mut lines {
            if is_in_comment {
                if line.find("*/").is_some() {
                    is_in_comment = false;
                }
                line.clear();
            } else {
                if let Some(_) = line.find("/*") {
                    is_in_comment = true;
                    line.clear();
                }

                if let Some(pos) = line.find("//") {
                    *line = line[..pos].to_string();
                }
            }
        }
    }

    lines = lines
        .into_iter()
        .take_while(|line| line.find("#[test]").is_none() && line.find("#[cfg(test)]").is_none())
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();

    let mut res = String::new();

    for line in &mut lines {
        for _ in 0..num_indent {
            res += "    ";
        }
        res += &line.replace("crate", "super");
        res += "\n";
    }

    res
}
