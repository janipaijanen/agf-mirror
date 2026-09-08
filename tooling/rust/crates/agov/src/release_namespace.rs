// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen

//! Read-only, fail-closed GitLab adapter for the standard pre-tag release gate.
//! No request other than GET and no git mutation is available to this module.

use clap::{Args, Subcommand};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const PAGE_SIZE: usize = 100;
const STATUSES: [&str; 6] = [
    "default",
    "hidden",
    "processing",
    "error",
    "pending_destruction",
    "deprecated",
];

#[derive(Subcommand)]
pub enum ReleaseCommand {
    /// Prove all four release namespaces absent, without creating a tag or artifact.
    NamespaceCheck(Options),
}

#[derive(Args)]
pub struct Options {
    /// Required checkpoint; pre-push also requires HEAD to equal the live target tip.
    #[arg(long, value_parser = ["mr", "preflight", "pre-push"])]
    phase: String,
    /// GitLab HTTPS API root (defaults to CI_API_V4_URL).
    #[arg(long)]
    api_url: Option<String>,
    /// Release project ID (defaults to CI_PROJECT_ID); never a registry repository ID.
    #[arg(long)]
    project_id: Option<String>,
    /// Expected canonical project path (defaults to CI_PROJECT_PATH).
    #[arg(long)]
    project_path: Option<String>,
    /// Publication registry base (defaults to CI_REGISTRY_IMAGE).
    #[arg(long)]
    registry_image: Option<String>,
    /// Release target branch (defaults to CI_MERGE_REQUEST_TARGET_BRANCH_NAME).
    #[arg(long)]
    target_branch: Option<String>,
}

pub fn run(command: ReleaseCommand) -> i32 {
    let ReleaseCommand::NamespaceCheck(options) = command;
    match execute(options) {
        Ok(evidence) => {
            println!(
                "{}",
                serde_json::to_string(&evidence).expect("serializable evidence")
            );
            0
        }
        Err(error) => {
            // Never render HTTP bodies, curl stderr, or credentials as diagnostics.
            eprintln!("result=BLOCKED {error}");
            1
        }
    }
}

#[derive(Serialize)]
struct Evidence {
    result: &'static str,
    phase: String,
    checked_at: String,
    project_id: u64,
    project_path: String,
    source_sha: String,
    target_sha: String,
    target_branch: String,
    pipeline_id: Option<u64>,
    version: String,
    tag: String,
    package: String,
    oci_reference: String,
    namespaces: BTreeMap<&'static str, &'static str>,
}

fn setting(value: Option<String>, env: &str) -> Result<String, String> {
    value
        .or_else(|| std::env::var(env).ok())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("configuration: supply {env} or its CLI option"))
}

fn number(s: &str) -> Result<u64, String> {
    s.parse::<u64>()
        .ok()
        .filter(|n| *n > 0)
        .ok_or_else(|| "identity: expected a positive numeric identifier".to_owned())
}

fn component(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 255
        && s != "."
        && s != ".."
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}

fn project_path(s: &str) -> bool {
    s.contains('/') && s.split('/').all(component)
}

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|_| "checkout: git unavailable".to_owned())?;
    if !out.status.success() {
        return Err("checkout: git inspection failed".into());
    }
    String::from_utf8(out.stdout)
        .map(|s| s.trim().to_owned())
        .map_err(|_| "checkout: invalid git output".to_owned())
}

fn sha(s: &str) -> bool {
    matches!(s.len(), 40 | 64) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

fn helper(root: &Path, version: &str, registry: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new("sh")
        .current_dir(root)
        .arg("tooling/rust/scripts/agov-oci-image.sh")
        .args(args)
        .env("AGOV_VERSION", version)
        .env("CI_REGISTRY_IMAGE", registry)
        .output()
        .map_err(|_| "oci: reference helper unavailable".to_owned())?;
    if !out.status.success() {
        return Err("oci: reference helper/verify-ref failed".into());
    }
    String::from_utf8(out.stdout)
        .map(|s| s.trim().to_owned())
        .map_err(|_| "oci: invalid helper output".to_owned())
}

fn execute(options: Options) -> Result<Evidence, String> {
    let root = std::path::PathBuf::from(git(Path::new("."), &["rev-parse", "--show-toplevel"])?);
    let source_sha = git(&root, &["rev-parse", "HEAD"])?;
    if !sha(&source_sha)
        || !git(
            &root,
            &["status", "--porcelain", "--untracked-files=normal"],
        )?
        .is_empty()
    {
        return Err(
            "checkout: clean committed worktree required for bound release evidence".into(),
        );
    }
    let api_url = setting(options.api_url, "CI_API_V4_URL")?;
    let id = number(&setting(options.project_id, "CI_PROJECT_ID")?)?;
    let path = setting(options.project_path, "CI_PROJECT_PATH")?;
    let registry = setting(options.registry_image, "CI_REGISTRY_IMAGE")?;
    let target_branch = setting(options.target_branch, "CI_MERGE_REQUEST_TARGET_BRANCH_NAME")?;
    if !project_path(&path)
        || target_branch.is_empty()
        || target_branch.chars().any(char::is_control)
    {
        return Err("identity: invalid project path or target branch".into());
    }
    let registry_path = registry
        .split_once('/')
        .map(|(_, p)| p)
        .filter(|p| *p == path)
        .ok_or("identity: registry base must match the release project path")?;
    if registry.contains('@')
        || registry.contains("://")
        || registry.chars().any(char::is_whitespace)
    {
        return Err("identity: invalid publication registry base".into());
    }
    let manifest_path = root.join("agf/dist/release-manifest.yaml");
    crate::dist::validate(&manifest_path, &root)
        .map_err(|_| "manifest: release manifest validation failed".to_owned())?;
    let manifest: serde_norway::Value = serde_norway::from_str(
        &std::fs::read_to_string(&manifest_path).map_err(|_| "manifest: unreadable")?,
    )
    .map_err(|_| "manifest: malformed YAML")?;
    let package = manifest["package"]["name"]
        .as_str()
        .filter(|s| component(s))
        .ok_or("manifest: invalid package name")?
        .to_owned();
    if manifest["package"]["version_file"].as_str() != Some("agf/VERSION") {
        return Err("manifest: version_file must be agf/VERSION for this release adapter".into());
    }
    let version = std::fs::read_to_string(root.join("agf/VERSION"))
        .map_err(|_| "version: unreadable")?
        .trim()
        .to_owned();
    if !component(&version) || !version.starts_with(|c: char| c.is_ascii_digit()) {
        return Err("version: unsafe or non-version value".into());
    }
    let tag = format!("v{version}");
    let oci_reference = helper(&root, &version, &registry, &["image-ref"])?;
    helper(&root, &version, &registry, &["verify-ref", &oci_reference])?;
    let (_, oci_path_tag) = oci_reference
        .split_once('/')
        .ok_or("oci: invalid derived reference")?;
    let (oci_path, oci_tag) = oci_path_tag.rsplit_once(':').ok_or("oci: tag missing")?;
    if !oci_path.starts_with(&format!("{registry_path}/"))
        || !project_path(oci_path)
        || !component(oci_tag)
    {
        return Err("oci: derived reference outside the publication repository".into());
    }
    let in_ci = std::env::var("CI").as_deref() == Ok("true");
    if (options.phase == "mr") != in_ci {
        return Err("phase: mr requires CI=true; operator phases must run outside CI".into());
    }
    let (header, token) = if options.phase == "mr" {
        (
            "JOB-TOKEN",
            std::env::var("CI_JOB_TOKEN").map_err(|_| "authentication: CI_JOB_TOKEN required")?,
        )
    } else {
        (
            "PRIVATE-TOKEN",
            std::env::var("AGOV_RELEASE_TOKEN")
                .map_err(|_| "authentication: operator AGOV_RELEASE_TOKEN required")?,
        )
    };
    let mut client = CurlClient::new(&api_url, header, token)?;
    let prefix = format!("/projects/{id}");
    let pipeline_id;
    if options.phase == "mr" {
        if std::env::var("CI_PIPELINE_SOURCE").as_deref() != Ok("merge_request_event")
            || std::env::var("CI_COMMIT_TAG").is_ok_and(|v| !v.is_empty())
        {
            return Err("phase: only a detached merge_request_event is eligible".into());
        }
        let iid = number(&setting(None, "CI_MERGE_REQUEST_IID")?)?;
        pipeline_id = Some(number(&setting(None, "CI_PIPELINE_ID")?)?);
        let mr = get_json(
            &mut client,
            &format!("{prefix}/merge_requests/{iid}"),
            "identity",
        )?;
        // The MR endpoint supports JOB-TOKEN; GET /projects/:id does not on
        // every supported GitLab. Its canonical full reference supplies the
        // independent path binding without introducing a PAT into CI.
        if mr["references"]["full"].as_str() != Some(&format!("{path}!{iid}"))
            || mr["source_project_id"].as_u64() != Some(id)
            || mr["target_project_id"].as_u64() != Some(id)
            || mr["sha"].as_str() != Some(&source_sha)
            || mr["target_branch"].as_str() != Some(&target_branch)
            || mr["state"].as_str() != Some("opened")
        {
            return Err("identity: MR project/path/source/target mismatch".into());
        }
        if setting(None, "CI_COMMIT_SHA")? != source_sha {
            return Err("identity: checkout differs from pipeline source SHA".into());
        }
    } else {
        pipeline_id = None;
        let project = get_json(&mut client, &prefix, "identity")?;
        if project["id"].as_u64() != Some(id)
            || project["path_with_namespace"].as_str() != Some(&path)
        {
            return Err("identity: release project/path mismatch".into());
        }
    }
    let branches = list(
        &mut client,
        &format!("{prefix}/repository/branches"),
        "identity",
    )?;
    let targets: Vec<_> = branches
        .iter()
        .filter(|b| b["name"].as_str() == Some(&target_branch))
        .collect();
    if targets.len() != 1 {
        return Err("identity: target branch missing or ambiguous".into());
    }
    let target_sha = targets[0]["commit"]["id"]
        .as_str()
        .filter(|s| sha(s))
        .ok_or("identity: malformed target commit")?
        .to_owned();
    if options.phase == "pre-push" && source_sha != target_sha {
        return Err("identity: pre-push HEAD must equal the live release target commit".into());
    }
    let oci_location = oci_reference
        .rsplit_once(':')
        .ok_or("oci: invalid location")?
        .0;
    check_namespaces(
        &mut client,
        &prefix,
        &tag,
        &package,
        oci_path,
        oci_tag,
        oci_location,
    )?;
    let final_branches = list(
        &mut client,
        &format!("{prefix}/repository/branches"),
        "identity",
    )?;
    let final_targets: Vec<_> = final_branches
        .iter()
        .filter(|b| b["name"].as_str() == Some(&target_branch))
        .collect();
    if final_targets.len() != 1
        || final_targets[0]["commit"]["id"].as_str() != Some(&target_sha)
        || git(&root, &["rev-parse", "HEAD"])? != source_sha
        || !git(
            &root,
            &["status", "--porcelain", "--untracked-files=normal"],
        )?
        .is_empty()
    {
        return Err("identity: checkout or target changed during namespace inspection".into());
    }
    Ok(Evidence {
        result: "ABSENT",
        phase: options.phase,
        checked_at: time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| "clock: invalid timestamp")?,
        project_id: id,
        project_path: path,
        source_sha,
        target_sha,
        target_branch,
        pipeline_id,
        version,
        tag,
        package,
        oci_reference,
        namespaces: ["git-tag", "release", "generic-package", "oci"]
            .into_iter()
            .map(|n| (n, "ABSENT"))
            .collect(),
    })
}

struct Response {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Value,
}
trait Api {
    fn get(&mut self, path: &str) -> Result<Response, String>;
}

struct CurlClient {
    base: String,
    auth: String,
    started: Instant,
}
impl CurlClient {
    fn new(base: &str, header: &str, token: String) -> Result<Self, String> {
        let base = base.trim_end_matches('/');
        let host = crate::provenance::parse_https_host(base)
            .map_err(|_| "configuration: API root must be HTTPS")?;
        if host.is_empty()
            || base.contains(['?', '#', '@', '\\', '"'])
            || base.chars().any(char::is_whitespace)
            || !base.ends_with("/api/v4")
        {
            return Err("configuration: invalid GitLab HTTPS API root".into());
        }
        if token.is_empty()
            || token
                .bytes()
                .any(|c| !c.is_ascii_graphic() || b"\"\\".contains(&c))
        {
            return Err("authentication: empty or invalid credential".into());
        }
        Ok(Self {
            base: base.into(),
            auth: format!("header = \"{header}: {token}\"\n"),
            started: Instant::now(),
        })
    }
}
impl Api for CurlClient {
    fn get(&mut self, path: &str) -> Result<Response, String> {
        if self.started.elapsed() > Duration::from_secs(120) {
            return Err("request budget exceeded".into());
        }
        let mut child = Command::new("curl")
            .args([
                "--disable",
                "--silent",
                "--proto",
                "=https",
                "--connect-timeout",
                "10",
                "--max-time",
                "20",
                "--max-filesize",
                "8388608",
                "--request",
                "GET",
                "--include",
                "--config",
                "-",
                "--url",
                &format!("{}{path}", self.base),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "curl unavailable".to_owned())?;
        let input_result = child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(self.auth.as_bytes());
        let out = child
            .wait_with_output()
            .map_err(|_| "HTTP transport failed")?;
        if input_result.is_err() || !out.status.success() {
            return Err("HTTP transport failed or timed out".into());
        }
        parse_response(&out.stdout)
    }
}

fn parse_response(bytes: &[u8]) -> Result<Response, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "invalid HTTP encoding")?;
    let mut rest = text;
    loop {
        let (head, body) = rest.split_once("\r\n\r\n").ok_or("missing HTTP headers")?;
        let mut lines = head.split("\r\n");
        let status_line = lines.next().ok_or("missing HTTP status")?;
        let mut parts = status_line.split_whitespace();
        if !parts.next().is_some_and(|s| s.starts_with("HTTP/")) {
            return Err("invalid HTTP status".into());
        }
        let status = parts
            .next()
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or("invalid HTTP status")?;
        // curl can emit a proxy CONNECT or interim header before the origin's
        // header. Never treat a redirect as such or follow its Location.
        if (status == 100 || status_line.contains("Connection established"))
            && body.starts_with("HTTP/")
        {
            rest = body;
            continue;
        }
        let mut headers = BTreeMap::new();
        for line in lines {
            let (name, value) = line.split_once(':').ok_or("malformed HTTP header")?;
            let name = name.to_ascii_lowercase();
            if name.starts_with("x-") && headers.contains_key(&name) {
                return Err("duplicate pagination/identity header".into());
            }
            headers.insert(name, value.trim().to_owned());
        }
        // Only success and object-not-found can proceed; all other HTTP
        // statuses fail even if their response body happens to be valid JSON.
        if status != 200 && status != 404 {
            return Err(format!("HTTP {status}"));
        }
        let body = serde_json::from_str(body).map_err(|_| "malformed JSON response")?;
        return Ok(Response {
            status,
            headers,
            body,
        });
    }
}

fn enc(s: &str) -> String {
    s.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
                (b as char).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}

fn get_json(api: &mut impl Api, path: &str, namespace: &str) -> Result<Value, String> {
    let response = api
        .get(path)
        .map_err(|e| format!("namespace={namespace} ambiguity: {e}"))?;
    if response.status != 200 || !response.body.is_object() {
        return Err(format!(
            "namespace={namespace} ambiguity: positive access not established"
        ));
    }
    Ok(response.body)
}

fn list(api: &mut impl Api, path: &str, namespace: &str) -> Result<Vec<Value>, String> {
    let mut all = Vec::new();
    let mut seen = BTreeSet::new();
    for page in 1..=1000 {
        let separator = if path.contains('?') { '&' } else { '?' };
        let response = api
            .get(&format!(
                "{path}{separator}per_page={PAGE_SIZE}&page={page}"
            ))
            .map_err(|e| format!("namespace={namespace} ambiguity: {e}"))?;
        let fail =
            || format!("namespace={namespace} ambiguity: incomplete/inaccessible pagination");
        if response.status != 200 {
            return Err(fail());
        }
        let values = response.body.as_array().ok_or_else(fail)?;
        let h = &response.headers;
        if h.get("x-page") != Some(&page.to_string())
            || h.get("x-per-page") != Some(&PAGE_SIZE.to_string())
            || !h.contains_key("x-next-page")
            || values.len() > PAGE_SIZE
        {
            return Err(fail());
        }
        for value in values {
            if !seen.insert(value.to_string()) {
                return Err(fail());
            }
        }
        if values.iter().any(|v| !v.is_object()) {
            return Err(fail());
        }
        all.extend(values.iter().cloned());
        let next = &h["x-next-page"];
        if let Some(links) = h.get("link") {
            let next_links: Vec<_> = links
                .split(',')
                .filter(|l| l.contains("rel=\"next\""))
                .collect();
            if next_links.len() > 1
                || (next.is_empty() && !next_links.is_empty())
                || (!next.is_empty() && next_links.is_empty())
            {
                return Err(fail());
            }
            if let Some(link) = next_links.first() {
                let url = link
                    .trim()
                    .split_once('>')
                    .ok_or_else(fail)?
                    .0
                    .trim_start_matches('<');
                let next_query = url.split_once('?').ok_or_else(fail)?.1;
                let pages: Vec<_> = next_query
                    .split('&')
                    .filter_map(|p| p.strip_prefix("page="))
                    .collect();
                if pages != [next.as_str()] {
                    return Err(fail());
                }
            }
        }
        if next.is_empty() {
            if h.get("x-total")
                .is_some_and(|s| s.parse::<usize>().ok() != Some(all.len()))
            {
                return Err(fail());
            }
            if h.get("x-total-pages")
                .is_some_and(|s| s.parse::<usize>().ok() != Some(page) && !(page == 1 && s == "0"))
            {
                return Err(fail());
            }
            return Ok(all);
        }
        if values.is_empty() || next.parse::<usize>().ok() != Some(page + 1) {
            return Err(fail());
        }
    }
    Err(format!(
        "namespace={namespace} ambiguity: page budget exceeded"
    ))
}

fn object_absent(api: &mut impl Api, path: &str, namespace: &str) -> Result<(), String> {
    let response = api
        .get(path)
        .map_err(|e| format!("namespace={namespace} ambiguity: {e}"))?;
    match response.status {
        404 if response.body.is_object() => Ok(()),
        200 => Err(format!("namespace={namespace} occupied")),
        _ => Err(format!(
            "namespace={namespace} ambiguity: invalid object response"
        )),
    }
}

fn check_namespaces(
    api: &mut impl Api,
    prefix: &str,
    tag: &str,
    package: &str,
    oci_path: &str,
    oci_tag: &str,
    oci_location: &str,
) -> Result<(), String> {
    for (namespace, endpoint, key) in [
        ("git-tag", "repository/tags", "name"),
        ("release", "releases", "tag_name"),
    ] {
        let path = format!("{prefix}/{endpoint}");
        let objects = list(api, &path, namespace)?;
        for object in objects {
            let name = object[key]
                .as_str()
                .ok_or_else(|| format!("namespace={namespace} ambiguity: missing identity"))?;
            if name == tag {
                return Err(format!("namespace={namespace} occupied"));
            }
        }
        object_absent(api, &format!("{path}/{}", enc(tag)), namespace)?;
    }
    for status in STATUSES {
        let path = format!(
            "{prefix}/packages?package_type=generic&package_name={}&status={status}",
            enc(package)
        );
        for object in list(api, &path, "generic-package")? {
            let name = object["name"]
                .as_str()
                .ok_or("namespace=generic-package ambiguity: missing name")?;
            let version = object["version"]
                .as_str()
                .ok_or("namespace=generic-package ambiguity: missing version")?;
            let kind = object["package_type"]
                .as_str()
                .ok_or("namespace=generic-package ambiguity: missing type")?;
            if name == package && version == tag && kind == "generic" {
                return Err(format!(
                    "namespace=generic-package occupied status={status}"
                ));
            }
        }
    }
    let mut repositories = Vec::new();
    for repo in list(api, &format!("{prefix}/registry/repositories"), "oci")? {
        let path = repo["path"]
            .as_str()
            .ok_or("namespace=oci ambiguity: missing repository path")?;
        if path == oci_path {
            if repo["location"].as_str() != Some(oci_location) {
                return Err("namespace=oci ambiguity: repository location mismatch".into());
            }
            repositories.push(repo);
        }
    }
    if repositories.len() > 1 {
        return Err("namespace=oci ambiguity: duplicate exact repository".into());
    }
    if let Some(repo) = repositories.first() {
        let id = repo["id"]
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("namespace=oci ambiguity: missing repository id")?;
        // Establish access to this exact repository's tag collection as well
        // as project repository discovery; their visibility may differ.
        let path = format!("{prefix}/registry/repositories/{id}/tags");
        for object in list(api, &path, "oci")? {
            let name = object["name"]
                .as_str()
                .ok_or("namespace=oci ambiguity: missing tag name")?;
            if name == oci_tag {
                return Err("namespace=oci occupied".into());
            }
        }
        object_absent(api, &format!("{path}/{}", enc(oci_tag)), "oci")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Default)]
    struct Fake {
        responses: BTreeMap<String, Response>,
        calls: Vec<String>,
    }
    fn page(values: Value, current: usize, next: &str) -> Response {
        Response {
            status: 200,
            body: values,
            headers: [
                ("x-page".into(), current.to_string()),
                ("x-per-page".into(), "100".into()),
                ("x-next-page".into(), next.into()),
            ]
            .into(),
        }
    }
    impl Api for Fake {
        fn get(&mut self, path: &str) -> Result<Response, String> {
            self.calls.push(path.into());
            if let Some(response) = self.responses.remove(path) {
                return Ok(response);
            }
            if path.contains("per_page=") {
                let body = if path.contains("registry/repositories?") {
                    json!([{"id":99,"path":"fixture/project/agov","location":"registry.fixture.invalid/fixture/project/agov"}])
                } else {
                    json!([])
                };
                Ok(page(body, 1, ""))
            } else {
                Ok(Response {
                    status: 404,
                    headers: BTreeMap::new(),
                    body: json!({"message":"Not Found"}),
                })
            }
        }
    }
    fn check(fake: &mut Fake) -> Result<(), String> {
        check_namespaces(
            fake,
            "/projects/42",
            "v9.8.7",
            "agf",
            "fixture/project/agov",
            "9.8.7",
            "registry.fixture.invalid/fixture/project/agov",
        )
    }
    #[test]
    fn authorized_absence_walks_all_statuses_and_exact_unprefixed_oci_tag() {
        let mut fake = Fake::default();
        check(&mut fake).unwrap();
        for status in STATUSES {
            assert!(fake
                .calls
                .iter()
                .any(|p| p.contains(&format!("status={status}&"))));
        }
        assert!(fake
            .calls
            .contains(&"/projects/42/registry/repositories/99/tags/9.8.7".into()));
        assert!(!fake
            .calls
            .iter()
            .any(|p| p.ends_with("/tags/v9.8.7") && p.contains("registry")));
    }
    #[test]
    fn every_occupied_namespace_blocks_before_publication() {
        for (endpoint, identity, namespace) in [
            ("repository/tags", json!({"name":"v9.8.7"}), "git-tag"),
            ("releases", json!({"tag_name":"v9.8.7"}), "release"),
            (
                "packages?package_type=generic&package_name=agf&status=pending_destruction",
                json!({"name":"agf","version":"v9.8.7","package_type":"generic"}),
                "generic-package",
            ),
            (
                "registry/repositories/99/tags",
                json!({"name":"9.8.7"}),
                "oci",
            ),
        ] {
            let mut fake = Fake::default();
            let sep = if endpoint.contains('?') { '&' } else { '?' };
            fake.responses.insert(
                format!("/projects/42/{endpoint}{sep}per_page=100&page=1"),
                page(json!([identity]), 1, ""),
            );
            let error = check(&mut fake).unwrap_err();
            assert!(
                error.contains(&format!("namespace={namespace} occupied")),
                "{error}"
            );
            println!("negative namespace={namespace}: {error}; publication not invoked");
        }
    }
    #[test]
    fn nondefault_package_on_later_page_blocks_but_fuzzy_match_does_not() {
        let path = "/projects/42/packages?package_type=generic&package_name=agf&status=hidden&per_page=100&page=";
        let mut fake = Fake::default();
        let fuzzy = json!({"name":"other-agf","version":"v9.8.7","package_type":"generic"});
        fake.responses
            .insert(format!("{path}1"), page(json!([fuzzy]), 1, "2"));
        fake.responses.insert(
            format!("{path}2"),
            page(
                json!([{"name":"agf","version":"v9.8.7","package_type":"generic"}]),
                2,
                "",
            ),
        );
        assert!(check(&mut fake)
            .unwrap_err()
            .contains("occupied status=hidden"));
        let mut fake = Fake::default();
        fake.responses
            .insert(format!("{path}1"), page(json!([fuzzy]), 1, ""));
        check(&mut fake).unwrap();
    }
    #[test]
    fn denied_parent_or_missing_pagination_never_means_absent() {
        for namespace in [
            "repository/tags",
            "releases",
            "registry/repositories",
            "registry/repositories/99/tags",
        ] {
            for status in [401, 403, 404, 429, 500] {
                let mut fake = Fake::default();
                fake.responses.insert(
                    format!("/projects/42/{namespace}?per_page=100&page=1"),
                    Response {
                        status,
                        body: json!([]),
                        headers: BTreeMap::new(),
                    },
                );
                assert!(check(&mut fake).unwrap_err().contains("ambiguity"));
            }
        }
        let mut fake = Fake::default();
        fake.responses.insert(
            "/projects/42/repository/tags?per_page=100&page=1".into(),
            Response {
                status: 200,
                body: json!([]),
                headers: BTreeMap::new(),
            },
        );
        assert!(check(&mut fake).is_err());
    }
    #[test]
    fn missing_fields_and_duplicate_repositories_fail_closed() {
        for values in [
            json!([{}]),
            json!([{"id":99,"path":"fixture/project/agov","location":"registry.fixture.invalid/fixture/project/agov"},{"id":100,"path":"fixture/project/agov","location":"registry.fixture.invalid/fixture/project/agov"}]),
        ] {
            let mut fake = Fake::default();
            fake.responses.insert(
                "/projects/42/registry/repositories?per_page=100&page=1".into(),
                page(values, 1, ""),
            );
            assert!(check(&mut fake).unwrap_err().contains("ambiguity"));
        }
    }
    #[test]
    fn pagination_loops_and_partial_walks_fail_closed() {
        for next in ["1", "3", "not-a-number"] {
            let mut fake = Fake::default();
            fake.responses.insert(
                "/projects/42/repository/tags?per_page=100&page=1".into(),
                page(json!([{"name":"other"}]), 1, next),
            );
            assert!(check(&mut fake).is_err());
        }
        let mut fake = Fake::default();
        fake.responses.insert(
            "/projects/42/repository/tags?per_page=100&page=1".into(),
            page(json!([{"name":"other"}]), 1, "2"),
        );
        // The default response to page 2 claims page 1, which must fail.
        assert!(check(&mut fake).is_err());
    }
    #[test]
    fn http_errors_malformed_json_and_redirects_fail_without_echoing_body() {
        for raw in [
            "HTTP/2 200 OK\r\n\r\nnot-json-secret",
            "HTTP/2 403 Forbidden\r\n\r\n{\"secret\":1}",
            "HTTP/2 302 Found\r\nLocation: https://elsewhere.invalid\r\n\r\n{}",
            "HTTP/2 200 OK\r\nx-page: 1\r\nx-page: 2\r\n\r\n[]",
        ] {
            let err = parse_response(raw.as_bytes()).err().unwrap();
            assert!(!err.contains("secret"));
        }
        assert_eq!(
            parse_response(b"HTTP/1.1 200 Connection established\r\n\r\nHTTP/2 200 OK\r\n\r\n[]")
                .unwrap()
                .status,
            200
        );
    }
    #[test]
    fn credential_and_origin_injection_are_rejected() {
        for token in ["", "abc\nurl=https://elsewhere.invalid", "abc\"", "abc\\"] {
            assert!(
                CurlClient::new("https://gitlab.example/api/v4", "JOB-TOKEN", token.into())
                    .is_err()
            );
        }
        for base in [
            "http://gitlab.example/api/v4",
            "https://user@gitlab.example/api/v4",
            "https://gitlab.example/api/v4?token=bad",
        ] {
            assert!(CurlClient::new(base, "JOB-TOKEN", "safe-placeholder".into()).is_err());
        }
    }
}
