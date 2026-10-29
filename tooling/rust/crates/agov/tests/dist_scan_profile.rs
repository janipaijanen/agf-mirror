// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen

// agov is a binary crate; exercise the same validation implementation directly.
#[allow(dead_code)]
#[path = "../src/dist.rs"]
mod dist;
#[allow(dead_code)]
#[path = "../src/provenance.rs"]
mod provenance;

use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct Parameters {
    schema_version: u8,
    depth: usize,
    buckets: usize,
    regular_files: usize,
    untracked: Vec<String>,
    hidden: Vec<String>,
    symlink: Link,
    excluded: Vec<String>,
    expected: Expected,
}
#[derive(Deserialize)]
struct Link {
    path: String,
    target: String,
}
#[derive(Deserialize)]
struct Expected {
    tree_regular_files: usize,
    symlinks: usize,
    git_untracked_paths: usize,
    selected_files: usize,
}

fn write(root: &Path, path: &str, contents: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, contents).unwrap();
}

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_DATE", "2026-09-20T00:00:00Z")
        .env("GIT_COMMITTER_DATE", "2026-09-20T00:00:00Z")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn fixture() -> (tempfile::TempDir, Parameters, BTreeSet<String>) {
    let p: Parameters =
        serde_norway::from_str(include_str!("fixtures/dist_scan_profile.yaml")).unwrap();
    assert_eq!(p.schema_version, 1);
    assert!(p.depth >= 4);
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    git(root, &["init", "--template=", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "Distribution fixture"]);
    let prefix = (0..p.depth)
        .map(|i| format!("d{i}"))
        .collect::<Vec<_>>()
        .join("/");
    let mut selected = BTreeSet::new();
    for i in 0..p.regular_files {
        let path = format!("tree/{prefix}/bucket{:02}/file{i:03}.txt", i % p.buckets);
        write(root, &path, "fixed fixture content\n");
        selected.insert(path);
    }
    write(root, "VERSION", "1.0.0\n");
    write(root, "notes/release.md", "fixture release\n");
    selected.extend(["VERSION".into(), "notes/release.md".into()]);
    git(root, &["add", "."]);
    git(
        root,
        &[
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "tracked baseline",
        ],
    );
    assert_eq!(
        git(root, &["ls-files"]).lines().count(),
        p.regular_files + 2
    );

    for path in p.untracked.iter().chain(&p.hidden) {
        write(root, path, "genuinely untracked content\n");
        selected.insert(path.clone());
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(&p.symlink.target, root.join(&p.symlink.path)).unwrap();
    #[cfg(not(unix))]
    compile_error!("the supported distribution profiling fixture requires Unix symlinks");
    selected.insert(p.symlink.path.clone());
    assert!(root
        .join(&p.symlink.path)
        .canonicalize()
        .unwrap()
        .starts_with(root));
    assert_eq!(
        fs::read(root.join(&p.symlink.path)).unwrap(),
        b"fixed fixture content\n"
    );
    write(root, "manifest.yaml", "schema_version: 1\npackage:\n  name: fixture\n  version_file: VERSION\n  output_dir: dist\n  basename: fixture-{{version}}\n  formats: [tar.gz]\ninclude:\n  - path: VERSION\n  - glob: tree/**\n  - glob: notes/*.md\nexclude:\n  - '**/__pycache__/**'\n  - '**/*.pyc'\nforbidden:\n  - '**/*.secret'\n");

    let mut actual_untracked: BTreeSet<String> =
        git(root, &["ls-files", "--others", "--exclude-standard"])
            .lines()
            .map(str::to_owned)
            .collect();
    assert_eq!(actual_untracked.len(), p.expected.git_untracked_paths);
    assert!(actual_untracked.remove("manifest.yaml"));
    let expected_untracked: BTreeSet<_> = p
        .untracked
        .iter()
        .chain(&p.hidden)
        .cloned()
        .chain([p.symlink.path.clone()])
        .collect();
    assert_eq!(actual_untracked, expected_untracked);
    assert_eq!(
        p.untracked
            .iter()
            .filter(|s| s.matches('/').count() >= 3)
            .count(),
        2
    );
    assert_eq!(
        p.hidden
            .iter()
            .filter(|s| s.split('/').any(|c| c.starts_with('.')))
            .count(),
        p.hidden.len()
    );
    let (regular, links) = count_tree(&root.join("tree"));
    assert_eq!(regular, p.expected.tree_regular_files);
    assert_eq!(links, p.expected.symlinks);
    for path in &p.excluded {
        assert!(selected.remove(path), "excluded fixture must exist: {path}");
    }
    assert_eq!(selected.len(), p.expected.selected_files);
    (temp, p, selected)
}

fn count_tree(root: &Path) -> (usize, usize) {
    let (mut regular, mut links) = (0, 0);
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let kind = entry.file_type().unwrap();
        if kind.is_symlink() {
            links += 1;
        } else if kind.is_dir() {
            let (r, l) = count_tree(&entry.path());
            regular += r;
            links += l;
        } else if kind.is_file() {
            regular += 1;
        }
    }
    (regular, links)
}

#[test]
fn filesystem_membership_includes_true_untracked_hidden_and_in_root_symlink() {
    let (temp, p, selected) = fixture();
    let spec = dist::validate(Path::new("manifest.yaml"), temp.path()).unwrap();
    for path in selected {
        assert!(
            spec.contains(&path),
            "missing selected fixture path: {path}"
        );
    }
    for path in p.excluded {
        assert!(
            !spec.contains(&path),
            "excluded fixture path selected: {path}"
        );
    }
    assert!(dist::format_validate_summary(&spec)
        .contains(&format!("Files:    {}", p.expected.selected_files)));
}

#[cfg(not(feature = "dist-profile"))]
#[test]
fn default_command_surface_has_no_diagnose() {
    let out = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .args(["dist", "--help"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!String::from_utf8_lossy(&out.stdout).contains("diagnose"));
    let out = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .args(["dist", "diagnose"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[cfg(feature = "dist-profile")]
fn counts(report: &dist::DiagnoseReport) -> (u64, u64, u64) {
    (
        report.profile.read_dir_calls,
        report.profile.expand_glob_calls,
        report.profile.glob_matches_calls,
    )
}

#[cfg(feature = "dist-profile")]
#[test]
fn counters_are_per_invocation_and_keep_partial_failure_evidence() {
    let (temp, p, _) = fixture();
    let manifest = Path::new("manifest.yaml");
    let first = dist::diagnose(manifest, temp.path());
    assert!(first.errors.is_empty());
    assert_eq!(first.selected_files, Some(p.expected.selected_files));
    assert!((p.depth as u64..=20).contains(&first.profile.read_dir_calls));
    assert_eq!(first.profile.expand_glob_calls, 1);
    assert!(
        (2 * p.expected.selected_files as u64..=4 * p.expected.selected_files as u64)
            .contains(&first.profile.glob_matches_calls)
    );
    assert!(first.profile.expand_includes_elapsed_ns > 0);
    assert!(first.profile.filter_elapsed_ns > 0);
    assert!(
        first.total_elapsed_ns
            >= first.profile.expand_includes_elapsed_ns + first.profile.filter_elapsed_ns
    );
    assert_eq!(
        counts(&dist::diagnose(manifest, temp.path())),
        counts(&first)
    );
    std::thread::scope(|scope| {
        let parallel = scope.spawn(|| dist::diagnose(manifest, temp.path()));
        let other = dist::diagnose(manifest, temp.path());
        assert_eq!(counts(&parallel.join().unwrap()), counts(&first));
        assert_eq!(counts(&other), counts(&first));
    });

    let missing = dist::diagnose(Path::new("missing.yaml"), temp.path());
    assert_eq!(counts(&missing), (0, 0, 0));
    assert!(missing.selected_files.is_none());
    assert_eq!(
        missing.errors,
        dist::validate(Path::new("missing.yaml"), temp.path()).unwrap_err()
    );
    let content = fs::read_to_string(temp.path().join(manifest)).unwrap();
    write(
        temp.path(),
        "manifest.yaml",
        &content.replace("'**/*.secret'", "'**/untracked.txt'"),
    );
    let rejected = dist::diagnose(manifest, temp.path());
    assert_eq!(
        rejected.errors,
        dist::validate(manifest, temp.path()).unwrap_err()
    );
    assert_eq!(rejected.errors.len(), 2);
    assert!(rejected
        .errors
        .iter()
        .all(|e| e.contains("forbidden file selected")));
    assert_eq!(
        rejected.profile.read_dir_calls,
        first.profile.read_dir_calls
    );
    assert!(rejected.profile.glob_matches_calls > 0);
    assert!(rejected.profile.filter_elapsed_ns > 0);
}

#[cfg(feature = "dist-profile")]
fn diagnostic(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root)
        .env("AGOV_TOKEN_ROI", "0")
        .args(["dist", "diagnose"])
        .args(args)
        .output()
        .unwrap()
}

#[cfg(feature = "dist-profile")]
#[test]
fn diagnostic_cli_exposes_feature_only_json_text_and_failure() {
    let (temp, p, _) = fixture();
    let out = diagnostic(temp.path(), &["--manifest", "manifest.yaml", "--json"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["selected_files"], p.expected.selected_files);
    assert_eq!(json["expand_glob_calls"], 1);
    assert!(json["read_dir_calls"].as_u64().unwrap() > 0);
    assert!(json["glob_matches_calls"].as_u64().unwrap() > 0);
    assert_eq!(json["errors"], serde_json::json!([]));
    let text = diagnostic(temp.path(), &["--manifest", "manifest.yaml"]);
    assert!(text.status.success());
    assert!(String::from_utf8_lossy(&text.stdout).contains("one validation"));
    let failed = diagnostic(temp.path(), &["--manifest", "missing.yaml", "--json"]);
    assert_eq!(failed.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert!(json["selected_files"].is_null());
    assert_eq!(json["read_dir_calls"], 0);
    assert_eq!(json["errors"].as_array().unwrap().len(), 1);
    let invalid = diagnostic(temp.path(), &["--unknown"]);
    assert_eq!(invalid.status.code(), Some(2));
}

/// Manual baseline only: generated fixture is removed at test completion.
/// Each sample is a new process; no filesystem-cache eviction is attempted.
#[cfg(feature = "dist-profile")]
#[test]
#[ignore = "manual profiling baseline, not a performance gate"]
fn record_fixture_baseline() {
    let (temp, _, _) = fixture();
    for sample in 1..=3 {
        let out = diagnostic(temp.path(), &["--manifest", "manifest.yaml", "--json"]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        println!(
            "fixture sample {sample}:\n{}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}
