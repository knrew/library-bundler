use std::{collections::BTreeSet, fs, path::Path};

use syn::parse_file;

/// sourceでuseされているクレートのうち，自作ライブラリだけを集める
pub fn collect_libraries(
    source: &str,
    library_dir: impl AsRef<Path>,
    library_name: &str,
) -> Vec<String> {
    let uses = collect_uses(&source);

    let mut libraries = BTreeSet::new();
    // let mut is_all = false;

    for elem in uses
        .iter()
        .filter(|elem| !elem.is_empty() && &elem[0] == library_name)
    {
        if elem.len() == 1 || elem[1] == "*" {
            // is_all = true;
            // break;
        }

        let mut stk = vec![elem[1].clone()];

        while let Some(lib) = stk.pop() {
            if libraries.contains(&lib) {
                continue;
            }

            libraries.insert(lib.clone());

            let path = library_dir
                .as_ref()
                .join("src")
                .join(&lib)
                .with_extension("rs");

            if !path.exists() {
                continue;
            }

            let s = fs::read_to_string(&path).unwrap();
            for elem in collect_uses(&s)
                .iter()
                .filter(|elem| !elem.is_empty() && &elem[0] == "crate")
            {
                stk.push(elem[1].clone())
            }
        }
    }

    libraries.into_iter().collect()
}

/// useされているクレートを集める
/// 例: `std::collections::HashMap`は`["std", "collections", "HashMap"]`となる
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
