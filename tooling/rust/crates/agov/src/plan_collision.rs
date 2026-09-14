// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Plan/ref overlap detection. Advisory by default; MR-blocking only under
//! `--fail-on-unacknowledged-collision` (Row 6 S4, issue #123). Default
//! (no-flag) behavior is unchanged from the original advisory-only shape.
use super::*;
#[path = "plan_collision_forge.rs"]
mod forge;

pub(super) struct Source {
    pub label: String,
    pub paths: Vec<String>,
    /// Identifying token another source's `depends_on` may name to
    /// acknowledge a collision with this source: a plan's own `id`, a
    /// branch's full ref name, or `!<iid>` for a forge MR. `None` is
    /// never expected in practice but avoids a panic on malformed input.
    pub token: Option<String>,
    /// This source's own declared `depends_on` entries (plans only;
    /// always empty for refs and MRs, which have no such field).
    pub depends_on: Vec<String>,
    /// True only for a `docs/plans/*.md` source.
    pub is_plan: bool,
    /// True iff this plan's file differs between `--base-ref` and the
    /// current commit. Always computed for plans (not only under
    /// `--changed-only`); always false for refs/MRs.
    pub changed: bool,
}

/// Conservative intersection over the validated whole-segment-** dialect.
/// Wildcard segments may over-report; future files require no enumeration.
fn overlaps(left: &str, right: &str) -> bool {
    // Git emits canonical paths. Preserve equivalent user spellings without
    // turning ./file or repeated separators into false evidence of no overlap.
    let mut left = left
        .split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .peekable();
    let mut right = right
        .split('/')
        .filter(|s| !s.is_empty() && *s != ".")
        .peekable();
    // A repository-root declaration is conservatively whole-repository scope.
    if left.peek().is_none() || right.peek().is_none() {
        return true;
    }
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

/// The set of `docs/plans/*.md` files that differ between `base` and the
/// current commit (`HEAD`). Used for `--changed-only`'s eligibility filter
/// and for acknowledgment matching's "only the changed side may declare
/// `depends_on`" rule -- computed unconditionally (not only when
/// `--changed-only` is passed) so acknowledgment matching behaves
/// consistently either way.
fn changed_plan_files(base: &str) -> Result<BTreeSet<String>, String> {
    let diff = git_output(&[
        "diff",
        "--name-status",
        "-z",
        base,
        "HEAD",
        "--",
        "docs/plans",
    ])?;
    let paths = parse_changes(&diff)?;
    Ok(paths
        .into_iter()
        .filter(|p| p.starts_with("docs/plans/") && p.ends_with(".md"))
        .collect())
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

fn string_array(value: &serde_json::Value, field: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_str())
        .map(|s| s.to_string())
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn run(
    base_ref: &str,
    include_forge: bool,
    plans_only: bool,
    changed_only: bool,
    fail_on_unacknowledged_collision: bool,
    args: &[String],
) -> i32 {
    let mut output = OutputCounter::default();
    let mut unavailable = vec![];
    let mut unassessed = vec![];
    let mut invalid = vec![];
    let mut plans = vec![];
    let mut plan_count = 0;

    // Resolve once so a moving base cannot silently change scope halfway
    // through, and so changed-plan detection and (non-plans-only) ref/forge
    // scanning share one pinned commit.
    let base = git_output(&["rev-parse", "--verify", &format!("{base_ref}^{{commit}}")]);
    let changed_plan_files = match &base {
        Ok(base) => match changed_plan_files(base.trim()) {
            Ok(set) => set,
            Err(e) => {
                unavailable.push(format!("changed-plan detection: {e}"));
                BTreeSet::new()
            }
        },
        Err(_) => BTreeSet::new(),
    };

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
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let label = format!("plan docs/plans/{file_name}");
                let repo_path = format!("docs/plans/{file_name}");
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
                                token: string_field(&plan.raw, "id").map(|s| s.to_string()),
                                depends_on: string_array(&plan.raw, "depends_on"),
                                is_plan: true,
                                changed: changed_plan_files.contains(&repo_path),
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
    let mut mr_count = 0;
    if !plans_only {
        match (&base, git_branch_refs()) {
            (Ok(base), Ok(branches)) => {
                ref_count = branches.len();
                for branch in branches {
                    let label = format!("ref {} @ {}", branch.full_name, branch.oid);
                    match changed_paths(base.trim(), &branch.oid) {
                        Ok(paths) => refs.push(Source {
                            label,
                            paths,
                            token: Some(branch.full_name.clone()),
                            depends_on: vec![],
                            is_plan: false,
                            changed: false,
                        }),
                        Err(e) => unavailable.push(format!("{label}: {e}")),
                    }
                }
            }
            (Err(e), _) => unavailable.push(format!("base {base_ref}: {e}")),
            (_, Err(e)) => unavailable.push(format!("ref inventory: {e}")),
        }
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
    }

    let incomplete = !unavailable.is_empty() || !unassessed.is_empty() || !invalid.is_empty();
    output.stdout(format!("Coverage: {} | mode: {} | base: {} | plans inspected: {} | refs inspected: {} | MRs inspected: {} | unassessed: {} | invalid (excluded): {} | unavailable: {}",
        if incomplete { "INCOMPLETE" } else { "COMPLETE" },
        if plans_only { "plans-only" } else if include_forge { "local + GitLab (fresh)" } else { "local (no network)" },
        base.as_deref().unwrap_or(base_ref).trim(), plan_count, ref_count, mr_count, unassessed.len(), invalid.len(), unavailable.len()));

    let mut eligible = 0usize;
    let mut unacknowledged = 0usize;
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
            if pairs.is_empty() {
                continue;
            }

            if changed_only && !plan.changed && !(other.is_plan && other.changed) {
                continue;
            }
            eligible += 1;

            let acknowledged = (plan.changed
                && other
                    .token
                    .as_deref()
                    .is_some_and(|t| plan.depends_on.iter().any(|d| d == t)))
                || (other.is_plan
                    && other.changed
                    && plan
                        .token
                        .as_deref()
                        .is_some_and(|t| other.depends_on.iter().any(|d| d == t)));
            if !acknowledged {
                unacknowledged += 1;
            }

            let severity_suffix = if fail_on_unacknowledged_collision {
                if acknowledged {
                    "  severity: informational (acknowledged via depends_on)"
                } else {
                    "  severity: blocking"
                }
            } else {
                ""
            };
            output.stdout(format!(
                "Possible overlap: {} <-> {}{severity_suffix}",
                plan.label, other.label
            ));
            for (a, b) in pairs {
                output.stdout(format!("  {a} <-> {b}"));
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

    if fail_on_unacknowledged_collision {
        let acknowledged_count = eligible - unacknowledged;
        output.stdout(format!(
            "{unacknowledged} unacknowledged collision(s) -- severity: blocking; {acknowledged_count} acknowledged"
        ));
    } else {
        output.stdout(format!(
            "Possible overlaps within the coverage above: {eligible} (advisory only)"
        ));
    }

    agov_core::roi::record_tool_output(
        "agov.plan.collision-check",
        args,
        "plan-collision-check",
        output.chars,
    );

    if fail_on_unacknowledged_collision {
        // An unresolvable base means no plan could be evaluated as
        // "changed" at all -- never silently report a clean pass for an
        // unevaluated candidate. The CI job itself also fails closed
        // before reaching this point when its own base resolution fails
        // (see the plan's "Design: CI job" section); this is a second,
        // in-process guard for direct invocations that skip that wrapper.
        if base.is_err() {
            return 1;
        }
        if unacknowledged > 0 {
            1
        } else {
            0
        }
    } else if eligible > 0 {
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
