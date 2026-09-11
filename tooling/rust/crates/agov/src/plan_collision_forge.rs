// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Fresh, bounded GET-only GitLab evidence through the operator's glab config.
//! API contracts: https://docs.gitlab.com/api/merge_requests/ and
//! https://docs.gitlab.com/api/repositories/#compare-commits-branches-and-tags
use super::Source;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    io::Read,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

pub(super) struct Report {
    pub inspected: usize,
    pub sources: Vec<Source>,
    pub unavailable: Vec<String>,
}

struct Response {
    body: Value,
    next: Option<String>,
}
trait Api {
    fn get(&mut self, path: &str) -> Result<Response, String>;
}
pub(super) struct Glab;
impl Api for Glab {
    fn get(&mut self, path: &str) -> Result<Response, String> {
        let mut child = Command::new("glab")
            .args(["api", "--method", "GET", "--include", path])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "glab unavailable; configure glab for this GitLab repository")?;
        let stdout = child.stdout.take().ok_or("glab stdout unavailable")?;
        let reader = std::thread::spawn(move || {
            let mut bytes = vec![];
            stdout
                .take(32 * 1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map(|_| bytes)
        });
        let start = Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Ok(status),
                Ok(None) if start.elapsed() < Duration::from_secs(30) => {
                    std::thread::sleep(Duration::from_millis(20))
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break Err("GitLab request failed or timed out");
                }
            }
        };
        let bytes = reader
            .join()
            .map_err(|_| "GitLab reader failed")?
            .map_err(|_| "GitLab response unreadable")?;
        if !status?.success() {
            return Err("GitLab GET failed (check glab authentication/network)".into());
        }
        if bytes.len() > 32 * 1024 * 1024 {
            return Err("GitLab response exceeds coverage limit".into());
        }
        let text = String::from_utf8(bytes)
            .map_err(|_| "GitLab response is not UTF-8")?
            .replace("\r\n", "\n");
        let (headers, body) = text
            .split_once("\n\n")
            .ok_or("GitLab response lacks HTTP headers")?;
        if headers
            .lines()
            .next()
            .and_then(|s| s.split_whitespace().nth(1))
            != Some("200")
        {
            return Err("GitLab response status is not 200".into());
        }
        let next = headers
            .lines()
            .filter_map(|s| s.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("x-next-page"))
            .map(|(_, v)| v.trim().into());
        Ok(Response {
            body: serde_json::from_str(body).map_err(|_| "GitLab returned invalid JSON")?,
            next,
        })
    }
}

fn sha(value: &Value) -> Result<&str, String> {
    value
        .as_str()
        .filter(|s| matches!(s.len(), 40 | 64) && s.bytes().all(|c| c.is_ascii_hexdigit()))
        .ok_or_else(|| "missing or malformed exact head SHA".into())
}

fn page(
    api: &mut impl Api,
    endpoint: &str,
    current: usize,
) -> Result<(Vec<Value>, Option<usize>), String> {
    let separator = if endpoint.contains('?') { '&' } else { '?' };
    let response = api.get(&format!("{endpoint}{separator}per_page=100&page={current}"))?;
    let rows = response
        .body
        .as_array()
        .ok_or("paginated response is not an array")?
        .clone();
    let next = match response.next.as_deref() {
        Some("") => None,
        Some(n) => {
            let n = n.parse::<usize>().map_err(|_| "invalid next-page header")?;
            if n != current + 1 || n > 10000 {
                return Err("incomplete or looping pagination".into());
            }
            Some(n)
        }
        None => return Err("pagination coverage unavailable: missing X-Next-Page".into()),
    };
    if next.is_some() && rows.is_empty() {
        return Err("empty page with continuation".into());
    }
    Ok((rows, next))
}

fn diff_paths(rows: &[Value]) -> Result<Vec<String>, String> {
    let mut paths = BTreeSet::new();
    for row in rows {
        if ["collapsed", "too_large", "overflow"]
            .iter()
            .any(|key| row[*key] == true)
        {
            return Err("diff response is collapsed or size-limited".into());
        }
        for key in ["old_path", "new_path"] {
            let path = row[key]
                .as_str()
                .filter(|s| !s.is_empty())
                .ok_or("diff path missing")?;
            paths.insert(path.into());
        }
    }
    Ok(paths.into_iter().collect())
}

fn mr(api: &mut impl Api, iid: u64, initial: &str, base: &str) -> Result<Source, String> {
    let endpoint = format!("projects/:id/merge_requests/{iid}");
    let mut expected = initial.to_string();
    let mut changes = 0;
    loop {
        let before = api.get(&endpoint)?.body;
        let head = sha(&before["sha"])?;
        if head != expected {
            changes += 1;
            if changes > 1 {
                return Err("MR head changed twice during scan".into());
            }
            expected = head.into();
        }
        if before["state"] != "opened" || sha(&before["diff_refs"]["head_sha"])? != expected {
            return Err("MR closed or diff head not current".into());
        }
        let evidence: Result<Vec<String>, String> = (|| {
            let count = before["changes_count"]
                .as_str()
                .and_then(|s| s.parse::<usize>().ok())
                .ok_or("MR changed-file count missing or size-limited")?;
            let mut rows = vec![];
            let mut number = 1;
            loop {
                let (mut chunk, next) = page(api, &format!("{endpoint}/diffs"), number)?;
                rows.append(&mut chunk);
                match next {
                    Some(n) => number = n,
                    None => break,
                }
            }
            if rows.len() != count {
                return Err("MR diff file count does not match metadata".into());
            }
            let _ = diff_paths(&rows)?;
            // Explicit base applies to forge comparisons too, not just local refs.
            // The commit response binds this comparison to the requested exact head.
            let comparison = api
                .get(&format!(
                    "projects/:id/repository/compare?from={base}&to={expected}&straight=false"
                ))?
                .body;
            if comparison["compare_timeout"] != false
                || comparison["overflow"] == true
                || sha(&comparison["commit"]["id"])? != expected
            {
                return Err("base comparison incomplete or not bound to requested head".into());
            }
            let compared = comparison["diffs"]
                .as_array()
                .ok_or("base comparison lacks diffs")?;
            if compared.len() < count {
                return Err(
                    "base comparison has fewer files than MR metadata; coverage unproven".into(),
                );
            }
            let paths = diff_paths(compared)?;
            Ok(paths)
        })();
        let after = api.get(&endpoint)?.body;
        let new_head = sha(&after["sha"])?;
        if new_head != expected {
            changes += 1;
            if changes > 1 {
                return Err("MR head changed twice during scan".into());
            }
            expected = new_head.into();
            continue;
        }
        if after["state"] != "opened"
            || after["diff_refs"] != before["diff_refs"]
            || after["changes_count"] != before["changes_count"]
        {
            return Err("MR diff metadata changed during scan".into());
        }
        return Ok(Source {
            label: format!("MR !{iid} @ {expected} (base {base})"),
            paths: evidence?,
        });
    }
}

pub(super) fn scan(api: &mut Glab, base: &str) -> Report {
    scan_with(api, base)
}
fn scan_with(api: &mut impl Api, base: &str) -> Report {
    let mut report = Report {
        inspected: 0,
        sources: vec![],
        unavailable: vec![],
    };
    let mut number = 1;
    let mut seen = BTreeSet::new();
    loop {
        let (rows, next) = match page(
            api,
            "projects/:id/merge_requests?state=opened&scope=all&order_by=created_at&sort=asc",
            number,
        ) {
            Ok(page) => page,
            Err(e) => {
                report
                    .unavailable
                    .push(format!("MR inventory page {number}: {e}"));
                break;
            }
        };
        for row in rows {
            let Some(iid) = row["iid"].as_u64().filter(|id| *id > 0) else {
                report
                    .unavailable
                    .push("MR inventory contains malformed IID".into());
                continue;
            };
            if !seen.insert(iid) {
                report
                    .unavailable
                    .push(format!("MR inventory repeated !{iid}; pagination unstable"));
                continue;
            }
            report.inspected += 1;
            match sha(&row["sha"]).and_then(|head| mr(api, iid, head, base)) {
                Ok(source) => report.sources.push(source),
                Err(e) => report.unavailable.push(format!("MR !{iid}: {e}")),
            }
        }
        match next {
            Some(n) => number = n,
            None => break,
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::VecDeque;
    const HEAD: &str = "1111111111111111111111111111111111111111";
    const NEW: &str = "2222222222222222222222222222222222222222";
    const THIRD: &str = "3333333333333333333333333333333333333333";
    const BASE: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    struct Mock {
        calls: Vec<String>,
        replies: VecDeque<Result<Response, String>>,
    }
    impl Api for Mock {
        fn get(&mut self, path: &str) -> Result<Response, String> {
            self.calls.push(path.into());
            self.replies.pop_front().expect("unexpected API call")
        }
    }
    fn response(body: Value, next: Option<&str>) -> Result<Response, String> {
        Ok(Response {
            body,
            next: next.map(Into::into),
        })
    }
    fn detail(head: &str) -> Value {
        json!({"sha":head,"state":"opened","changes_count":"1","diff_refs":{"head_sha":head,"base_sha":BASE}})
    }
    fn diff(path: &str) -> Value {
        json!({"old_path":path,"new_path":path,"diff":"@@ change","collapsed":false,"too_large":false})
    }
    fn comparison(head: &str) -> Value {
        json!({"commit":{"id":head},"compare_timeout":false,"diffs":[diff("new/file.rs")]})
    }
    fn happy(head: &str) -> Vec<Result<Response, String>> {
        vec![
            response(detail(head), None),
            response(json!([diff("new/file.rs")]), Some("")),
            response(comparison(head), None),
            response(detail(head), None),
        ]
    }
    fn mock(replies: Vec<Result<Response, String>>) -> Mock {
        Mock {
            calls: vec![],
            replies: replies.into(),
        }
    }

    #[test]
    fn pagination_crosses_one_hundred_mrs_and_uses_requested_base() {
        let first = (1..=100)
            .map(|iid| json!({"iid":iid,"sha":HEAD}))
            .collect::<Vec<_>>();
        let mut replies = vec![response(json!(first), Some("2"))];
        for _ in 0..100 {
            replies.extend(happy(HEAD));
        }
        replies.push(response(json!([{"iid":101,"sha":HEAD}]), Some("")));
        replies.extend(happy(HEAD));
        let mut api = mock(replies);
        let report = scan_with(&mut api, BASE);
        assert_eq!(report.inspected, 101);
        assert_eq!(report.sources.len(), 101);
        assert!(report.unavailable.is_empty());
        assert!(api
            .calls
            .iter()
            .any(|s| s.contains("scope=all") && s.ends_with("page=2")));
        assert_eq!(
            api.calls
                .iter()
                .filter(|s| s.contains(&format!("from={BASE}&to={HEAD}&straight=false")))
                .count(),
            101
        );
        assert!(api.replies.is_empty());
    }

    #[test]
    fn failed_mr_and_missing_pagination_are_unavailable() {
        let mut replies = vec![response(
            json!([{"iid":1,"sha":HEAD},{"iid":2,"sha":HEAD}]),
            Some(""),
        )];
        replies.extend(happy(HEAD));
        replies.push(Err("timeout".into()));
        let mut api = mock(replies);
        let report = scan_with(&mut api, BASE);
        assert_eq!(report.sources.len(), 1);
        assert_eq!(report.inspected, 2);
        assert!(report.unavailable[0].contains("MR !2: timeout"));
        let mut api = mock(vec![response(json!([]), None)]);
        assert!(scan_with(&mut api, BASE).unavailable[0].contains("pagination"));
    }

    #[test]
    fn changed_head_retries_once_and_never_returns_stale_paths() {
        let mut replies = happy(HEAD);
        replies[3] = response(detail(NEW), None);
        replies.extend(happy(NEW));
        let mut api = mock(replies);
        let source = mr(&mut api, 1, HEAD, BASE).unwrap();
        assert!(source.label.contains(NEW));
        assert!(!source.label.contains(HEAD));
        assert!(api.replies.is_empty());
        let mut replies = happy(HEAD);
        replies[3] = response(detail(NEW), None);
        let mut second = happy(NEW);
        second[3] = response(detail(THIRD), None);
        replies.extend(second);
        let mut api = mock(replies);
        assert!(mr(&mut api, 1, HEAD, BASE)
            .err()
            .unwrap()
            .contains("changed twice"));
    }

    #[test]
    fn collapsed_short_or_timed_out_diffs_cannot_claim_coverage() {
        for failure in 0..4 {
            let mut replies = happy(HEAD);
            match failure {
                0 => replies[1] = response(json!([]), Some("")),
                1 => {
                    let mut d = diff("new/file.rs");
                    d["collapsed"] = json!(true);
                    replies[1] = response(json!([d]), Some(""));
                }
                2 => {
                    let mut c = comparison(HEAD);
                    c["compare_timeout"] = json!(true);
                    replies[2] = response(c, None);
                }
                _ => {
                    let mut c = comparison(HEAD);
                    c["diffs"] = json!([]);
                    replies[2] = response(c, None);
                }
            }
            // Early diff failure skips comparison but still checks for a live push.
            if failure < 2 {
                let _ = replies.remove(2);
            }
            let mut api = mock(replies);
            assert!(mr(&mut api, 1, HEAD, BASE).is_err());
            assert!(api.replies.is_empty());
        }
    }

    #[test]
    fn diff_pagination_and_renames_retain_every_path() {
        let mut d = detail(HEAD);
        d["changes_count"] = json!("2");
        let renamed = json!({"old_path":"old.rs","new_path":"new.rs","renamed_file":true});
        let mut c = comparison(HEAD);
        c["diffs"] = json!([renamed.clone(), diff("other")]);
        let mut api = mock(vec![
            response(d.clone(), None),
            response(json!([renamed]), Some("2")),
            response(json!([diff("other")]), Some("")),
            response(c, None),
            response(d, None),
        ]);
        assert_eq!(
            mr(&mut api, 1, HEAD, BASE).unwrap().paths,
            vec!["new.rs", "old.rs", "other"]
        );
    }
}
