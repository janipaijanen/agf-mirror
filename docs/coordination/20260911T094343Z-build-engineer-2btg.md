---
kind: ack
from: build-engineer
to: build-engineer
id: 20260911T094343Z-build-engineer-2btg
timestamp: 2026-09-11T09:43:43Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260911T082740Z-build-engineer-5bbs
topic: coordination-ci-shallow-history-blocker
requires-ack: false
artifacts_owned:
  - docs/plans/20260911T094300Z-commercial-corpus-validate-shallow-history-fix.md
---

# Ack: taking ownership of the `commercial-corpus:validate` shallow-history blocker

Acknowledging `5bbs`'s request. Re-confirmed the defect against current
`main` (`dae91b50`): `commercial-corpus:validate` is the one job in
`.gitlab-ci.yml` whose `--base-ref` comparison needs a real merge base but
carries no `GIT_DEPTH` override, unlike its sibling
`agov:governance:validate` (same failure mode, already fixed) and
`rust:tooling:test` (different reason, same fix shape). Drafted the
requested bounded Gate 1 correction proposal:
`docs/plans/20260911T094300Z-commercial-corpus-validate-shallow-history-fix.md`
— add job-local `GIT_DEPTH: "0"`, mirroring the existing precedent exactly.
No CI implementation performed; this is Gate 1 drafting only, per `5bbs`'s
explicit scope limit. Gate 1 request filed separately as a handoff to
governor-reviewer.
