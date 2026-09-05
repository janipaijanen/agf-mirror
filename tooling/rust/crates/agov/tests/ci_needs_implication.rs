use agov_validators::ci_rules::{job_selected, CiContext};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const EDGE_FIXTURE: &str = include_str!("fixtures/ci-needs/mandatory-edges.tsv");
const CONTEXT_FIXTURE: &str = include_str!("fixtures/ci-needs/context-matrix.tsv");
const SHARED_SURFACE_REGISTRY: &str =
    include_str!("fixtures/gate-change/shared-ci-surface-registry.yaml");

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Edge {
    consumer: String,
    producer: String,
}

#[derive(Clone, Debug)]
struct MatrixContext {
    name: String,
    source: String,
    branch: Option<String>,
    tag: Option<String>,
    before_sha: Option<String>,
    changed_path: String,
}

#[test]
fn live_configuration_preserves_every_mandatory_needs_implication() {
    let configuration = read_live_configuration();
    let errors = implication_errors(&configuration).expect("evaluate live CI configuration");
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn mandatory_edge_fixture_matches_the_complete_ci_inventory() {
    let configuration = read_live_configuration();
    let actual = mandatory_edges(&configuration).expect("extract mandatory needs edges");
    let expected = fixture_edges();
    assert_eq!(
        actual, expected,
        "mandatory-edges.tsv must enumerate every non-optional top-level needs edge"
    );
}

#[test]
fn detects_missing_feature_push_rule_for_nri_1_producer() {
    let configuration = read_live_configuration();
    let producer =
        top_level_block(&configuration, "rust:tooling:preview").expect("rust preview job block");
    let rule = "    - if: '$CI_PIPELINE_SOURCE == \"push\" && $CI_COMMIT_BRANCH && $CI_COMMIT_BRANCH != $CI_DEFAULT_BRANCH'\n      changes:\n        compare_to: 'refs/heads/$CI_DEFAULT_BRANCH'\n        paths: *rust_tooling_preview_paths\n";
    assert!(producer.contains(rule), "expected corrected producer rule");
    let mutated = replace_top_level_block(
        &configuration,
        "rust:tooling:preview",
        &producer.replacen(rule, "", 1),
    );

    assert_failure_names(
        &mutated,
        "agov:oci:mr-build-scan",
        "rust:tooling:preview",
        "later-push-rust",
    );
}

#[test]
fn detects_missing_rust_input_for_nri_2_producer() {
    let configuration = read_live_configuration();
    let producer =
        top_level_block(&configuration, "agf:dist:preview").expect("framework preview job block");
    let input = "        - tooling/rust/**/*\n";
    assert_eq!(
        producer.matches(input).count(),
        2,
        "expected both branch rules"
    );
    let mutated = replace_top_level_block(
        &configuration,
        "agf:dist:preview",
        &producer.replace(input, ""),
    );

    assert_failure_names(
        &mutated,
        "release:assets-verify-preview",
        "agf:dist:preview",
        "mr-rust",
    );
}

#[test]
fn detects_source_agnostic_branch_rule_on_nri_1_consumer() {
    let configuration = read_live_configuration();
    let consumer = top_level_block(&configuration, "agov:oci:mr-build-scan")
        .expect("OCI preview scan job block");
    let guarded = "    - if: '$CI_PIPELINE_SOURCE == \"push\" && $CI_COMMIT_BRANCH'\n";
    assert_eq!(consumer.matches(guarded).count(), 1, "expected push guard");
    let mutated = replace_top_level_block(
        &configuration,
        "agov:oci:mr-build-scan",
        &consumer.replacen(guarded, "    - if: $CI_COMMIT_BRANCH\n", 1),
    );

    assert_failure_names(
        &mutated,
        "agov:oci:mr-build-scan",
        "rust:tooling:preview",
        "schedule-feature",
    );
}

#[test]
fn ignores_semantically_irrelevant_changes_list_order() {
    let configuration = read_live_configuration();
    let consumer = top_level_block(&configuration, "release:assets-verify-preview")
        .expect("preview verification job block");
    let original = "        - tooling/rust/**/*\n        - .gitlab-ci.yml\n";
    let reordered = "        - .gitlab-ci.yml\n        - tooling/rust/**/*\n";
    assert_eq!(
        consumer.matches(original).count(),
        2,
        "expected both equivalent rules"
    );
    let mutated = replace_top_level_block(
        &configuration,
        "release:assets-verify-preview",
        &consumer.replace(original, reordered),
    );
    let errors = implication_errors(&mutated).expect("evaluate reordered CI configuration");
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn shared_surface_requires_each_active_gates_own_test_evidence() {
    let fixture = GateFixture::new();
    fixture.write(".gitlab-ci.yml", "workflow: corrected\n");
    fixture.write("tests/ci-needs/mutation.txt", "covered correction\n");
    fixture.commit("change shared CI surface with one gate's evidence");

    let blocked = fixture.check();
    assert_eq!(blocked.status.code(), Some(1), "{}", diagnostics(&blocked));
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    assert!(stderr.contains("gate-change-check"), "{stderr}");
    assert!(stderr.contains("no changed declared test path"), "{stderr}");
    assert!(!stderr.contains("ci-needs-implication"), "{stderr}");

    fixture.write(
        "tests/gate_change/shared-surface.txt",
        "shared-surface regression evidence\n",
    );
    fixture.commit("add the other active gate's own evidence");
    let passed = fixture.check();
    assert!(passed.status.success(), "{}", diagnostics(&passed));
}

#[test]
fn live_mandatory_linux_packaging_is_native_and_container_free() {
    let configuration = read_live_configuration();
    assert_native_linux_packaging(&configuration);
}

#[test]
fn detects_reintroduced_container_build_on_the_mandatory_linux_path() {
    let configuration = read_live_configuration();
    let package = top_level_block(&configuration, ".rust-tooling-package")
        .expect("mandatory Rust packaging template");
    let native = "        cargo build --release --target \"$target\" -p agov -p agov-mcp\n";
    assert_eq!(
        package.matches(native).count(),
        1,
        "expected native build line"
    );
    let mutated_package = package.replacen(
        native,
        "        cross build --release --target \"$target\" -p agov -p agov-mcp\n",
        1,
    );
    let mutated =
        replace_top_level_block(&configuration, ".rust-tooling-package", &mutated_package);

    let errors = native_linux_packaging_errors(&mutated);
    assert!(
        errors.iter().any(|error| error.contains("cargo build")),
        "{}",
        errors.join("\n")
    );
}

fn assert_native_linux_packaging(configuration: &str) {
    let errors = native_linux_packaging_errors(configuration);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

fn native_linux_packaging_errors(configuration: &str) -> Vec<String> {
    let Some(package) = top_level_block(configuration, ".rust-tooling-package") else {
        return vec!["mandatory Rust packaging template is missing".to_string()];
    };
    let mut errors = Vec::new();
    for forbidden in [
        "    - docker:dind\n",
        "    DOCKER_HOST:",
        "    DOCKER_DRIVER:",
        "    DOCKER_TLS_CERTDIR:",
        "    CROSS_REMOTE:",
        "docker.io",
        "cargo install cross",
    ] {
        if package.contains(forbidden) {
            errors.push(format!(
                "mandatory Rust packaging template retains forbidden container dependency `{}`",
                forbidden.trim()
            ));
        }
    }
    for required in [
        "        rustup target add \"$target\"\n",
        "        cargo build --release --target \"$target\" -p agov -p agov-mcp\n",
    ] {
        if package.matches(required).count() != 1 {
            errors.push(format!(
                "mandatory Rust packaging template must contain exactly one `{}`",
                required.trim()
            ));
        }
    }
    for consumer in ["rust:tooling:preview", "rust:tooling:release"] {
        let Some(block) = top_level_block(configuration, consumer) else {
            errors.push(format!("mandatory consumer `{consumer}` is missing"));
            continue;
        };
        if !block.contains("  extends: .rust-tooling-package\n") {
            errors.push(format!(
                "mandatory consumer `{consumer}` no longer extends the native packaging template"
            ));
        }
    }
    errors
}

fn implication_errors(configuration: &str) -> Result<Vec<String>, String> {
    let actual_edges = mandatory_edges(configuration)?;
    let expected_edges = fixture_edges();
    if actual_edges != expected_edges {
        return Err(format!(
            "mandatory edge fixture mismatch: expected {expected_edges:?}, actual {actual_edges:?}"
        ));
    }

    let jobs = expected_edges
        .iter()
        .flat_map(|edge| [&edge.consumer, &edge.producer])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|name| {
            let block = top_level_block(configuration, name)
                .ok_or_else(|| format!("top-level job `{name}` is missing"))?;
            let job = serde_norway::from_str::<Value>(&block)
                .map_err(|error| format!("parse top-level job `{name}`: {error}"))?;
            Ok((name.to_string(), job[name].clone()))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;

    let mut errors = Vec::new();
    for context in fixture_contexts() {
        let ci_context = context.as_ci_context();
        for edge in &expected_edges {
            let consumer = job_is_selected(&jobs[&edge.consumer], &ci_context)?;
            let producer = job_is_selected(&jobs[&edge.producer], &ci_context)?;
            if consumer && !producer {
                errors.push(format!(
                    "mandatory needs implication failed: consumer `{}` selected while producer `{}` omitted in context `{}`",
                    edge.consumer, edge.producer, context.name
                ));
            }
        }
    }
    Ok(errors)
}

fn job_is_selected(job: &Value, context: &CiContext) -> Result<bool, String> {
    match job.get("rules") {
        Some(rules) => job_selected(rules, context),
        None => Ok(true),
    }
}

fn mandatory_edges(configuration: &str) -> Result<BTreeSet<Edge>, String> {
    let mut edges = BTreeSet::new();
    for (consumer, block) in top_level_blocks(configuration) {
        if !block.contains("\n  needs:") {
            continue;
        }
        let document = serde_norway::from_str::<Value>(&block)
            .map_err(|error| format!("parse top-level job `{consumer}`: {error}"))?;
        let Some(needs) = document[&consumer].get("needs").and_then(Value::as_array) else {
            continue;
        };
        for need in needs {
            let producer = match need {
                Value::String(producer) => producer,
                Value::Object(object) if object.get("optional") == Some(&Value::Bool(true)) => {
                    continue;
                }
                Value::Object(object) => object
                    .get("job")
                    .and_then(Value::as_str)
                    .ok_or_else(|| format!("job `{consumer}` has a needs object without `job`"))?,
                _ => return Err(format!("job `{consumer}` has unsupported needs syntax")),
            };
            edges.insert(Edge {
                consumer: consumer.clone(),
                producer: producer.to_string(),
            });
        }
    }
    Ok(edges)
}

fn fixture_edges() -> BTreeSet<Edge> {
    EDGE_FIXTURE
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (consumer, producer) = line.split_once('\t').expect("edge fixture has two columns");
            Edge {
                consumer: consumer.to_string(),
                producer: producer.to_string(),
            }
        })
        .collect()
}

fn fixture_contexts() -> Vec<MatrixContext> {
    CONTEXT_FIXTURE
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let columns = line.split('\t').collect::<Vec<_>>();
            assert_eq!(columns.len(), 6, "context fixture row: {line}");
            MatrixContext {
                name: columns[0].to_string(),
                source: columns[1].to_string(),
                branch: optional(columns[2]),
                tag: optional(columns[3]),
                before_sha: optional(columns[4]),
                changed_path: columns[5].to_string(),
            }
        })
        .collect()
}

impl MatrixContext {
    fn as_ci_context(&self) -> CiContext {
        let mut context = CiContext::new()
            .with_variable("CI_PIPELINE_SOURCE", &self.source)
            .with_variable("CI_DEFAULT_BRANCH", "main")
            .with_changed_path(&self.changed_path);
        if let Some(branch) = &self.branch {
            context = context.with_variable("CI_COMMIT_BRANCH", branch);
        }
        if let Some(tag) = &self.tag {
            context = context.with_variable("CI_COMMIT_TAG", tag);
        }
        if let Some(before_sha) = &self.before_sha {
            context = context.with_variable("CI_COMMIT_BEFORE_SHA", before_sha);
        }
        context
    }
}

fn optional(value: &str) -> Option<String> {
    (value != "-").then(|| value.to_string())
}

fn assert_failure_names(configuration: &str, consumer: &str, producer: &str, context: &str) {
    let errors = implication_errors(configuration).expect("evaluate mutated CI configuration");
    let joined = errors.join("\n");
    assert!(joined.contains(consumer), "{joined}");
    assert!(joined.contains(producer), "{joined}");
    assert!(joined.contains(context), "{joined}");
}

fn read_live_configuration() -> String {
    std::fs::read_to_string(repo_root().join(".gitlab-ci.yml")).expect("read live CI configuration")
}

fn top_level_block(configuration: &str, name: &str) -> Option<String> {
    top_level_blocks(configuration)
        .into_iter()
        .find_map(|(candidate, block)| (candidate == name).then_some(block))
}

fn top_level_blocks(configuration: &str) -> Vec<(String, String)> {
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in configuration.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if !trimmed.is_empty() && !trimmed.starts_with([' ', '\t', '#']) && trimmed.ends_with(':') {
            starts.push((offset, trimmed.trim_end_matches(':').to_string()));
        }
        offset += line.len();
    }
    starts
        .iter()
        .enumerate()
        .map(|(index, (start, name))| {
            let end = starts
                .get(index + 1)
                .map(|(next, _)| *next)
                .unwrap_or(configuration.len());
            (name.clone(), configuration[*start..end].to_string())
        })
        .collect()
}

fn replace_top_level_block(configuration: &str, name: &str, replacement: &str) -> String {
    let original = top_level_block(configuration, name).expect("top-level block to replace");
    assert_ne!(original, replacement, "mutation must change the block");
    assert_eq!(
        configuration.matches(&original).count(),
        1,
        "unique job block"
    );
    configuration.replacen(&original, replacement, 1)
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

struct GateFixture {
    root: tempfile::TempDir,
    base: String,
}

impl GateFixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("temp repository");
        git(root.path(), &["init"]);
        git(
            root.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(root.path(), &["config", "user.name", "CI Needs Test"]);
        write(root.path(), ".agf/gates.yaml", SHARED_SURFACE_REGISTRY);
        write(root.path(), ".gitlab-ci.yml", "workflow: baseline\n");
        write(
            root.path(),
            "tests/gate_change/shared-surface.txt",
            "baseline gate evidence\n",
        );
        write(
            root.path(),
            "tests/ci-needs/mutation.txt",
            "baseline needs evidence\n",
        );
        git(root.path(), &["add", "."]);
        git(root.path(), &["commit", "-m", "baseline"]);
        let base = git_output(root.path(), &["rev-parse", "HEAD"]);
        Self { root, base }
    }

    fn write(&self, path: &str, content: &str) {
        write(self.root.path(), path, content);
    }

    fn commit(&self, message: &str) {
        git(self.root.path(), &["add", "-A"]);
        git(self.root.path(), &["commit", "-m", message]);
    }

    fn check(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_agov"))
            .current_dir(self.root.path())
            .env("AGOV_TOKEN_ROI", "0")
            .env("AGOV_SCHEMA_DIR", repo_root().join("agf/schemas"))
            .args(["gate", "change-check", "--base-ref", &self.base])
            .output()
            .expect("run gate change-check")
    }
}

fn write(root: &Path, path: &str, content: &str) {
    let target = root.join(path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("create fixture parent");
    }
    std::fs::write(target, content).expect("write fixture path");
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{}", diagnostics(&output));
}

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(output.status.success(), "{}", diagnostics(&output));
    String::from_utf8(output.stdout)
        .expect("git output utf-8")
        .trim()
        .to_string()
}

fn diagnostics(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}
