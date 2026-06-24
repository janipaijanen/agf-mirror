use crate::{CollectionResult, FileError};
use agov_core::{Finding, Severity};
use std::path::Path;

const LABEL: &str = "agov.validate.assistant_loader";

const LOADER_CANDIDATES: &[(&str, &str)] = &[
    ("AGENTS.md", "Codex / GitHub Copilot"),
    (".claude/CLAUDE.md", "Claude Code"),
];

pub fn validate_all(root: &Path) -> CollectionResult {
    let agf_version = read_agf_version(root);
    let mut findings = Vec::new();

    let mut found_any = false;
    for (rel_path, tool_name) in LOADER_CANDIDATES {
        let full_path = root.join(rel_path);
        if full_path.exists() {
            found_any = true;
            if let Some(ref expected) = agf_version {
                check_version_freshness(&full_path, rel_path, tool_name, expected, &mut findings);
            }
        }
    }

    if !found_any {
        findings.push(Finding {
            severity: Severity::Advisory,
            location: None,
            message: format!(
                "no assistant loader file found (checked: {}); \
                 copy agf/templates/AGENTS.md to the project root or create a .claude/CLAUDE.md loader; \
                 without a loader, AI agents miss cold-start commands on session start",
                LOADER_CANDIDATES
                    .iter()
                    .map(|(p, _)| *p)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        });
    }

    if findings.is_empty() {
        CollectionResult {
            ok_files: vec![LABEL.to_string()],
            file_errors: vec![],
        }
    } else {
        CollectionResult {
            ok_files: vec![],
            file_errors: vec![FileError {
                file: LABEL.to_string(),
                findings,
            }],
        }
    }
}

fn read_agf_version(root: &Path) -> Option<String> {
    std::fs::read_to_string(root.join("agf").join("VERSION"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn check_version_freshness(
    full_path: &Path,
    rel_path: &str,
    tool_name: &str,
    expected: &str,
    findings: &mut Vec<Finding>,
) {
    let Ok(content) = std::fs::read_to_string(full_path) else {
        return;
    };
    if let Some(found) = extract_agf_version_comment(&content) {
        if found != expected {
            findings.push(Finding {
                severity: Severity::Advisory,
                location: Some(rel_path.to_string()),
                message: format!(
                    "{rel_path} ({tool_name} loader) references AGF {found} but agf/VERSION is \
                     {expected}; update the '<!-- AGF: ... -->' comment in {rel_path} after \
                     each framework upgrade"
                ),
            });
        }
    }
    // No comment present: no warning — template may not have been used or loader predates the convention
}

pub(crate) fn extract_agf_version_comment(content: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(pos) = line.find("<!-- AGF:") {
            let rest = line[pos + "<!-- AGF:".len()..].trim_start();
            let token = rest.split_whitespace().next()?;
            // Strip comment-close characters that may be attached when there is no space before -->
            let version = token
                .trim_end_matches('>')
                .trim_end_matches('-');
            if !version.is_empty() {
                return Some(version.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn write(path: PathBuf, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn warns_when_no_loader_exists() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let finding = &result.file_errors[0].findings[0];
        assert!(finding.message.contains("no assistant loader file found"));
        assert!(finding.message.contains("AGENTS.md"));
    }

    #[test]
    fn ok_when_agents_md_exists_with_no_version_comment() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");
        write(temp.path().join("AGENTS.md"), "# My Project\nno version comment");

        let result = validate_all(temp.path());

        assert!(result.is_ok());
    }

    #[test]
    fn ok_when_version_matches() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");
        write(
            temp.path().join("AGENTS.md"),
            "<!-- AGF: 2.2.0 — update this after upgrade -->\n# My Project",
        );

        let result = validate_all(temp.path());

        assert!(result.is_ok());
    }

    #[test]
    fn warns_when_agents_md_version_is_stale() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");
        write(
            temp.path().join("AGENTS.md"),
            "<!-- AGF: 2.1.6 — update this after upgrade -->\n# My Project",
        );

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let finding = &result.file_errors[0].findings[0];
        assert!(finding.message.contains("2.1.6"));
        assert!(finding.message.contains("2.2.0"));
        assert!(finding.message.contains("AGENTS.md"));
    }

    #[test]
    fn warns_when_claude_md_version_is_stale() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");
        write(
            temp.path().join(".claude/CLAUDE.md"),
            "<!-- AGF: 2.1.6 -->\n# Claude Loader",
        );

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let finding = &result.file_errors[0].findings[0];
        assert!(finding.message.contains("2.1.6"));
        assert!(finding.message.contains(".claude/CLAUDE.md"));
    }

    #[test]
    fn ok_when_only_claude_md_exists_no_version_comment() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/VERSION"), "2.2.0");
        write(temp.path().join(".claude/CLAUDE.md"), "# Claude Loader");

        let result = validate_all(temp.path());

        assert!(result.is_ok());
    }

    #[test]
    fn no_warning_when_no_agf_version_file() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("AGENTS.md"), "<!-- AGF: 2.1.6 -->\n# My Project");

        let result = validate_all(temp.path());

        // Found a loader, no version to compare against — ok
        assert!(result.is_ok());
    }

    #[test]
    fn extract_version_from_spaced_comment() {
        assert_eq!(
            extract_agf_version_comment("<!-- AGF: 2.2.0 — update this line -->\n"),
            Some("2.2.0".to_string())
        );
    }

    #[test]
    fn extract_version_from_tight_comment() {
        assert_eq!(
            extract_agf_version_comment("<!-- AGF: 2.2.0-->"),
            Some("2.2.0".to_string())
        );
    }

    #[test]
    fn extract_rc_version() {
        assert_eq!(
            extract_agf_version_comment("<!-- AGF: 2.2.0-rc.1 — update -->\n"),
            Some("2.2.0-rc.1".to_string())
        );
    }

    #[test]
    fn returns_none_when_no_comment() {
        assert_eq!(extract_agf_version_comment("# Just a heading\nsome content"), None);
    }
}
