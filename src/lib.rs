pub mod bundling_option;
pub mod module_collector;
pub mod module_tree;
pub mod simplifier;

use std::{fmt::Write, fs, path::PathBuf};

use anyhow::{Context, Result};

use module_collector::collect_all_uses;
use simplifier::simplify;

use crate::{bundling_option::BundlingOption, module_tree::ModuleTree};

pub fn bundle() -> Result<String> {
    let option = BundlingOption::new()?;

    let mut tree = ModuleTree::new(option.library_dir.join("src"));

    let uses = collect_all_uses(&option);
    for u in uses {
        tree.insert(&u);
    }

    let mut res = fs::read_to_string(&option.souce_file)?;

    if tree.len() <= 1 {
        return Ok(res);
    }

    writeln!(&mut res)?;

    for line in option.comment.lines() {
        writeln!(&mut res, "/// {}", line)?;
    }
    writeln!(&mut res, "#[allow(unused)]")?;

    let module = traverse_tree(&option, &tree, 0, !0, PathBuf::new(), 0)?;
    write!(&mut res, "{}", module)?;

    Ok(res)
}

fn traverse_tree(
    option: &BundlingOption,
    tree: &ModuleTree,
    id: usize,
    prev_id: usize,
    path: PathBuf,
    depth: usize,
) -> Result<String> {
    let mut res = String::new();

    let path = path.join(tree.path(id));

    let mod_name = if id == 0 {
        &option.library_name
    } else {
        tree.path(id)
            .to_str()
            .context("failed to convert tree path to str")?
    };
    insert_indent(&mut res, depth)?;
    writeln!(res, "pub mod {} {{", mod_name)?;

    if tree.childs(id).is_empty() {
        let path = path.with_extension("rs");

        if id != 0 {
            let mut source = fs::read_to_string(path)?;
            if option.enabled_simplification {
                source = simplify(source)
            }

            source = source.replace("crate", &format!("crate::{}", option.library_name));

            for line in source.lines() {
                insert_indent(&mut res, depth + 1)?;
                writeln!(res, "{}", line)?;
            }
        }
    }

    for &next_id in tree.childs(id) {
        if next_id == prev_id {
            continue;
        }

        write!(
            res,
            "{}",
            traverse_tree(option, tree, next_id, id, path.clone(), depth + 1)?
        )?;
    }

    insert_indent(&mut res, depth)?;
    writeln!(res, "}}")?;

    Ok(res)
}

fn insert_indent(s: &mut String, depth: usize) -> Result<()> {
    for _ in 0..depth {
        write!(s, "    ")?;
    }
    Ok(())
}
