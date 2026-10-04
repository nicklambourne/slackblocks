use serde_json::Value;
use slackblocks_conformance::{invalid::INVALID, valid::VALID};
use std::{collections::BTreeSet, fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(root().join(path)).unwrap()).unwrap()
}
fn unique<'a>(ids: impl Iterator<Item = &'a str>) -> BTreeSet<&'a str> {
    let mut set = BTreeSet::new();
    for id in ids {
        assert!(set.insert(id), "duplicate ID: {id}");
    }
    set
}
#[test]
fn valid_builders_and_checked_ingress_match_every_fixture() {
    let manifest = read("spec/manifest.json");
    let expected = unique(
        manifest["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap()),
    );
    assert_eq!(unique(VALID.iter().map(|v| v.id)), expected);
    assert_eq!(manifest["spec_version"], slackblocks::SPEC_VERSION);
    for case in VALID {
        let expected = read(&format!("spec/fixtures/valid/{}.json", case.id));
        assert_eq!(
            (case.build)().unwrap_or_else(|e| panic!("{} native: {e}", case.id)),
            expected,
            "{} native",
            case.id
        );
        assert_eq!(
            (case.parse)(expected.clone()).unwrap_or_else(|e| panic!("{} ingress: {e}", case.id)),
            expected,
            "{} ingress",
            case.id
        );
    }
}
#[test]
fn all_invalid_cases_have_normative_categories_and_paths() {
    let manifest = read("spec/fixtures/invalid/manifest.json");
    let expected = unique(
        manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap()),
    );
    assert_eq!(unique(INVALID.iter().map(|v| v.id)), expected);
    let mut failures = Vec::new();
    for case in INVALID {
        let category = manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == case.id)
            .unwrap()["category"]
            .as_str()
            .unwrap();
        for (route, attempt) in [("native", case.build), ("ingress", case.parse)] {
            match attempt() {
                Ok(()) => failures.push(format!("{} {route}: accepted", case.id)),
                Err(e) => {
                    if e.category().as_str() != category
                        || e.path().is_empty()
                        || e.message().is_empty()
                    {
                        failures.push(format!(
                            "{} {route}: expected {category}, got {e:?}",
                            case.id
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
#[test]
fn no_skipped_cases_and_fixture_disk_matches_registry() {
    assert!(
        fs::read_to_string(root().join("rust/conformance/skiplist.txt"))
            .unwrap()
            .trim()
            .is_empty()
    );
    fn walk(dir: &std::path::Path, base: &std::path::Path, files: &mut BTreeSet<String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, base, files);
            } else if path.extension().is_some_and(|v| v == "json") {
                files.insert(
                    path.strip_prefix(base)
                        .unwrap()
                        .with_extension("")
                        .to_str()
                        .unwrap()
                        .replace('\\', "/"),
                );
            }
        }
    }
    let base = root().join("spec/fixtures/valid");
    let mut actual = BTreeSet::new();
    walk(&base, &base, &mut actual);
    assert_eq!(actual, VALID.iter().map(|v| v.id.to_owned()).collect());
}
