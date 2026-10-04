//! Print public documentation from Rust syntax for the site generator.
fn main() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    println!(
        "{}",
        serde_json::to_string(&slackblocks_conformance::audit::documentation(&source)).unwrap()
    );
}
