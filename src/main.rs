use std::{fs, path::PathBuf};

use clap::Parser;
use library_bundler::{library_collector::collect_libraries, simplifier::simplify};

// const LIBRARY_DIR: &str = "/home/rew/codes/rewac/";
// const LIBRARY_NAME: &str = "rewac";

fn main() {
    let args = Args::parse();

    let source = fs::read_to_string(&args.source_file).unwrap_or_else(|_| {
        panic!("failed to read {:?}", args.source_file);
    });

    let libraries = collect_libraries(&source, &args.library_dir, &args.library_name());

    let mut res = String::new();

    res += &source;
    res += "\n";
    res += "#[allow(dead_code)]\n";
    res += &format!("mod {} {{\n", args.library_name());

    for library in &libraries {
        let filename = args
            .library_dir
            .join("src")
            .join(&library)
            .with_extension("rs");
        res += &format!("pub mod {}{{\n{}}}\n", library, simplify(&filename));
    }

    res += "}";
    println!("{}", res);
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'l', long = "library")]
    library_dir: PathBuf,

    #[arg(short = 'n', long = "name")]
    library_name: Option<String>,

    source_file: PathBuf,
}

impl Args {
    fn library_name(&self) -> String {
        if let Some(name) = &self.library_name {
            name.clone()
        } else {
            self.library_dir
                .file_name()
                .expect("failed to parse filename")
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
