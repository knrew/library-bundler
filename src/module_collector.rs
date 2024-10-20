use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use syn::parse_file;

/// 使用されているライブラリ(が定義されているファイル)を集める
/// まずsource内のuseのうちlibrary_nameで指定されているモジュールを集め，
/// 再帰的にそのファイル内で使用されているモジュールを集める
pub fn collect_module_files(
    source: &str,
    library_dir: impl AsRef<Path>,
    library_name: &str,
) -> Vec<PathBuf> {
    let uses = collect_uses(&source);

    let mut files = BTreeSet::new();

    for module in uses
        .iter()
        .filter(|module| !module.is_empty() && &module[0] == library_name)
    {
        let mut file = library_dir.as_ref().join("src");
        for s in module.iter().skip(1) {
            file = file.join(s);
            if file.with_extension("rs").exists() {
                let file = file.with_extension("rs").canonicalize().unwrap();
                files.insert(file);
                break;
            }
        }
    }

    let mut res = vec![];

    while let Some(file) = files.pop_first() {
        let source = fs::read_to_string(&file).unwrap();
        for module in collect_uses(&source)
            .iter()
            .filter(|elem| !elem.is_empty() && &elem[0] == "crate")
        {
            let mut file = library_dir.as_ref().join("src");
            for s in module.iter().skip(1) {
                file = file.join(s);
                if file.with_extension("rs").exists() {
                    let file = file.with_extension("rs").canonicalize().unwrap();
                    files.insert(file);
                    break;
                }
            }
        }

        res.push(file);
    }

    res.sort_unstable();
    res.dedup();

    res
}

/// sourceで指定されている文字列の中からuseされているモジュールを集める
/// 例: `std::collections::HashMap`は`["std", "collections", "HashMap"]`という形で格納される
fn collect_uses(source: &str) -> Vec<Vec<String>> {
    fn dfs(tree: &syn::UseTree, items: &mut Vec<String>, uses: &mut Vec<Vec<String>>) {
        match tree {
            syn::UseTree::Path(ref path) => {
                items.push(path.ident.to_string());
                dfs(&path.tree, items, uses);
                items.pop();
            }
            syn::UseTree::Name(ref name) => {
                items.push(name.ident.to_string());
                uses.push(items.clone());
                items.pop();
            }
            syn::UseTree::Rename(ref rename) => {
                items.push(rename.rename.to_string());
                uses.push(items.clone());
                items.pop();
            }
            syn::UseTree::Glob(_) => {
                items.push("*".to_string());
                uses.push(items.clone());
                items.pop();
            }
            syn::UseTree::Group(ref group) => {
                for item in &group.items {
                    dfs(item, items, uses);
                }
            }
        }
    }

    let file = parse_file(source).expect("failed to parse source file.");

    let mut uses = vec![];

    for item in file.items.iter().filter_map(|item| {
        if let syn::Item::Use(u) = item {
            Some(u)
        } else {
            None
        }
    }) {
        dfs(&item.tree, &mut vec![], &mut uses);
    }

    uses.sort_unstable();
    uses.dedup();

    uses
}
