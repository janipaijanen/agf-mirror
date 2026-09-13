//! Optional publication declarations. Validation never authenticates a role,
//! publication, release authorization, or permission to delay an obligation.
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime, UtcOffset};

type Errors = Vec<(String, String)>;

/// Canonical logical target identity, using Git's branch-name restrictions.
/// Ref existence and remote provenance are checked separately by the reader.
pub fn canonical_branch(value: &str) -> bool {
    let Some(name) = value.strip_prefix("refs/heads/") else {
        return false;
    };
    !name.is_empty()
        && name != "@"
        && !name.contains("..")
        && !name.contains("@{")
        && !name.ends_with('.')
        && !name
            .chars()
            .any(|c| c.is_control() || c == ' ' || "~^:?*[\\".contains(c))
        && name
            .split('/')
            .all(|part| !part.is_empty() && !part.starts_with('.') && !part.ends_with(".lock"))
}

pub fn utc_timestamp(value: &str) -> Option<OffsetDateTime> {
    let parsed = OffsetDateTime::parse(value, &Rfc3339).ok()?;
    (parsed.offset() == UtcOffset::UTC).then_some(parsed)
}

/// Cross-field checks that draft-07 cannot express. Called only after the
/// ordinary coordination schema has accepted field types and required keys.
/// Cross-record chronology/reference resolution belongs to the status reader.
pub fn validate_fields(raw: &Value) -> Result<(), Errors> {
    let Some(hold) = raw.get("publication_hold") else {
        return Ok(());
    };
    let mut errors = Vec::new();
    let mut error = |message: &str, pointer: &str| errors.push((message.into(), pointer.into()));
    if !hold
        .get("target_branch")
        .and_then(Value::as_str)
        .is_some_and(canonical_branch)
    {
        error(
            "hold target must be a canonical refs/heads/<branch> identity",
            "/publication_hold/target_branch",
        );
    }
    let start = raw
        .get("timestamp")
        .and_then(Value::as_str)
        .and_then(utc_timestamp);
    if start.is_none() {
        error("hold control timestamp must be RFC3339 UTC", "/timestamp");
    }
    match hold.get("action").and_then(Value::as_str) {
        Some("start") => {
            let expires = hold
                .get("expires_at")
                .and_then(Value::as_str)
                .and_then(utc_timestamp);
            match (start, expires) {
                (Some(start), Some(expires))
                    if expires > start && expires - start <= Duration::hours(24) => {}
                _ => error(
                    "hold expiry must be UTC, after its timestamp and at most 24 hours later",
                    "/publication_hold/expires_at",
                ),
            }
        }
        Some("end") => {
            let id = hold.get("hold_id").and_then(Value::as_str);
            if id.is_none()
                || !raw
                    .get("references-to")
                    .and_then(Value::as_array)
                    .is_some_and(|refs| refs.iter().any(|r| r.as_str() == id))
            {
                error(
                    "hold end must reference the exact hold_id in references-to",
                    "/references-to",
                );
            }
        }
        _ => error("unknown hold action", "/publication_hold/action"),
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord_validator::validate_content_with_schema;
    use agov_core::schema;
    use serde_json::json;

    fn base() -> Value {
        json!({"id":"fixture-normal", "kind":"note", "from":"coder-tester", "to":"all",
            "timestamp":"2026-09-12T12:00:00Z", "immutable":true, "references-to":[],
            "topic":"fixture", "requires-ack":false})
    }
    fn start() -> Value {
        let mut v = base();
        v["kind"] = json!("decision-record");
        v["from"] = json!("governor-reviewer");
        v["urgency"] = json!("urgent");
        v["references-to"] = json!(["release-work-order"]);
        v["publication_hold"] = json!({"action":"start","target_branch":"refs/heads/main",
            "target_sha":"0123456789abcdef0123456789abcdef01234567","expires_at":"2026-09-12T13:00:00Z"});
        v
    }
    fn valid(v: &Value) -> bool {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../agf/schemas/coordination-message.schema.json");
        let compiled = schema::load_and_compile(&path).unwrap();
        let text = format!("---\n{}\n---\n", serde_norway::to_string(v).unwrap());
        validate_content_with_schema(&text, &compiled).is_ok()
    }
    #[test]
    fn ordinary_records_remain_compatible_without_field_rewriting() {
        let v = base();
        assert!(valid(&v));
        assert!(v.get("urgency").is_none());
        for urgency in [json!("normal"), json!("urgent")] {
            let mut v = base();
            v["urgency"] = urgency;
            v["deferred_by"] = json!("some-hold");
            assert!(valid(&v));
        }
        for urgency in [json!(null), json!(true), json!("low"), json!([])] {
            let mut v = base();
            v["urgency"] = urgency;
            assert!(!valid(&v));
        }
        for association in [json!(null), json!(false), json!(""), json!([])] {
            let mut v = base();
            v["deferred_by"] = association;
            assert!(!valid(&v));
        }
    }
    #[test]
    fn control_authority_labels_and_exact_shapes_are_enforced() {
        assert!(valid(&start()));
        for (key, value) in [
            ("kind", json!("note")),
            ("from", json!("coder-tester")),
            ("urgency", json!("normal")),
            ("deferred_by", json!("other")),
            ("references-to", json!([])),
        ] {
            let mut v = start();
            v[key] = value;
            assert!(!valid(&v), "{key}");
        }
        let mut v = start();
        v.as_object_mut().unwrap().remove("urgency");
        assert!(!valid(&v));
        for (key, value) in [
            ("hold_id", json!("other")),
            ("extra", json!(true)),
            ("target_sha", json!("abc")),
            ("action", json!("extend")),
        ] {
            let mut v = start();
            v["publication_hold"][key] = value;
            assert!(!valid(&v), "{key}");
        }
        for value in [json!(null), json!([]), json!("start")] {
            let mut v = start();
            v["publication_hold"] = value;
            assert!(!valid(&v));
        }
    }
    #[test]
    fn duration_and_branch_identity_boundaries_are_validated() {
        for expiry in [
            "2026-09-12T12:00:00Z",
            "2026-09-12T11:59:59Z",
            "2026-09-13T12:00:01Z",
            "9999-12-31T23:59:59Z",
            "2026-09-12T13:00:00+01:00",
        ] {
            let mut v = start();
            v["publication_hold"]["expires_at"] = json!(expiry);
            assert!(!valid(&v), "{expiry}");
        }
        let mut v = start();
        v["publication_hold"]["expires_at"] = json!("2026-09-13T12:00:00Z");
        assert!(valid(&v));
        for target in [
            "main",
            "origin/main",
            "refs/heads/",
            "refs/heads/a..b",
            "refs/heads/a.lock",
            "refs/heads/a//b",
            "refs/heads/.hidden",
            "refs/heads/a@{b",
            "refs/heads/a b",
        ] {
            let mut v = start();
            v["publication_hold"]["target_branch"] = json!(target);
            assert!(!valid(&v), "{target}");
        }
        let mut v = start();
        v["publication_hold"]["target_branch"] = json!("refs/heads/release/v2.11");
        assert!(valid(&v));
    }
    #[test]
    fn end_requires_exact_reference_and_forbids_start_fields() {
        let mut v = start();
        v["publication_hold"] =
            json!({"action":"end","target_branch":"refs/heads/main","hold_id":"hold-start"});
        v["references-to"] = json!(["hold-start"]);
        assert!(valid(&v));
        let mut bad = v.clone();
        bad["references-to"] = json!(["hold-start.md"]);
        assert!(!valid(&bad));
        for (key, value) in [
            ("expires_at", json!("2026-09-12T13:00:00Z")),
            (
                "target_sha",
                json!("0123456789abcdef0123456789abcdef01234567"),
            ),
        ] {
            let mut bad = v.clone();
            bad["publication_hold"][key] = value;
            assert!(!valid(&bad));
        }
    }
}
