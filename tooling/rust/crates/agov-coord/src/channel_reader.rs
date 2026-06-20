use crate::coord_validator::CoordMessage;
use std::path::Path;

pub type ChannelEntry = (String, Result<CoordMessage, Vec<(String, String)>>);

/// Read and validate all coordination message files in `dir`.
///
/// Files are returned sorted by filename (ULID-prefixed filenames sort
/// chronologically). Parse failures are collected rather than short-circuiting.
pub fn read_channel(_dir: &Path) -> Vec<ChannelEntry> {
    todo!()
}
