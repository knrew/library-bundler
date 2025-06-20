use std::path::PathBuf;

use clap::Parser;

pub struct BundlingOption {
    /// ライブラリ名
    pub library_name: String,

    /// ライブラリのディレクトリ
    pub library_dir: PathBuf,

    /// バンドル元のソースファイル
    pub souce_file: PathBuf,

    /// バンドル時にライブラリを簡略化するかどうか
    /// (コメントや空行削除など)
    pub enabled_simplification: bool,

    /// バンドル時にライブラリの説明として付与するコメント
    pub comment: Option<String>,
}

impl BundlingOption {
    pub fn new() -> Self {
        let args = Args::parse();

        let library_name = args.library_name.unwrap_or_else(|| {
            args.library_dir
                .file_name()
                .expect("failed to parse filename")
                .to_str()
                .unwrap()
                .to_string()
        });

        BundlingOption {
            library_name,
            library_dir: args.library_dir,
            souce_file: args.source_file,
            comment: args.comment,
            enabled_simplification: !args.disabled_simplification,
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'l', long = "library")]
    library_dir: PathBuf,

    #[arg(short = 'n', long = "name")]
    library_name: Option<String>,

    #[arg(long)]
    disabled_simplification: bool,

    #[arg(short = 'c', long = "comment")]
    comment: Option<String>,

    source_file: PathBuf,
}
