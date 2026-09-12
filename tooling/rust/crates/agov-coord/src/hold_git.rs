//! Read-only, explicitly scoped Git snapshot inventory for publication holds.
//! Resolve all requested refs before reading any channel. Never read a working
//! tree, enumerate candidate branches implicitly, or contact a remote.
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub source_id: String,
    pub requested_refs: Vec<String>,
    pub sha: Option<String>,
    pub is_target: bool,
    pub available: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blob {
    pub source_id: String,
    pub path: String,
    pub oid: String,
    pub bytes: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadProblem {
    pub code: &'static str,
    pub operation: &'static str,
    pub requested_ref: Option<String>,
    pub sha: Option<String>,
    pub path: Option<String>,
    pub source_id: String,
    pub message: String,
}
#[derive(Debug, Default)]
pub struct Inventory {
    pub target_ref: Option<String>,
    pub target_branch: Option<String>,
    pub target_remote: Option<String>,
    pub target_sha: Option<String>,
    pub sources: Vec<Source>,
    pub blobs: Vec<Blob>,
    pub problems: Vec<ReadProblem>,
}

pub(crate) trait Git {
    fn run(&self, args: &[&str], input: Option<Vec<u8>>) -> Result<Vec<u8>, String>;
}
struct LocalGit<'a>(&'a Path);
impl Git for LocalGit<'_> {
    fn run(&self, args: &[&str], input: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut child = Command::new("git")
            .current_dir(self.0)
            .args(args)
            .env("GIT_TERMINAL_PROMPT", "0")
            .stdin(if input.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        // Feed batch input on another thread so large inventories cannot deadlock
        // while Git writes results before it has consumed the complete request.
        let writer = input.map(|bytes| {
            let mut stdin = child.stdin.take().expect("piped stdin");
            std::thread::spawn(move || stdin.write_all(&bytes))
        });
        let output = child.wait_with_output().map_err(|e| e.to_string());
        let written = writer
            .map(|w| {
                w.join()
                    .map_err(|_| "Git input writer panicked".to_string())
                    .and_then(|r| r.map_err(|e| e.to_string()))
            })
            .transpose();
        let output = output?;
        written?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
        }
        // Git may return success while warning that a requested revision is
        // ambiguous. Never accept such a resolution as deterministic evidence.
        if output
            .stderr
            .windows(b"ambiguous".len())
            .any(|w| w == b"ambiguous")
        {
            return Err("ambiguous Git reference".into());
        }
        Ok(output.stdout)
    }
}

pub fn read_inventory(cwd: &Path, target: &str, candidates: &[String], channel: &str) -> Inventory {
    read_with_git(&LocalGit(cwd), target, candidates, channel)
}

fn text(bytes: Vec<u8>) -> Result<String, String> {
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
fn sha(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn valid_channel(channel: &str) -> bool {
    !channel.is_empty()
        && !channel.contains('\\')
        && !channel.contains(':')
        && !channel.contains('\0')
        && channel
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn resolve_target(
    git: &dyn Git,
    target: &str,
) -> Result<(String, String, Option<String>, String), String> {
    if target == "HEAD" || target.ends_with("/HEAD") || target.is_empty() || target.starts_with('-')
    {
        return Err("target must identify an unambiguous nonsymbolic branch".into());
    }
    let remotes = text(git.run(&["remote"], None)?)?;
    let refs = text(git.run(
        &[
            "for-each-ref",
            "--format=%(refname)%09%(objectname)%09%(symref)",
            "refs/heads",
            "refs/remotes",
        ],
        None,
    )?)?;
    let mut matches = Vec::new();
    for line in refs.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err("malformed branch inventory".into());
        }
        let full = fields[0];
        let (logical, remote, short) = if let Some(name) = full.strip_prefix("refs/heads/") {
            (format!("refs/heads/{name}"), None, name.to_owned())
        } else if let Some(tail) = full.strip_prefix("refs/remotes/") {
            let possible: Vec<_> = remotes
                .lines()
                .filter_map(|r| tail.strip_prefix(&format!("{r}/")).map(|n| (r, n)))
                .collect();
            if possible.len() != 1 {
                if target == full || target == tail {
                    return Err("ambiguous or unconfigured remote prefix".into());
                }
                continue;
            }
            let (remote, name) = possible[0];
            (
                format!("refs/heads/{name}"),
                Some(remote.to_owned()),
                tail.to_owned(),
            )
        } else {
            continue;
        };
        if target != full && target != short {
            continue;
        }
        if !fields[2].is_empty()
            || !crate::publication_hold::canonical_branch(&logical)
            || !sha(fields[1])
        {
            return Err("symbolic, invalid, or unsupported target branch".into());
        }
        matches.push((full.to_owned(), logical, remote, fields[1].to_owned()));
    }
    if matches.len() == 1 {
        Ok(matches.remove(0))
    } else {
        Err("target is missing or ambiguous; use a full branch ref".into())
    }
}
fn resolve_candidate(git: &dyn Git, reference: &str) -> Result<String, String> {
    if reference.is_empty()
        || reference.starts_with('-')
        || reference.contains('\0')
        || reference.contains('\n')
    {
        return Err("invalid candidate reference".into());
    }
    let oid = text(git.run(
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{reference}^{{commit}}"),
        ],
        None,
    )?)?;
    let oid = oid.trim();
    if sha(oid) {
        Ok(oid.to_owned())
    } else {
        Err("unsupported or malformed commit object ID".into())
    }
}

pub(crate) fn read_with_git(
    git: &dyn Git,
    target: &str,
    candidates: &[String],
    channel: &str,
) -> Inventory {
    let mut out = Inventory::default();
    let mut target_source = Source {
        source_id: "target".into(),
        requested_refs: vec![target.into()],
        sha: None,
        is_target: true,
        available: false,
    };
    match resolve_target(git, target) {
        Ok((full, logical, remote, oid)) => {
            out.target_ref = Some(full);
            out.target_branch = Some(logical);
            out.target_remote = remote;
            out.target_sha = Some(oid.clone());
            target_source.sha = Some(oid);
        }
        Err(message) => out.problems.push(ReadProblem {
            code: "invalid-input",
            operation: "resolve-target",
            requested_ref: Some(target.into()),
            sha: None,
            path: None,
            source_id: "target".into(),
            message,
        }),
    }
    out.sources.push(target_source);
    let default = vec!["HEAD".into()];
    let candidates = if candidates.is_empty() {
        &default
    } else {
        candidates
    };
    let mut resolved: BTreeMap<String, Source> = BTreeMap::new();
    for reference in candidates {
        match resolve_candidate(git, reference) {
            Ok(oid) => {
                let id = format!("candidate:{oid}");
                let source = resolved.entry(id.clone()).or_insert(Source {
                    source_id: id,
                    requested_refs: Vec::new(),
                    sha: Some(oid),
                    is_target: false,
                    available: false,
                });
                source.requested_refs.push(reference.clone());
            }
            Err(message) => {
                let id = format!("unresolved:{reference}");
                out.problems.push(ReadProblem {
                    code: "invalid-input",
                    operation: "resolve-candidate",
                    requested_ref: Some(reference.clone()),
                    sha: None,
                    path: None,
                    source_id: id.clone(),
                    message,
                });
                resolved.entry(id.clone()).or_insert(Source {
                    source_id: id,
                    requested_refs: vec![reference.clone()],
                    sha: None,
                    is_target: false,
                    available: false,
                });
            }
        }
    }
    out.sources.extend(resolved.into_values());
    out.sources.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    // All ref resolution above completes before the first tree/blob read.
    for source in &mut out.sources {
        source.requested_refs.sort();
        source.requested_refs.dedup();
        let Some(oid) = source.sha.as_deref() else {
            continue;
        };
        let result = if valid_channel(channel) {
            read_channel(git, oid, channel, &source.source_id)
        } else {
            Err((
                "invalid-input",
                "read-tree",
                channel.to_owned(),
                "channel must be a normalized repository-relative Git tree path".into(),
            ))
        };
        match result {
            Ok(blobs) => {
                source.available = true;
                out.blobs.extend(blobs);
            }
            Err((code, operation, path, message)) => out.problems.push(ReadProblem {
                code,
                operation,
                requested_ref: None,
                sha: Some(oid.to_owned()),
                path: Some(path),
                source_id: source.source_id.clone(),
                message,
            }),
        }
    }
    out.blobs
        .sort_by(|a, b| (&a.source_id, &a.path, &a.oid).cmp(&(&b.source_id, &b.path, &b.oid)));
    out
}

type ReadError = (&'static str, &'static str, String, String);
fn tree(git: &dyn Git, oid: &str) -> Result<Vec<(String, String, String)>, String> {
    let bytes = git.run(&["ls-tree", "-z", oid], None)?;
    bytes
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|entry| {
            let tab = entry
                .iter()
                .position(|b| *b == b'\t')
                .ok_or("malformed Git tree entry")?;
            let head =
                std::str::from_utf8(&entry[..tab]).map_err(|_| "non-UTF8 Git tree header")?;
            let h: Vec<_> = head.split(' ').collect();
            if h.len() != 3 || !sha(h[2]) {
                return Err("malformed Git tree metadata".into());
            }
            let name =
                std::str::from_utf8(&entry[tab + 1..]).map_err(|_| "non-UTF8 Git tree path")?;
            Ok((h[0].to_owned(), h[2].to_owned(), name.to_owned()))
        })
        .collect()
}
fn read_channel(
    git: &dyn Git,
    sha: &str,
    channel: &str,
    source: &str,
) -> Result<Vec<Blob>, ReadError> {
    let mut oid = sha.to_owned();
    let mut prefix = String::new();
    for part in channel.split('/') {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(part);
        let entries =
            tree(git, &oid).map_err(|e| ("git-read-failed", "read-tree", prefix.clone(), e))?;
        let (mode, next, _) = entries
            .into_iter()
            .find(|(_, _, n)| n == part)
            .ok_or_else(|| {
                (
                    "git-read-failed",
                    "read-tree",
                    prefix.clone(),
                    "requested channel tree is missing".into(),
                )
            })?;
        if mode != "040000" {
            return Err((
                "nonregular-entry",
                "read-tree",
                prefix,
                "channel component is not a Git tree".into(),
            ));
        }
        oid = next;
    }
    let entries =
        tree(git, &oid).map_err(|e| ("git-read-failed", "read-tree", channel.into(), e))?;
    let mut selected = Vec::new();
    for (mode, oid, name) in entries {
        let path = format!("{channel}/{name}");
        if mode != "100644" && mode != "100755" {
            return Err((
                "nonregular-entry",
                "read-tree",
                path,
                "nonregular channel entry".into(),
            ));
        }
        if name.ends_with(".md") {
            selected.push((path, oid));
        }
    }
    if selected.is_empty() {
        return Ok(Vec::new());
    }
    let input = selected
        .iter()
        .map(|(_, oid)| format!("{oid}\n"))
        .collect::<String>()
        .into_bytes();
    let output = git
        .run(&["cat-file", "--batch"], Some(input))
        .map_err(|e| ("git-read-failed", "read-blob", channel.into(), e))?;
    let mut cursor = 0;
    let mut blobs = Vec::new();
    for (path, oid) in selected {
        let parse = || -> Result<(Vec<u8>, usize), String> {
            let rest = output.get(cursor..).ok_or("truncated batch response")?;
            let end = rest
                .iter()
                .position(|b| *b == b'\n')
                .ok_or("missing batch header")?;
            let h = std::str::from_utf8(&rest[..end])
                .map_err(|_| "invalid batch header")?
                .split(' ')
                .collect::<Vec<_>>();
            if h.len() != 3 || h[0] != oid || h[1] != "blob" {
                return Err("missing or mismatched batch blob".into());
            }
            let len = h[2].parse::<usize>().map_err(|_| "invalid blob length")?;
            let begin = end + 1;
            let finish = begin.checked_add(len).ok_or("blob size overflow")?;
            let bytes = rest.get(begin..finish).ok_or("truncated blob")?.to_vec();
            if rest.get(finish) != Some(&b'\n') {
                return Err("missing blob delimiter".into());
            }
            Ok((bytes, cursor + finish + 1))
        };
        let (bytes, next) =
            parse().map_err(|e| ("git-read-failed", "read-blob", path.clone(), e))?;
        cursor = next;
        blobs.push(Blob {
            source_id: source.into(),
            path,
            oid,
            bytes,
        });
    }
    if cursor != output.len() {
        return Err((
            "git-read-failed",
            "read-blob",
            channel.into(),
            "unexpected batch output".into(),
        ));
    }
    Ok(blobs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    fn run(p: &Path, args: &[&str]) -> String {
        text(LocalGit(p).run(args, None).unwrap())
            .unwrap()
            .trim()
            .into()
    }
    fn fixture() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        let p = d.path();
        run(p, &["init", "-q", "-b", "main"]);
        run(p, &["config", "user.email", "test@example.invalid"]);
        run(p, &["config", "user.name", "Test"]);
        std::fs::create_dir_all(p.join("docs/coordination")).unwrap();
        std::fs::write(
            p.join("docs/coordination/base.md"),
            b"base\r\n\0exact bytes",
        )
        .unwrap();
        run(p, &["add", "."]);
        run(p, &["commit", "-qm", "base"]);
        run(
            p,
            &["remote", "add", "origin", "https://example.invalid/repo"],
        );
        run(p, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        run(p, &["checkout", "-qb", "candidate"]);
        std::fs::write(p.join("docs/coordination/new.md"), b"new bytes\n").unwrap();
        run(p, &["add", "."]);
        run(p, &["commit", "-qm", "candidate"]);
        d
    }
    #[test]
    fn target_identity_candidates_and_raw_bytes_are_pinned() {
        let d = fixture();
        let p = d.path();
        let oid = run(p, &["rev-parse", "HEAD"]);
        let out = read_inventory(
            p,
            "origin/main",
            &["candidate".into(), oid.clone()],
            "docs/coordination",
        );
        assert!(out.problems.is_empty(), "{:?}", out.problems);
        assert_eq!(out.target_branch.as_deref(), Some("refs/heads/main"));
        assert_eq!(out.target_ref.as_deref(), Some("refs/remotes/origin/main"));
        assert_eq!(out.target_remote.as_deref(), Some("origin"));
        assert_eq!(out.sources.len(), 2);
        let candidate = out.sources.iter().find(|s| !s.is_target).unwrap();
        let mut expected_labels = vec![oid, "candidate".into()];
        expected_labels.sort();
        assert_eq!(candidate.requested_refs, expected_labels);
        assert_eq!(out.blobs.len(), 3);
        assert_eq!(
            out.blobs
                .iter()
                .find(|b| b.source_id == "target")
                .unwrap()
                .bytes,
            b"base\r\n\0exact bytes"
        );
        std::fs::write(p.join("docs/coordination/new.md"), "dirty replacement").unwrap();
        std::fs::write(p.join("docs/coordination/untracked.md"), "untracked").unwrap();
        let after = read_inventory(p, "origin/main", &["candidate".into()], "docs/coordination");
        assert_eq!(after.blobs, out.blobs);
    }
    #[test]
    fn target_rejects_aliases_tags_shas_expressions_and_ambiguous_branches() {
        let d = fixture();
        let p = d.path();
        let oid = run(p, &["rev-parse", "main"]);
        run(p, &["tag", "release", "main"]);
        run(
            p,
            &[
                "symbolic-ref",
                "refs/remotes/origin/HEAD",
                "refs/remotes/origin/main",
            ],
        );
        run(p, &["branch", "origin/main", "main"]);
        for target in [
            "HEAD",
            "origin/HEAD",
            "release",
            &oid,
            "main~0",
            "origin/main",
        ] {
            let out = read_inventory(p, target, &[], "docs/coordination");
            assert!(out.target_sha.is_none(), "{target}");
            assert!(!out.problems.is_empty());
        }
        for target in ["main", "refs/heads/main", "refs/remotes/origin/main"] {
            assert!(
                read_inventory(p, target, &[], "docs/coordination")
                    .target_sha
                    .is_some(),
                "{target}"
            );
        }
        let tagged = read_inventory(p, "main", &["release".into()], "docs/coordination");
        assert!(tagged.problems.is_empty());
    }
    #[test]
    fn invalid_channels_missing_trees_and_nonregular_entries_fail_closed() {
        let d = fixture();
        let p = d.path();
        for channel in [
            "",
            "/docs/coordination",
            "docs//coordination",
            "docs/../coordination",
            "./docs/coordination",
            "missing",
        ] {
            let out = read_inventory(p, "main", &[], channel);
            assert!(out.sources.iter().all(|s| !s.available), "{channel}");
            assert!(!out.problems.is_empty());
        }
        let oid = run(p, &["hash-object", "-w", "--stdin"]);
        run(
            p,
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("120000,{oid},docs/coordination/link.md"),
            ],
        );
        run(p, &["commit", "-qm", "symlink"]);
        let out = read_inventory(p, "main", &[], "docs/coordination");
        assert!(out.problems.iter().any(|p| p.code == "nonregular-entry"));
        assert!(out.sources.iter().find(|s| s.is_target).unwrap().available);
    }
    struct Moving<'a> {
        git: LocalGit<'a>,
        moved: Cell<bool>,
        calls: RefCell<Vec<Vec<String>>>,
    }
    impl Git for Moving<'_> {
        fn run(&self, args: &[&str], input: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
            self.calls
                .borrow_mut()
                .push(args.iter().map(|a| a.to_string()).collect());
            if args[0] == "ls-tree" && !self.moved.replace(true) {
                self.git
                    .run(&["update-ref", "-d", "refs/remotes/origin/main"], None)?;
                self.git
                    .run(&["update-ref", "-d", "refs/heads/candidate"], None)?;
                self.git
                    .run(&["symbolic-ref", "HEAD", "refs/heads/unborn"], None)?;
            }
            self.git.run(args, input)
        }
    }
    #[test]
    fn all_refs_resolve_before_reads_and_later_deletion_cannot_change_inventory() {
        let d = fixture();
        let p = d.path();
        let expected = read_inventory(p, "origin/main", &["candidate".into()], "docs/coordination");
        let git = Moving {
            git: LocalGit(p),
            moved: Cell::new(false),
            calls: RefCell::new(Vec::new()),
        };
        let actual = read_with_git(
            &git,
            "origin/main",
            &["candidate".into()],
            "docs/coordination",
        );
        assert_eq!(actual.blobs, expected.blobs);
        assert!(actual.problems.is_empty());
        let calls = git.calls.borrow();
        let first = calls.iter().position(|a| a[0] == "ls-tree").unwrap();
        assert!(calls[first..]
            .iter()
            .all(|a| a[0] == "ls-tree" || a[0] == "cat-file"));
        assert_eq!(calls.iter().filter(|a| a[0] == "rev-parse").count(), 1);
    }
    struct Failing<'a> {
        git: LocalGit<'a>,
        at: usize,
        count: Cell<usize>,
    }
    impl Git for Failing<'_> {
        fn run(&self, args: &[&str], input: Option<Vec<u8>>) -> Result<Vec<u8>, String> {
            let n = self.count.get() + 1;
            self.count.set(n);
            if n == self.at {
                Err("injected platform error wording".into())
            } else {
                self.git.run(args, input)
            }
        }
    }
    #[test]
    fn every_decisive_git_failure_is_reported_as_unavailable() {
        let d = fixture();
        let g = Failing {
            git: LocalGit(d.path()),
            at: usize::MAX,
            count: Cell::new(0),
        };
        assert!(read_with_git(&g, "main", &[], "docs/coordination")
            .problems
            .is_empty());
        for at in 1..=g.count.get() {
            let f = Failing {
                git: LocalGit(d.path()),
                at,
                count: Cell::new(0),
            };
            let out = read_with_git(&f, "main", &[], "docs/coordination");
            assert!(!out.problems.is_empty(), "call{at}");
            assert!(out.sources.iter().any(|s| !s.available), "call{at}");
        }
    }
    #[test]
    fn unresolved_candidate_retains_label_and_does_not_hide_readable_target() {
        let d = fixture();
        let out = read_inventory(
            d.path(),
            "main",
            &["missing-candidate".into()],
            "docs/coordination",
        );
        let unavailable = out.sources.iter().find(|s| !s.is_target).unwrap();
        assert_eq!(unavailable.source_id, "unresolved:missing-candidate");
        assert_eq!(unavailable.sha, None);
        assert!(out.sources.iter().find(|s| s.is_target).unwrap().available);
        assert_eq!(out.blobs.len(), 1);
    }
}
