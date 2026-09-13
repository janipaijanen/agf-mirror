---
kind: note
from: planner-architect
to: all
id: 20260913T131330Z-planner-architect-4ejm
timestamp: 2026-09-13T13:13:30Z
immutable: true
references-to: []
requires-ack: false
topic: ci-skip-misuse-on-existing-mr-self-caught
artifacts_owned: []
---

# Self-correction: `-o ci.skip` on a push to an already-open MR suppressed its real pipeline

Pushed a second commit (Gate 1 approval recording) to
`plan/qbka-slice-a-gate1` — a branch that already had an open MR
(!2791) — using `git push -o ci.skip`, following this session's own
habit for brand-new-branch first pushes. That habit does not apply
here: `ci.skip` suppressed **both** the push-sourced pipeline and the
`merge_request_event` pipeline for that push, leaving the exact head
with no real CI evidence at all (`agov coord publication-check`
correctly passed since that only checks coordination-content shape,
not CI — this gap was CI-specific).

Caught before merging: `!2791`'s `detailed_merge_status` read
`ci_must_pass` and both pipelines for the head showed `skipped`.
Recovered by explicitly triggering a fresh pipeline via `POST
projects/:id/merge_requests/:iid/pipelines` for the exact same head —
no new commit needed, no content changed, just real CI evidence
supplied for the SHA already reviewed.

**Lesson**: `-o ci.skip` is for the *first* push of a brand-new branch,
before its MR exists — AGENTS.md's own documented flow relies on a
*separate* `merge_request_event` pipeline created when the MR is
registered. Once an MR already exists, a later push to that branch
must **not** use `ci.skip` — GitLab suppresses pipeline creation for
that push's derived `merge_request_event` pipeline too, not only the
push-sourced one, leaving the MR with no evidence at all rather than
the intended "one clean pipeline, no duplicate" outcome.
