// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! qbka Slice B: permanent regression guard for `docs/plans/*.md` corpus
//! validity. `67kq` (`20260903T215909Z-coder-tester-67kq`) found 12 of 191
//! plan files invalid via a one-time manual `agov plan validate` sweep; this
//! test makes that sweep a real, CI-enforced check so the corpus cannot
//! silently regress back to invalid.
use std::{
    path::PathBuf,
    process::Command,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap()
        .to_path_buf()
}

#[test]
fn every_plan_file_in_the_repo_validates() {
    let root = workspace_root();
    let plans_dir = root.join("docs/plans");
    let schema_dir = root.join("agf/schemas");

    let mut plan_files: Vec<PathBuf> = std::fs::read_dir(&plans_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();
    plan_files.sort();

    // Sanity floor: if this drops far below the corpus size at qbka Slice B
    // authoring time (223), something is wrong with path resolution rather
    // than the corpus actually shrinking that much -- fail loudly instead of
    // silently validating zero or a handful of files.
    assert!(
        plan_files.len() >= 100,
        "expected at least 100 plan files under {}; found {} -- check path resolution",
        plans_dir.display(),
        plan_files.len()
    );

    let mut failures = Vec::new();
    for path in &plan_files {
        let output = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(["plan", "validate"])
            .arg(path)
            .env("AGOV_SCHEMA_DIR", &schema_dir)
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(&root)
            .output()
            .unwrap();
        if !output.status.success() {
            failures.push(format!(
                "{}:\n{}{}",
                path.file_name().unwrap().to_string_lossy(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} plan files failed validation:\n\n{}",
        failures.len(),
        plan_files.len(),
        failures.join("\n---\n")
    );
}
