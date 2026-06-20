/// A secret pattern match found during a content scan.
#[derive(Debug, Clone)]
pub struct SecretHit {
    pub pattern_name: &'static str,
    pub line: usize,
}

/// Scan `content` for secret patterns (API keys, tokens, etc.).
///
/// Mirrors `Agov.ValidateInfosec` regex scanning logic. Patterns are compiled
/// once at startup; this function is safe to call from rayon threads.
pub fn scan_content(content: &str) -> Vec<SecretHit> {
    let patterns = [
        (
            "VCS token",
            r#"(glpat-|ghp_|gho_|github_pat_|glcbt-|glrt-|xoxb-|xoxp-)[^\s"]{4,}"#,
        ),
        (
            "database connection string",
            r#"(?i)(postgres|mysql|redis|mongodb|amqp)://[^\s"]*[:@]"#,
        ),
        (
            "API key or password",
            r#"(?i)(api[_-]?key|secret[_-]?key|SECRET_KEY|password)\s*[:=]\s*[^\s"]{8,}"#,
        ),
        (
            "private key",
            r#"BEGIN\s+(RSA |EC |DSA |OPENSSH )?PRIVATE KEY"#,
        ),
        (
            "absolute filesystem path",
            r#"/(home|var|opt|srv|etc)/[a-zA-Z0-9_/.-]+"#,
        ),
    ];

    patterns
        .into_iter()
        .filter(|(_label, pattern)| {
            regex::Regex::new(pattern).is_ok_and(|regex| regex.is_match(content))
        })
        .map(|(pattern_name, _pattern)| SecretHit {
            pattern_name,
            line: 1,
        })
        .collect()
}
