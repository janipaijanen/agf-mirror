use agov_validators::ci_rules::{job_selected, CiContext};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const EDGE_FIXTURE: &str = include_str!("fixtures/ci-needs/mandatory-edges.tsv");
const CONTEXT_FIXTURE: &str = include_str!("fixtures/ci-needs/context-matrix.tsv");
const SHARED_SURFACE_REGISTRY: &str =
    include_str!("fixtures/gate-change/shared-ci-surface-registry.yaml");
const DIND_JOBS: [&str; 6] = [
    "agov:oci:mr-build-scan",
    "rust:tooling:oci-pull-test",
    "agov:oci:release",
    "release:verify",
    "rust:tooling:preview:windows-package",
    "rust:tooling:build:windows:preview",
];

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
fn namespace_gate_is_pre_tag_only_and_reuses_the_preview_producer() {
    let yaml: serde_norway::Value = serde_norway::from_str(&read_live_configuration()).unwrap();
    let rules = &yaml["release:namespace-check"]["rules"];
    let rules: Value = serde_json::to_value(rules).unwrap();
    for (source, changed, selected) in [
        ("merge_request_event", "agf/VERSION", true),
        ("merge_request_event", "README.md", false),
        ("push", "agf/VERSION", false),
        ("schedule", "agf/VERSION", false),
    ] {
        let context = CiContext::new()
            .with_variable("CI_PIPELINE_SOURCE", source)
            .with_changed_path(changed);
        assert_eq!(job_selected(&rules, &context).unwrap(), selected);
    }
    let block = top_level_block(&read_live_configuration(), "release:namespace-check").unwrap();
    assert!(block.contains("allow_failure: false"));
    assert!(block.contains("job: rust:tooling:preview"));
    assert!(!block.contains("cargo build") && !block.contains("dind"));
}

#[test]
fn live_configuration_declares_split_runner_routing() {
    let configuration = read_live_configuration();
    let errors = split_runner_routing_errors(&configuration);
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn detects_a_dind_override_on_an_ordinary_job() {
    let configuration = read_live_configuration();
    let ordinary =
        top_level_block(&configuration, "rust:tooling:test").expect("ordinary Rust test job block");
    let mutated_ordinary = ordinary.replacen(
        "rust:tooling:test:\n",
        "rust:tooling:test:\n  tags:\n    - dind\n",
        1,
    );
    let mutated = replace_top_level_block(&configuration, "rust:tooling:test", &mutated_ordinary);

    let errors = split_runner_routing_errors(&mutated);
    assert!(
        errors
            .iter()
            .any(|error| error.contains("rust:tooling:test") && error.contains("unexpected")),
        "{}",
        errors.join("\n")
    );
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
fn commercial_corpus_validate_guarantees_merge_base_ancestry() {
    // Regression guard for the shallow-history "no merge base" failure
    // (MR !2678, pipeline 2840038842): commercial-corpus:validate compares
    // against CI_MERGE_REQUEST_DIFF_BASE_SHA and needs a real merge base,
    // which the project default GIT_DEPTH (20) cannot guarantee on a
    // branch more than ~20 commits past its target. Mirrors the existing
    // agov:governance:validate override for the identical reason.
    let configuration = read_live_configuration();
    let block = top_level_block(&configuration, "commercial-corpus:validate")
        .expect("commercial-corpus:validate job block");
    assert!(
        block.contains("GIT_DEPTH: \"0\""),
        "commercial-corpus:validate must set GIT_DEPTH: \"0\" to guarantee \
         merge-base ancestry for its --base-ref comparison:\n{block}"
    );
}

#[test]
fn rust_tooling_deny_clears_stale_advisory_db_before_check() {
    // Regression guard for the cargo-deny advisory-db crash (pipeline
    // 2842901098, required job rust:tooling:deny, 2026-09-12): the
    // advisory-db clone lives under CARGO_HOME but is not a cached path
    // (only .cargo/registry, .cargo/bin, and tooling/rust/target are). A
    // prior run's partial state there made cargo-deny try to read an
    // existing clone's FETCH_HEAD and crash when the directory was
    // actually missing, misparsing git's fatal error as an ISO-8601
    // timestamp instead of cloning fresh.
    let configuration = read_live_configuration();
    let block =
        top_level_block(&configuration, "rust:tooling:deny").expect("rust:tooling:deny job block");
    let clear_pos = block.find("rm -rf \"$CARGO_HOME/advisory-dbs\"").unwrap_or_else(|| {
        panic!(
            "rust:tooling:deny must clear $CARGO_HOME/advisory-dbs before \
             running cargo deny check, so a torn prior clone can't make it \
             crash instead of cloning fresh:\n{block}"
        )
    });
    let check_pos = block
        .find("cargo deny check")
        .expect("rust:tooling:deny must still run cargo deny check");
    assert!(
        clear_pos < check_pos,
        "the advisory-db clear must run before cargo deny check, not after:\n{block}"
    );
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

fn split_runner_routing_errors(configuration: &str) -> Vec<String> {
    let mut errors = Vec::new();

    let expected_default = ["podman", "linux", "amd64"].map(str::to_string);
    let actual_default = top_level_block(configuration, "default")
        .as_deref()
        .and_then(top_level_tags);
    if actual_default.as_deref() != Some(expected_default.as_slice()) {
        errors.push(format!(
            "default runner tags must be exactly {:?}, found {:?}",
            expected_default, actual_default
        ));
    }

    for job in DIND_JOBS {
        let actual = top_level_block(configuration, job)
            .as_deref()
            .and_then(top_level_tags);
        if actual != Some(vec!["dind".to_string()]) {
            errors.push(format!(
                "DinD job `{job}` must override runner tags with exactly [dind], found {:?}",
                actual
            ));
        }
    }

    let allowed = DIND_JOBS.into_iter().collect::<BTreeSet<_>>();
    for (name, block) in top_level_blocks(configuration) {
        if name == "default" || allowed.contains(name.as_str()) {
            continue;
        }
        if top_level_tags(&block).is_some() {
            errors.push(format!(
                "top-level CI entry `{name}` has an unexpected runner tags override"
            ));
        }
    }
    errors
}

fn top_level_tags(block: &str) -> Option<Vec<String>> {
    let mut lines = block.lines();
    lines.find(|line| *line == "  tags:")?;
    Some(
        lines
            .take_while(|line| line.starts_with("    "))
            .filter_map(|line| line.strip_prefix("    - "))
            .map(str::to_string)
            .collect(),
    )
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
        "apt-get install -y -qq binutils jq musl-tools zip\n",
        "    - export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc\n",
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
