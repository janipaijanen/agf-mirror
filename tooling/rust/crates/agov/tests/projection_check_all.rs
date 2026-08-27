use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Fixture {
    root: tempfile::TempDir,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("temp repo");
        git(root.path(), &["init"]);
        git(
            root.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(root.path(), &["config", "user.name", "Test User"]);
        write(root.path(), "AGENTS.md", "fixture\n");
        write(root.path(), "agf/VERSION", "2.6.1\n");
        write(
            root.path(),
            "agf/dist/release-manifest.yaml",
            r#"
schema_version: 1
package:
  name: agf
  version_file: agf/VERSION
  output_dir: dist
  basename: agf-{{version}}
  formats: [tar.gz]
include:
  - path: AGENTS.md
  - glob: agf/**
"#,
        );
        let source_schema = repo_root().join("agf/schemas/rule-projection-manifest.schema.json");
        let target_schema = root
            .path()
            .join("agf/schemas/rule-projection-manifest.schema.json");
        std::fs::create_dir_all(target_schema.parent().unwrap()).unwrap();
        std::fs::copy(source_schema, target_schema).unwrap();
        Self { root }
    }

    fn path(&self) -> &Path {
        self.root.path()
    }

    fn write_manifest(&self, name: &str, owner: &str, changed: &str) {
        let content = manifest(owner, changed);
        write(
            self.path(),
            &format!("docs/reviews/rule-projection/{name}.yaml"),
            &content,
        );
    }

    fn commit(&self, message: &str) {
        git(self.path(), &["add", "."]);
        git(self.path(), &["commit", "-m", message]);
    }

    fn check_all(&self, base_ref: &str) -> Output {
        let outside = tempfile::tempdir().unwrap();
        self.check_all_from(base_ref, outside.path(), outside.path())
    }

    fn check_all_from(&self, base_ref: &str, cwd: &Path, schema_dir: &Path) -> Output {
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", schema_dir)
            .current_dir(cwd)
            .args([
                "plan",
                "projection-check-all",
                "--repo-root",
                self.path().to_str().unwrap(),
                "--base-ref",
                base_ref,
            ])
            .output()
            .expect("run projection-check-all")
    }

    fn check_one(&self, allow_planned: bool) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_agov"));
        command
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", self.path().join("agf/schemas"))
            .current_dir(self.path())
            .args(["plan", "projection-check", "--base-ref", "HEAD"]);
        if allow_planned {
            command.arg("--allow-planned");
        }
        command
            .arg("docs/reviews/rule-projection/candidate.yaml")
            .output()
            .expect("run projection-check")
    }
}

#[test]
fn gate1_relaxation_and_gate2_strict_command_diverge_for_absent_owner() {
    let fixture = Fixture::new();
    fixture.write_manifest("candidate", "future/owner.md", "AGENTS.md");
    fixture.commit("baseline planned owner");

    let relaxed = fixture.check_one(true);
    assert_eq!(relaxed.status.code(), Some(0), "{}", stderr(&relaxed));

    let strict = fixture.check_one(false);
    assert_eq!(strict.status.code(), Some(1), "{}", stderr(&strict));
    assert!(stderr(&strict).contains("does not resolve to a real file"));
}

#[test]
fn full_tree_check_catches_stale_unchanged_manifest_on_owner_only_diff() {
    let fixture = Fixture::new();
    fixture.write_manifest("candidate", "agf/rules/workflow/new-rule.md", "AGENTS.md");
    fixture.commit("baseline planned owner");
    write(
        fixture.path(),
        "agf/rules/workflow/new-rule.md",
        "# New rule\n",
    );
    fixture.commit("create owner without touching manifest");

    let output = fixture.check_all("HEAD^");
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stderr(&output).contains("already exists"));
    assert!(stderr(&output).contains("full-tree path/lifecycle"));
}

#[test]
fn changed_page_coverage_is_limited_to_changed_manifests() {
    let fixture = Fixture::new();
    fixture.write_manifest("candidate", "AGENTS.md", "AGENTS.md");
    fixture.write_manifest("unrelated", "AGENTS.md", "AGENTS.md");
    fixture.commit("baseline manifests");

    write(
        fixture.path(),
        "agf/docs/site/docs/concepts/new-page.md",
        "# New page\n",
    );
    fixture.write_manifest(
        "candidate",
        "AGENTS.md",
        "agf/docs/site/docs/concepts/new-page.md",
    );
    fixture.commit("declare changed page in changed manifest");

    let output = fixture.check_all("HEAD^");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(!stderr(&output).contains("unrelated.yaml [diff coverage]"));
}

#[test]
fn changed_manifest_still_fails_when_it_omits_a_changed_page() {
    let fixture = Fixture::new();
    fixture.write_manifest("candidate", "AGENTS.md", "AGENTS.md");
    fixture.commit("baseline manifest");

    write(
        fixture.path(),
        "agf/docs/site/docs/concepts/omitted-page.md",
        "# Omitted page\n",
    );
    let path = fixture
        .path()
        .join("docs/reviews/rule-projection/candidate.yaml");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("# changed without declaring the page\n");
    std::fs::write(path, content).unwrap();
    fixture.commit("change manifest but omit changed page");

    let output = fixture.check_all("HEAD^");
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(stderr(&output).contains("omitted-page.md"));
    assert!(stderr(&output).contains("diff coverage"));
}

#[test]
fn explicit_repo_root_never_discovers_a_caller_checkout_manifest() {
    let fixture = Fixture::new();
    fixture.write_manifest("candidate", "AGENTS.md", "AGENTS.md");
    fixture.commit("fixture manifest");

    let caller = tempfile::tempdir().unwrap();
    write(
        caller.path(),
        "docs/reviews/rule-projection/caller-only.yaml",
        "not: a valid manifest\n",
    );
    write(
        caller.path(),
        "rule-projection-manifest.schema.json",
        "{\"not\": \"the fixture schema\"}\n",
    );

    let output = fixture.check_all_from("HEAD", caller.path(), caller.path());
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("caller-only.yaml"));
}

#[test]
fn current_repository_manifest_corpus_passes() {
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .env(
            "AGOV_SCHEMA_DIR",
            repo_root().join("agf/schemas").to_str().unwrap(),
        )
        .current_dir(repo_root())
        .args(["plan", "projection-check-all", "--base-ref", "HEAD"])
        .output()
        .expect("run current-corpus projection-check-all");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
}

fn manifest(owner: &str, changed: &str) -> String {
    let stage = if owner == "AGENTS.md" {
        "existing"
    } else {
        "planned"
    };
    format!(
        r#"
plan_id: fixture-plan
changed_pages_or_runbooks:
  - {changed}
surfaces:
  normative_owner:
    disposition: updated
    stage: {stage}
    path: {owner}
    reason: proposed by fixture plan
  adopter_facing_explanation:
    disposition: updated
    path: AGENTS.md
    reason: fixture
  runbook_situation_map:
    disposition: updated
    path: AGENTS.md
    reason: fixture
  ai_landing_loaders:
    disposition: updated
    path: AGENTS.md
    reason: fixture
  validation_drift_evidence:
    disposition: implemented
    path: AGENTS.md
    reason: fixture
  distribution:
    disposition: confirmed
    paths:
      - {changed}
    reason: fixture
"#
    )
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root")
        .to_path_buf()
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{}", stderr(&output));
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
