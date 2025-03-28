use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use syn::{parse_file, Item, UseTree};

/// sourceファイルでuseされているモジュールのうち，
/// バンドルするライブラリであるものを再帰的に集める
pub fn collect_library_uses(
    source_path: impl AsRef<Path>,
    library_dir: impl AsRef<Path>,
    library_name: &str,
) -> Vec<Vec<PathBuf>> {
    let library_dir = library_dir.as_ref();

    let mut modules = BTreeSet::new();

    // useをさがす対象となるファイル
    let mut targets = vec![source_path.as_ref().to_path_buf()];

    while let Some(p) = targets.pop() {
        let source = fs::read_to_string(&p).expect("failed to read file.");

        for u in collect_uses(&source) {
            match u.get(0) {
                Some(s) if s == library_name => {}
                Some(s) if s == "crate" => {}
                Some(s) if s == "super" => {
                    unimplemented!();
                }
                _ => {
                    continue;
                }
            }

            let mut module = vec![];
            let mut path = library_dir.join("src");

            for s in u.into_iter().skip(1) {
                path = path.join(&s);
                module.push(PathBuf::from(&s));
                if path.with_extension("rs").is_file() {
                    path = path.with_extension("rs");
                    break;
                }
            }

            if path.is_file() {
                if !modules.contains(&module) {
                    modules.insert(module);
                    targets.push(path);
                }
            }
        }
    }

    modules.into_iter().collect()
}

/// sourceファイルでuseされているモジュールを集める
/// 例: `use std::collections::HashMap`があれば`["std", "collections", "HashMap"]`という形で格納される
fn collect_uses(source: &str) -> Vec<Vec<String>> {
    fn dfs(tree: &UseTree, uses: &mut Vec<Vec<String>>, current: &mut Vec<String>) {
        match tree {
            UseTree::Path(ref path) => {
                current.push(path.ident.to_string());
                dfs(&path.tree, uses, current);
                current.pop();
            }
            UseTree::Name(ref name) => {
                current.push(name.ident.to_string());
                uses.push(current.clone());
                current.pop();
            }
            UseTree::Rename(ref rename) => {
                current.push(rename.rename.to_string());
                uses.push(current.clone());
                current.pop();
            }
            UseTree::Glob(_) => {
                current.push("*".to_string());
                uses.push(current.clone());
                current.pop();
            }
            UseTree::Group(ref group) => {
                for item in &group.items {
                    dfs(item, uses, current);
                }
            }
        }
    }

    let file = parse_file(source).expect("failed to parse source file.");

    let mut uses = vec![];

    for item in file.items {
        if let Item::Use(u) = item {
            dfs(&u.tree, &mut uses, &mut vec![]);
        }
    }

    uses
}
