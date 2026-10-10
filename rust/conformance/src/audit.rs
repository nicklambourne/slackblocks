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
            Item::Struct(s) if public(&s.vis) => {
                names.insert(s.ident.to_string());
            }
            Item::Enum(e) if public(&e.vis) => {
                names.insert(e.ident.to_string());
            }
            Item::Type(t) if public(&t.vis) => {
                names.insert(t.ident.to_string());
            }
            Item::Trait(t) if public(&t.vis) => {
                names.insert(t.ident.to_string());
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
                    if let Some((t, _)) = &i.trait_ {
                        entries.insert(format!("impl {}", t.to_token_stream()));
                    } else {
                        for item in i.items {
                            match item {
                                syn::ImplItem::Fn(f) if public(&f.vis) => {
                                    entries.insert(f.sig.to_token_stream().to_string());
                                }
                                syn::ImplItem::Const(c) if public(&c.vis) => {
                                    entries.insert(format!(
                                        "const {}: {}",
                                        c.ident,
                                        c.ty.to_token_stream()
                                    ));
                                }
                                _ => (),
                            }
                        }
                    }
                }
                Item::Struct(s) if public(&s.vis) => {
                    if let Some(entries) = result.get_mut(&s.ident.to_string()) {
                        for (i, f) in s.fields.iter().enumerate().filter(|(_, f)| public(&f.vis)) {
                            entries.insert(format!(
                                "field {}: {}",
                                f.ident
                                    .as_ref()
                                    .map(ToString::to_string)
                                    .unwrap_or_else(|| i.to_string()),
                                f.ty.to_token_stream()
                            ));
                        }
                    }
                }
                Item::Enum(e) if public(&e.vis) => {
                    if let Some(entries) = result.get_mut(&e.ident.to_string()) {
                        for v in e.variants {
                            entries.insert(format!(
                                "variant {} {}",
                                v.ident,
                                v.fields.to_token_stream()
                            ));
                        }
                    }
                }
                Item::Type(t) if public(&t.vis) => {
                    if let Some(entries) = result.get_mut(&t.ident.to_string()) {
                        entries.insert(t.to_token_stream().to_string());
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

fn doc(attributes: &[syn::Attribute]) -> String {
    attributes
        .iter()
        .filter_map(|attribute| {
            if !attribute.path().is_ident("doc") {
                return None;
            }
            let syn::Meta::NameValue(value) = &attribute.meta else {
                return None;
            };
            let syn::Expr::Lit(literal) = &value.value else {
                return None;
            };
            let syn::Lit::Str(text) = &literal.lit else {
                return None;
            };
            Some(text.value().trim().to_owned())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn signature(sig: &syn::Signature) -> String {
    let function: syn::ItemFn = syn::parse_quote!(#sig {});
    let file = syn::File {
        frontmatter: None,
        shebang: None,
        attrs: vec![],
        items: vec![Item::Fn(function)],
    };
    let formatted = prettyplease::unparse(&file);
    formatted[..formatted.rfind('{').expect("function has a body")]
        .trim_end()
        .to_owned()
}
/// Extract actual public Rust documentation and signatures for the site renderer.
/// This uses the stable Rust syntax tree, not unstable rustdoc JSON or regex parsing.
pub fn documentation(source: &Path) -> Value {
    let names = inventory(source);
    let mut result: BTreeMap<String, Value> = names
        .as_object()
        .unwrap()
        .keys()
        .map(|name| {
            (
                name.clone(),
                json!({"name":name,"doc":"","see":[],"constants":[],"members":[]}),
            )
        })
        .collect();
    let mut files = Vec::new();
    sources(source, &mut files);
    for file in files {
        for item in file.items {
            match item {
                Item::Struct(item) => {
                    if let Some(entry) = result.get_mut(&item.ident.to_string()) {
                        entry["doc"] = json!(doc(&item.attrs));
                    }
                }
                Item::Enum(item) => {
                    if let Some(entry) = result.get_mut(&item.ident.to_string()) {
                        entry["doc"] = json!(doc(&item.attrs));
                        for variant in item.variants {
                            entry["constants"].as_array_mut().unwrap().push(json!({"name":variant.ident.to_string(),"wire":variant.fields.to_token_stream().to_string(),"doc":doc(&variant.attrs)}));
                        }
                    }
                }
                Item::Impl(item) if item.trait_.is_none() => {
                    let syn::Type::Path(p) = *item.self_ty else {
                        continue;
                    };
                    let name = p.path.segments.last().unwrap().ident.to_string();
                    let Some(entry) = result.get_mut(&name) else {
                        continue;
                    };
                    for member in item.items {
                        match member {
                            syn::ImplItem::Fn(f) if public(&f.vis)=>entry["members"].as_array_mut().unwrap().push(json!({"name":f.sig.ident.to_string(),"signature":signature(&f.sig),"doc":doc(&f.attrs),"params":[],"returns":"","throws":[]})),
                            syn::ImplItem::Const(c) if public(&c.vis)=>entry["constants"].as_array_mut().unwrap().push(json!({"name":c.ident.to_string(),"wire":c.ty.to_token_stream().to_string(),"doc":doc(&c.attrs)})),
                            _=>(),
                        }
                    }
                }
                Item::Fn(f) if public(&f.vis) => {
                    if let Some(entry) = result.get_mut(&f.sig.ident.to_string()) {
                        entry["doc"] = json!(doc(&f.attrs));
                        entry["members"].as_array_mut().unwrap().push(json!({"name":f.sig.ident.to_string(),"signature":signature(&f.sig),"doc":"","params":[],"returns":"","throws":[]}));
                    }
                }
                Item::Const(c) if public(&c.vis) => {
                    if let Some(entry) = result.get_mut(&c.ident.to_string()) {
                        entry["doc"] = json!(doc(&c.attrs));
                    }
                }
                _ => (),
            }
        }
    }
    json!({"types":result.into_values().collect::<Vec<_>>()})
}
