use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../../../.git/index");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_COMMIT");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_DIRTY");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_SOURCE_DIRTY");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let repo_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(4)
        .unwrap_or_else(|| Path::new("."));
    let commit = env_or("AGOV_BUILD_COMMIT").unwrap_or_else(|| {
        git_output(repo_root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into())
    });
    let dirty = bool_env("AGOV_BUILD_DIRTY").unwrap_or_else(|| {
        git_output(repo_root, &["status", "--porcelain"]).is_some_and(|out| !out.is_empty())
    });
    let source_dirty =
        bool_env("AGOV_BUILD_SOURCE_DIRTY").unwrap_or_else(|| tracked_source_dirty(repo_root));

    println!("cargo:rustc-env=AGOV_GIT_SHA={commit}");
    println!("cargo:rustc-env=AGOV_GIT_DIRTY={dirty}");
    println!("cargo:rustc-env=AGOV_SOURCE_DIRTY={source_dirty}");
}

fn env_or(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn bool_env(name: &str) -> Option<bool> {
    env_or(name).and_then(|value| match value.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })
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
        .is_ok_and(|s| s.success())
}

fn git_output(repo_root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
