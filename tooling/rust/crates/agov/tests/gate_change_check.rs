use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BASE_REGISTRY: &str = include_str!("fixtures/gate-change/base-registry.yaml");
const SHARED_CI_SURFACE_REGISTRY: &str =
    include_str!("fixtures/gate-change/shared-ci-surface-registry.yaml");

struct Fixture {
    root: tempfile::TempDir,
    base: String,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("temp repository");
        git(root.path(), &["init"]);
        git(
            root.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(root.path(), &["config", "user.name", "Gate Test"]);
        write(root.path(), ".agf/gates.yaml", BASE_REGISTRY);
        write(root.path(), "tooling/checker.rs", "fn check() {}\n");
        write(root.path(), "ci/feature.sh", "#!/bin/sh\nexit 0\n");
        write(root.path(), "tests/gate_change.rs", "negative control\n");
        write(
            root.path(),
            "tests/feature/negative.txt",
            "negative control\n",
        );
        write(root.path(), "README.md", "fixture\n");
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "baseline"]);
        let base = git_output(root.path(), &["rev-parse", "HEAD"]);
        Self { root, base }
    }

    fn write(&self, path: &str, content: &str) {
        write(self.root.path(), path, content);
    }

    fn remove(&self, path: &str) {
        std::fs::remove_file(self.root.path().join(path)).expect("remove fixture path");
    }

    fn commit(&self, message: &str) {
        git(self.root.path(), &["add", "-A"]);
        git(self.root.path(), &["commit", "-m", message]);
    }

    fn check(&self) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_agov"));
        command
            .current_dir(self.root.path())
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
            .env_remove("CI_JOB_TOKEN")
            .env_remove("GITLAB_TOKEN")
            .env_remove("PRIVATE_TOKEN")
            .args(["gate", "change-check", "--base-ref", &self.base]);
        command.output().expect("run gate change-check")
    }
}

#[test]
fn blocks_surface_change_without_declared_test_change_for_the_claimed_reason() {
    let fixture = Fixture::new();
    fixture.write("ci/feature.sh", "#!/bin/sh\nexit 1\n");
    fixture.commit("change surface only");

    let output = fixture.check();
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("feature-gate"), "{stderr}");
    assert!(stderr.contains("changed surface"), "{stderr}");
    assert!(stderr.contains("no changed declared test path"), "{stderr}");
    assert!(
        !stderr.contains("does not resolve"),
        "wrong failure reason: {stderr}"
    );
}

#[test]
fn passes_surface_change_with_declared_test_change() {
    let fixture = Fixture::new();
    fixture.write("ci/feature.sh", "#!/bin/sh\nexit 1\n");
    fixture.write("tests/feature/negative.txt", "updated negative control\n");
    fixture.commit("change surface and test");
    assert_success(fixture.check());
}

#[test]
fn passes_unregistered_change() {
    let fixture = Fixture::new();
    fixture.write("README.md", "unregistered change\n");
    fixture.commit("change unregistered documentation");
    assert_success(fixture.check());
}

#[test]
fn blocks_unresolved_path_for_active_gate() {
    let fixture = Fixture::new();
    let registry = BASE_REGISTRY.replace("      - ci/feature.sh", "      - ci/missing.sh");
    fixture.write(".agf/gates.yaml", &registry);
    fixture.write("tests/gate_change.rs", "registry path negative control\n");
    fixture.commit("register missing active path");

    let output = fixture.check();
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ci/missing.sh"), "{stderr}");
    assert!(stderr.contains("does not resolve"), "{stderr}");
}

#[test]
fn blocks_deletion_of_active_entry() {
    assert_entry_deletion_blocks(false);
}

#[test]
fn blocks_deletion_of_retired_entry() {
    assert_entry_deletion_blocks(true);
}

#[test]
fn retirement_transition_requires_gate_checker_test_change() {
    let fixture = Fixture::new();
    fixture.write(".agf/gates.yaml", &retired_registry());
    fixture.commit("retire gate without checker test");

    let output = fixture.check();
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("gate-change-check"), "{stderr}");
    assert!(stderr.contains("changed surface"), "{stderr}");
}

#[test]
fn sole_gate_cannot_retire_itself_while_changing_its_surface_without_a_test_change() {
    let fixture = Fixture::new();
    fixture.write(".agf/gates.yaml", &gate_checker_only());
    fixture.write("tests/gate_change.rs", "sole gate baseline\n");
    fixture.commit("reduce registry to the sole self-checking gate");
    let sole_gate_base = git_output(fixture.root.path(), &["rev-parse", "HEAD"]);

    fixture.write(".agf/gates.yaml", &retired_gate_checker_only());
    fixture.write("tooling/checker.rs", "fn check() { /* gutted */ }\n");
    fixture.commit("retire the sole gate while gutting its checker");

    let output = check_against(&fixture, &sole_gate_base);
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("gate-change-check"), "{stderr}");
    assert!(stderr.contains("active in the comparison base"), "{stderr}");
    assert!(stderr.contains("changed surface"), "{stderr}");
    assert!(stderr.contains("no changed declared test path"), "{stderr}");
}

#[test]
fn retired_gate_may_remove_obsolete_paths_but_keeps_entry() {
    let fixture = Fixture::new();
    fixture.write(".agf/gates.yaml", &retired_registry());
    fixture.remove("ci/feature.sh");
    fixture.remove("tests/feature/negative.txt");
    fixture.write(
        "tests/gate_change.rs",
        "retirement negative control updated\n",
    );
    fixture.commit("retire gate and remove obsolete paths");
    assert_success(fixture.check());
}

#[test]
fn unrelated_touch_inside_declared_test_path_satisfies_proxy_by_design() {
    let fixture = Fixture::new();
    fixture.write("ci/feature.sh", "#!/bin/sh\nexit 1\n");
    fixture.write("tests/feature/negative.txt", "comment-only touch\n");
    fixture.commit("demonstrate review-attention proxy limit");
    assert_success(fixture.check());
}

#[test]
fn shared_ci_change_still_requires_gate_change_checks_own_evidence() {
    let root = tempfile::tempdir().expect("temp repository");
    git(root.path(), &["init"]);
    git(
        root.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(root.path(), &["config", "user.name", "Gate Test"]);
    write(root.path(), ".agf/gates.yaml", SHARED_CI_SURFACE_REGISTRY);
    write(root.path(), ".gitlab-ci.yml", "workflow: baseline\n");
    write(
        root.path(),
        "tests/gate_change/shared-surface.txt",
        "baseline gate evidence\n",
    );
    write(
        root.path(),
        "tests/ci-needs/native-package.txt",
        "baseline packaging evidence\n",
    );
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-m", "baseline"]);
    let base = git_output(root.path(), &["rev-parse", "HEAD"]);

    write(root.path(), ".gitlab-ci.yml", "workflow: native package\n");
    write(
        root.path(),
        "tests/ci-needs/native-package.txt",
        "container-free packaging evidence\n",
    );
    git(root.path(), &["add", "."]);
    git(
        root.path(),
        &["commit", "-m", "change CI with one gates evidence"],
    );

    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(root.path())
        .env("AGOV_TOKEN_ROI", "0")
        .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
        .args(["gate", "change-check", "--base-ref", &base])
        .output()
        .expect("run gate change-check");
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("gate-change-check"), "{stderr}");
    assert!(stderr.contains("no changed declared test path"), "{stderr}");
    assert!(!stderr.contains("ci-needs-implication"), "{stderr}");
}

#[test]
fn live_registry_assigns_shared_ci_surface_to_both_active_gates() {
    let registry_path = repo_root().join(".agf/gates.yaml");
    let registry = std::fs::read_to_string(&registry_path).expect("read live gate registry");
    let (owners, unresolved) = live_registry_errors(&registry);
    assert_eq!(
        owners,
        BTreeSet::from([
            "ci-needs-implication".to_string(),
            "gate-change-check".to_string(),
        ]),
        ".gitlab-ci.yml must remain registered to exactly both active gates"
    );
    assert!(unresolved.is_empty(), "{}", unresolved.join("\n"));
}

#[test]
fn live_registry_guard_detects_an_unresolved_declared_path() {
    let registry = std::fs::read_to_string(repo_root().join(".agf/gates.yaml"))
        .expect("read live gate registry");
    let mutated = registry.replacen(
        "      - tooling/rust/crates/agov/tests/gate_change_check.rs",
        "      - tooling/rust/crates/agov/tests/does-not-exist.rs",
        1,
    );
    let (_, unresolved) = live_registry_errors(&mutated);
    assert!(
        unresolved
            .iter()
            .any(|error| error.contains("does-not-exist.rs")),
        "{}",
        unresolved.join("\n")
    );
}

#[test]
fn override_registry_path_is_supported() {
    let fixture = Fixture::new();
    fixture.write("config/gates.yaml", BASE_REGISTRY);
    fixture.commit("add alternate registry");
    let output = Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(fixture.root.path())
        .env("AGOV_TOKEN_ROI", "0")
        .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
        .args([
            "gate",
            "change-check",
            "--base-ref",
            &fixture.base,
            "--registry",
            "config/gates.yaml",
        ])
        .output()
        .expect("run alternate registry check");
    assert_success(output);
}

fn live_registry_errors(registry: &str) -> (BTreeSet<String>, Vec<String>) {
    let document = serde_norway::from_str::<Value>(registry).expect("parse live gate registry");
    let gates = document["gates"]
        .as_array()
        .expect("gate registry contains gates array");
    let mut owners = BTreeSet::new();
    let mut unresolved = Vec::new();

    for gate in gates {
        if gate["status"].as_str() != Some("active") {
            continue;
        }
        let id = gate["id"].as_str().expect("active gate id");
        let surfaces = gate["surfaces"].as_array().expect("active gate surfaces");
        if surfaces
            .iter()
            .any(|path| path.as_str() == Some(".gitlab-ci.yml"))
        {
            owners.insert(id.to_string());
        }
        for field in ["surfaces", "tests"] {
            for declared in gate[field].as_array().expect("active gate path list") {
                let declared = declared.as_str().expect("declared gate path");
                let resolved = if let Some(prefix) = declared.strip_suffix("/**") {
                    repo_root().join(prefix).is_dir()
                } else {
                    repo_root().join(declared).exists()
                };
                if !resolved {
                    unresolved.push(format!(
                        "active gate `{id}` declares {field} path `{declared}` that does not resolve"
                    ));
                }
            }
        }
    }
    (owners, unresolved)
}

fn assert_entry_deletion_blocks(retired_first: bool) {
    let fixture = Fixture::new();
    if retired_first {
        fixture.write(".agf/gates.yaml", &retired_registry());
        fixture.write("tests/gate_change.rs", "retire fixture\n");
        fixture.commit("retire feature gate");
        let retired_head = git_output(fixture.root.path(), &["rev-parse", "HEAD"]);
        let registry = gate_checker_only();
        fixture.write(".agf/gates.yaml", &registry);
        fixture.write("tests/gate_change.rs", "deletion fixture\n");
        fixture.commit("delete retired entry");
        let output = check_against(&fixture, &retired_head);
        assert_deletion_failure(output);
    } else {
        fixture.write(".agf/gates.yaml", &gate_checker_only());
        fixture.write("tests/gate_change.rs", "deletion fixture\n");
        fixture.commit("delete active entry");
        assert_deletion_failure(fixture.check());
    }
}

fn check_against(fixture: &Fixture, base: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .current_dir(fixture.root.path())
        .env("AGOV_TOKEN_ROI", "0")
        .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
        .args(["gate", "change-check", "--base-ref", base])
        .output()
        .expect("run gate change-check")
}

fn assert_deletion_failure(output: Output) {
    assert_eq!(output.status.code(), Some(1), "{}", diagnostics(&output));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("feature-gate"), "{stderr}");
    assert!(stderr.contains("was deleted"), "{stderr}");
    assert!(stderr.contains("including retired entries"), "{stderr}");
}

fn retired_registry() -> String {
    BASE_REGISTRY.replacen(
        "  - id: feature-gate\n    status: active",
        "  - id: feature-gate\n    status: retired",
        1,
    ) + "    retired:\n      reason: Replaced by a native check.\n      date: 2026-08-30\n"
}

fn gate_checker_only() -> String {
    BASE_REGISTRY
        .split("  - id: feature-gate")
        .next()
        .unwrap()
        .to_string()
}

fn retired_gate_checker_only() -> String {
    gate_checker_only().replacen(
        "  - id: gate-change-check\n    status: active",
        "  - id: gate-change-check\n    status: retired",
        1,
    ) + "    retired:\n      reason: Replaced by a native check.\n      date: 2026-08-30\n"
}

fn assert_success(output: Output) {
    assert_eq!(output.status.code(), Some(0), "{}", diagnostics(&output));
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("OK gate change-check"),
        "{}",
        diagnostics(&output)
    );
}

fn diagnostics(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repository root")
        .to_path_buf()
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).expect("create fixture directory");
    std::fs::write(path, content).expect("write fixture");
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{}", diagnostics(&output));
}

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{}", diagnostics(&output));
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
