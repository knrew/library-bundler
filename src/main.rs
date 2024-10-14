use clap::Parser;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use rewac_bundler::line_checker::LineChecker;

#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    let lib_dir = PathBuf::from_str(&args.libdir)
        .unwrap_or_else(|e| panic!("{e}"))
        .canonicalize()
        .unwrap_or_else(|_| panic!("Not found: {}", args.libdir));

    let mut files = vec![];

    if args.libname.to_lowercase() == "all" {
        let src_dir = lib_dir.join("src");
        for file in fs::read_dir(&src_dir).unwrap_or_else(|e| panic!("{e}")) {
            let file = file.unwrap_or_else(|e| panic!("{e}")).path();
            if file.file_stem().unwrap() == "lib" {
                continue;
            } else {
                files.push(file);
            }
        }
    } else {
        let file = lib_dir.join("src").join(&args.libname).with_extension("rs");
        files.push(file);
    }

    let mut str = String::new();
    str += "\n";

    for file in &files {
        str += "#[allow(dead_code)]\n";
        str += "mod ";
        str += &format!("{} {{\n", file.file_stem().unwrap().to_string_lossy());
        str += &simpify_file(file);
        str += "}\n";
    }

    println!("{str}");
}

fn simpify_file(file: &PathBuf) -> String {
    let reader = BufReader::new(
        File::open(file)
            .unwrap_or_else(|_| panic!("could not open file: {}", file.to_string_lossy())),
    );

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
            str += &line;
            str += "\n";
        } else {
        }
    }

    str
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'l', long = "lib", default_value = "/home/rew/codes/rewac/")]
    libdir: String,

    #[arg(short = 'n', long = "name")]
    libname: String,
}
