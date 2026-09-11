// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Advisory plan/ref overlap; missing evidence is never a clean result.
use super::*;
#[path = "plan_collision_forge.rs"]
mod forge;

pub(super) struct Source {
    pub label: String,
    pub paths: Vec<String>,
}

/// Conservative intersection over the validated whole-segment-** dialect.
/// Wildcard segments may over-report; future files require no enumeration.
fn overlaps(left: &str, right: &str) -> bool {
    let mut left = left.split('/');
    let mut right = right.split('/');
    loop {
        match (left.next(), right.next()) {
            (Some("**"), _) | (_, Some("**")) => return true,
            (None, None) => return true,
            (Some(a), Some(b)) if a == b || a.contains(['*', '?']) || b.contains(['*', '?']) => (),
            _ => return false,
        }
    }
}

fn changed_paths(base: &str, head: &str) -> Result<Vec<String>, String> {
    let ancestor = git_output(&["merge-base", base, head])?;
    let diff = git_output(&[
        "diff",
        "--name-status",
        "-z",
        "--find-renames",
        ancestor.trim(),
        head,
        "--",
    ])?;
    parse_changes(&diff)
}

fn parse_changes(diff: &str) -> Result<Vec<String>, String> {
    let mut fields = diff.split_terminator('\0');
    let mut paths = BTreeSet::new();
    while let Some(status) = fields.next() {
        if !status.starts_with(['A', 'C', 'D', 'M', 'R', 'T', 'U', 'X', 'B']) {
            return Err("unrecognized name-status diff".into());
        }
        let count = if status.starts_with(['R', 'C']) { 2 } else { 1 };
        for _ in 0..count {
            let path = fields
                .next()
                .filter(|s| !s.is_empty())
                .ok_or("incomplete name-status diff")?;
            paths.insert(path.to_string());
        }
    }
    Ok(paths.into_iter().collect())
}

pub(super) fn run(base_ref: &str, include_forge: bool, args: &[String]) -> i32 {
    let mut output = OutputCounter::default();
    let mut unavailable = vec![];
    let mut unassessed = vec![];
    let mut invalid = vec![];
    let mut plans = vec![];
    let mut plan_count = 0;
    let dir = agov_core::repo_root().join("docs/plans");
    match std::fs::read_dir(&dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(e) => {
                        unavailable.push(format!("plan directory: {e}"));
                        continue;
                    }
                };
                if entry.path().extension().and_then(|s| s.to_str()) != Some("md") {
                    continue;
                }
                plan_count += 1;
                let label = format!("plan docs/plans/{}", entry.file_name().to_string_lossy());
                match agov_coord::plan_validator::validate_file(&entry.path()) {
                    Ok(plan)
                        if matches!(
                            string_field(&plan.raw, "status"),
                            Some("pending-gate1" | "approved")
                        ) =>
                    {
                        match plan.raw.get("affected_paths").and_then(|v| v.as_array()) {
                            Some(paths) => plans.push(Source {
                                label,
                                paths: paths.iter().map(|v| v.as_str().unwrap().into()).collect(),
                            }),
                            None => unassessed.push(format!("{label}: no affected_paths declared")),
                        }
                    }
                    Ok(_) => (),
                    Err(errors) => invalid.push(format!(
                        "{label}: {}",
                        serde_json::to_string(&errors).unwrap()
                    )),
                }
            }
        }
        Err(e) => unavailable.push(format!("plan directory: {e}")),
    }
    plans.sort_by(|a, b| a.label.cmp(&b.label));
    let mut refs = vec![];
    let mut ref_count = 0;
    // Resolve once so a moving base cannot silently change scope halfway through.
    let base = git_output(&["rev-parse", "--verify", &format!("{base_ref}^{{commit}}")]);
    match (&base, git_branch_refs()) {
        (Ok(base), Ok(branches)) => {
            ref_count = branches.len();
            for branch in branches {
                let label = format!("ref {} @ {}", branch.full_name, branch.oid);
                match changed_paths(base.trim(), &branch.oid) {
                    Ok(paths) => refs.push(Source { label, paths }),
                    Err(e) => unavailable.push(format!("{label}: {e}")),
                }
            }
        }
        (Err(e), _) => unavailable.push(format!("base {base_ref}: {e}")),
        (_, Err(e)) => unavailable.push(format!("ref inventory: {e}")),
    }
    let mut mr_count = 0;
    if include_forge {
        match &base {
            Ok(base) => {
                let report = forge::scan(&mut forge::Glab, base.trim());
                mr_count = report.inspected;
                refs.extend(report.sources);
                unavailable.extend(report.unavailable);
            }
            Err(_) => unavailable.push("forge inventory: base commit unavailable".into()),
        }
    }
    let incomplete = !unavailable.is_empty() || !unassessed.is_empty() || !invalid.is_empty();
    output.stdout(format!("Coverage: {} | mode: {} | base: {} | plans inspected: {} | refs inspected: {} | MRs inspected: {} | unassessed: {} | invalid (excluded): {} | unavailable: {}",
        if incomplete { "INCOMPLETE" } else { "COMPLETE" },
        if include_forge { "local + GitLab (fresh)" } else { "local (no network)" },
        base.as_deref().unwrap_or(base_ref).trim(), plan_count, ref_count, mr_count, unassessed.len(), invalid.len(), unavailable.len()));
    let mut collisions = 0;
    for (index, plan) in plans.iter().enumerate() {
        for other in plans.iter().skip(index + 1).chain(refs.iter()) {
            let pairs = plan
                .paths
                .iter()
                .flat_map(|p| {
                    other
                        .paths
                        .iter()
                        .filter(move |q| overlaps(p, q))
                        .map(move |q| (p, q))
                })
                .collect::<Vec<_>>();
            if !pairs.is_empty() {
                collisions += 1;
                output.stdout(format!(
                    "Possible overlap: {} <-> {}",
                    plan.label, other.label
                ));
                for (a, b) in pairs {
                    output.stdout(format!("  {a} <-> {b}"));
                }
            }
        }
    }
    unassessed.sort();
    invalid.sort();
    unavailable.sort();
    for item in unassessed {
        output.stdout(format!("Unassessed: {item}"));
    }
    for item in invalid {
        output.stdout(format!("Invalid (excluded): {item}"));
    }
    for item in unavailable {
        output.stdout(format!("Unavailable: {item}"));
    }
    output.stdout(format!(
        "Possible overlaps within the coverage above: {collisions} (advisory only)"
    ));
    agov_core::roi::record_tool_output(
        "agov.plan.collision-check",
        args,
        "plan-collision-check",
        output.chars,
    );
    if collisions > 0 {
        1
    } else if incomplete {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conservative_future_path_intersections() {
        for (a, b) in [
            ("new/**", "new/file.rs"),
            ("src/*.rs", "src/a.txt"),
            ("a/**/z", "a/q/x"),
            ("a/**", "a"),
        ] {
            assert!(overlaps(a, b));
            assert!(overlaps(b, a));
        }
        assert!(!overlaps("a/file.rs", "b/file.rs"));
        assert!(!overlaps("a/*", "a/b/c"));
    }
    #[test]
    fn renames_keep_both_paths_and_nul_framing() {
        assert_eq!(
            parse_changes("R100\0old name\0new\tname\0M\0other\0").unwrap(),
            vec!["new\tname", "old name", "other"]
        );
        assert!(parse_changes("R100\0old\0").is_err());
    }
}
