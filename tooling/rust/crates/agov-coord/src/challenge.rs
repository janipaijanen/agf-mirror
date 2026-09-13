// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Jani Päijänen
//! Advisory challenge evaluation. Declared roles/sessions are not authentication.
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

#[derive(Clone, Debug)]
pub struct Record {
    pub file: String,
    pub raw: Value,
    pub valid: bool,
}
#[derive(Debug, Serialize, PartialEq)]
pub struct Row {
    pub file: String,
    pub challenge_id: String,
    pub state: String,
    pub detail: String,
    pub replies: Vec<String>,
}
fn field<'a>(v: &'a Value, name: &str) -> &'a str {
    v.get(name).and_then(Value::as_str).unwrap_or("")
}
fn resolution(r: &Record) -> &str {
    field(&r.raw["challenge"], "resolution")
}
fn raising(r: &Record) -> bool {
    matches!(resolution(r), "" | "open")
}
fn key(s: &str) -> &str {
    s.strip_suffix(".md").unwrap_or(s)
}
fn references(r: &Record, id: &str) -> bool {
    r.raw["references-to"].as_array().is_some_and(|refs| {
        refs.iter()
            .any(|v| v.as_str().is_some_and(|s| key(s) == key(id)))
    })
}
fn timestamp(r: &Record) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(field(&r.raw, "timestamp"), &Rfc3339).ok()
}

/// Stable field/link checks; observation-time eligibility belongs only in evaluate.
pub fn validate_fields(raw: &Value) -> Result<(), Vec<(String, String)>> {
    let Some(challenge) = raw.get("challenge") else {
        return Ok(());
    };
    if matches!(field(challenge, "resolution"), "" | "open") {
        let disputed = field(challenge, "disputed_record_id");
        let r = Record {
            file: String::new(),
            raw: raw.clone(),
            valid: true,
        };
        if disputed.is_empty() || !references(&r, disputed) {
            return Err(vec![(
                "raising challenge must reference its disputed_record_id".into(),
                "/challenge/disputed_record_id".into(),
            )]);
        }
    }
    Ok(())
}

/// Return collisions across raising records only; replies intentionally share the ID.
pub fn duplicate_ids(records: &[Record]) -> Vec<(String, String)> {
    let mut seen = BTreeMap::new();
    let mut errors = Vec::new();
    for r in records
        .iter()
        .filter(|r| r.raw.get("challenge").is_some() && raising(r))
    {
        let id = field(&r.raw["challenge"], "challenge_id");
        if let Some(first) = seen.insert(id, r.file.as_str()) {
            errors.push((
                r.file.clone(),
                format!("duplicate challenge_id '{id}' also raised by {first}"),
            ));
        }
    }
    errors
}

/// Deterministic reduction: source validity, activation time, then terminal/reply/expiry.
pub fn evaluate(records: &[Record], now: OffsetDateTime, enabled: bool) -> Vec<Row> {
    let mut identities: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
    let mut groups: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
    for r in records {
        identities
            .entry(key(field(&r.raw, "id")))
            .or_default()
            .push(r);
        if r.raw.get("challenge").is_some() {
            groups
                .entry(field(&r.raw["challenge"], "challenge_id"))
                .or_default()
                .push(r);
        }
    }
    let mut rows = Vec::new();
    for (id, group) in groups {
        let raises: Vec<_> = group.iter().copied().filter(|r| raising(r)).collect();
        if raises.len() != 1 {
            for r in &group {
                rows.push(Row {
                    file: r.file.clone(),
                    challenge_id: id.into(),
                    state: "needs-review".into(),
                    detail: "raising record missing or challenge_id ambiguous".into(),
                    replies: vec![],
                });
            }
            continue;
        }
        let r = raises[0];
        let mut row = Row {
            file: r.file.clone(),
            challenge_id: id.into(),
            state: "needs-review".into(),
            detail: String::new(),
            replies: vec![],
        };
        if !enabled {
            row.state = "disabled — needs review".into();
            row.detail = "challenge evaluation disabled; conservative visibility retained".into();
            rows.push(row);
            continue;
        }
        let disputed = key(field(&r.raw["challenge"], "disputed_record_id"));
        let source = identities
            .get(disputed)
            .filter(|v| v.len() == 1)
            .map(|v| v[0]);
        let identity_ok = identities
            .get(key(field(&r.raw, "id")))
            .is_some_and(|v| v.len() == 1);
        if !r.valid || !identity_ok || source.is_none_or(|s| !s.valid) || !references(r, disputed) {
            row.detail =
                "unavailable: raising/disputed source invalid, missing or ambiguous".into();
            rows.push(row);
            continue;
        }
        let Some(start) = timestamp(r) else {
            row.detail = "invalid raising timestamp".into();
            rows.push(row);
            continue;
        };
        let hours = r.raw["challenge"]["window_hours"]
            .as_i64()
            .filter(|h| (1..=720).contains(h));
        let deadline = hours.and_then(|h| start.checked_add(Duration::hours(h)));
        let Some(deadline) = deadline else {
            row.detail = "invalid window or deadline overflow".into();
            rows.push(row);
            continue;
        };
        if start > now {
            row.state = "not yet effective".into();
            row.detail = field(&r.raw, "timestamp").into();
            rows.push(row);
            continue;
        }
        let mut terminal = false;
        let mut replies = Vec::new();
        for candidate in group.iter().copied().filter(|c| !raising(c)) {
            let linked = references(candidate, field(&r.raw, "id"));
            if !linked {
                rows.push(Row {
                    file: candidate.file.clone(),
                    challenge_id: id.into(),
                    state: "needs-review".into(),
                    detail: "response does not reference its raising record".into(),
                    replies: vec![],
                });
                continue;
            }
            if !candidate.valid
                || !identities
                    .get(key(field(&candidate.raw, "id")))
                    .is_some_and(|v| v.len() == 1)
            {
                continue;
            }
            let Some(at) = timestamp(candidate).filter(|at| *at >= start && *at <= now) else {
                continue;
            };
            if resolution(candidate) == "escalated"
                && field(&candidate.raw, "from") == "governor-reviewer"
            {
                terminal = true;
            }
            if resolution(candidate) == "replied" {
                let session = field(&candidate.raw, "session");
                if !session.is_empty()
                    && (session == field(&r.raw, "session")
                        || session == field(&source.unwrap().raw, "session"))
                {
                    replies.push((at, candidate.file.clone()));
                }
            }
        }
        replies.sort();
        row.replies = replies.into_iter().map(|(_, file)| file).collect();
        if terminal {
            continue;
        }
        row.state = if !row.replies.is_empty() {
            "replied — decision pending"
        } else if now > deadline {
            "window expired — escalate to governor"
        } else {
            "open"
        }
        .into();
        row.detail = format!("deadline {}", deadline.format(&Rfc3339).unwrap_or_default());
        rows.push(row);
    }
    rows.sort_by(|a, b| (&a.challenge_id, &a.file).cmp(&(&b.challenge_id, &b.file)));
    rows
}

/// Retain malformed/unreadable inputs as diagnostics instead of treating failed reads as empty.
pub fn read_directory(dir: &std::path::Path) -> Result<(Vec<Record>, Vec<String>), String> {
    let schema = agov_core::schema::load_and_compile(
        &agov_core::schema_dir().join("coordination-message.schema.json"),
    )
    .map_err(|e| format!("{e:?}"))?;
    let mut records = Vec::new();
    let mut problems = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                problems.push(e.to_string());
                continue;
            }
        };
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let file = entry.file_name().to_string_lossy().to_string();
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                problems.push(format!("unavailable {file}: {e}"));
                continue;
            }
        };
        let raw = match agov_core::yaml::parse_frontmatter_json(&content) {
            Ok(v) => v,
            Err(e) => {
                problems.push(format!("unavailable {file}: {e}"));
                continue;
            }
        };
        let valid = crate::coord_validator::validate_content_with_schema(&content, &schema).is_ok();
        if !valid {
            problems.push(format!("invalid record: {file}"));
        }
        records.push(Record { file, raw, valid });
    }
    records.sort_by(|a, b| a.file.cmp(&b.file));
    problems.sort();
    Ok((records, problems))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn now() -> OffsetDateTime {
        OffsetDateTime::parse("2026-09-13T12:00:00Z", &Rfc3339).unwrap()
    }
    fn record(id: &str, session: &str) -> Record {
        Record {
            file: format!("{id}.md"),
            valid: true,
            raw: json!({"id":id,"kind":"review","from":"coder-tester","session":session,"timestamp":"2026-09-13T10:00:00Z","references-to":[]}),
        }
    }
    fn fixture() -> Vec<Record> {
        let source = record("source", "author");
        let mut raising = record("raising", "challenger");
        raising.raw["challenge"] =
            json!({"challenge_id":"one","disputed_record_id":"source","window_hours":2});
        raising.raw["references-to"] = json!(["source"]);
        vec![source, raising]
    }
    fn reply(id: &str, res: &str, session: &str) -> Record {
        let mut r = record(id, session);
        r.raw["timestamp"] = json!("2026-09-13T11:00:00Z");
        r.raw["challenge"] = json!({"challenge_id":"one","resolution":res});
        r.raw["references-to"] = json!(["raising"]);
        if res == "escalated" {
            r.raw["from"] = json!("governor-reviewer");
        }
        r
    }
    fn state(rs: &[Record], at: OffsetDateTime) -> String {
        evaluate(rs, at, true)[0].state.clone()
    }
    #[test]
    fn equality_expiry_and_reply_evidence() {
        let mut rs = fixture();
        assert_eq!(state(&rs, now()), "open");
        assert!(state(&rs, now() + Duration::seconds(1)).starts_with("window expired"));
        rs.push(reply("reply", "replied", "author"));
        assert_eq!(state(&rs, now()), "replied — decision pending");
        assert_eq!(
            state(&rs, now() + Duration::days(2)),
            "replied — decision pending"
        );
    }
    #[test]
    fn terminal_precedence_and_unavailable_source_guard() {
        let mut rs = fixture();
        rs.push(reply("a", "replied", "author"));
        rs.push(reply("z", "escalated", "governor"));
        assert!(evaluate(&rs, now(), true).is_empty());
        rs.remove(0);
        assert_eq!(state(&rs, now()), "needs-review");
    }
    #[test]
    fn session_mismatch_and_future_or_backdated_candidates_never_close() {
        let mut rs = fixture();
        rs.push(reply("third", "replied", "unrelated"));
        assert_eq!(state(&rs, now()), "open");
        let mut e = reply("end", "escalated", "governor");
        e.raw["timestamp"] = json!("2026-09-14T12:00:00Z");
        rs.push(e);
        assert_eq!(state(&rs, now()), "open");
        assert!(evaluate(&rs, now() + Duration::days(1), true).is_empty());
        rs.last_mut().unwrap().raw["timestamp"] = json!("2026-09-12T12:00:00Z");
        assert_eq!(state(&rs, now()), "open");
    }
    #[test]
    fn future_raising_overflow_and_disabled_are_visible() {
        let mut rs = fixture();
        rs[1].raw["timestamp"] = json!("2026-09-14T12:00:00Z");
        assert_eq!(state(&rs, now()), "not yet effective");
        assert_eq!(state(&rs, now() + Duration::days(1)), "open");
        rs[1].raw["timestamp"] = json!("9999-12-31T23:00:00Z");
        assert_eq!(state(&rs, now()), "needs-review");
        assert_eq!(
            evaluate(&rs, now(), false)[0].state,
            "disabled — needs review"
        );
    }
    #[test]
    fn ambiguous_identity_collisions_and_orphans_do_not_disappear() {
        let mut rs = fixture();
        rs.push(rs[0].clone());
        assert_eq!(state(&rs, now()), "needs-review");
        rs.pop();
        let mut other = rs[1].clone();
        other.file = "other.md".into();
        other.raw["id"] = json!("other");
        rs.push(other);
        assert_eq!(duplicate_ids(&rs).len(), 1);
        assert!(evaluate(&rs, now(), true)
            .iter()
            .all(|r| r.state == "needs-review"));
        let orphan = reply("orphan", "replied", "author");
        assert_eq!(state(&[orphan], now()), "needs-review");
    }
    #[test]
    fn context_citations_never_change_party_and_same_class_ties_are_stable() {
        let mut rs = fixture();
        rs.push(record("context", "other"));
        rs[1].raw["references-to"] = json!(["source", "context"]);
        rs.push(reply("context-reply", "replied", "other"));
        assert_eq!(state(&rs, now()), "open");
        rs.push(reply("z", "replied", "author"));
        rs.push(reply("a", "replied", "challenger"));
        assert_eq!(evaluate(&rs, now(), true)[0].replies, vec!["a.md", "z.md"]);
        rs.reverse();
        assert_eq!(evaluate(&rs, now(), true)[0].replies, vec!["a.md", "z.md"]);
    }
}
