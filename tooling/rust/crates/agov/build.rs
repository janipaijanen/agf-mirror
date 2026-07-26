use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_COMMIT");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_DIRTY");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_SOURCE_DIRTY");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let repo_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(4)
        .unwrap_or_else(|| Path::new("."));
    rerun_if_git_path_changes(repo_root, "HEAD");
    rerun_if_git_path_changes(repo_root, "index");

    let commit = std::env::var("AGOV_BUILD_COMMIT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            git_output(repo_root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_string())
        });
    let dirty = std::env::var("AGOV_BUILD_DIRTY")
        .ok()
        .filter(|value| value == "true" || value == "false")
        .unwrap_or_else(|| {
            let clean =
                git_output(repo_root, &["status", "--porcelain"]).is_some_and(|out| out.is_empty());
            (!clean).to_string()
        });

    println!("cargo:rustc-env=AGOV_GIT_SHA={commit}");
    println!("cargo:rustc-env=AGOV_GIT_DIRTY={dirty}");
    let source_dirty = std::env::var("AGOV_BUILD_SOURCE_DIRTY")
        .ok()
        .filter(|value| value == "true" || value == "false")
        .unwrap_or_else(|| tracked_source_dirty(repo_root).to_string());
    println!("cargo:rustc-env=AGOV_SOURCE_DIRTY={source_dirty}");
}

fn rerun_if_git_path_changes(repo_root: &Path, name: &str) {
    if let Some(path) = git_output(repo_root, &["rev-parse", "--git-path", name]) {
        let path = Path::new(&path);
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            repo_root.join(path)
        };
        println!("cargo:rerun-if-changed={}", absolute.display());
    }
}

fn tracked_source_dirty(repo_root: &Path) -> bool {
    const PATHS: &[&str] = &[
        "tooling/agov",
        "tooling/rust",
        "agf/schemas",
        "agf/agents/MANIFEST.yaml",
    ];
    let mut unstaged = vec!["diff", "--quiet", "--"];
    unstaged.extend(PATHS);
    let mut staged = vec!["diff", "--cached", "--quiet", "--"];
    staged.extend(PATHS);
    !git_success(repo_root, &unstaged) || !git_success(repo_root, &staged)
}

fn git_success(repo_root: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .status()
        .is_ok_and(|status| status.success())
}

fn git_output(repo_root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
