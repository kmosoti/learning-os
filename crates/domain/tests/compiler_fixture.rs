//! Independent executable oracle: compiler diagnostics, not the rubric implementation.
use std::{path::PathBuf, process::Command};
#[test]
fn pinned_rust_programs_have_expected_semantics() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/rust");
    let dir = std::env::temp_dir().join(format!("learningos-rust-oracle-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for (file, expected) in [
        ("move.rs", Some("E0382")),
        ("overlap.rs", Some("E0502")),
        ("non_mutable.rs", Some("E0596")),
        ("mutable_overlap.rs", Some("E0499")),
        ("shared.rs", None),
        ("last_use.rs", None),
    ] {
        let output = Command::new("rustc")
            .arg("--edition=2024")
            .arg("--error-format=json")
            .arg("--emit=metadata")
            .arg("--out-dir")
            .arg(&dir)
            .arg(root.join(file))
            .output()
            .unwrap();
        assert_eq!(output.status.success(), expected.is_none(), "{file}");
        let diagnostics = String::from_utf8(output.stderr).unwrap();
        if let Some(code) = expected {
            let codes: Vec<String> = diagnostics
                .lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .filter_map(|v| v.get("code")?.get("code")?.as_str().map(str::to_owned))
                .collect();
            assert!(
                codes.iter().any(|actual| actual == code),
                "{file}: {codes:?}"
            );
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
