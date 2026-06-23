/// Parsed components of a readable coordination id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadableIdParts<'a> {
    pub timestamp: &'a str,
    pub role: &'a str,
    pub suffix: &'a str,
}

/// Generate a readable coordination id for `role`.
pub fn readable_id(role: &str, now: time::OffsetDateTime) -> String {
    format!(
        "{}-{}-{}",
        readable_timestamp(now),
        role_slug(role),
        readable_suffix()
    )
}

/// Parse a readable coordination id.
pub fn parse_readable_id(id: &str) -> Option<ReadableIdParts<'_>> {
    let (timestamp, rest) = id.split_once('-')?;
    if timestamp.len() != 16
        || !timestamp.ends_with('Z')
        || !timestamp[..8].chars().all(|ch| ch.is_ascii_digit())
        || &timestamp[8..9] != "T"
        || !timestamp[9..15].chars().all(|ch| ch.is_ascii_digit())
    {
        return None;
    }

    let (role, suffix) = rest.rsplit_once('-')?;
    if role.is_empty()
        || !role
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        || !role
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
        || !(4..=6).contains(&suffix.len())
        || !suffix
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
    {
        return None;
    }

    Some(ReadableIdParts {
        timestamp,
        role,
        suffix,
    })
}

pub fn is_readable_id(id: &str) -> bool {
    parse_readable_id(id).is_some()
}

pub fn role_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;

    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "unknown".to_string()
    } else {
        slug
    }
}

pub fn readable_timestamp(now: time::OffsetDateTime) -> String {
    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub fn iso_timestamp(now: time::OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub fn readable_timestamp_from_iso(timestamp: &str) -> Option<String> {
    if timestamp.len() >= 20
        && timestamp.as_bytes().get(4) == Some(&b'-')
        && timestamp.as_bytes().get(7) == Some(&b'-')
        && timestamp.as_bytes().get(10) == Some(&b'T')
        && timestamp.as_bytes().get(13) == Some(&b':')
        && timestamp.as_bytes().get(16) == Some(&b':')
        && timestamp.ends_with('Z')
    {
        Some(format!(
            "{}{}{}T{}{}{}Z",
            &timestamp[0..4],
            &timestamp[5..7],
            &timestamp[8..10],
            &timestamp[11..13],
            &timestamp[14..16],
            &timestamp[17..19]
        ))
    } else {
        None
    }
}

pub fn readable_suffix() -> String {
    let alphabet = b"0123456789abcdefghjkmnpqrstvwxyz";
    let ulid = agov_core::ulid_util::new();

    ulid.bytes()
        .rev()
        .take(4)
        .map(|byte| alphabet[(byte as usize) % alphabet.len()] as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn role_slug_normalises_role_names() {
        assert_eq!(role_slug("coder-tester"), "coder-tester");
        assert_eq!(role_slug("Planner Architect"), "planner-architect");
        assert_eq!(role_slug("__Governor Reviewer__"), "governor-reviewer");
        assert_eq!(role_slug(""), "unknown");
    }

    #[test]
    fn timestamps_share_one_format_contract() {
        let now = datetime!(2026-06-23 18:17:38 UTC);

        assert_eq!(readable_timestamp(now), "20260623T181738Z");
        assert_eq!(iso_timestamp(now), "2026-06-23T18:17:38Z");
        assert_eq!(
            readable_timestamp_from_iso("2026-06-23T18:17:38Z"),
            Some("20260623T181738Z".to_string())
        );
    }

    #[test]
    fn parses_readable_id_parts() {
        assert_eq!(
            parse_readable_id("20260623T181738Z-coder-tester-dkhm"),
            Some(ReadableIdParts {
                timestamp: "20260623T181738Z",
                role: "coder-tester",
                suffix: "dkhm"
            })
        );
    }

    #[test]
    fn rejects_non_readable_ids() {
        assert!(!is_readable_id("01KVTBQZTAV34BVW5N8KHECESY"));
        assert!(!is_readable_id("20260623T181738Z-Coder-Tester-dkhm"));
        assert!(!is_readable_id("20260623T181738Z-coder-tester-dk"));
    }

    #[test]
    fn generates_readable_id_with_expected_prefix() {
        let now = datetime!(2026-06-23 18:17:38 UTC);
        let id = readable_id("Coder Tester", now);

        assert!(id.starts_with("20260623T181738Z-coder-tester-"));
        assert!(is_readable_id(&id));
    }
}
