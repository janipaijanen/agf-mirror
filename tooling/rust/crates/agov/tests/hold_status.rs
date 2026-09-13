use serde_json::Value;
use std::{fs, path::Path, process::Command};
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap().trim().into()
}
fn fixture() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    let root = d.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "Test"]);
    fs::create_dir_all(root.join("docs/coordination")).unwrap();
    fs::write(root.join("docs/coordination/.keep"), "").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "base"]);
    git(root, &["checkout", "-b", "candidate"]);
    fs::write(root.join("docs/coordination/note.md"), "---\nid: note\nkind: note\nfrom: coder-tester\nto: all\ntimestamp: 2026-09-12T11:00:00Z\nimmutable: true\nreferences-to: []\ntopic: fixture\nrequires-ack: false\n---\n").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "candidate"]);
    d
}
fn run(root: &Path, target: &str, candidate: &str) -> (i32, Value) {
    let before = time::OffsetDateTime::now_utc();
    let o = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root)
        .args([
            "coord",
            "hold-status",
            "--target-branch",
            target,
            "--candidate-ref",
            candidate,
            "--format",
            "json",
        ])
        .env("AGOV_TOKEN_ROI", "0")
        .output()
        .unwrap();
    let after = time::OffsetDateTime::now_utc();
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    let now = time::OffsetDateTime::parse(
        v["now"].as_str().unwrap(),
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap();
    assert!(now >= before && now <= after && now.offset() == time::UtcOffset::UTC);
    let schema = agov_core::schema::compile_str(include_str!(
        "../../../../../agf/schemas/coordination-hold-status.schema.json"
    ))
    .unwrap();
    assert!(agov_core::schema::validate(&schema, &v).is_empty());
    assert_eq!(v["publication_verified"], false);
    assert_eq!(v["authority_verified"], false);
    (o.status.code().unwrap(), v)
}
#[test]
fn pinned_git_reads_ignore_dirty_bytes_and_work_from_subdirectories() {
    let d = fixture();
    let root = d.path();
    fs::write(root.join("docs/coordination/note.md"), "broken draft").unwrap();
    let (status, mut a) = run(root, "main", "HEAD");
    assert_eq!(status, 0);
    assert_eq!(a["records"][0]["state"], "unassociated");
    let (status, mut b) = run(&root.join("docs"), "main", "HEAD");
    assert_eq!(status, 0);
    a.as_object_mut().unwrap().remove("now");
    b.as_object_mut().unwrap().remove("now");
    assert_eq!(a, b);
    assert_eq!(
        run(root, "candidate", "HEAD").1["records"],
        serde_json::json!([])
    );
}
#[test]
fn incomplete_and_semantic_errors_use_the_same_envelope_and_status_precedence() {
    let d = fixture();
    let root = d.path();
    assert_eq!(run(root, "HEAD", "HEAD").0, 2);
    assert_eq!(run(root, "main", "missing").0, 2);
    fs::write(root.join("docs/coordination/bad.md"), "broken").unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "malformed"]);
    let (status, v) = run(root, "main", "HEAD");
    assert_eq!(status, 1);
    assert!(v["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["state"] == "publish-now"));
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root)
        .args(["coord", "hold-status", "--target-branch", "main"])
        .output()
        .unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    for check in [
        "publication",
        "governor_authority",
        "record_urgency_and_obligations",
        "publication_target",
        "current_target_and_expiry",
        "release_hold_authorization",
    ] {
        assert!(text.contains(check));
    }
    assert!(text.contains("Review-required never permits delay"));
}
