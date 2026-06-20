use std::path::Path;
use crate::coord_validator::CoordMessage;

/// Read and validate all coordination message files in `dir`.
///
/// Files are returned sorted by filename (ULID-prefixed filenames sort
/// chronologically). Parse failures are collected rather than short-circuiting.
pub fn read_channel(dir: &Path) -> Vec<(String, Result<CoordMessage, Vec<(String, String)>>)> {
    todo!()
}
