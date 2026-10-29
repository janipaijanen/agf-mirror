// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Create-only release publisher tests. Real jq and the real shell helper run;
//! only curl transport is replaced, and no production endpoint is contacted.
#![cfg(unix)]

use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const TOKEN: &str = "fixture-job-token-not-a-credential";
const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

struct Fixture {
    temp: tempfile::TempDir,
    dist: PathBuf,
    payload: PathBuf,
    bin: PathBuf,
    log: PathBuf,
    state: PathBuf,
    work_dirs: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let dist = temp.path().join("dist");
        let bin = temp.path().join("bin");
        fs::create_dir_all(&dist).unwrap();
        fs::create_dir_all(&bin).unwrap();
        for name in ["agf-9.9.9.tar.gz", "agf-9.9.9.tar.gz.sha256"] {
            fs::write(dist.join(name), name).unwrap();
        }
        let curl = bin.join("curl");
        fs::write(&curl, CURL_FIXTURE).unwrap();
        fs::set_permissions(&curl, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            payload: temp.path().join("release-intent.json"),
            log: temp.path().join("requests.log"),
            state: temp.path().join("release-created"),
            work_dirs: temp.path().join("work-dirs.log"),
            temp,
            dist,
            bin,
        }
    }

    fn command(&self, action: &str, scenario: &str) -> Command {
        let mut command = Command::new("sh");
        command
            .arg(helper())
            .arg(action)
            .arg(&self.dist)
            .arg(&self.payload)
            .env(
                "PATH",
                format!("{}:{}", self.bin.display(), std::env::var("PATH").unwrap()),
            )
            .env("TMPDIR", self.temp.path())
            .env("CI_API_V4_URL", "https://gitlab.fixture.invalid/api/v4")
            .env("CI_PROJECT_ID", "42")
            .env("CI_PROJECT_PATH", "fixture/project")
            .env("CI_COMMIT_TAG", "v9.9.9/rc\"quoted")
            .env("CI_COMMIT_SHA", SHA)
            .env("CI_JOB_TOKEN", TOKEN)
            .env("EXPECTED_TOKEN", TOKEN)
            .env("FIXTURE_SHA", SHA)
            .env("SCENARIO", scenario)
            .env("REQUEST_LOG", &self.log)
            .env("STATE_FILE", &self.state)
            .env("WORK_DIR_LOG", &self.work_dirs)
            .env("FIXTURE_ROOT", self.temp.path());
        command
    }

    fn run(&self, action: &str, scenario: &str) -> Output {
        self.command(action, scenario).output().unwrap()
    }

    fn preflight(&self, scenario: &str) -> Output {
        self.run("preflight", scenario)
    }

    fn publish(&self, scenario: &str) -> Output {
        self.run("publish", scenario)
    }

    fn log(&self) -> String {
        fs::read_to_string(&self.log).unwrap_or_default()
    }

    fn assert_token_absent_from_files_and_output(&self, output: &Output) {
        assert!(!String::from_utf8_lossy(&output.stdout).contains(TOKEN));
        assert!(!String::from_utf8_lossy(&output.stderr).contains(TOKEN));
        visit_files(self.temp.path(), &mut |path| {
            if path == self.bin.join("curl") {
                return;
            }
            let bytes = fs::read(path).unwrap();
            assert!(
                !String::from_utf8_lossy(&bytes).contains(TOKEN),
                "token persisted in {}",
                path.display()
            );
        });
        for path in fs::read_to_string(&self.work_dirs)
            .unwrap_or_default()
            .lines()
        {
            assert!(
                !Path::new(path).exists(),
                "helper work directory leaked: {path}"
            );
        }
    }
}

#[test]
fn successful_create_uses_exact_intent_one_post_and_no_mutating_fallback() {
    let fixture = Fixture::new();
    let preflight = fixture.preflight("success");
    assert_success(&preflight);
    fixture.assert_token_absent_from_files_and_output(&preflight);

    let payload: serde_json::Value =
        serde_json::from_slice(&fs::read(&fixture.payload).unwrap()).unwrap();
    let package_base =
        "https://gitlab.fixture.invalid/api/v4/projects/42/packages/generic/agf/v9.9.9%2Frc%22quoted";
    assert_eq!(
        payload,
        serde_json::json!({
            "tag_name": "v9.9.9/rc\"quoted",
            "name": "agents-govern v9.9.9/rc\"quoted",
            "description": "See [CHANGELOG](https://gitlab.com/fixture/project/-/blob/v9.9.9/rc\"quoted/agf/CHANGELOG.md) for release notes.",
            "assets": {"links": [
                {
                    "name": "agf-9.9.9.tar.gz",
                    "url": format!("{package_base}/agf-9.9.9.tar.gz"),
                    "link_type": "package"
                },
                {
                    "name": "agf-9.9.9.tar.gz.sha256",
                    "url": format!("{package_base}/agf-9.9.9.tar.gz.sha256"),
                    "link_type": "package"
                }
            ]}
        })
    );

    let publish = fixture.publish("post-reordered");
    assert_success(&publish);
    fixture.assert_token_absent_from_files_and_output(&publish);
    let log = fixture.log();
    assert_eq!(
        log.lines().filter(|line| line.starts_with("POST ")).count(),
        1
    );
    assert!(!log.lines().any(|line| line.starts_with("PUT ")));
    assert!(!log.lines().any(|line| line.starts_with("DELETE ")));
    assert!(!log.contains("--location") && !log.contains("--retry"));
    assert!(fixture.state.exists());
    let requests: Vec<_> = log
        .lines()
        .filter(|line| !line.starts_with("ARGV "))
        .collect();
    assert_eq!(requests.len(), 8, "{requests:?}");
    assert!(requests[0].starts_with("GET ") && requests[0].contains("/repository/tags/"));
    assert!(requests[1].starts_with("GET ") && requests[1].contains("/releases?per_page=1"));
    assert!(
        requests[2].starts_with("GET ") && requests[2].contains("/releases/v9.9.9%2Frc%22quoted")
    );
    assert!(requests[3].starts_with("GET ") && requests[3].contains("/repository/tags/"));
    assert!(requests[4].starts_with("GET ") && requests[4].contains("/releases?per_page=1"));
    assert!(
        requests[5].starts_with("GET ") && requests[5].contains("/releases/v9.9.9%2Frc%22quoted")
    );
    assert!(requests[6].starts_with("POST ") && requests[6].ends_with("/releases"));
    assert!(
        requests[7].starts_with("GET ") && requests[7].contains("/releases/v9.9.9%2Frc%22quoted")
    );
}

#[test]
fn initial_preflight_failures_make_no_mutating_request() {
    for (scenario, expected) in [
        ("missing-tag", "tag consistency returned HTTP 404"),
        ("wrong-sha", "tag commit mismatch"),
        (
            "unauthorized",
            "Releases namespace access returned HTTP 401",
        ),
        (
            "inaccessible",
            "Releases namespace access returned HTTP 403",
        ),
        ("existing-release", "release already exists"),
        ("redirect", "tag consistency returned HTTP 302"),
        ("malformed", "tag consistency returned malformed JSON"),
        ("multi-tag-json", "tag consistency returned malformed JSON"),
        (
            "multi-namespace-json",
            "Releases namespace access returned malformed JSON",
        ),
        (
            "release-404-malformed",
            "release absence returned malformed JSON",
        ),
        (
            "release-404-html",
            "release absence returned malformed JSON",
        ),
        (
            "release-404-empty",
            "release absence returned malformed JSON",
        ),
        (
            "release-404-truncated",
            "release absence returned malformed JSON",
        ),
        (
            "release-404-wrong-json",
            "release absence returned an unexpected 404 response",
        ),
    ] {
        let fixture = Fixture::new();
        let output = fixture.preflight(scenario);
        assert!(!output.status.success(), "{scenario}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(expected), "{scenario}: {stderr}");
        let log = fixture.log();
        assert!(!log.lines().any(|line| line.starts_with("POST ")), "{log}");
        assert!(!fixture.state.exists());
        fixture.assert_token_absent_from_files_and_output(&output);
    }
}

#[test]
fn repeated_preflight_rejects_changed_or_malformed_state_before_post() {
    for (scenario, expected) in [
        (
            "repeat-multi-namespace-json",
            "Releases namespace access returned malformed JSON",
        ),
        (
            "repeat-release-404-html",
            "release absence returned malformed JSON",
        ),
    ] {
        let fixture = Fixture::new();
        assert_success(&fixture.preflight(scenario));
        let output = fixture.publish(scenario);
        assert!(!output.status.success(), "{scenario}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{scenario}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            fixture
                .log()
                .lines()
                .filter(|line| line.starts_with("POST "))
                .count(),
            0
        );
        assert!(!fixture.state.exists());
        fixture.assert_token_absent_from_files_and_output(&output);
    }
}

#[test]
fn conflict_and_postcondition_mismatch_fail_without_update_or_delete() {
    for (scenario, expected) in [
        ("conflict", "release creation returned HTTP 409"),
        ("post-mismatch", "created release does not match"),
        ("post-duplicate-link", "created release does not match"),
        ("post-missing-link", "created release does not match"),
        ("post-wrong-url", "created release does not match"),
        ("post-wrong-link-type", "created release does not match"),
        (
            "post-malformed",
            "release verification returned malformed JSON",
        ),
        (
            "post-multi-json",
            "release verification returned malformed JSON",
        ),
        ("followup-mismatch", "created release does not match"),
        (
            "followup-malformed",
            "release verification returned malformed JSON",
        ),
        (
            "followup-multi-json",
            "release verification returned malformed JSON",
        ),
    ] {
        let fixture = Fixture::new();
        assert_success(&fixture.preflight("success"));
        let output = fixture.publish(scenario);
        assert!(!output.status.success(), "{scenario}");
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
        let log = fixture.log();
        assert_eq!(
            log.lines().filter(|line| line.starts_with("POST ")).count(),
            1
        );
        assert!(!log.lines().any(|line| line.starts_with("PUT ")));
        assert!(!log.lines().any(|line| line.starts_with("DELETE ")));
        fixture.assert_token_absent_from_files_and_output(&output);
    }
}

#[test]
fn lost_post_response_performs_one_diagnostic_get_and_never_retries() {
    let fixture = Fixture::new();
    assert_success(&fixture.preflight("success"));
    let before = fixture.log().lines().count();
    let output = fixture.publish("lost-post");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("indeterminate POST"), "{stderr}");
    assert!(
        stderr.contains("diagnostic release state=present"),
        "{stderr}"
    );
    let log = fixture.log();
    let new_requests: Vec<_> = log.lines().skip(before).collect();
    assert_eq!(
        new_requests
            .iter()
            .filter(|line| line.starts_with("POST "))
            .count(),
        1
    );
    let post = new_requests
        .iter()
        .position(|line| line.starts_with("POST "))
        .unwrap();
    assert_eq!(
        new_requests[post + 1..]
            .iter()
            .filter(|line| line.starts_with("GET "))
            .count(),
        1,
        "{new_requests:?}"
    );
    fixture.assert_token_absent_from_files_and_output(&output);
}

#[test]
fn changed_payload_is_rejected_before_publish_checks_or_post() {
    let fixture = Fixture::new();
    assert_success(&fixture.preflight("success"));
    let mut payload: serde_json::Value =
        serde_json::from_slice(&fs::read(&fixture.payload).unwrap()).unwrap();
    payload["name"] = serde_json::json!("substituted release");
    fs::write(&fixture.payload, serde_json::to_vec(&payload).unwrap()).unwrap();
    let before = fixture.log().lines().count();
    let output = fixture.publish("success");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no longer matches"));
    assert_eq!(fixture.log().lines().count(), before);
    assert!(!fixture.state.exists());
}

#[test]
fn empty_release_intent_is_rejected_before_any_request() {
    let fixture = Fixture::new();
    for entry in fs::read_dir(&fixture.dist).unwrap() {
        fs::remove_file(entry.unwrap().path()).unwrap();
    }
    let output = fixture.preflight("success");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("no asset links"));
    assert!(fixture.log().is_empty());
}

#[test]
fn same_project_mr_job_token_reads_existing_tag_and_release_without_mutation() {
    if std::env::var("CI").as_deref() != Ok("true")
        || std::env::var("CI_PIPELINE_SOURCE").as_deref() != Ok("merge_request_event")
        || std::env::var("CI_PROJECT_PATH").as_deref() != Ok("janipaijanen/agents-govern-framework")
    {
        eprintln!("SKIP read-only project probe: not the canonical same-project MR pipeline");
        return;
    }

    let api = std::env::var("CI_API_V4_URL").expect("canonical MR CI_API_V4_URL");
    assert_eq!(api, "https://gitlab.com/api/v4");
    let project = std::env::var("CI_PROJECT_ID").expect("canonical MR CI_PROJECT_ID");
    assert!(project.bytes().all(|byte| byte.is_ascii_digit()));
    let token = std::env::var("CI_JOB_TOKEN").expect("same-project MR CI_JOB_TOKEN");
    let temp = tempfile::tempdir().unwrap();

    let tag_body = temp.path().join("tag.json");
    let tag_status = read_with_job_token(
        &format!("{api}/projects/{project}/repository/tags/v2.12.0"),
        &tag_body,
        &token,
    );
    assert_eq!(tag_status, "200", "single-tag JOB-TOKEN probe failed");
    let tag: serde_json::Value = serde_json::from_slice(&fs::read(&tag_body).unwrap()).unwrap();
    assert_eq!(tag["name"], "v2.12.0");
    assert_eq!(tag["protected"], true);
    let tag_sha = tag["commit"]["id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .expect("tag response commit id");

    let namespace_body = temp.path().join("releases.json");
    let namespace_status = read_with_job_token(
        &format!("{api}/projects/{project}/releases?per_page=1"),
        &namespace_body,
        &token,
    );
    assert_eq!(
        namespace_status, "200",
        "Releases namespace JOB-TOKEN probe failed"
    );
    let namespace: serde_json::Value =
        serde_json::from_slice(&fs::read(&namespace_body).unwrap()).unwrap();
    assert!(
        namespace.is_array(),
        "Releases namespace response is not an array"
    );

    let release_body = temp.path().join("release.json");
    let release_status = read_with_job_token(
        &format!("{api}/projects/{project}/releases/v2.12.0"),
        &release_body,
        &token,
    );
    assert_eq!(
        release_status, "200",
        "existing Release JOB-TOKEN probe failed"
    );
    let release: serde_json::Value =
        serde_json::from_slice(&fs::read(&release_body).unwrap()).unwrap();
    assert_eq!(release["tag_name"], "v2.12.0");
    assert_eq!(release["commit"]["id"], tag_sha);
}

fn read_with_job_token(url: &str, body: &Path, token: &str) -> String {
    let mut child = Command::new("curl");
    child
        .args([
            "--disable",
            "--config",
            "-",
            "--silent",
            "--show-error",
            "--connect-timeout",
            "15",
            "--max-time",
            "90",
            "--proto",
            "=https",
            "--request",
            "GET",
            "--output",
        ])
        .arg(body)
        .args(["--write-out", "%{http_code}", url])
        .env_remove("CI_JOB_TOKEN")
        .env_remove("GITLAB_TOKEN")
        .env_remove("PRIVATE_TOKEN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = child.spawn().expect("spawn read-only curl probe");
    writeln!(
        child.stdin.take().expect("curl config stdin"),
        "header = \"JOB-TOKEN: {token}\""
    )
    .expect("write curl credential through stdin");
    let output = child.wait_with_output().expect("wait for curl probe");
    assert!(
        output.status.success(),
        "read-only curl probe transport failed"
    );
    String::from_utf8(output.stdout)
        .expect("curl status is UTF-8")
        .trim()
        .to_owned()
}

fn helper() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/create-release.sh")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn visit_files(root: &Path, visitor: &mut impl FnMut(&Path)) {
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            visit_files(&path, visitor);
        } else {
            visitor(&path);
        }
    }
}

const CURL_FIXTURE: &str = r##"#!/bin/sh
set -eu
config=$(cat)
case "$config" in
  *"JOB-TOKEN: $EXPECTED_TOKEN"*) : ;;
  *) exit 92 ;;
esac
argv="$*"
case "$argv" in
  *"$EXPECTED_TOKEN"*) exit 90 ;;
  *--location*|*--retry*) exit 91 ;;
esac

method=
output=
payload=
url=
while [ "$#" -gt 0 ]; do
  case "$1" in
    --request) shift; method="$1" ;;
    --output) shift; output="$1" ;;
    --data-binary) shift; payload=${1#@} ;;
    https://*) url="$1" ;;
  esac
  shift
done
[ -n "$method" ] && [ -n "$output" ] && [ -n "$url" ] || exit 93
dirname "$output" >>"$WORK_DIR_LOG"
if find "$FIXTURE_ROOT" -type f -exec grep -F -l -- "$EXPECTED_TOKEN" {} + | grep -q .; then
  exit 96
fi
printf '%s %s\n' "$method" "$url" >>"$REQUEST_LOG"
printf 'ARGV %s\n' "$argv" >>"$REQUEST_LOG"

status=200
body='[]'
case "$url" in
  */repository/tags/*)
    case "$SCENARIO" in
      missing-tag) status=404; body='{"message":"404 Tag Not Found"}' ;;
      wrong-sha) body='{"commit":{"id":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"}}' ;;
      redirect) status=302; body='' ;;
      malformed) body='not-json' ;;
      multi-tag-json) body="{\"commit\":{\"id\":\"$FIXTURE_SHA\"}}
{\"commit\":{\"id\":\"$FIXTURE_SHA\"}}" ;;
      *) body="{\"commit\":{\"id\":\"$FIXTURE_SHA\"}}" ;;
    esac
    ;;
  *'/releases?per_page=1')
    if [ "$SCENARIO" = unauthorized ]; then status=401; body='{"message":"Unauthorized"}'; fi
    if [ "$SCENARIO" = inaccessible ]; then status=403; body='{"message":"Forbidden"}'; fi
    namespace_count=$(grep -c '^GET .*releases?per_page=1' "$REQUEST_LOG")
    if [ "$SCENARIO" = multi-namespace-json ] ||
       { [ "$SCENARIO" = repeat-multi-namespace-json ] && [ "$namespace_count" -gt 1 ]; }; then
      body='{}
[]'
    fi
    ;;
  */releases/*)
    if [ -f "$STATE_FILE" ] || [ "$SCENARIO" = existing-release ]; then
      if [ -f "$STATE_FILE" ]; then
        body=$(cat "$STATE_FILE")
        if [ "$SCENARIO" = followup-mismatch ]; then
          body=$(printf '%s' "$body" | jq '.commit.id="dddddddddddddddddddddddddddddddddddddddd"')
        fi
        if [ "$SCENARIO" = followup-malformed ]; then body='not-json'; fi
        if [ "$SCENARIO" = followup-multi-json ]; then body="$body
{}"; fi
      else
        body='{"tag_name":"v9.9.9/rc"}'
      fi
      status=200
    else
      status=404
      release_count=$(grep -c '^GET .*releases/v' "$REQUEST_LOG")
      case "$SCENARIO" in
        release-404-malformed) body='not-json' ;;
        release-404-html) body='<html><body>not found</body></html>' ;;
        release-404-empty) body='' ;;
        release-404-truncated) body='{"message":"404 Release Not Found"' ;;
        release-404-wrong-json) body='{"message":"404 Project Not Found"}' ;;
        repeat-release-404-html)
          if [ "$release_count" -gt 1 ]; then
            body='<html><body>not found</body></html>'
          else
            body='{"message":"404 Release Not Found"}'
          fi
          ;;
        *) body='{"message":"404 Release Not Found"}' ;;
      esac
    fi
    ;;
  */releases)
    [ "$method" = POST ] || exit 94
    case "$SCENARIO" in
      conflict) status=409; body='{"message":"Release already exists"}' ;;
      *)
        body=$(jq --arg sha "$FIXTURE_SHA" '. + {commit:{id:$sha}}' "$payload")
        if [ "$SCENARIO" = post-mismatch ]; then
          body=$(printf '%s' "$body" | jq '.commit.id="cccccccccccccccccccccccccccccccccccccccc"')
        fi
        if [ "$SCENARIO" = post-reordered ]; then
          body=$(printf '%s' "$body" | jq '.assets.links |= reverse')
        fi
        if [ "$SCENARIO" = post-duplicate-link ]; then
          body=$(printf '%s' "$body" | jq '.assets.links[1] = .assets.links[0]')
        fi
        if [ "$SCENARIO" = post-missing-link ]; then
          body=$(printf '%s' "$body" | jq 'del(.assets.links[-1])')
        fi
        if [ "$SCENARIO" = post-wrong-url ]; then
          body=$(printf '%s' "$body" | jq '.assets.links[0].url = "https://example.invalid/wrong"')
        fi
        if [ "$SCENARIO" = post-wrong-link-type ]; then
          body=$(printf '%s' "$body" | jq '.assets.links[0].link_type = "other"')
        fi
        if [ "$SCENARIO" = post-malformed ]; then body='not-json'; fi
        if [ "$SCENARIO" = post-multi-json ]; then body="$body
{}"; fi
        printf '%s' "$body" >"$STATE_FILE"
        if [ "$SCENARIO" = lost-post ]; then exit 28; fi
        status=201
        ;;
    esac
    ;;
  *) exit 95 ;;
esac
printf '%s' "$body" >"$output"
printf '%s' "$status"
"##;
