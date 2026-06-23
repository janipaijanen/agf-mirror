use std::process::Command;

#[test]
fn coord_brief_defaults_to_readable_ids() {
    let fixture = tempfile::tempdir().unwrap();

    let output = agov(&[
        "coord",
        "brief",
        "--from",
        "coder-tester",
        "--to",
        "governor-reviewer",
        "--kind",
        "note",
        "--topic",
        "readable-default-test",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("from: coder-tester"));
    assert!(stdout.contains("timestamp: 20"));

    let id = frontmatter_value(&stdout, "id").expect("generated id");
    assert_readable_id(&id);
    assert!(stdout.contains(&format!("# Suggested file: {id}.md")));
}

#[test]
fn coord_brief_keeps_ulid_compatibility_option() {
    let fixture = tempfile::tempdir().unwrap();

    let output = agov(&[
        "coord",
        "brief",
        "--id-format",
        "ulid",
        "--from",
        "coder-tester",
        "--to",
        "governor-reviewer",
        "--kind",
        "note",
        "--topic",
        "ulid-compat-test",
        "--dir",
        fixture.path().to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);

    let id = frontmatter_value(&stdout, "id").expect("generated id");
    assert_eq!(id.len(), 26);
    assert!(id.chars().all(|ch| matches!(
        ch,
        '0'..='9' | 'A'..='H' | 'J'..='K' | 'M'..='N' | 'P'..='T' | 'V'..='Z'
    )));
    assert!(stdout.contains(&format!("# Suggested file: {id}-coder-tester.md")));
    assert!(stdout.contains("timestamp: <TIMESTAMP>"));
}

fn frontmatter_value(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let (line_key, value) = line.split_once(": ")?;
        (line_key == key).then(|| value.to_string())
    })
}

fn assert_readable_id(id: &str) {
    let (timestamp, rest) = id.split_once('-').expect("timestamp separator");
    let (role, suffix) = rest.rsplit_once('-').expect("suffix separator");

    assert_eq!(timestamp.len(), 16);
    assert_eq!(&timestamp[8..9], "T");
    assert!(timestamp.ends_with('Z'));
    assert!(timestamp[..8].chars().all(|ch| ch.is_ascii_digit()));
    assert!(timestamp[9..15].chars().all(|ch| ch.is_ascii_digit()));
    assert_eq!(role, "coder-tester");
    assert_eq!(suffix.len(), 4);
    assert!(suffix
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit()));
}

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .args(args)
        .output()
        .unwrap()
}
