use agov_core::Finding;
use regex::Regex;
use std::path::Path;

/// Result from README stats validation.
pub struct ReadmeStatsResult {
    pub findings: Vec<Finding>,
    /// true when README.md was found and parseable (even if stats differ).
    pub readme_found: bool,
}

/// Validate that numeric claims in README.md match actual repository state.
///
/// Checks three stats: governance rule count, agent definition count, and
/// skills count. Mismatches and missing claims are advisory (never blocking).
pub fn validate(root: &Path) -> ReadmeStatsResult {
    let readme_path = root.join("README.md");
    let Ok(readme) = std::fs::read_to_string(&readme_path) else {
        return ReadmeStatsResult {
            findings: vec![Finding::advisory(
                "README.md not found — cannot check stat claims",
            )],
            readme_found: false,
        };
    };

    let actual_rules = count_rules(&root.join("agf").join("rules"));
    let actual_agents = count_agents(&root.join("agf").join("agents"));
    let actual_skills = count_skills(&root.join("agf").join("skills"));

    let mut findings = Vec::new();

    check_stat(
        &readme,
        r"(\d+) governance rules",
        actual_rules,
        0,
        "governance rules",
        &mut findings,
    );
    check_stat(
        &readme,
        r"(\d+) agent definitions",
        actual_agents,
        0,
        "agent definitions",
        &mut findings,
    );
    check_stat(
        &readme,
        r"(\d+) skills",
        actual_skills,
        0,
        "skills",
        &mut findings,
    );

    ReadmeStatsResult {
        findings,
        readme_found: true,
    }
}

fn check_stat(
    readme: &str,
    pattern: &str,
    actual: usize,
    tolerance: usize,
    label: &str,
    findings: &mut Vec<Finding>,
) {
    let re = Regex::new(pattern).unwrap();
    match re.captures(readme).and_then(|c| c[1].parse::<usize>().ok()) {
        None => {
            let mut f = Finding::advisory(format!(
                "{label}: claim not found in README.md (actual: {actual})"
            ));
            f.location = Some("README.md".to_string());
            findings.push(f);
        }
        Some(claimed) => {
            let diff = claimed.abs_diff(actual);
            if diff > tolerance {
                let mut f = Finding::advisory(format!(
                    "{label}: README claims {claimed}, actual {actual} (off by {diff})"
                ));
                f.location = Some("README.md".to_string());
                findings.push(f);
            }
        }
    }
}

// ─── File counters ────────────────────────────────────────────────────────────

fn count_rules(rules_dir: &Path) -> usize {
    walkdir_md(rules_dir, &|name| name != "README.md")
}

fn count_agents(agents_dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(agents_dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|e| {
            let p = e.path();
            let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            (ext == "yaml" || ext == "yml") && name != "MANIFEST.yaml"
        })
        .count()
}

fn count_skills(skills_dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(skills_dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|e| e.path().is_dir() && e.path().join("SKILL.md").exists())
        .count()
}

fn walkdir_md(dir: &Path, include: &dyn Fn(&str) -> bool) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut count = 0;
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            count += walkdir_md(&p, include);
        } else if p.extension().and_then(|s| s.to_str()) == Some("md") {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if include(name) {
                count += 1;
            }
        }
    }
    count
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup(tmp: &TempDir, rules: usize, agents: usize, skills: usize) {
        let root = tmp.path();
        let agf = root.join("agf");

        // rules
        let rules_dir = agf.join("rules").join("always");
        fs::create_dir_all(&rules_dir).unwrap();
        for i in 0..rules {
            fs::write(rules_dir.join(format!("rule-{i}.md")), "# rule").unwrap();
        }
        // agents
        let agents_dir = agf.join("agents");
        fs::create_dir_all(&agents_dir).unwrap();
        fs::write(agents_dir.join("MANIFEST.yaml"), "").unwrap();
        for i in 0..agents {
            fs::write(agents_dir.join(format!("agent-{i}.yaml")), "").unwrap();
        }
        // skills
        let skills_dir = agf.join("skills");
        fs::create_dir_all(&skills_dir).unwrap();
        for i in 0..skills {
            let skill = skills_dir.join(format!("skill-{i}"));
            fs::create_dir_all(&skill).unwrap();
            fs::write(skill.join("SKILL.md"), "").unwrap();
        }
    }

    #[test]
    fn ok_when_readme_matches() {
        let tmp = TempDir::new().unwrap();
        setup(&tmp, 3, 2, 1);
        fs::write(
            tmp.path().join("README.md"),
            "The agf/ distribution ships **3 governance rules**, **2 agent definitions**, and **1 skills**.",
        )
        .unwrap();
        let result = validate(tmp.path());
        assert!(result.readme_found);
        assert!(
            result.findings.is_empty(),
            "expected no findings, got: {:?}",
            result.findings
        );
    }

    #[test]
    fn warns_when_rules_stale() {
        let tmp = TempDir::new().unwrap();
        setup(&tmp, 5, 2, 1);
        fs::write(
            tmp.path().join("README.md"),
            "The agf/ distribution ships **3 governance rules**, **2 agent definitions**, and **1 skills**.",
        )
        .unwrap();
        let result = validate(tmp.path());
        assert!(result.readme_found);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.message.contains("governance rules")),
            "expected rules mismatch finding"
        );
    }

    #[test]
    fn warns_when_claim_missing() {
        let tmp = TempDir::new().unwrap();
        setup(&tmp, 3, 2, 1);
        fs::write(tmp.path().join("README.md"), "No stats here.").unwrap();
        let result = validate(tmp.path());
        assert!(result.readme_found);
        assert_eq!(
            result.findings.len(),
            3,
            "expected three 'not found' findings"
        );
    }

    #[test]
    fn readme_not_found_produces_advisory() {
        let tmp = TempDir::new().unwrap();
        setup(&tmp, 3, 2, 1);
        let result = validate(tmp.path());
        assert!(!result.readme_found);
        assert_eq!(result.findings.len(), 1);
    }

    #[test]
    fn manifest_excluded_from_agent_count() {
        let tmp = TempDir::new().unwrap();
        setup(&tmp, 1, 4, 1);
        fs::write(
            tmp.path().join("README.md"),
            "The agf/ distribution ships **1 governance rules**, **4 agent definitions**, and **1 skills**.",
        )
        .unwrap();
        let result = validate(tmp.path());
        assert!(
            result.findings.is_empty(),
            "MANIFEST.yaml must not be counted"
        );
    }
}
