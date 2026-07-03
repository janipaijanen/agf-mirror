use crate::{CollectionResult, FileError};
use agov_core::{Finding, Severity};
use std::path::Path;
use std::process::Command;

const LABEL: &str = "agov.validate.adopter_docs";

pub fn validate_all(root: &Path) -> CollectionResult {
    match validate(root) {
        Ok(findings) if findings.is_empty() => CollectionResult {
            ok_files: vec![],
            file_errors: vec![],
        },
        Ok(findings) => CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: LABEL.to_string(),
                findings,
            }],
        },
        Err(findings) => CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: LABEL.to_string(),
                findings,
            }],
        },
    }
}

fn validate(root: &Path) -> Result<Vec<Finding>, Vec<Finding>> {
    let Some(tag) = latest_release_tag(root)? else {
        return Ok(vec![Finding {
            severity: Severity::Advisory,
            location: Some("git".to_string()),
            message: "no reachable v* tag found; skipping adopter-docs release-range check"
                .to_string(),
        }]);
    };

    let changed = changed_paths(root, &tag)?;
    let trigger_paths: Vec<&str> = changed
        .iter()
        .map(String::as_str)
        .filter(|path| is_adopter_relevant_trigger(path))
        .collect();

    if trigger_paths.is_empty() {
        return Ok(Vec::new());
    }

    if changed
        .iter()
        .any(|path| is_downstream_facing_coverage(path))
    {
        return Ok(Vec::new());
    }

    if changelog_has_waiver_marker(root, &tag)? {
        return Ok(Vec::new());
    }

    Err(vec![Finding::blocking_at(
        format!(
            "adopter-relevant changes since {tag} require downstream-facing docs/templates or an agf/CHANGELOG.md waiver marker; triggers: {}",
            trigger_paths.join(", ")
        ),
        format!("{tag}..HEAD"),
    )])
}

fn latest_release_tag(root: &Path) -> Result<Option<String>, Vec<Finding>> {
    let output = git(
        root,
        &["describe", "--tags", "--match", "v*", "--abbrev=0", "HEAD"],
    )?;
    if output.status.success() {
        let tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Ok((!tag.is_empty()).then_some(tag));
    }

    Ok(None)
}

fn changed_paths(root: &Path, tag: &str) -> Result<Vec<String>, Vec<Finding>> {
    let range = format!("{tag}..HEAD");
    let output = git(root, &["diff", "--name-only", &range])?;
    if !output.status.success() {
        return Err(vec![git_failure("git diff --name-only", &output)]);
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().replace('\\', "/"))
        .filter(|line| !line.is_empty())
        .collect())
}

fn changelog_has_waiver_marker(root: &Path, tag: &str) -> Result<bool, Vec<Finding>> {
    let range = format!("{tag}..HEAD");
    let output = git(
        root,
        &["diff", "--unified=0", &range, "--", "agf/CHANGELOG.md"],
    )?;
    if !output.status.success() {
        return Err(vec![git_failure("git diff agf/CHANGELOG.md", &output)]);
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .any(|line| {
            line.contains("Adopter docs: not applicable") || line.contains("Adopter docs: waived")
        }))
}

fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, Vec<Finding>> {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|err| {
            vec![Finding::blocking_at(
                format!("failed to run git: {err}"),
                root.display().to_string(),
            )]
        })
}

fn git_failure(command: &str, output: &std::process::Output) -> Finding {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Finding::blocking_at(
        if stderr.is_empty() {
            format!("{command} failed")
        } else {
            format!("{command} failed: {stderr}")
        },
        "git",
    )
}

fn is_adopter_relevant_trigger(path: &str) -> bool {
    path == "AGENTS.md"
        || path == "agf/docs/site/reference/environment-variables.md"
        || path.starts_with("tooling/rust/")
        || path.starts_with("agf/schemas/")
        || path.starts_with("agf/rules/")
        || path.starts_with("agf/agents/")
        || path.starts_with("agf/templates/")
        || path.starts_with("agf/dist/")
}

fn is_downstream_facing_coverage(path: &str) -> bool {
    path == "agf/ADOPTING.md"
        || path.starts_with("agf/templates/")
        || path.starts_with("agf/docs/site/docs/adopt/")
        || path.starts_with("agf/docs/site/docs/operate/")
        || path.starts_with("agf/docs/site/docs/advanced/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn init_repo() -> tempfile::TempDir {
        let temp = tempfile::tempdir().unwrap();
        run(temp.path(), &["init"]);
        run(
            temp.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        run(temp.path(), &["config", "user.name", "Test User"]);
        write(temp.path(), "agf/schemas/.keep", "");
        write(temp.path(), "tooling/.keep", "");
        write(temp.path(), "README.md", "baseline\n");
        run(temp.path(), &["add", "."]);
        run(temp.path(), &["commit", "-m", "baseline"]);
        run(temp.path(), &["tag", "v1.0.0"]);
        temp
    }

    fn write(root: &Path, rel: &str, content: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn commit(root: &Path, message: &str) {
        run(root, &["add", "."]);
        run(root, &["commit", "-m", message]);
    }

    fn run(root: &Path, args: &[&str]) {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn blocking_messages(result: CollectionResult) -> Vec<String> {
        result
            .file_errors
            .into_iter()
            .flat_map(|file| file.findings)
            .filter(|finding| finding.severity == Severity::Blocking)
            .map(|finding| finding.message)
            .collect()
    }

    #[test]
    fn fails_when_tooling_changes_only_reference_docs() {
        let repo = init_repo();
        write(repo.path(), "tooling/rust/src/main.rs", "fn main() {}\n");
        write(
            repo.path(),
            "agf/docs/site/reference/environment-variables.md",
            "# Env\n",
        );
        commit(repo.path(), "add tooling and reference docs");

        let messages = blocking_messages(validate_all(repo.path()));

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("downstream-facing docs/templates"));
    }

    #[test]
    fn passes_when_tooling_change_has_adopting_docs() {
        let repo = init_repo();
        write(repo.path(), "tooling/rust/src/main.rs", "fn main() {}\n");
        write(repo.path(), "agf/ADOPTING.md", "Adopter docs parity\n");
        commit(repo.path(), "add tooling and adopting docs");

        assert!(validate_all(repo.path()).is_ok());
    }

    #[test]
    fn passes_when_schema_change_has_changelog_waiver() {
        let repo = init_repo();
        write(repo.path(), "agf/schemas/example.schema.json", "{}\n");
        write(
            repo.path(),
            "agf/CHANGELOG.md",
            "## Unreleased\n\nAdopter docs: waived 20260703T100000Z-governor-reviewer-test\n",
        );
        commit(repo.path(), "add schema with waiver");

        assert!(validate_all(repo.path()).is_ok());
    }

    #[test]
    fn passes_when_only_reference_docs_change() {
        let repo = init_repo();
        write(
            repo.path(),
            "agf/docs/site/reference/release-info.md",
            "# Release info\n",
        );
        commit(repo.path(), "update reference docs");

        assert!(validate_all(repo.path()).is_ok());
    }

    #[test]
    fn advisory_skip_when_no_previous_tag_exists() {
        let temp = tempfile::tempdir().unwrap();
        run(temp.path(), &["init"]);
        run(
            temp.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        run(temp.path(), &["config", "user.name", "Test User"]);
        write(temp.path(), "agf/schemas/.keep", "");
        write(temp.path(), "tooling/rust/src/main.rs", "fn main() {}\n");
        run(temp.path(), &["add", "."]);
        run(temp.path(), &["commit", "-m", "initial"]);

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let finding = &result.file_errors[0].findings[0];
        assert_eq!(finding.severity, Severity::Advisory);
        assert!(finding.message.contains("no reachable v* tag found"));
    }

    #[test]
    fn trigger_and_coverage_path_sets_match_policy() {
        assert!(is_adopter_relevant_trigger(
            "tooling/rust/crates/agov/src/main.rs"
        ));
        assert!(is_adopter_relevant_trigger(
            "agf/schemas/coordination-message.schema.json"
        ));
        assert!(is_adopter_relevant_trigger(
            "agf/docs/site/reference/environment-variables.md"
        ));
        assert!(!is_adopter_relevant_trigger(
            "agf/docs/site/reference/release-info.md"
        ));

        assert!(is_downstream_facing_coverage("agf/ADOPTING.md"));
        assert!(is_downstream_facing_coverage("agf/templates/AGENTS.md"));
        assert!(is_downstream_facing_coverage(
            "agf/docs/site/docs/adopt/tooling-and-tokens.md"
        ));
        assert!(!is_downstream_facing_coverage(
            "agf/docs/site/reference/environment-variables.md"
        ));
    }
}
