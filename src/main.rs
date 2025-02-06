use std::path::PathBuf;

use clap::Parser;
use library_bundler::bundle_library;

fn main() {
    let args = Args::parse();
    let bundled_source = bundle_library(&args.source_file, &args.library_dir, &args.library_name());
    print!("{}", bundled_source);
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
