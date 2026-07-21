---
kind: note
from: coder-tester
to: all
id: 20260721T094905Z-coder-tester-tbrj
timestamp: 2026-07-21T09:49:05Z
immutable: true
references-to:
  - 20260721T094458Z-coder-tester-k3er
  - 20260721T093448Z-coder-tester-7mhk
topic: v2-3-0-checksum-consumer-path-fix
requires-ack: false
artifacts_owned: []
---

# Ack: checksum consumer-path BLOCK (k3er) — fix filed as MR !308

Both findings in `k3er` confirmed correct on independent inspection.

1. **Checksum path defect** — reproduced locally: the Rust tooling job's
   `sha256sum "../../dist/${archive}"` embeds the invocation path into the
   checksum file, breaking `sha256sum -c` when the checksum and archive sit
   side by side (the standard consumer layout). Fixed in `.gitlab-ci.yml`
   (both occurrences, the shared `.rust-tooling-package` template) by
   cd-ing into the dist directory before hashing, matching the pattern
   `agov dist build` already uses correctly for framework archives. Verified
   the fix against a real `sha256sum -c` consumer check (`OK`, exit 0)
   before committing. Filed as **MR !308**
   (`fix/rust-checksum-consumer-path` -> `main`), same governed pattern as
   MR !303 (separate branch, not committed directly onto the frozen
   `release-v2.3.0` branch). Not self-merging — filing for review, same as
   !303.

2. **Stale branch-tip statement in `7mhk`** — confirmed: `7mhk` said "the
   branch tip is now `98c3b6f`", but `bc57265` (the commit carrying `7mhk`
   itself) is the actual tip. Noting the correction here; will be restated
   correctly in the next Gate 2 handoff.

## Next steps

Once MR !308 merges: rebase/re-cut `release-v2.3.0` onto the new `main`
tip, re-run the full validation matrix, and publish a corrected Gate 2
handoff with the new content head and regenerated preview-archive evidence
(the checksum fix specifically needs a fresh CI-built archive to confirm,
not just the local reproduction above). MR !306 stays open in the meantime;
not merging it with the known-bad checksums.
