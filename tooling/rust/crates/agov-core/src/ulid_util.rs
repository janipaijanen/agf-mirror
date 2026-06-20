/// Generate a new ULID string in the standard 26-character Crockford base-32 encoding.
pub fn new() -> String {
    ulid::Ulid::new().to_string()
}
