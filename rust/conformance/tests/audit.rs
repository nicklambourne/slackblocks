use serde_json::Value;
use slackblocks_conformance::{audit::inventory, valid::VALID};
use std::{collections::BTreeSet, fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn json(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(root().join(path)).unwrap()).unwrap()
}
#[test]
fn actual_public_exports_and_signatures_match_reviewed_inventory() {
    let actual = inventory(&root().join("rust/src"));
    assert_eq!(
        actual,
        json("rust/conformance/api-inventory.json"),
        "Review new exports, methods and trait implementations before updating the inventory"
    );
    let decisions = json("rust/conformance/api-coverage.json");
    assert_eq!(
        actual.as_object().unwrap().keys().collect::<BTreeSet<_>>(),
        decisions.as_object().unwrap().keys().collect()
    );
    for (name, decision) in decisions.as_object().unwrap() {
        assert!(
            !decision.as_str().unwrap().trim().is_empty(),
            "missing coverage decision for {name}"
        );
    }
}
#[test]
fn every_shared_capability_maps_to_native_types_and_independent_examples() {
    let mappings = json("rust/conformance/capabilities.json");
    let coverage = json("spec/coverage.json");
    let capabilities = coverage["capabilities"].as_object().unwrap();
    assert_eq!(
        mappings
            .as_object()
            .unwrap()
            .values()
            .map(|v| v.as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        capabilities.keys().map(String::as_str).collect()
    );
    let model = json("spec/model.json");
    let rename = |s: &str| match s {
        "Option" => "SelectOption".to_owned(),
        "OptionGroup" => "SelectOptionGroup".to_owned(),
        "Confirmation" => "ConfirmationDialogue".to_owned(),
        _ => s.to_owned(),
    };
    assert_eq!(
        mappings
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        model["types"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| rename(t["name"].as_str().unwrap()))
            .collect()
    );
    let actual = inventory(&root().join("rust/src"));
    for name in mappings.as_object().unwrap().keys() {
        assert!(actual.get(name).is_some(), "not exported: {name}");
    }
    let ids = VALID.iter().map(|v| v.id).collect::<BTreeSet<_>>();
    for examples in capabilities.values() {
        assert!(!examples.as_array().unwrap().is_empty());
        for example in examples.as_array().unwrap() {
            assert!(ids.contains(example.as_str().unwrap()));
        }
    }
}
#[test]
fn audit_detects_a_new_handwritten_export_and_method() {
    // Mutate a tiny synthetic crate, never the checkout or the expected inventory.
    let base = std::env::var_os("TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join(format!("slackblocks-audit-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("lib.rs"), "mod helper; pub use helper::NewHelper;").unwrap();
    fs::write(
        dir.join("helper.rs"),
        "pub struct NewHelper; impl NewHelper { pub fn emit(&self) -> String { String::new() } }",
    )
    .unwrap();
    let actual = inventory(&dir);
    fs::remove_dir_all(&dir).unwrap();
    assert!(
        actual["NewHelper"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().unwrap().contains("emit"))
    );
    assert!(
        json("rust/conformance/api-inventory.json")
            .get("NewHelper")
            .is_none()
    );
}
fn scalar_leaves(
    value: &Value,
    prefix: &str,
    result: &mut std::collections::BTreeMap<String, i64>,
) {
    for (key, value) in value.as_object().unwrap() {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        if value.is_object() {
            scalar_leaves(value, &path, result);
        } else {
            assert!(result.insert(path, value.as_i64().unwrap()).is_none());
        }
    }
}
#[test]
fn scalar_tables_equal_every_shared_leaf_and_every_leaf_has_a_case() {
    let mut expected = std::collections::BTreeMap::new();
    scalar_leaves(&json("spec/limits.json"), "", &mut expected);
    let source = fs::read_to_string(root().join("rust/src/generated/constants.rs")).unwrap();
    let file = syn::parse_file(&source).unwrap();
    let constant = file
        .items
        .iter()
        .find_map(|i| match i {
            syn::Item::Const(c) if c.ident == "LIMITS" => Some(c),
            _ => None,
        })
        .unwrap();
    let syn::Expr::Reference(reference) = constant.expr.as_ref() else {
        panic!()
    };
    let syn::Expr::Array(array) = reference.expr.as_ref() else {
        panic!()
    };
    let mut actual = std::collections::BTreeMap::new();
    for expr in &array.elems {
        let syn::Expr::Tuple(tuple) = expr else {
            panic!()
        };
        let syn::Expr::Lit(key) = &tuple.elems[0] else {
            panic!()
        };
        let syn::Lit::Str(key) = &key.lit else {
            panic!()
        };
        let syn::Expr::Lit(value) = &tuple.elems[1] else {
            panic!()
        };
        let syn::Lit::Int(value) = &value.lit else {
            panic!()
        };
        assert!(
            actual
                .insert(key.value(), value.base10_parse::<i64>().unwrap())
                .is_none()
        );
    }
    assert_eq!(actual, expected);
    let manifest = json("spec/fixtures/invalid/manifest.json");
    let constraints = manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v["constraint"].as_str())
        .collect::<BTreeSet<_>>();
    for path in expected.keys() {
        assert!(
            constraints.contains(path.as_str()),
            "uncovered scalar: {path}"
        );
    }
}
#[test]
fn vocabulary_tables_equal_shared_contract() {
    let vocabulary = json("spec/vocabulary.json");
    let file = syn::parse_file(
        &fs::read_to_string(root().join("rust/src/generated/constants.rs")).unwrap(),
    )
    .unwrap();
    for (constant, expected) in [
        ("ICONS", &vocabulary["slack_icon_names"]),
        (
            "MESSAGE_BLOCKS",
            &vocabulary["surface_block_types"]["message"],
        ),
        ("MODAL_BLOCKS", &vocabulary["surface_block_types"]["modal"]),
        ("HOME_BLOCKS", &vocabulary["surface_block_types"]["home"]),
    ] {
        let value = file
            .items
            .iter()
            .find_map(|i| match i {
                syn::Item::Const(c) if c.ident == constant => Some(c),
                _ => None,
            })
            .unwrap();
        let syn::Expr::Reference(reference) = value.expr.as_ref() else {
            panic!()
        };
        let syn::Expr::Array(array) = reference.expr.as_ref() else {
            panic!()
        };
        let actual = array
            .elems
            .iter()
            .map(|e| {
                let syn::Expr::Lit(l) = e else { panic!() };
                let syn::Lit::Str(s) = &l.lit else { panic!() };
                s.value()
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(actual.len(), array.elems.len(), "duplicate {constant}");
        assert_eq!(
            actual,
            expected
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect(),
            "{constant}"
        );
    }
}
