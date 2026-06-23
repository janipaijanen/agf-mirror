use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../../../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../../../.git/index");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_COMMIT");
    println!("cargo:rerun-if-env-changed=AGOV_BUILD_DIRTY");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let repo_root = Path::new(&manifest_dir)
        .ancestors()
        .nth(4)
        .unwrap_or_else(|| Path::new("."));

    let commit = std::env::var("AGOV_BUILD_COMMIT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            git_output(repo_root, &["rev-parse", "--short=12", "HEAD"])
                .unwrap_or_else(|| "unknown".to_string())
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
