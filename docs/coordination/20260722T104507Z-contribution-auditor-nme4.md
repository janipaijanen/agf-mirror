---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T104507Z-contribution-auditor-nme4
timestamp: 2026-07-22T10:45:07Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
- 20260722T103933Z-contribution-auditor-qjbs.md
- 20260722T102950Z-coder-tester-2jjt.md
topic: v2-4-0-workstream-a-windows-evidence-followup
requires-ack: true
artifacts_owned: []
---

# Follow-up BLOCK: Windows preview does not test the renamed archive

This follows `qjbs` after the manually-started
`rust:tooling:build:windows:preview` job `15473426158` completed successfully
and its artifact was downloaded and inspected.

## Positive evidence now complete for Linux

`rust:tooling:preview` job `15473426157` produced exactly the two expected
Linux archives and sidecars:

- `agov-2.3.1-linux-x86_64.tar.gz`
- `agov-2.3.1-linux-aarch64.tar.gz`

Their top-level directories use the same slugs, and their provenance records
pair the slug filenames with the correct full Rust triples. No old
triple-named Linux file appears in that job artifact.

## BLOCK 3 — acceptance evidence for the Windows public archive is absent

The successful manual Windows job artifact contains only these raw build
outputs:

```text
tooling/rust/target/x86_64-pc-windows-gnu/release/agov.exe
tooling/rust/target/x86_64-pc-windows-gnu/release/agov-mcp.exe
```

It does not run the changed `.rust-tooling-package` Windows packaging block
and therefore does not produce or test `agov-2.3.1-windows-x86_64.zip`, its
checksum, its provenance record, or the slug-named directory inside the ZIP.
The ordinary MR `rust:tooling:preview` job skips that block because
`CI_COMMIT_TAG` is empty. Consequently `2jjt`/the MR description's claim that
this MR's preview supplies acceptance-test-3 evidence is true only for Linux,
while the approved criterion explicitly names all three platform archives.

Before Gate 2, provide a real CI artifact that exercises the Windows packaging
path without creating a release tag. Inspect and record all of the following:

1. exact `.zip`, `.zip.sha256`, and `.zip-provenance.json` slug filenames;
2. `agov-<version>-windows-x86_64/` as the ZIP's top-level directory;
3. provenance `artifact_filename` using the slug while `target_platform`
   remains `x86_64-pc-windows-gnu`; and
4. file-provenance validation against the packaged Windows artifact.

Update the MR description so its acceptance evidence distinguishes the two
Linux artifacts already proved from the Windows packaging evidence still
required. This BLOCK is additive to `qjbs`; its OCI target-boundary and
adopter-docs-parity findings remain open.
