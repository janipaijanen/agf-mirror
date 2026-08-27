use crate::{CollectionResult, FileError};
use agov_core::{Finding, Severity};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const LABEL: &str = "agov.validate.assistant_loader";
const MAX_IMPORT_TARGET_BYTES: u64 = 64 * 1024;

#[derive(Clone, Copy)]
struct LoaderSurface {
    token: &'static str,
    rel_path: &'static str,
    tool_name: &'static str,
}

const LOADER_SURFACES: &[LoaderSurface] = &[
    LoaderSurface {
        token: "codex",
        rel_path: "AGENTS.md",
        tool_name: "Codex / GitHub Copilot",
    },
    LoaderSurface {
        token: "claude-code",
        rel_path: ".claude/CLAUDE.md",
        tool_name: "Claude Code",
    },
];

pub fn validate_all(root: &Path) -> CollectionResult {
    let agf_version = read_agf_version(root);
    let declared = read_declared_surfaces(root);
    let mut findings = Vec::new();

    match declared {
        Some(tokens) => {
            validate_declared_surfaces(root, &tokens, agf_version.as_deref(), &mut findings)
        }
        None => validate_legacy_surfaces(root, agf_version.as_deref(), &mut findings),
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

fn read_declared_surfaces(root: &Path) -> Option<Vec<String>> {
    let manifest = root.join("agf").join("agents").join("MANIFEST.yaml");
    let data = agov_core::yaml::read_yaml_file_json(&manifest).ok()?;
    let value = data.get("supported_assistant_loaders")?;
    Some(
        value
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .map(|item| item.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_else(|| vec!["<invalid-non-array-value>".to_string()]),
    )
}

fn validate_declared_surfaces(
    root: &Path,
    tokens: &[String],
    expected: Option<&str>,
    findings: &mut Vec<Finding>,
) {
    let mut required_paths = Vec::new();

    if tokens.is_empty() {
        findings.push(advisory_at(
            "supported_assistant_loaders is present but empty; declare at least one of: codex, claude-code, github-copilot",
            "agf/agents/MANIFEST.yaml",
        ));
    }

    for token in tokens {
        let normalized = match token.as_str() {
            "github-copilot" => "codex",
            other => other,
        };
        let Some(surface) = LOADER_SURFACES
            .iter()
            .find(|surface| surface.token == normalized)
        else {
            findings.push(advisory_at(
                format!(
                    "unknown supported_assistant_loaders value '{token}'; accepted values are codex, claude-code, and github-copilot (alias of codex)"
                ),
                "agf/agents/MANIFEST.yaml",
            ));
            continue;
        };
        if !required_paths.contains(&surface.rel_path) {
            required_paths.push(surface.rel_path);
            validate_required_surface(root, *surface, expected, findings);
        }
    }
}

fn validate_required_surface(
    root: &Path,
    surface: LoaderSurface,
    expected: Option<&str>,
    findings: &mut Vec<Finding>,
) {
    let full_path = root.join(surface.rel_path);
    if !full_path.is_file() {
        findings.push(advisory_at(
            format!(
                "declared assistant surface '{}' requires loader file {}",
                surface.token, surface.rel_path
            ),
            surface.rel_path,
        ));
        return;
    }
    check_loader(root, &full_path, surface, expected, findings);
}

fn validate_legacy_surfaces(root: &Path, expected: Option<&str>, findings: &mut Vec<Finding>) {
    let mut found_any = false;
    for surface in LOADER_SURFACES {
        let full_path = root.join(surface.rel_path);
        if full_path.is_file() {
            found_any = true;
            check_loader(root, &full_path, *surface, expected, findings);
        }
    }

    if !found_any {
        findings.push(Finding::advisory(format!(
            "no assistant loader file found (checked: {}); copy agf/templates/AGENTS.md to the project root or create a .claude/CLAUDE.md loader; without a loader, AI agents miss cold-start commands on session start",
            LOADER_SURFACES
                .iter()
                .map(|surface| surface.rel_path)
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
}

fn read_agf_version(root: &Path) -> Option<String> {
    std::fs::read_to_string(root.join("agf").join("VERSION"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn check_loader(
    root: &Path,
    full_path: &Path,
    surface: LoaderSurface,
    expected: Option<&str>,
    findings: &mut Vec<Finding>,
) {
    let content = match std::fs::read_to_string(full_path) {
        Ok(content) => content,
        Err(error) => {
            findings.push(advisory_at(
                format!(
                    "could not read assistant loader {}: {error}",
                    surface.rel_path
                ),
                surface.rel_path,
            ));
            return;
        }
    };

    let freshness_content = match single_import_token(&content) {
        Some(raw_import) => match read_import_target(root, full_path, raw_import) {
            Ok(imported) => imported,
            Err(error) => {
                findings.push(advisory_at(
                    error.message(surface.rel_path),
                    surface.rel_path,
                ));
                return;
            }
        },
        None => content,
    };

    let Some(expected) = expected else {
        return;
    };
    if let Some(found) = extract_agf_version_comment(&freshness_content) {
        if found != expected {
            findings.push(advisory_at(
                format!(
                    "{} ({} loader) references AGF {found} but agf/VERSION is {expected}; update the '<!-- AGF: ... -->' comment in the loaded content after each framework upgrade",
                    surface.rel_path, surface.tool_name
                ),
                surface.rel_path,
            ));
        }
    }
    // No comment after a successful read preserves the historical silent fallback.
}

fn single_import_token(content: &str) -> Option<&str> {
    let trimmed = content.trim();
    if trimmed.lines().count() != 1 {
        return None;
    }
    trimmed.strip_prefix('@').filter(|token| !token.is_empty())
}

#[derive(Debug, PartialEq, Eq)]
enum ImportError {
    Absolute,
    Missing,
    EscapesRoot,
    NonRegular,
    Unreadable,
    Oversized,
    InvalidUtf8,
}

impl ImportError {
    fn message(&self, loader: &str) -> String {
        let detail = match self {
            Self::Absolute => "import target is absolute",
            Self::Missing => "import target is missing",
            Self::EscapesRoot => "import target escapes repository root",
            Self::NonRegular => "import target is not a regular file",
            Self::Unreadable => "import target is unreadable",
            Self::Oversized => "import target exceeds size bound",
            Self::InvalidUtf8 => "import target is not valid UTF-8",
        };
        format!("{loader}: {detail}")
    }
}

fn read_import_target(
    root: &Path,
    importing_file: &Path,
    raw_import: &str,
) -> Result<String, ImportError> {
    read_import_target_with_hook(root, importing_file, raw_import, || {})
}

fn read_import_target_with_hook<F>(
    root: &Path,
    importing_file: &Path,
    raw_import: &str,
    before_open: F,
) -> Result<String, ImportError>
where
    F: FnOnce(),
{
    let raw_path = Path::new(raw_import);
    if raw_path.is_absolute() {
        return Err(ImportError::Absolute);
    }

    let canonical_root = std::fs::canonicalize(root).map_err(|_| ImportError::Unreadable)?;
    let parent = importing_file.parent().ok_or(ImportError::Unreadable)?;
    let candidate = parent.join(raw_path);
    let canonical_target = std::fs::canonicalize(&candidate).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            ImportError::Missing
        } else {
            ImportError::Unreadable
        }
    })?;

    if !canonical_target.starts_with(&canonical_root) {
        return Err(ImportError::EscapesRoot);
    }
    let metadata = std::fs::metadata(&canonical_target).map_err(|_| ImportError::Unreadable)?;
    if !metadata.is_file() {
        return Err(ImportError::NonRegular);
    }
    if metadata.len() > MAX_IMPORT_TARGET_BYTES {
        return Err(ImportError::Oversized);
    }

    before_open();

    let file = File::open(&canonical_target).map_err(|_| ImportError::Unreadable)?;
    let mut bytes = Vec::with_capacity((MAX_IMPORT_TARGET_BYTES + 1) as usize);
    file.take(MAX_IMPORT_TARGET_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ImportError::Unreadable)?;
    if bytes.len() as u64 > MAX_IMPORT_TARGET_BYTES {
        return Err(ImportError::Oversized);
    }
    String::from_utf8(bytes).map_err(|_| ImportError::InvalidUtf8)
}

fn advisory_at(message: impl Into<String>, location: impl Into<String>) -> Finding {
    Finding {
        severity: Severity::Advisory,
        message: message.into(),
        location: Some(location.into()),
    }
}

pub(crate) fn extract_agf_version_comment(content: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(pos) = line.find("<!-- AGF:") {
            let rest = line[pos + "<!-- AGF:".len()..].trim_start();
            let token = rest.split_whitespace().next()?;
            let version = token.trim_end_matches('>').trim_end_matches('-');
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

    fn manifest(root: &Path, loaders: Option<&str>) {
        let field = loaders
            .map(|loaders| format!("supported_assistant_loaders: [{loaders}]\n"))
            .unwrap_or_default();
        write(
            root.join("agf/agents/MANIFEST.yaml"),
            &(field + "agents: {}\n"),
        );
    }

    fn setup(root: &Path) {
        write(root.join("agf/VERSION"), "2.2.0");
    }

    fn messages(result: &CollectionResult) -> Vec<&str> {
        result
            .file_errors
            .iter()
            .flat_map(|error| {
                error
                    .findings
                    .iter()
                    .map(|finding| finding.message.as_str())
            })
            .collect()
    }

    #[test]
    fn warns_when_no_loader_exists_without_declaration() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), None);
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("no assistant loader file found"));
    }

    #[test]
    fn declared_claude_requires_its_own_loader_even_when_agents_exists() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join("AGENTS.md"), "# Present but not declared");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("claude-code"));
        assert!(messages(&result)[0].contains(".claude/CLAUDE.md"));
    }

    #[test]
    fn declared_codex_only_accepts_agents_md() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("codex"));
        write(temp.path().join("AGENTS.md"), "# Loader");
        assert!(validate_all(temp.path()).is_ok());
    }

    #[test]
    fn github_copilot_alias_requires_agents_md() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("github-copilot"));
        write(temp.path().join("AGENTS.md"), "# Loader");
        assert!(validate_all(temp.path()).is_ok());
    }

    #[test]
    fn both_declared_surfaces_require_both_files() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("codex, claude-code"));
        write(temp.path().join("AGENTS.md"), "# Loader");
        let result = validate_all(temp.path());
        assert_eq!(messages(&result).len(), 1);
        assert!(messages(&result)[0].contains(".claude/CLAUDE.md"));
    }

    #[test]
    fn unknown_declared_surface_is_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("cursor"));
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("unknown supported_assistant_loaders value 'cursor'"));
        assert!(result.file_errors[0]
            .findings
            .iter()
            .all(|finding| finding.severity == Severity::Advisory));
    }

    #[test]
    fn no_declaration_preserves_any_loader_behavior() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), None);
        write(temp.path().join("AGENTS.md"), "# Loader");
        assert!(validate_all(temp.path()).is_ok());
    }

    #[test]
    fn stale_direct_loader_is_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("codex"));
        write(temp.path().join("AGENTS.md"), "<!-- AGF: 2.1.6 -->\n");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("2.1.6"));
    }

    #[test]
    fn imported_loader_follows_version_comment() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@../AGENTS.md\n");
        write(temp.path().join("AGENTS.md"), "<!-- AGF: 2.1.6 -->\n");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("2.1.6"));
        assert!(messages(&result)[0].contains(".claude/CLAUDE.md"));
    }

    #[test]
    fn imported_loader_accepts_matching_version() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@../AGENTS.md\n");
        write(temp.path().join("AGENTS.md"), "<!-- AGF: 2.2.0 -->\n");
        assert!(validate_all(temp.path()).is_ok());
    }

    #[test]
    fn absolute_import_is_visible_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@/etc/passwd\n");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("import target is absolute"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_drive_and_unc_imports_are_absolute() {
        assert!(Path::new(r"C:\\Windows\\system.ini").is_absolute());
        assert!(Path::new(r"\\server\share\AGENTS.md").is_absolute());
    }

    #[test]
    fn traversal_escape_is_visible_advisory() {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("repo");
        setup(&root);
        manifest(&root, Some("claude-code"));
        write(parent.path().join("outside.md"), "<!-- AGF: 2.2.0 -->\n");
        write(root.join(".claude/CLAUDE.md"), "@../../outside.md\n");
        let result = validate_all(&root);
        assert!(messages(&result)[0].contains("escapes repository root"));
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_is_visible_advisory() {
        use std::os::unix::fs::symlink;
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("repo");
        setup(&root);
        manifest(&root, Some("claude-code"));
        write(parent.path().join("outside.md"), "<!-- AGF: 2.2.0 -->\n");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        symlink(
            parent.path().join("outside.md"),
            root.join(".claude/outside.md"),
        )
        .unwrap();
        write(root.join(".claude/CLAUDE.md"), "@outside.md\n");
        let result = validate_all(&root);
        assert!(messages(&result)[0].contains("escapes repository root"));
    }

    #[test]
    fn missing_import_is_visible_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@missing.md\n");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("import target is missing"));
    }

    #[test]
    fn non_regular_import_is_visible_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        std::fs::create_dir_all(temp.path().join("target-dir")).unwrap();
        write(temp.path().join(".claude/CLAUDE.md"), "@../target-dir\n");
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("not a regular file"));
    }

    #[test]
    fn open_failure_after_resolution_is_reported_as_unreadable() {
        let temp = tempfile::tempdir().unwrap();
        let importer = temp.path().join(".claude/CLAUDE.md");
        let target = temp.path().join("AGENTS.md");
        write(importer.clone(), "@../AGENTS.md\n");
        write(target.clone(), "<!-- AGF: 2.2.0 -->\n");
        let result = read_import_target_with_hook(
            temp.path(),
            &importer,
            "../AGENTS.md",
            || std::fs::remove_file(&target).unwrap(),
        );
        assert_eq!(result, Err(ImportError::Unreadable));
        assert_eq!(
            ImportError::Unreadable.message(".claude/CLAUDE.md"),
            ".claude/CLAUDE.md: import target is unreadable"
        );
    }

    #[test]
    fn invalid_utf8_import_is_visible_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@../AGENTS.md\n");
        std::fs::write(temp.path().join("AGENTS.md"), [0xff, 0xfe]).unwrap();
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("not valid UTF-8"));
    }

    #[test]
    fn exact_size_import_is_accepted() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@../AGENTS.md\n");
        write(
            temp.path().join("AGENTS.md"),
            &"x".repeat(MAX_IMPORT_TARGET_BYTES as usize),
        );
        assert!(validate_all(temp.path()).is_ok());
    }

    #[test]
    fn oversized_import_is_visible_advisory() {
        let temp = tempfile::tempdir().unwrap();
        setup(temp.path());
        manifest(temp.path(), Some("claude-code"));
        write(temp.path().join(".claude/CLAUDE.md"), "@../AGENTS.md\n");
        write(
            temp.path().join("AGENTS.md"),
            &"x".repeat((MAX_IMPORT_TARGET_BYTES + 1) as usize),
        );
        let result = validate_all(temp.path());
        assert!(messages(&result)[0].contains("exceeds size bound"));
    }

    #[test]
    fn read_time_bound_catches_growth_after_metadata_check() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let importer = root.join(".claude/CLAUDE.md");
        let target = root.join("AGENTS.md");
        write(importer.clone(), "@../AGENTS.md\n");
        write(
            target.clone(),
            &"x".repeat(MAX_IMPORT_TARGET_BYTES as usize),
        );
        let result = read_import_target_with_hook(root, &importer, "../AGENTS.md", || {
            use std::io::Write;
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&target)
                .unwrap();
            file.write_all(b"x").unwrap();
        });
        assert_eq!(result, Err(ImportError::Oversized));
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
        assert_eq!(
            extract_agf_version_comment("# Just a heading\nsome content"),
            None
        );
    }
}
