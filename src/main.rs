use clap::Parser;
use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use rewac_bundler::line_checker::LineChecker;

#[allow(dead_code)]
fn main() {
    let args = Args::parse();

    let libdir = PathBuf::from_str(&args.libdir)
        .unwrap_or_else(|e| panic!("{e}"))
        .canonicalize()
        .unwrap_or_else(|_| panic!("Not found: {}", args.libdir));

    let file = libdir.join("src").join(&args.libname).with_extension("rs");

    let res = simpify_file(&file);

    println!("");
    println!("#[allow(dead_code)]");
    println!("{}", res);
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
