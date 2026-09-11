// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
use std::{fs, path::PathBuf, process::Command};

struct Repo(tempfile::TempDir);
impl Repo {
    fn new() -> Self {
        let r = Self(tempfile::tempdir().unwrap());
        r.git(&["init", "-b", "main"]);
        r.git(&["config", "user.email", "test@example.invalid"]);
        r.git(&["config", "user.name", "Test"]);
        r.write("docs/plans/.keep", "");
        r.write("old.txt", "rename me\n");
        r.commit();
        r
    }
    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(self.0.path())
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
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
    fn run(&self, args: &[&str]) -> (i32, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .unwrap()
            .to_path_buf();
        let out = Command::new(env!("CARGO_BIN_EXE_agov"))
            .args(args)
            .env("AGOV_SCHEMA_DIR", root.join("agf/schemas"))
            .env("AGOV_TOKEN_ROI", "0")
            .current_dir(self.0.path())
            .output()
            .unwrap();
        (
            out.status.code().unwrap(),
            format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )
    }
    fn check(&self) -> (i32, String) {
        self.run(&["plan", "collision-check"])
    }
    fn plan(&self, name: &str, field: &str) {
        self.write(&format!("docs/plans/{name}.md"), &format!("---\nid: {name}\nstatus: approved\n{field}\nadvisory_consultation:\n  - role: infosec-sentinel\n    status: consulted\n  - role: contribution-auditor\n    status: consulted\n---\nPlan.\n"));
    }
}

#[test]
fn optional_field_preserves_validation_and_names_unassessed_coverage() {
    let r = Repo::new();
    r.plan("old", "");
    assert_eq!(r.run(&["plan", "validate", "docs/plans/old.md"]).0, 0);
    let (code, text) = r.check();
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("INCOMPLETE") && text.contains("no affected_paths declared"));
}

#[test]
fn literal_and_future_overlaps_need_no_existing_files() {
    let r = Repo::new();
    for (left, right) in [
        ("future/file.rs", "future/file.rs"),
        ("future/**", "future/not-yet.rs"),
    ] {
        r.plan("a", &format!("affected_paths: ['{left}']"));
        r.plan("b", &format!("affected_paths: ['{right}']"));
        let (code, text) = r.check();
        assert_eq!(code, 1, "{text}");
        assert!(text.contains("plan docs/plans/a.md <-> plan docs/plans/b.md"));
        assert!(text.contains(left) && text.contains(right));
    }
}

#[test]
fn clean_and_incomplete_are_distinct_even_with_overlaps() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['a.rs']");
    r.plan("b", "affected_paths: ['b.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("Coverage: COMPLETE"));
    r.plan("old", "");
    assert_eq!(r.check().0, 2);
    r.plan("b", "affected_paths: ['a.rs']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("Coverage: INCOMPLETE"));
}

#[test]
fn local_and_remote_refs_include_both_rename_paths_and_base_scope() {
    let r = Repo::new();
    r.git(&["checkout", "-b", "rename"]);
    r.git(&["mv", "old.txt", "new.txt"]);
    let head = r.commit();
    r.git(&["update-ref", "refs/remotes/other/rename", &head]);
    r.git(&["checkout", "main"]);
    r.plan("old", "affected_paths: ['old.txt']");
    r.plan("new", "affected_paths: ['new.txt']");
    let (code, text) = r.check();
    assert_eq!(code, 1, "{text}");
    assert!(
        text.contains("ref refs/heads/rename") && text.contains("ref refs/remotes/other/rename")
    );
    assert!(text.contains("old.txt <-> old.txt") && text.contains("new.txt <-> new.txt"));
    // Changing the base changes diffs, never the working-tree plan inventory.
    let (code, text) = r.run(&["plan", "collision-check", "--base-ref", "rename"]);
    assert_eq!(code, 0, "{text}");
    assert!(text.contains("plans inspected: 2"));
}

#[test]
fn malformed_shape_and_embedded_double_star_are_excluded_with_errors() {
    let r = Repo::new();
    for field in [
        "affected_paths: [42]",
        "affected_paths: ['']",
        "affected_paths: ['/absolute']",
        "affected_paths: ['a/../b']",
        "affected_paths: ['src/a**/x.rs']",
        "affected_paths: ['pre**post/file.rs']",
    ] {
        r.plan("bad", field);
        let (code, text) = r.run(&["plan", "validate", "docs/plans/bad.md"]);
        assert_ne!(code, 0, "{field}: {text}");
        if field.contains("**") {
            assert!(
                text.contains("** is allowed only as a whole path segment"),
                "{text}"
            );
        }
        let (code, text) = r.check();
        assert_eq!(code, 2, "{text}");
        assert!(text.contains("Invalid (excluded): plan docs/plans/bad.md"));
    }
    for pattern in ["**", "a/**", "**/a", "a/**/b", "a/*.rs", "a/?.rs"] {
        r.plan("bad", &format!("affected_paths: ['{pattern}']"));
        assert_eq!(
            r.run(&["plan", "validate", "docs/plans/bad.md"]).0,
            0,
            "{pattern}"
        );
    }
}

#[test]
fn unreadable_ref_is_named_without_losing_known_overlap() {
    let r = Repo::new();
    r.plan("a", "affected_paths: ['a']");
    r.write(
        ".git/refs/heads/broken",
        "1111111111111111111111111111111111111111\n",
    );
    let (code, text) = r.check();
    assert_eq!(code, 2, "{text}");
    assert!(text.contains("refs/heads/broken"));
    r.plan("b", "affected_paths: ['a']");
    assert_eq!(r.check().0, 1);
}
