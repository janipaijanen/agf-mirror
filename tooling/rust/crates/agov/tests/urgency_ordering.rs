// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

fn write(dir: &Path, suffix: &str, hour: u8, kind: &str, urgent: bool) -> String {
    let id = format!("20260913T{hour:02}0000Z-planner-architect-{suffix}");
    let mut record = json!({"id":id,"kind":kind,"from":"planner-architect","to":"coder-tester",
        "timestamp":format!("2026-09-13T{hour:02}:00:00Z"),"immutable":true,"references-to":[],
        "requires-ack":true,"topic":suffix,"artifacts_owned":[]});
    if urgent {
        record["urgency"] = json!("urgent");
    }
    if kind == "handoff" {
        record["handoff"] = json!({"next_agent":"coder-tester","status":"ready",
        "task":[suffix],"must_read":[],"do_not_touch":[],"for_session":"other-session"});
    }
    fs::write(
        dir.join(format!("{id}.md")),
        format!("---\n{record}\n---\n\nFixture.\n"),
    )
    .unwrap();
    id
}
fn run(root: &Path, args: &[&str]) -> String {
    let schema = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap()
        .join("agf/schemas");
    let out = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root)
        .args(args)
        .env("AGOV_SCHEMA_DIR", schema)
        .env("AGOV_TOKEN_ROI", "0")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn git(root: &Path, args: &[&str]) {
    assert!(Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
        .status
        .success());
}
#[test]
fn urgency_changes_selection_but_not_ties_or_addressing() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path();
    let older = write(dir, "aaaa", 1, "handoff", false);
    write(dir, "bbbb", 2, "handoff", false);
    let newest = write(dir, "cccc", 2, "handoff", false);
    let args = [
        "handoff",
        "next",
        "coder-tester",
        "--dir",
        ".",
        "--session",
        "this-session",
    ];
    let baseline = run(dir, &args);
    assert!(baseline.contains(&newest));
    assert!(baseline.contains("Addressed elsewhere: other-session"));
    write(dir, "aaaa", 1, "handoff", true);
    let urgent = run(dir, &args);
    assert!(urgent.contains(&older));
    assert!(urgent.contains("Addressed elsewhere: other-session"));
    write(dir, "bbbb", 2, "handoff", true);
    write(dir, "cccc", 2, "handoff", true);
    assert!(run(dir, &args).contains(&newest));
}
#[test]
fn pending_and_resume_preserve_resolution_and_surface_urgent_obligations() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path();
    let old = write(dir, "aaaa", 1, "review", false);
    let urgent = write(dir, "bbbb", 2, "review", true);
    let args = ["coord", "pending", "--dir", ".", "--format", "json"];
    let rows: Vec<Value> = serde_json::from_str(&run(dir, &args)).unwrap();
    assert_eq!(rows[0]["id"], urgent);
    assert_eq!(rows[1]["id"], old);
    let resume = run(
        dir,
        &["session", "resume", "--role", "coder-tester", "--dir", "."],
    );
    assert!(resume.contains("Urgent pending obligations (advisory):"));
    assert!(resume.contains(&urgent));
    let reply = json!({"id":"20260913T030000Z-coder-tester-cccc","kind":"note","from":"coder-tester","to":"planner-architect",
        "timestamp":"2026-09-13T03:00:00Z","immutable":true,"references-to":[urgent],"requires-ack":false,"topic":"reply","artifacts_owned":[]});
    fs::write(
        dir.join("20260913T030000Z-coder-tester-cccc.md"),
        format!("---\n{reply}\n---\n\nReply.\n"),
    )
    .unwrap();
    let rows: Vec<Value> = serde_json::from_str(&run(dir, &args)).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["id"], old);
    assert!(!run(
        dir,
        &["session", "resume", "--role", "coder-tester", "--dir", "."]
    )
    .contains("Urgent pending obligations (advisory):"));
}
#[test]
fn branch_listing_keeps_all_candidates_and_deduplicates_identical_sources() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("docs/coordination");
    fs::create_dir_all(&dir).unwrap();
    git(root.path(), &["init", "-b", "main"]);
    git(
        root.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(root.path(), &["config", "user.name", "Test"]);
    let normal = write(&dir, "aaaa", 1, "handoff", false);
    let urgent = write(&dir, "bbbb", 2, "handoff", true);
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-m", "fixture"]);
    git(root.path(), &["branch", "copy"]);
    let out = run(
        root.path(),
        &["handoff", "next", "coder-tester", "--include-branch-local"],
    );
    assert!(out.find(&urgent).unwrap() < out.find(&normal).unwrap());
    assert_eq!(out.matches("Candidate:").count(), 2);
    assert!(out.contains("partial_coverage: false"));
}

fn update_record(dir: &Path, id: &str, edit: impl FnOnce(&mut Value)) {
    let path = dir.join(format!("{id}.md"));
    let text = fs::read_to_string(&path).unwrap();
    let mut v: Value = serde_json::from_str(
        text.strip_prefix("---\n")
            .unwrap()
            .split("\n---")
            .next()
            .unwrap(),
    )
    .unwrap();
    edit(&mut v);
    fs::write(path, format!("---\n{v}\n---\n\nFixture.\n")).unwrap();
}
#[test]
fn challenge_cli_keeps_ack_separate_and_requires_sources_before_closure() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path();
    let source = write(dir, "aaaa", 1, "review", false);
    let raise = write(dir, "bbbb", 2, "review", false);
    update_record(dir, &source, |v| v["session"] = json!("author"));
    update_record(dir, &raise, |v| {
        v["session"] = json!("challenger");
        v["references-to"] = json!([source]);
        v["challenge"] = json!({"challenge_id":"one","disputed_record_id":source,"window_hours":2});
    });
    let args = ["coord", "pending", "--dir", ".", "--format", "json"];
    let view: Value = serde_json::from_str(&run(dir, &args)).unwrap();
    assert_eq!(view["pending"].as_array().unwrap().len(), 1);
    assert_eq!(view["challenges"].as_array().unwrap().len(), 1);
    let response = write(dir, "cccc", 3, "review", false);
    update_record(dir, &response, |v| {
        v["session"] = json!("author");
        v["requires-ack"] = json!(false);
        v["references-to"] = json!([raise]);
        v["challenge"] = json!({"challenge_id":"one","resolution":"replied"});
    });
    let view: Value = serde_json::from_str(&run(dir, &args)).unwrap();
    assert!(view["pending"].as_array().unwrap().is_empty());
    assert_eq!(view["challenges"][0]["state"], "replied — decision pending");
    // The kind remains review; governor identity is a declaration, not authentication.
    update_record(dir, &response, |v| {
        v["from"] = json!("governor-reviewer");
        v["challenge"]["resolution"] = json!("escalated");
    });
    let out = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(dir)
        .args(args)
        .env(
            "AGOV_SCHEMA_DIR",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(4)
                .unwrap()
                .join("agf/schemas"),
        )
        .output()
        .unwrap();
    let view: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(view["challenges"].as_array().unwrap().is_empty());
    fs::remove_file(dir.join(format!("{source}.md"))).unwrap();
    let view: Value = serde_json::from_str(&run(dir, &args)).unwrap();
    assert_eq!(view["challenges"][0]["state"], "needs-review");
}
#[test]
fn schema_rejects_non_governor_terminal_and_contextual_collisions() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path();
    let id = write(dir, "aaaa", 1, "review", false);
    update_record(dir, &id, |v| {
        v["challenge"] = json!({"challenge_id":"one","resolution":"escalated"})
    });
    let schema = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap()
        .join("agf/schemas");
    let check = || {
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .current_dir(dir)
            .args(["coord", "validate"])
            .arg(dir.join(format!("{id}.md")))
            .env("AGOV_SCHEMA_DIR", &schema)
            .output()
            .unwrap()
    };
    assert_eq!(check().status.code(), Some(1));
    update_record(dir, &id, |v| {
        v["references-to"] = json!(["20260913T000000Z-coder-tester-src1"]);
        v["challenge"] = json!({"challenge_id":"one","disputed_record_id":"20260913T000000Z-coder-tester-src1","window_hours":721});
    });
    assert_eq!(check().status.code(), Some(1));
    update_record(dir, &id, |v| v["challenge"]["window_hours"] = json!(2));
    assert!(check().status.success());
    let other = write(dir, "bbbb", 2, "review", false);
    update_record(dir, &other, |v| {
        v["references-to"] = json!(["20260913T000000Z-coder-tester-src1"]);
        v["challenge"] = json!({"challenge_id":"one","disputed_record_id":"20260913T000000Z-coder-tester-src1","window_hours":2});
    });
    let out = check();
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("duplicate challenge_id")
            || String::from_utf8_lossy(&out.stdout).contains("duplicate challenge_id")
    );
}

#[test]
fn unrelated_invalid_legacy_record_keeps_challenge_free_json_array() {
    let root = tempfile::tempdir().unwrap();
    let id = write(root.path(), "aaaa", 1, "review", false);
    fs::write(root.path().join("broken.md"), "---\nlegacy: [\n---\n").unwrap();
    let view: Value = serde_json::from_str(&run(
        root.path(),
        &["coord", "pending", "--dir", ".", "--format", "json"],
    ))
    .unwrap();
    assert_eq!(view.as_array().unwrap().len(), 1);
    assert_eq!(view[0]["id"], id);
}

#[test]
fn invalid_challenge_record_still_has_visible_diagnostics() {
    let root = tempfile::tempdir().unwrap();
    let id = write(root.path(), "aaaa", 1, "review", false);
    update_record(root.path(), &id, |v| {
        v["challenge"] = json!({"challenge_id": "broken"})
    });
    let view: Value = serde_json::from_str(&run(
        root.path(),
        &["coord", "pending", "--dir", ".", "--format", "json"],
    ))
    .unwrap();
    assert!(!view["problems"].as_array().unwrap().is_empty());
    assert_eq!(view["challenges"][0]["state"], "needs-review");
}
