//! Tombstone guard: this repository is DEPRECATED in favor of
//! `fiducia-memory.rs`. This test enforces that the README keeps saying so.
//! If someone starts un-deprecating this repo (removing the marker or the
//! pointer to the successor), CI fails here and forces the change to be
//! deliberate rather than accidental.

#[test]
fn readme_keeps_the_deprecation_marker_and_successor_pointer() {
    let readme_path = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");
    let readme = std::fs::read_to_string(readme_path)
        .unwrap_or_else(|error| panic!("read {readme_path}: {error}"));

    assert!(
        readme.contains("DEPRECATED"),
        "README.md must keep its deprecation marker; this repo is a tombstone"
    );
    assert!(
        readme.contains("fiducia-memory.rs"),
        "README.md must keep pointing readers at the successor repo fiducia-memory.rs"
    );
}
