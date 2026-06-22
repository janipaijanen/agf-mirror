use crate::{CollectionResult, FileError};
use agov_core::Finding;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const LABEL: &str = "agov.validate.assistant_drift";
const MAX_DETAILED_FINDINGS: usize = 8;

pub fn validate_all(root: &Path) -> CollectionResult {
    let mut findings = Vec::new();

    findings.extend(compare_tree(
        root,
        root.join("agf").join("rules"),
        root.join(".claude").join("rules"),
        "rules",
    ));
    findings.extend(compare_tree(
        root,
        root.join("agf").join("skills"),
        root.join(".claude").join("skills"),
        "skills",
    ));

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

fn compare_tree(root: &Path, canonical: PathBuf, copied: PathBuf, kind: &str) -> Vec<Finding> {
    if !canonical.is_dir() || !copied.is_dir() {
        return Vec::new();
    }

    let canonical_files = files_by_relative_path(&canonical);
    let copied_files = files_by_relative_path(&copied);
    let mut rels: BTreeSet<PathBuf> = canonical_files.keys().cloned().collect();
    rels.extend(copied_files.keys().cloned());

    let mut findings = Vec::new();
    for rel in rels {
        match (canonical_files.get(&rel), copied_files.get(&rel)) {
            (Some(canonical_file), Some(copied_file)) => {
                if file_hash(canonical_file) != file_hash(copied_file) {
                    findings.push(Finding {
                        severity: agov_core::Severity::Advisory,
                        location: Some(relative_to(root, copied_file)),
                        message: format!(
                            "copied .claude/{kind} file differs from canonical {}; replace copied AGF content with a loader reference, or move intentional project-specific overrides outside .claude/{kind}/",
                            relative_to(root, canonical_file)
                        ),
                    });
                }
            }
            (Some(canonical_file), None) => {
                findings.push(Finding {
                    severity: agov_core::Severity::Advisory,
                    location: Some(relative_to(root, &copied.join(&rel))),
                    message: format!(
                        "copied .claude/{kind} tree is missing canonical {}; copied AGF trees drift on upgrade, so prefer a loader reference to agf/{kind}/",
                        relative_to(root, canonical_file)
                    ),
                });
            }
            (None, Some(copied_file)) => {
                findings.push(Finding {
                    severity: agov_core::Severity::Advisory,
                    location: Some(relative_to(root, copied_file)),
                    message: format!(
                        "file has no canonical agf/{kind}/ counterpart; keep project-specific assistant instructions outside copied AGF {kind} trees"
                    ),
                });
            }
            (None, None) => {}
        }
    }

    summarize_if_needed(root, &copied, kind, findings)
}

fn files_by_relative_path(root: &Path) -> BTreeMap<PathBuf, PathBuf> {
    let mut files = BTreeMap::new();
    collect_files(root, root, &mut files);
    files
}

fn collect_files(root: &Path, dir: &Path, files: &mut BTreeMap<PathBuf, PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, files);
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(root) {
                files.insert(rel.to_path_buf(), path);
            }
        }
    }
}

fn file_hash(path: &Path) -> Option<[u8; 32]> {
    let bytes = std::fs::read(path).ok()?;
    Some(Sha256::digest(bytes).into())
}

fn relative_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn summarize_if_needed(
    root: &Path,
    copied: &Path,
    kind: &str,
    findings: Vec<Finding>,
) -> Vec<Finding> {
    if findings.len() <= MAX_DETAILED_FINDINGS {
        return findings;
    }

    let differing = findings
        .iter()
        .filter(|finding| finding.message.contains("differs from canonical"))
        .count();
    let missing = findings
        .iter()
        .filter(|finding| finding.message.contains("is missing canonical"))
        .count();
    let extra = findings
        .iter()
        .filter(|finding| finding.message.contains("no canonical agf/"))
        .count();
    let examples = findings
        .iter()
        .filter_map(|finding| finding.location.as_deref())
        .take(5)
        .collect::<Vec<_>>()
        .join(", ");

    vec![Finding {
        severity: agov_core::Severity::Advisory,
        location: Some(relative_to(root, copied)),
        message: format!(
            "copied .claude/{kind} tree has {} drift finding(s): {differing} differ, {missing} missing, {extra} extra; examples: {examples}; replace copied AGF content with a loader reference, or move intentional project-specific overrides outside .claude/{kind}/",
            findings.len()
        ),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warns_for_differing_copied_rule() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/rules/foo.md"), "canonical");
        write(temp.path().join(".claude/rules/foo.md"), "copied");

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let findings = &result.file_errors[0].findings;
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("differs from canonical"));
        assert!(findings[0]
            .message
            .contains("replace copied AGF content with a loader reference"));
        assert_eq!(
            findings[0].location.as_deref(),
            Some(".claude/rules/foo.md")
        );
    }

    #[test]
    fn warns_for_missing_canonical_rule_copy_when_copied_tree_exists() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/rules/foo.md"), "canonical");
        std::fs::create_dir_all(temp.path().join(".claude/rules")).unwrap();

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let finding = &result.file_errors[0].findings[0];
        assert!(finding.message.contains("is missing canonical"));
        assert_eq!(finding.location.as_deref(), Some(".claude/rules/foo.md"));
    }

    #[test]
    fn does_not_warn_for_loader_only_layout() {
        let temp = tempfile::tempdir().unwrap();
        write(temp.path().join("agf/rules/foo.md"), "canonical");
        write(temp.path().join(".claude/CLAUDE.md"), "@AGENTS.md");

        let result = validate_all(temp.path());

        assert!(result.is_ok());
    }

    #[test]
    fn summarizes_large_drift_sets() {
        let temp = tempfile::tempdir().unwrap();
        for index in 0..10 {
            write(
                temp.path().join(format!("agf/rules/rule-{index}.md")),
                "canonical",
            );
            write(
                temp.path().join(format!(".claude/rules/rule-{index}.md")),
                "copied",
            );
        }

        let result = validate_all(temp.path());

        assert!(!result.is_ok());
        let findings = &result.file_errors[0].findings;
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].location.as_deref(), Some(".claude/rules"));
        assert!(findings[0].message.contains("10 drift finding(s)"));
        assert!(findings[0].message.contains("examples:"));
    }

    fn write(path: PathBuf, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }
}
