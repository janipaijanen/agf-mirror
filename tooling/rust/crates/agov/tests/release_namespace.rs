// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Exercise the real CLI, real OCI helper, and an operator stop boundary.
//! Only the HTTP transport is stubbed; no live release namespace is contacted.
#![cfg(unix)]
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output};

struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    bin: PathBuf,
    sha: String,
    log: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("repo");
        let bin = temp.path().join("bin");
        fs::create_dir_all(root.join("agf/dist")).unwrap();
        fs::create_dir_all(root.join("tooling/rust/scripts")).unwrap();
        fs::create_dir_all(&bin).unwrap();
        fs::write(root.join("agf/VERSION"), "9.8.7\n").unwrap();
        fs::write(root.join(".gitignore"), "dist/\n").unwrap();
        fs::write(root.join("agf/dist/release-manifest.yaml"), "schema_version: 1\npackage:\n  name: release-fixture\n  version_file: agf/VERSION\n  output_dir: dist\n  basename: agf-{{version}}\n  formats: [tar.gz]\ninclude: [agf/VERSION]\n").unwrap();
        fs::write(
            root.join("tooling/rust/scripts/agov-oci-image.sh"),
            include_str!("../../../scripts/agov-oci-image.sh"),
        )
        .unwrap();
        fs::write(bin.join("curl"), CURL).unwrap();
        fs::set_permissions(bin.join("curl"), fs::Permissions::from_mode(0o755)).unwrap();
        for args in [
            vec!["init", "-q"],
            vec!["add", "."],
            vec![
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        ] {
            assert!(Command::new("git")
                .current_dir(&root)
                .args(args)
                .status()
                .unwrap()
                .success());
        }
        let sha = String::from_utf8(
            Command::new("git")
                .current_dir(&root)
                .args(["rev-parse", "HEAD"])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_owned();
        let log = temp.path().join("requests");
        Self {
            _temp: temp,
            root,
            bin,
            sha,
            log,
        }
    }
    fn command(&self, scenario: &str, phase: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_agov"));
        c.current_dir(&self.root)
            .args(["release", "namespace-check", "--phase", phase])
            .env(
                "PATH",
                format!("{}:{}", self.bin.display(), std::env::var("PATH").unwrap()),
            )
            .env("CI", "false")
            .env_remove("CI_JOB_TOKEN")
            .env_remove("CI_COMMIT_TAG")
            .env("AGOV_RELEASE_TOKEN", "fixture-token-not-a-credential")
            .env("CI_API_V4_URL", "https://gitlab.fixture.invalid/api/v4")
            .env("CI_PROJECT_ID", "42")
            .env("CI_PROJECT_PATH", "fixture/project")
            .env(
                "CI_REGISTRY_IMAGE",
                "registry.fixture.invalid/fixture/project",
            )
            .env("CI_MERGE_REQUEST_TARGET_BRANCH_NAME", "main")
            .env("FIXTURE_SHA", &self.sha)
            .env("SCENARIO", scenario)
            .env("REQUEST_LOG", &self.log);
        c
    }
    fn run(&self, scenario: &str) -> Output {
        self.command(scenario, "pre-push").output().unwrap()
    }
}

#[test]
fn negative_operator_gate_stops_each_occupied_namespace_before_a_tag() {
    for scenario in [
        "git-tag",
        "release",
        "generic-package",
        "oci",
        "wrong-project",
        "hidden-404",
        "oci-hidden-404",
        "timeout",
        "malformed",
        "pagination",
    ] {
        let f = Fixture::new();
        let result = f.run(scenario);
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert!(
            !result.status.success(),
            "{scenario}: {}",
            String::from_utf8_lossy(&result.stdout)
        );
        assert!(stderr.contains("result=BLOCKED"), "{stderr}");
        assert!(!stderr.contains("fixture-token-not-a-credential"));
        // This is the operator's actual boolean publication boundary: it may
        // invoke publication only after a zero exit from the required check.
        let publish_marker = f._temp.path().join("publish-invoked");
        if result.status.success() {
            fs::write(&publish_marker, "publish").unwrap();
        }
        assert!(!publish_marker.exists());
        let tags = Command::new("git")
            .current_dir(&f.root)
            .args(["tag", "--list"])
            .output()
            .unwrap();
        assert!(tags.stdout.is_empty());
        println!(
            "negative scenario={scenario} exit={} {} publication_invoked=false tags=0",
            result.status.code().unwrap(),
            stderr.trim()
        );
        if scenario == "wrong-project" {
            let requests = fs::read_to_string(&f.log).unwrap();
            assert!(
                !requests.contains("/repository/tags"),
                "identity must fail before namespaces"
            );
        }
    }
}

#[test]
fn real_helper_manifest_and_positive_access_produce_bound_absence_evidence() {
    for scenario in ["absent", "fuzzy-package", "no-oci-repository"] {
        let f = Fixture::new();
        let out = f.run(scenario);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["result"], "ABSENT");
        assert_eq!(v["package"], "release-fixture");
        assert_eq!(
            v["oci_reference"],
            "registry.fixture.invalid/fixture/project/agov:9.8.7"
        );
        assert_eq!(v["source_sha"], f.sha);
        assert_eq!(v["target_sha"], f.sha);
        let log = fs::read_to_string(&f.log).unwrap();
        assert!(!log.contains("fixture-token-not-a-credential"));
        assert!(log.contains("status=pending_destruction"));
        println!("positive scenario={scenario} result=ABSENT");
    }
}

#[test]
fn ci_uses_job_token_supported_mr_identity_and_binds_pipeline() {
    let f = Fixture::new();
    let out = f
        .command("absent", "mr")
        .env("CI", "true")
        .env("CI_JOB_TOKEN", "fixture-job-token")
        .env("CI_PIPELINE_SOURCE", "merge_request_event")
        .env("CI_MERGE_REQUEST_IID", "7")
        .env("CI_PIPELINE_ID", "123")
        .env("CI_COMMIT_SHA", &f.sha)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["pipeline_id"], 123);
    let log = fs::read_to_string(&f.log).unwrap();
    assert!(log.contains("/merge_requests/7"));
    assert!(!log.lines().any(|l| l.ends_with("/projects/42")));
    let denied = f
        .command("absent", "mr")
        .env("CI", "true")
        .output()
        .unwrap();
    assert!(
        !denied.status.success(),
        "must not fall back to operator token in CI"
    );
    let outside_ci = f
        .command("absent", "mr")
        .env("CI", "false")
        .env("CI_JOB_TOKEN", "fixture-job-token")
        .output()
        .unwrap();
    assert!(
        !outside_ci.status.success(),
        "MR evidence must not be forgeable from an operator shell"
    );
}

#[test]
fn pre_push_rejects_dirty_tree_or_unmerged_source() {
    let f = Fixture::new();
    let out = f.run("different-target");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("live release target"));
    fs::write(f.root.join("agf/VERSION"), "9.8.8").unwrap();
    assert!(String::from_utf8_lossy(&f.run("absent").stderr).contains("clean committed"));
}

#[test]
fn operator_phase_ignores_incidental_ci_job_token() {
    let f = Fixture::new();
    let out = f
        .command("absent", "pre-push")
        .env("CI_JOB_TOKEN", "fixture-job-token")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let log = fs::read_to_string(&f.log).unwrap();
    assert!(log.lines().any(|line| line == "AUTH=PRIVATE-TOKEN"));
    assert!(!log.lines().any(|line| line == "AUTH=JOB-TOKEN"));
}

#[test]
fn ci_adapter_consumes_preview_and_preserves_the_blocking_exit() {
    let f = Fixture::new();
    let staging = f._temp.path().join("archive/agov-9.8.7-linux-x86_64");
    fs::create_dir_all(&staging).unwrap();
    let launcher = staging.join("agov");
    fs::write(
        &launcher,
        "#!/bin/sh\nexec \"$AGOV_REAL_TEST_BIN\" \"$@\"\n",
    )
    .unwrap();
    fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
    fs::create_dir(f.root.join("dist")).unwrap();
    assert!(Command::new("tar")
        .args(["-czf"])
        .arg(f.root.join("dist/agov-9.8.7-linux-x86_64.tar.gz"))
        .arg("-C")
        .arg(staging.parent().unwrap())
        .arg("agov-9.8.7-linux-x86_64")
        .status()
        .unwrap()
        .success());
    let adapter =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/release-namespace-check.sh");
    for scenario in ["git-tag", "absent"] {
        let base = f.command(scenario, "mr");
        let mut c = Command::new("sh");
        c.arg(&adapter).current_dir(&f.root);
        for (key, value) in base.get_envs() {
            if let Some(value) = value {
                c.env(key, value);
            } else {
                c.env_remove(key);
            }
        }
        let out = c
            .env("CI", "true")
            .env("CI_JOB_TOKEN", "fixture-job-token")
            .env("CI_PIPELINE_SOURCE", "merge_request_event")
            .env("CI_MERGE_REQUEST_IID", "7")
            .env("CI_PIPELINE_ID", "123")
            .env("CI_COMMIT_SHA", &f.sha)
            .env("AGOV_REAL_TEST_BIN", env!("CARGO_BIN_EXE_agov"))
            .output()
            .unwrap();
        assert_eq!(
            out.status.success(),
            scenario == "absent",
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            f.root.join("dist/release-namespace-absence.json").exists(),
            scenario == "absent"
        );
    }
}

const CURL: &str = r##"#!/bin/sh
set -eu
# Consume config from stdin, never echo it. The real transport must not put a
# credential in argv and must request only GET without following redirects.
auth=$(cat)
case "$*" in *fixture-token*|*fixture-job-token*|*--location*) exit 90;; esac
case "$*" in *'--request GET'*) :;; *) exit 91;; esac
case "$auth" in
 *'header = "JOB-TOKEN:'*) auth_kind=JOB-TOKEN;;
 *'header = "PRIVATE-TOKEN:'*) auth_kind=PRIVATE-TOKEN;;
 *) exit 92;;
esac
for arg in "$@"; do url=$arg; done
printf 'AUTH=%s\n' "$auth_kind" >> "$REQUEST_LOG"
printf '%s\n' "$url" >> "$REQUEST_LOG"
status=200
body='[]'
page=1
next=''
case "$url" in *'page=2') page=2;; esac
case "$url" in
 */merge_requests/7) body="{\"references\":{\"full\":\"fixture/project!7\"},\"source_project_id\":42,\"target_project_id\":42,\"sha\":\"$FIXTURE_SHA\",\"target_branch\":\"main\",\"state\":\"opened\"}";;
 */projects/42) body='{"id":42,"path_with_namespace":"fixture/project"}'; if [ "$SCENARIO" = wrong-project ]; then body='{"id":42,"path_with_namespace":"other/project"}'; fi;;
 *'/repository/branches?'*) target=$FIXTURE_SHA; if [ "$SCENARIO" = different-target ]; then target=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa; fi; body="[{\"name\":\"main\",\"commit\":{\"id\":\"$target\"}}]";;
 *'/repository/tags?'*)
   case "$SCENARIO" in
    git-tag) body='[{"name":"v9.8.7"}]';;
    hidden-404) status=404; body='{"message":"Not Found"}';;
    timeout) exit 28;;
    malformed) body='invalid json';;
    pagination) next=3; body='[{"name":"other"}]';;
   esac;;
 *'/releases?'*) if [ "$SCENARIO" = release ]; then body='[{"tag_name":"v9.8.7"}]'; fi;;
 *'/packages?'*)
   case "$SCENARIO:$url" in
    generic-package:*status=pending_destruction*page=1) next=2; body='[{"name":"other-release-fixture","version":"v9.8.7","package_type":"generic"}]';;
    generic-package:*status=pending_destruction*page=2) body='[{"name":"release-fixture","version":"v9.8.7","package_type":"generic"}]';;
    fuzzy-package:*) body='[{"name":"other-release-fixture","version":"v9.8.7","package_type":"generic"}]';;
   esac;;
 *'/registry/repositories?'*) if [ "$SCENARIO" != no-oci-repository ]; then body='[{"id":99,"path":"fixture/project/agov","location":"registry.fixture.invalid/fixture/project/agov"}]'; fi;;
 *'/registry/repositories/99/tags?'*)
   if [ "$SCENARIO" = oci ]; then body='[{"name":"9.8.7"}]'; fi
   if [ "$SCENARIO" = oci-hidden-404 ]; then status=404; body='{"message":"Not Found"}'; fi;;
 */v9.8.7|*/9.8.7) status=404; body='{"message":"Not Found"}';;
 *) exit 93;;
esac
printf 'HTTP/1.1 %s Response\r\nx-page: %s\r\nx-per-page: 100\r\nx-next-page: %s\r\n\r\n%s' "$status" "$page" "$next" "$body"
"##;
