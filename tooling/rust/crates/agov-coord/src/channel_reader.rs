use crate::coord_validator::{self, CoordMessage};
use std::path::Path;

pub type ChannelEntry = (String, Result<CoordMessage, Vec<(String, String)>>);

/// Read and validate all coordination message files in `dir`.
///
/// Files are returned sorted by filename (timestamp-prefixed filenames sort
/// chronologically). Parse failures are collected rather than short-circuiting.
pub fn read_channel(dir: &Path) -> Vec<ChannelEntry> {
    let mut entries = Vec::new();

    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return entries,
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let result = coord_validator::validate_file(&path);
        entries.push((file_name.to_string(), result));
    }

    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    entries
}

/// Extract body-text references from a coordination file.
///
/// Scans the Markdown body (everything after the second `---`) for tokens that
/// look like coordination filenames and returns them normalised to `<id>.md`.
pub fn body_refs(path: &Path) -> Vec<String> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let body = content.splitn(3, "---").nth(2).unwrap_or(&content);

    body.split(|ch: char| {
        ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | '(' | ')' | ',')
    })
    .filter_map(clean_coord_filename)
    .collect()
}

fn clean_coord_filename(token: &str) -> Option<String> {
    let trimmed =
        token.trim_matches(|ch: char| matches!(ch, '.' | ':' | ';' | ']' | '[' | ')' | '('));
    let filename = trimmed.rsplit('/').next()?;
    coord_validator::normalize_coord_reference(filename)
}

/// All `references-to` filenames for a message, including body-text refs.
pub fn all_refs(path: &Path, msg: &CoordMessage) -> Vec<String> {
    let mut refs = msg.references_to();
    refs.extend(body_refs(path));
    refs.sort();
    refs.dedup();
    refs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_channel_returns_sorted_entries() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();

        // Write two minimal valid coordination files in reverse filename order
        let b_content = "---\nkind: note\nfrom: coder-tester\nto: planner-architect\nid: 20260623T120000Z-coder-tester-aaaa\ntimestamp: 2026-06-23T12:00:00Z\nimmutable: true\nreferences-to: []\ntopic: b\nrequires-ack: false\nartifacts_owned: []\n---\n# B\n";
        let a_content = "---\nkind: note\nfrom: coder-tester\nto: planner-architect\nid: 20260623T110000Z-coder-tester-bbbb\ntimestamp: 2026-06-23T11:00:00Z\nimmutable: true\nreferences-to: []\ntopic: a\nrequires-ack: false\nartifacts_owned: []\n---\n# A\n";

        std::fs::write(
            path.join("20260623T120000Z-coder-tester-aaaa.md"),
            b_content,
        )
        .unwrap();
        std::fs::write(
            path.join("20260623T110000Z-coder-tester-bbbb.md"),
            a_content,
        )
        .unwrap();

        let entries = read_channel(path);
        assert_eq!(entries.len(), 2);
        // Should be sorted by filename (110000 < 120000)
        assert!(entries[0].0.contains("110000"));
        assert!(entries[1].0.contains("120000"));
    }

    #[test]
    fn read_channel_collects_invalid_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("bad.md"), "not valid frontmatter").unwrap();
        let entries = read_channel(dir.path());
        assert_eq!(entries.len(), 1);
        assert!(entries[0].1.is_err());
    }

    #[test]
    fn body_refs_captures_bare_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        std::fs::write(
            &path,
            "---\nfoo: bar\n---\n\nSee 20260623T181212Z-governor-reviewer-grph\n",
        )
        .unwrap();
        let refs = body_refs(&path);
        assert!(
            refs.contains(&"20260623T181212Z-governor-reviewer-grph.md".to_string()),
            "bare ID should be normalised to .md: {:?}",
            refs
        );
    }

    #[test]
    fn body_refs_captures_md_suffixed_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        std::fs::write(
            &path,
            "---\nfoo: bar\n---\n\nSee 20260623T181212Z-governor-reviewer-grph.md\n",
        )
        .unwrap();
        let refs = body_refs(&path);
        assert!(refs.contains(&"20260623T181212Z-governor-reviewer-grph.md".to_string()));
    }

    #[test]
    fn body_refs_captures_path_qualified_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        std::fs::write(
            &path,
            "---\nfoo: bar\n---\n\nSee docs/coordination/20260623T181212Z-governor-reviewer-grph.md\n",
        )
        .unwrap();
        let refs = body_refs(&path);
        assert!(refs.contains(&"20260623T181212Z-governor-reviewer-grph.md".to_string()));
    }

    #[test]
    fn body_refs_rejects_non_coord_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        std::fs::write(
            &path,
            "---\nfoo: bar\n---\n\nSome random text with no coordination ID here\n",
        )
        .unwrap();
        let refs = body_refs(&path);
        assert!(refs.is_empty(), "no coord IDs expected, got: {:?}", refs);
    }
}
