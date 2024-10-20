use std::{collections::BTreeMap, fs, path::PathBuf};

use clap::Parser;
use library_bundler::{module_collector::collect_module_files, simplifier::simplify};

fn main() {
    let args = Args::parse();

    let source = fs::read_to_string(&args.source_file).unwrap_or_else(|_| {
        panic!("failed to read {:?}", args.source_file);
    });

    let files = collect_module_files(&source, &args.library_dir, &args.library_name());

    let mut res = String::new();

    res += &source;

    if files.is_empty() {
        println!("{}", res);
        return;
    }

    let mut mp = BTreeMap::new();

    for path in &files {
        let module_names = path
            .parent()
            .unwrap()
            .strip_prefix(&args.library_dir.join("src"))
            .unwrap()
            .iter()
            .map(|m| m.to_str().unwrap().to_string())
            .collect::<Vec<_>>();

        mp.entry(module_names).or_insert(vec![]).push(path);
    }

    res += "\n";
    res += "#[allow(dead_code)]\n";
    res += &format!("mod {} {{\n", args.library_name());

    for (parent_modules, paths) in &mp {
        for (d, module) in parent_modules.iter().enumerate() {
            for _ in 0..d + 1 {
                res += "    ";
            }
            res += &format!("pub mod {} {{\n", module);
        }

        for path in paths {
            for _ in 0..parent_modules.len() + 1 {
                res += "    ";
            }
            res += &format!(
                "pub mod {} {{\n",
                path.file_stem().unwrap().to_str().unwrap()
            );

            res += &simplify(&path, parent_modules.len() + 2);
            for _ in 0..parent_modules.len() + 1 {
                res += "    ";
            }
            res += "}\n";
        }

        for d in (0..parent_modules.len()).rev() {
            for _ in 0..d + 1 {
                res += "    ";
            }
            res += "}\n";
        }
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
