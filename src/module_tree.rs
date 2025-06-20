use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    path: PathBuf,
    child_ids: Vec<usize>,
}

impl Node {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            child_ids: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleTree {
    nodes: Vec<Node>,
    mp: BTreeMap<PathBuf, usize>,
}

impl ModuleTree {
    pub fn new(root: PathBuf) -> Self {
        let nodes = vec![Node::new(root.clone())];
        let mp = BTreeMap::from([(root, 0)]);
        Self { nodes, mp }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn path(&self, index: usize) -> &PathBuf {
        &self.nodes[index].path
    }

    pub fn childs(&self, index: usize) -> &[usize] {
        &self.nodes[index].child_ids
    }

    pub fn insert(&mut self, us: &[PathBuf]) {
        if us.is_empty() {
            return;
        }

        if !self.mp.contains_key(&us[0]) {
            let id = self.nodes.len();
            self.nodes[0].child_ids.push(id);
            self.nodes.push(Node::new(us[0].clone()));
            self.mp.insert(us[0].clone(), id);
        }

        for w in us.windows(2) {
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
