//! Print an inventory for human review; this never updates the checked-in audit.
fn main() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    println!(
        "{}",
        serde_json::to_string_pretty(&slackblocks_conformance::audit::inventory(&source)).unwrap()
    );
}
