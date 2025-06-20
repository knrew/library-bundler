use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone)]
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
    pub comment: String,
}

impl BundlingOption {
    pub fn new() -> Self {
        let args = Args::parse();

        let library_name = args
            .library_name
            .or_else(|| Some(args.library_dir.file_name()?.to_str()?.to_string()))
            .unwrap();

        let comment = args.comment.unwrap_or_else(|| String::new());

        BundlingOption {
            library_name,
            library_dir: args.library_dir,
            souce_file: args.source_file,
            comment,
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
