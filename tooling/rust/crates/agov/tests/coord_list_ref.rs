// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
use std::{fs, io::Write, path::PathBuf, process::Command, process::Stdio};

fn record(id: &str, timestamp: &str) -> String {
    format!("---\nid: {id}\nkind: note\nfrom: planner-architect\nto: all\ntimestamp: {timestamp}\nimmutable: true\nreferences-to: []\ntopic: fixture\nrequires-ack: false\nartifacts_owned: []\nmodel: test\n---\n\nBody.\n")
}

const A: &str = "20260911T000000Z-planner-architect-a001";
const B: &str = "20260911T000100Z-planner-architect-a002";

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        let r = Self(tempfile::tempdir().unwrap());
        r.git(&["init", "-b", "main"]);
        r.git(&["config", "user.email", "test@example.invalid"]);
        r.git(&["config", "user.name", "Test"]);
        r.git(&["config", "protocol.file.allow", "always"]);
        r.write(&format!("docs/coordination/{A}.md"), &record(A, "2026-09-11T00:00:00Z"));
        r.commit();
        r
    }
    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git").args(args).current_dir(self.0.path()).output().unwrap();
        assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap().trim().into()
    }
    fn write(&self, path: &str, content: &str) {
        let p = self.0.path().join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }
    fn commit(&self) -> String {
        self.git(&["add", "."]);
        self.git(&["commit", "-m", "fixture"]);
        self.git(&["rev-parse", "HEAD"])
    }
    fn list(&self, extra: &[&str]) -> std::process::Output {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4).unwrap().to_path_buf();
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(["coord", "list"])
            .args(extra)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path())
            .output()
            .unwrap()
    }
}

fn text(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}
fn err(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Regression for the false-empty delta: the checkout lacks a record the fetched ref has.
#[test]
fn ref_mode_finds_the_record_the_checkout_lacks() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "behind"]);
    r.git(&["checkout", "main"]);
    r.write(&format!("docs/coordination/{B}.md"), &record(B, "2026-09-11T00:01:00Z"));
    let main_oid = r.commit();
    r.git(&["checkout", "behind"]);

    // Filesystem mode on the stale checkout: the documented false empty.
    let fs_out = r.list(&["--since", "2026-09-11T00:00:30Z", "--format", "json"]);
    assert_eq!(fs_out.status.code(), Some(2));
    assert_eq!(text(&fs_out).trim(), "[]");

    let out = r.list(&["--ref", "main", "--since", "2026-09-11T00:00:30Z", "--format", "json"]);
    assert_eq!(out.status.code(), Some(0), "{}", err(&out));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["source_ref"], "main");
    assert_eq!(value["source_oid"], main_oid);
    assert_eq!(value["coordination_dir"], "docs/coordination");
    assert_eq!(value["messages"].as_array().unwrap().len(), 1);
    assert_eq!(value["messages"][0]["id"], B);

    let out = r.list(&["--ref", "main", "--since", A]);
    let lines = text(&out);
    assert!(lines.starts_with(&format!("source: ref=main oid={main_oid} dir=docs/coordination records=2 skipped=0\n")), "{lines}");
    assert!(lines.contains(B) && !lines.contains(&format!("fixture  {A}")), "{lines}");
}

#[test]
fn ref_mode_rows_equal_filesystem_rows_when_the_checkout_is_the_ref() {
    let r = Repo::new();
    r.write(&format!("docs/coordination/{B}.md"), &record(B, "2026-09-11T00:01:00Z"));
    r.commit();
    let fs_out: serde_json::Value = serde_json::from_slice(&r.list(&["--format", "json"]).stdout).unwrap();
    let ref_out: serde_json::Value = serde_json::from_slice(&r.list(&["--ref", "HEAD", "--format", "json"]).stdout).unwrap();
    assert_eq!(fs_out, ref_out["messages"]);
}

#[test]
fn empty_json_without_ref_is_an_array_and_exit_stays_two() {
    let r = Repo::new();
    let out = r.list(&["--since", "2099-01-01T00:00:00Z", "--format", "json"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(text(&out).trim(), "[]");
    let out = r.list(&["--since", "2099-01-01T00:00:00Z"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(text(&out).trim(), "No coordination messages found");
}

#[test]
fn empty_json_with_ref_keeps_provenance_object() {
    let r = Repo::new();
    let out = r.list(&["--ref", "HEAD", "--since", "2099-01-01T00:00:00Z", "--format", "json"]);
    assert_eq!(out.status.code(), Some(2));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["messages"], serde_json::json!([]));
    assert_eq!(value["source_ref"], "HEAD");
}

#[test]
fn unavailable_ref_or_directory_is_exit_two_with_an_error_not_a_clean_empty() {
    let r = Repo::new();
    for extra in [vec!["--ref", "no-such-ref"], vec!["--ref", "HEAD", "--dir", "docs/nope"]] {
        let out = r.list(&extra);
        assert_eq!(out.status.code(), Some(2), "{extra:?}");
        assert!(err(&out).starts_with("ERR  could not"), "{}", err(&out));
        assert!(text(&out).is_empty());
    }
}

#[test]
fn leading_dash_ref_is_a_value_not_a_flag() {
    let r = Repo::new();
    let out = r.list(&["--ref=-x"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(err(&out).contains("could not resolve --ref -x"), "{}", err(&out));
    // A ref that is a real option name must not be interpreted by git.
    let out = r.list(&["--ref=--all"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(err(&out).contains("could not resolve --ref --all"), "{}", err(&out));
}

#[test]
fn echoed_ref_is_escaped_and_length_capped() {
    let r = Repo::new();
    let out = r.list(&["--ref=a\u{1b}[31m\nb"]);
    let message = err(&out);
    assert_eq!(out.status.code(), Some(2));
    assert!(message.contains("a\\x1b[31m\\x0ab"), "{message:?}");
    assert!(!message.trim_end().chars().any(char::is_control), "{message:?}");
    let long = format!("--ref={}", "k".repeat(5000));
    let message = err(&r.list(&[&long]));
    assert!(message.len() < 600 && message.contains("...[truncated]"), "{}", message.len());
}

#[test]
fn unsafe_dir_forms_are_rejected() {
    let r = Repo::new();
    for dir in ["/etc", "../x", "docs/../x", "a\\b", "-d", "docs//coordination", "."] {
        let out = r.list(&["--ref", "HEAD", "--dir", dir]);
        assert_eq!(out.status.code(), Some(2), "{dir}");
        assert!(err(&out).contains("--dir must be a repository-relative path") || err(&out).contains("unexpected argument"), "{dir}: {}", err(&out));
    }
}

#[test]
fn symlinks_and_submodule_entries_are_skipped_and_counted() {
    let r = Repo::new();
    std::os::unix::fs::symlink(format!("{A}.md"), r.0.path().join("docs/coordination/link.md")).unwrap();
    let head = r.commit();
    r.git(&["update-index", "--add", "--cacheinfo", &format!("160000,{head},docs/coordination/sub.md")]);
    r.git(&["commit", "-m", "gitlink"]);
    let out = r.list(&["--ref", "HEAD"]);
    assert_eq!(out.status.code(), Some(0), "{}", err(&out));
    let first = text(&out).lines().next().unwrap().to_string();
    assert!(first.ends_with("records=1 skipped=2"), "{first}");
}

#[test]
fn oversized_blob_fails_closed() {
    let r = Repo::new();
    r.write("docs/coordination/big.md", &"x".repeat(1024 * 1024 + 1));
    r.commit();
    let out = r.list(&["--ref", "HEAD"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(err(&out).contains("is larger than"), "{}", err(&out));
}

#[test]
fn too_many_entries_fails_closed() {
    let r = Repo::new();
    let mut stream = String::from("commit refs/heads/many\ncommitter T <t@example.invalid> 1700000000 +0000\ndata 1\nm\n");
    for i in 0..50_001 {
        stream.push_str(&format!("M 100644 inline docs/coordination/e{i}.md\ndata 1\nx\n"));
    }
    let mut child = Command::new("git")
        .args(["fast-import", "--quiet"])
        .current_dir(r.0.path())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(stream.as_bytes()).unwrap();
    assert!(child.wait().unwrap().success());
    let out = r.list(&["--ref", "many"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(err(&out).contains("more than 50000 entries"), "{}", err(&out));
}

#[test]
fn invalid_records_are_ignored_like_filesystem_mode_and_ref_does_not_touch_the_network() {
    let r = Repo::new();
    r.write("docs/coordination/bad.md", "not a record");
    r.commit();
    // A bogus remote URL proves no fetch is attempted.
    r.git(&["remote", "add", "origin", "https://invalid.invalid/x.git"]);
    let out = r.list(&["--ref", "HEAD", "--format", "json"]);
    assert_eq!(out.status.code(), Some(0), "{}", err(&out));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["messages"].as_array().unwrap().len(), 1);
}
