use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

pub mod module_collector;
pub mod simplifier;

use module_collector::collect_library_uses;
use simplifier::simplify;

pub fn bundle_library(
    source_path: impl AsRef<Path>,
    library_dir: impl AsRef<Path>,
    library_name: &str,
    enabled_simplification: bool,
) -> String {
    let mut res = fs::read_to_string(&source_path).expect("failed to read file.");

    let uses = collect_library_uses(&source_path, &library_dir, &library_name);

    if uses.is_empty() {
        return res;
    }

    res += "\n";

    let mut tree = ModuleTree::new(Node::new(library_dir.as_ref().join("src")));
    for u in &uses {
        tree.insert(&u);
    }
    res += &bundle(&tree, library_name, enabled_simplification);

    res
}

fn bundle(tree: &ModuleTree, library_name: &str, enabled_simplification: bool) -> String {
    fn dfs(
        library_name: &str,
        nodes: &[Node],
        bundled: &mut [bool],
        path: &PathBuf,
        i: usize,
        depth: usize,
        enabled_simplification: bool,
    ) -> String {
        let mut s = String::new();

        bundled[i] = true;

        let path = path.join(&nodes[i].path);

        let mod_name = if i == 0 {
            library_name
        } else {
            nodes[i].path.to_str().unwrap()
        };
        for _ in 0..depth {
            write!(s, "    ").unwrap();
        }
        writeln!(s, "pub mod {} {{", mod_name).unwrap();

        if nodes[i].child_ids.is_empty() {
            let path = path.with_extension("rs");
            write!(
                s,
                "{}",
                simplify(&path, library_name, depth + 1, enabled_simplification)
            )
            .unwrap();
        } else {
            for &child_id in &nodes[i].child_ids {
                if bundled[child_id] {
                    continue;
                }
                write!(
                    s,
                    "{}",
                    dfs(
                        library_name,
                        nodes,
                        bundled,
                        &path,
                        child_id,
                        depth + 1,
                        enabled_simplification
                    )
                )
                .unwrap();
            }
        }

        for _ in 0..depth {
            write!(s, "    ").unwrap();
        }
        writeln!(s, "}}").unwrap();

        s
    }

    let mut res = String::new();
    res += "#[allow(dead_code)]\n";
    res += &dfs(
        library_name,
        &tree.nodes,
        &mut vec![false; tree.nodes.len()],
        &PathBuf::new(),
        0,
        0,
        enabled_simplification,
    );

    res
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    path: PathBuf,
    child_ids: Vec<usize>,
}

impl Node {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            child_ids: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleTree {
    nodes: Vec<Node>,
    mp: BTreeMap<PathBuf, usize>,
}

impl ModuleTree {
    fn new(root: Node) -> Self {
        let mut mp = BTreeMap::new();
        mp.insert(root.path.clone(), 0);
        Self {
            nodes: vec![root],
            mp,
        }
    }

    fn insert(&mut self, u: &[PathBuf]) {
        if u.is_empty() {
            return;
        }

        if !self.mp.contains_key(&u[0]) {
            let id = self.nodes.len();
            self.nodes[0].child_ids.push(id);
            self.nodes.push(Node::new(u[0].clone()));
            self.mp.insert(u[0].clone(), id);
        }

        for w in u.windows(2) {
            if !self.mp.contains_key(&w[1]) {
                let parent_id = self.mp[&w[0]];
                let id = self.nodes.len();
                self.nodes[parent_id].child_ids.push(id);
                self.nodes.push(Node::new(w[1].clone()));
                self.mp.insert(w[1].clone(), id);
            }
        }
    }
}
