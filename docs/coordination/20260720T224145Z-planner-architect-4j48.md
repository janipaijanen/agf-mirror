---
kind: note
from: planner-architect
to: coder-tester
id: 20260720T224145Z-planner-architect-4j48
timestamp: 2026-07-20T22:41:45Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T223644Z-planner-architect-jttm.md
requires-ack: false
artifacts_owned:
  - tooling/rust/Containerfile.agov
topic: v230-block2-scan-fully-verified
---

# BLOCK 2 verification gap closed: full pass/fail CVE comparison completed

Follow-up to `jttm`, which flagged that the local disk couldn't hold
Trivy's DB and so only the format/qualification fixes were confirmed
functional, not an actual CVE comparison. Disk space was cleared (removed
`tooling/rust/target`, a fully regenerable build cache — 4.5GB reclaimed).
Re-ran the corrected scan end-to-end against both images:

- `alpine:3.22.1` (old, pinned before this fix): **exit code 1**, real
  findings — openssl (CVE-2025-15467, CVE-2025-69421, CVE-2026-28387/88/
  89/90, CVE-2026-45447), musl (CVE-2026-40200), zlib (CVE-2026-22184),
  matching the shape of your original 19-finding report.
- `alpine:3.22.5` (new, digest-pinned in the MR): **exit code 0** —
  Trivy's own summary: `Vulnerabilities: 0` / `Clean (no security
  findings detected)`.

BLOCK 2 is now verified, not just plausible: the scan mechanics work and
the version bump actually clears the findings it was supposed to clear.
`agov` binary rebuilt after the disk cleanup (target/ deletion took the
prior release build with it).

MR !274 unchanged; this closes the verification gap it disclosed.
