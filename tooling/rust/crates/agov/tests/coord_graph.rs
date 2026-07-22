use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn coord_graph_empty_dir_exits_zero_emits_dot_header() {
    let fixture = Fixture::new();

    let output = agov(&[
        "coord",
        "graph",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("digraph agf_coordination {"));
    assert!(stdout.contains("rankdir=LR"));
    assert!(stdout.contains("}"));
}

#[test]
fn coord_graph_dot_emits_expected_edges() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-a001.md",
        &message(
            "20260621T000000Z-planner-architect-a001",
            "planner-architect",
            "coder-tester",
            "note",
        ),
    );
    fixture.write_coord(
        "20260621T000100Z-planner-architect-a002.md",
        &message(
            "20260621T000100Z-planner-architect-a002",
            "planner-architect",
            "coder-tester",
            "note",
        ),
    );
    fixture.write_coord(
        "20260621T000200Z-coder-tester-a003.md",
        &message(
            "20260621T000200Z-coder-tester-a003",
            "coder-tester",
            "governor-reviewer",
            "review",
        ),
    );

    let output = agov(&[
        "coord",
        "graph",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"planner-architect\" -> \"coder-tester\" [label=\"note (2)\"]"));
    assert!(stdout.contains("\"coder-tester\" -> \"governor-reviewer\" [label=\"review (1)\"]"));
    // governor-reviewer gets filled style
    assert!(stdout.contains("\"governor-reviewer\" [shape=box, style=filled"));
    // regular role gets box
    assert!(stdout.contains("\"planner-architect\" [shape=box]"));
}

#[test]
fn coord_graph_all_node_rendered_as_ellipse() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-a001.md",
        &message(
            "20260621T000000Z-planner-architect-a001",
            "planner-architect",
            "all",
            "note",
        ),
    );

    let output = agov(&[
        "coord",
        "graph",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"all\" [shape=ellipse]"));
}

#[test]
fn coord_graph_text_format_emits_readable_lines() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-a001.md",
        &message(
            "20260621T000000Z-planner-architect-a001",
            "planner-architect",
            "all",
            "note",
        ),
    );

    let output = agov(&[
        "coord",
        "graph",
        "--format",
        "text",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("planner-architect -> all  note  (1)"));
}

#[test]
fn coord_graph_missing_dir_exits_nonzero() {
    let output = agov(&[
        "coord",
        "graph",
        "--dir",
        "/tmp/agov_coord_graph_nonexistent_dir_xyz",
    ]);

    assert_ne!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("directory not found"));
}

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("run agov")
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "agov_coord_graph_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(root.join("coord")).unwrap();
        Self { root }
    }

    fn coord_dir(&self) -> PathBuf {
        self.root.join("coord")
    }

    fn write_coord(&self, name: &str, content: &str) -> PathBuf {
        let path = self.coord_dir().join(name);
        fs::write(&path, content).unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root")
        .to_path_buf()
}

fn message(id: &str, from: &str, to: &str, kind: &str) -> String {
    let timestamp = readable_timestamp(id);
    format!(
        r#"---
id: {id}
timestamp: {timestamp}
from: {from}
to: {to}
kind: {kind}
topic: coord graph test
immutable: true
references-to: []
requires-ack: false
---

# Coord graph test
"#
    )
}

fn readable_timestamp(id: &str) -> String {
    let raw = id.split('-').next().expect("readable id timestamp");
    format!(
        "{}-{}-{}T{}:{}:{}Z",
        &raw[0..4],
        &raw[4..6],
        &raw[6..8],
        &raw[9..11],
        &raw[11..13],
        &raw[13..15]
    )
}
