//! Parse Rust syntax to inventory the actual public facade, independently of the model.
use quote::ToTokens;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
use syn::{Item, UseTree, Visibility};

fn public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}
fn imports(tree: &UseTree, names: &mut BTreeSet<String>) {
    match tree {
        UseTree::Path(p) => imports(&p.tree, names),
        UseTree::Name(n) => {
            assert!(names.insert(n.ident.to_string()), "duplicate export");
        }
        UseTree::Rename(n) => {
            assert!(names.insert(n.rename.to_string()), "duplicate export");
        }
        UseTree::Group(g) => g.items.iter().for_each(|t| imports(t, names)),
        UseTree::Glob(_) => panic!("public wildcard exports require an explicit audit"),
    }
}
fn sources(path: &Path, output: &mut Vec<syn::File>) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, output);
        } else if path.extension().is_some_and(|v| v == "rs") {
            output.push(syn::parse_file(&fs::read_to_string(path).unwrap()).unwrap());
        }
    }
}
/// Inventory exports, inherent methods, and trait implementations from source syntax.
pub fn inventory(source: &Path) -> Value {
    let facade = syn::parse_file(&fs::read_to_string(source.join("lib.rs")).unwrap()).unwrap();
    let mut names = BTreeSet::new();
    for item in &facade.items {
        match item {
            Item::Use(u) if public(&u.vis) => imports(&u.tree, &mut names),
            Item::Const(c) if public(&c.vis) => {
                names.insert(c.ident.to_string());
            }
            Item::Fn(f) if public(&f.vis) => {
                names.insert(f.sig.ident.to_string());
            }
            Item::Mod(m) if public(&m.vis) => panic!("public modules require an explicit audit"),
            _ => (),
        }
    }
    let mut result: BTreeMap<String, BTreeSet<String>> =
        names.into_iter().map(|n| (n, BTreeSet::new())).collect();
    let mut files = Vec::new();
    sources(source, &mut files);
    for file in files {
        for item in file.items {
            match item {
                Item::Impl(i) => {
                    let syn::Type::Path(p) = *i.self_ty else {
                        continue;
                    };
                    let name = p.path.segments.last().unwrap().ident.to_string();
                    let Some(entries) = result.get_mut(&name) else {
                        continue;
                    };
                    if let Some((_, t, _)) = &i.trait_ {
                        entries.insert(format!("impl {}", t.to_token_stream()));
                    } else {
                        for item in i.items {
                            if let syn::ImplItem::Fn(f) = item {
                                if public(&f.vis) {
                                    entries.insert(f.sig.to_token_stream().to_string());
                                }
                            }
                        }
                    }
                }
                Item::Fn(f) if public(&f.vis) => {
                    if let Some(entries) = result.get_mut(&f.sig.ident.to_string()) {
                        entries.insert(f.sig.to_token_stream().to_string());
                    }
                }
                Item::Const(c) if public(&c.vis) => {
                    if let Some(entries) = result.get_mut(&c.ident.to_string()) {
                        entries.insert(format!("const {}", c.ty.to_token_stream()));
                    }
                }
                _ => (),
            }
        }
    }
    json!(result)
}
