use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
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
            "agov_coord_graph_index_{}_{}",
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

fn note(id: &str, from: &str, to: &str) -> String {
    let ts = readable_timestamp(id);
    format!(
        "---\nid: {id}\nkind: note\nfrom: {from}\nto: {to}\ntimestamp: {ts}\n\
         immutable: true\nreferences-to: []\ntopic: test note\nrequires-ack: false\n---\n\
         \n# Test note\n"
    )
}

fn note_with_body_ref(id: &str, from: &str, to: &str, body_ref: &str) -> String {
    let ts = readable_timestamp(id);
    format!(
        "---\nid: {id}\nkind: note\nfrom: {from}\nto: {to}\ntimestamp: {ts}\n\
         immutable: true\nreferences-to: []\ntopic: test note with body ref\nrequires-ack: false\n---\n\
         \n# Test note\n\nSee {body_ref}\n"
    )
}

fn handoff(id: &str, from: &str, to: &str) -> String {
    let ts = readable_timestamp(id);
    let header = format!(
        "---\nid: {id}\nkind: handoff\nfrom: {from}\nto: {to}\ntimestamp: {ts}\n\
         immutable: true\nreferences-to: []\ntopic: test handoff\nrequires-ack: false\n"
    );
    let block = format!(
        "handoff:\n  next_agent: {to}\n  status: ready\n  task:\n    - do the thing\n  must_read: []\n  do_not_touch: []\n"
    );
    format!("{header}{block}---\n\n# Test handoff\n")
}

fn ack(id: &str, from: &str, to: &str, references: &[&str]) -> String {
    let ts = readable_timestamp(id);
    let refs = references
        .iter()
        .map(|r| format!("  - {r}.md"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nid: {id}\nkind: ack\nfrom: {from}\nto: {to}\ntimestamp: {ts}\n\
         immutable: true\nreferences-to:\n{refs}\ntopic: test ack\nrequires-ack: false\n---\n\
         \n# Test ack\n"
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

// ── Tests ──────────────────────────────────────────────────────────────────

#[test]
fn coord_graph_index_empty_dir_exits_zero_valid_json() {
    let fixture = Fixture::new();

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0), "expected exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is valid JSON");
    assert!(v["nodes"].as_array().unwrap().is_empty());
    assert!(v["edges"].as_array().unwrap().is_empty());
    assert!(v["pending"].as_array().unwrap().is_empty());
    assert!(v["invalid_files"].as_array().unwrap().is_empty());
    assert!(v["generated_at"].as_str().is_some());
}

#[test]
fn coord_graph_index_note_has_null_resolved() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-n001.md",
        &note(
            "20260621T000000Z-planner-architect-n001",
            "planner-architect",
            "coder-tester",
        ),
    );

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    let nodes = v["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert!(
        nodes[0]["resolved"].is_null(),
        "note must have resolved: null, got: {}",
        nodes[0]["resolved"]
    );
}

#[test]
fn coord_graph_index_resolved_handoff_has_resolved_true() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-h001.md",
        &handoff(
            "20260621T000000Z-planner-architect-h001",
            "planner-architect",
            "coder-tester",
        ),
    );
    fixture.write_coord(
        "20260621T000100Z-coder-tester-a001.md",
        &ack(
            "20260621T000100Z-coder-tester-a001",
            "coder-tester",
            "planner-architect",
            &["20260621T000000Z-planner-architect-h001"],
        ),
    );

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    let nodes = v["nodes"].as_array().unwrap();
    let handoff_node = nodes
        .iter()
        .find(|n| n["kind"].as_str() == Some("handoff"))
        .expect("handoff node present");
    assert_eq!(
        handoff_node["resolved"].as_bool(),
        Some(true),
        "handoff resolved by ack should be true"
    );
    assert!(v["pending"].as_array().unwrap().is_empty());
}

#[test]
fn coord_graph_index_pending_handoff_in_pending_list() {
    let fixture = Fixture::new();
    fixture.write_coord(
        "20260621T000000Z-planner-architect-h002.md",
        &handoff(
            "20260621T000000Z-planner-architect-h002",
            "planner-architect",
            "coder-tester",
        ),
    );

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    let nodes = v["nodes"].as_array().unwrap();
    let handoff_node = nodes
        .iter()
        .find(|n| n["kind"].as_str() == Some("handoff"))
        .expect("handoff node present");
    assert_eq!(
        handoff_node["resolved"].as_bool(),
        Some(false),
        "unresolved handoff should be false"
    );
    let pending = v["pending"].as_array().unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(
        pending[0]["file"].as_str(),
        Some("20260621T000000Z-planner-architect-h002.md")
    );
}

#[test]
fn coord_graph_index_output_file_writes_json() {
    let fixture = Fixture::new();
    let out_path = fixture.root.join("index.json");

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
        "--output",
        out_path.to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("wrote graph index to"),
        "stdout should confirm write, got: {stdout}"
    );
    let content = fs::read_to_string(&out_path).expect("output file written");
    let v: serde_json::Value = serde_json::from_str(&content).expect("output file is valid JSON");
    assert!(v["nodes"].is_array());
}

#[test]
fn coord_graph_index_missing_dir_exits_nonzero() {
    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        "/tmp/agov_coord_graph_index_nonexistent_xyz",
    ]);

    assert_ne!(
        output.status.code(),
        Some(0),
        "missing dir should exit nonzero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("directory not found"),
        "expected 'directory not found' in stderr, got: {stderr}"
    );
}

#[test]
fn coord_graph_index_body_ref_emits_body_reference_edge() {
    let fixture = Fixture::new();
    // Write a handoff
    fixture.write_coord(
        "20260621T000000Z-planner-architect-h003.md",
        &handoff(
            "20260621T000000Z-planner-architect-h003",
            "planner-architect",
            "coder-tester",
        ),
    );
    // Write a note that references the handoff only in body text (bare ID, no .md)
    fixture.write_coord(
        "20260621T000100Z-coder-tester-n002.md",
        &note_with_body_ref(
            "20260621T000100Z-coder-tester-n002",
            "coder-tester",
            "planner-architect",
            "20260621T000000Z-planner-architect-h003",
        ),
    );

    let output = agov(&[
        "coord",
        "graph-index",
        "--dir",
        fixture.coord_dir().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    let edges = v["edges"].as_array().unwrap();
    let body_edge = edges.iter().find(|e| {
        e["type"].as_str() == Some("body-reference")
            && e["to"].as_str() == Some("20260621T000000Z-planner-architect-h003.md")
    });
    assert!(
        body_edge.is_some(),
        "body-reference edge to handoff expected, edges: {edges:?}"
    );
}
