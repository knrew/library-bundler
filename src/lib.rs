use std::{collections::BTreeMap, fmt::Write, fs, path::PathBuf};

pub mod bundling_option;
pub mod module_collector;
pub mod simplifier;

use bundling_option::BundlingOption;
use module_collector::collect_library_uses;
use simplifier::simplify;

pub fn bundle(option: &BundlingOption) -> String {
    let mut res = fs::read_to_string(&option.souce_file).expect("failed to read file.");

    let uses = collect_library_uses(&option);

    if uses.is_empty() {
        return res;
    }

    res += "\n";

    let mut tree = ModuleTree::new(Node::new(option.library_dir.join("src")));
    for u in &uses {
        tree.insert(&u);
    }

    if let Some(comment) = &option.comment {
        for line in comment.lines() {
            write!(&mut res, "/// {}\n", line).unwrap();
        }
    }

    write!(
        &mut res,
        "#[allow(unused)]\n{}",
        bundle_dfs(
            &option,
            &tree.nodes,
            &mut vec![false; tree.nodes.len()],
            &PathBuf::new(),
            0,
            0,
        )
    )
    .unwrap();

    res
}

fn bundle_dfs(
    option: &BundlingOption,
    nodes: &[Node],
    bundled: &mut [bool],
    path: &PathBuf,
    i: usize,
    depth: usize,
) -> String {
    let mut s = String::new();

    bundled[i] = true;

    let path = path.join(&nodes[i].path);

    for _ in 0..depth {
        write!(s, "    ").unwrap();
    }
    writeln!(
        s,
        "pub mod {} {{",
        if i == 0 {
            &option.library_name
        } else {
            nodes[i].path.to_str().unwrap()
        }
    )
    .unwrap();

    if nodes[i].child_ids.is_empty() {
        let path = path.with_extension("rs");
        write!(s, "{}", simplify(&option, &path, depth + 1)).unwrap();
    } else {
        for &child_id in &nodes[i].child_ids {
            if bundled[child_id] {
                continue;
            }
            write!(
                s,
                "{}",
                bundle_dfs(option, nodes, bundled, &path, child_id, depth + 1,)
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
