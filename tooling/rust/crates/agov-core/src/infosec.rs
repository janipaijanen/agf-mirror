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
    todo!()
}
