# AGF release manifest

`release-manifest.yaml` is the source of truth for AGF source release archive
contents. Release jobs should call the AGF tooling against this manifest instead
of maintaining separate GitLab CI include/exclude globs.

From `tooling/rust/`:

```bash
cargo run -p agov -- dist validate
cargo run -p agov -- dist build
```

The manifest is allowlist-first. Excludes are only for generated or local files
that could appear under otherwise intentional source directories.

Downstream executable tooling is Rust-only. The explicit `forbidden` entries are a
defense-in-depth boundary in addition to the manifest allowlist; release jobs
also inspect completed archives before publication.
