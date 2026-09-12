//! Repository-contained local artifact discovery. Glob matching never follows
//! symlinks, including intermediate components and recursive `**` traversal.
use crate::artifact_config::contained_relative_path;
use glob::Pattern;
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_VISITS: usize = 100_000;

pub struct RepositoryFiles {
    root: PathBuf,
}
impl RepositoryFiles {
    pub fn new(root: &Path) -> Result<Self, String> {
        let root = root
            .canonicalize()
            .map_err(|e| format!("repository root: {e}"))?;
        if !root.is_dir() {
            return Err("repository root is not a directory".into());
        }
        Ok(Self { root })
    }
    fn checked(&self, relative: &str, directory: bool) -> Result<PathBuf, String> {
        if !contained_relative_path(relative) {
            return Err(format!("invalid repository-relative path {relative:?}"));
        }
        let mut path = self.root.clone();
        let parts: Vec<_> = relative.split('/').collect();
        for (i, part) in parts.iter().enumerate() {
            path.push(part);
            let meta = fs::symlink_metadata(&path).map_err(|e| format!("{relative}: {e}"))?;
            if meta.file_type().is_symlink() {
                return Err(format!("{relative}: symlink traversal is forbidden"));
            }
            if i + 1 < parts.len() || directory {
                if !meta.is_dir() {
                    return Err(format!("{relative}: expected a directory"));
                }
            } else if !meta.is_file() {
                return Err(format!("{relative}: expected a regular file"));
            }
            if !path
                .canonicalize()
                .map_err(|e| format!("{relative}: {e}"))?
                .starts_with(&self.root)
            {
                return Err(format!("{relative}: path escapes repository"));
            }
        }
        Ok(path)
    }
    pub fn read(&self, relative: &str) -> Result<Vec<u8>, String> {
        let path = self.checked(relative, false)?;
        let file = fs::File::open(&path).map_err(|e| format!("{relative}: {e}"))?;
        let before = file.metadata().map_err(|e| format!("{relative}: {e}"))?;
        if !before.is_file() || before.len() > MAX_FILE_BYTES {
            return Err(format!(
                "{relative}: not a regular file within the 16 MiB limit"
            ));
        }
        let mut bytes = Vec::new();
        file.take(MAX_FILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("{relative}: {e}"))?;
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err(format!("{relative}: exceeds 16 MiB limit"));
        }
        self.checked(relative, false)?;
        let after = fs::metadata(path).map_err(|e| format!("{relative}: {e}"))?;
        if before.len() != after.len()
            || before.modified().ok() != after.modified().ok()
            || bytes.len() as u64 != before.len()
        {
            return Err(format!("{relative}: file changed during read"));
        }
        Ok(bytes)
    }
    pub fn discover(&self, patterns: &[String]) -> Result<Vec<String>, Vec<String>> {
        let mut paths = BTreeSet::new();
        let mut errors = BTreeSet::new();
        for pattern in patterns {
            match self.expand(pattern) {
                Ok(found) if found.is_empty() => {
                    errors.insert(format!("{pattern}: pattern matched no regular files"));
                }
                Ok(found) => paths.extend(found),
                Err(e) => {
                    errors.insert(e);
                }
            }
        }
        if errors.is_empty() {
            Ok(paths.into_iter().collect())
        } else {
            Err(errors.into_iter().collect())
        }
    }
    fn expand(&self, pattern: &str) -> Result<BTreeSet<String>, String> {
        if !contained_relative_path(pattern) {
            return Err(format!("invalid repository-relative glob {pattern:?}"));
        }
        let parts: Vec<_> = pattern.split('/').collect();
        let compiled: Vec<_> = parts
            .iter()
            .map(|p| Pattern::new(p).map_err(|e| format!("{pattern}: {e}")))
            .collect::<Result<_, _>>()?;
        if parts.iter().any(|p| p.contains("**") && *p != "**") {
            return Err(format!("{pattern}: ** must be a complete path component"));
        }
        let mut stack = vec![(String::new(), 0)];
        let mut visited = BTreeSet::new();
        let mut result = BTreeSet::new();
        while let Some((prefix, index)) = stack.pop() {
            if !visited.insert((prefix.clone(), index)) {
                continue;
            }
            if visited.len() > MAX_VISITS {
                return Err(format!("{pattern}: discovery exceeds {MAX_VISITS} states"));
            }
            if index == parts.len() {
                if !prefix.is_empty() {
                    self.checked(&prefix, false)?;
                    result.insert(prefix);
                }
                continue;
            }
            let dir = if prefix.is_empty() {
                self.root.clone()
            } else {
                self.checked(&prefix, true)?
            };
            if parts[index] == "**" && index + 1 < parts.len() {
                stack.push((prefix.clone(), index + 1));
            }
            for entry in fs::read_dir(dir).map_err(|e| format!("{pattern}: {e}"))? {
                let entry = entry.map_err(|e| format!("{pattern}: {e}"))?;
                let name = entry
                    .file_name()
                    .into_string()
                    .map_err(|_| format!("{pattern}: non-UTF-8 entry"))?;
                if parts[index] != "**" && !compiled[index].matches(&name) {
                    continue;
                }
                let relative = if prefix.is_empty() {
                    name
                } else {
                    format!("{prefix}/{name}")
                };
                let ty = entry.file_type().map_err(|e| format!("{relative}: {e}"))?;
                if ty.is_symlink() {
                    return Err(format!("{relative}: symlink traversal is forbidden"));
                }
                if parts[index] == "**" {
                    if ty.is_dir() {
                        stack.push((relative, index));
                    } else if index + 1 == parts.len() && ty.is_file() {
                        result.insert(relative);
                    }
                } else if index + 1 == parts.len() || ty.is_dir() {
                    stack.push((relative, index + 1));
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        fs::create_dir_all(d.path().join("items/nested")).unwrap();
        fs::write(d.path().join("items/a.json"), "{\"id\":\"a\"}").unwrap();
        fs::write(d.path().join("items/nested/b.json"), "{\"id\":\"b\"}").unwrap();
        d
    }
    #[test]
    fn discovery_is_sorted_deduplicated_and_recursive_without_implicit_sources() {
        let d = fixture();
        let reader = RepositoryFiles::new(d.path()).unwrap();
        assert_eq!(
            reader
                .discover(&["items/**/*.json".into(), "items/a.json".into()])
                .unwrap(),
            ["items/a.json", "items/nested/b.json"]
        );
        assert_eq!(reader.read("items/a.json").unwrap(), b"{\"id\":\"a\"}");
        assert!(reader.discover(&["missing/*.json".into()]).is_err());
        assert!(reader.discover(&["../*.json".into()]).is_err());
        assert!(reader.discover(&["items/[".into()]).is_err());
    }
    #[test]
    fn reading_missing_directory_or_oversize_input_fails() {
        let d = fixture();
        let reader = RepositoryFiles::new(d.path()).unwrap();
        assert!(reader.read("items").is_err());
        assert!(reader.read("missing").is_err());
        fs::File::create(d.path().join("huge"))
            .unwrap()
            .set_len(MAX_FILE_BYTES + 1)
            .unwrap();
        assert!(reader.read("huge").unwrap_err().contains("16 MiB"));
    }
    #[cfg(unix)]
    #[test]
    fn direct_intermediate_and_recursive_symlinks_never_supply_evidence() {
        use std::os::unix::fs::symlink;
        let d = fixture();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("secret.json"), "secret").unwrap();
        let reader = RepositoryFiles::new(d.path()).unwrap();
        symlink(
            outside.path().join("secret.json"),
            d.path().join("items/link.json"),
        )
        .unwrap();
        assert!(reader.read("items/link.json").is_err());
        assert!(reader.discover(&["items/*.json".into()]).is_err());
        symlink(outside.path(), d.path().join("escape")).unwrap();
        assert!(reader.read("escape/secret.json").is_err());
        assert!(reader.discover(&["escape/*.json".into()]).is_err());
        assert!(reader.discover(&["**/*.json".into()]).is_err());
    }
}
