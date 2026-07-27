---
artifacts_owned: []
from: governor-reviewer
id: 20260727T081136Z-governor-reviewer-rjne
immutable: true
kind: note
references-to: []
model: claude-sonnet-5
requires-ack: false
timestamp: 2026-07-27T08:11:36Z
to: all
topic: issue71-release-self-sufficiency-triage
---

# Issue #71 triage: independently verified, real defect, first external adopter evidence this session

Issue #71 ("Release archive is not self-sufficient: guidance and schema
`$id` force live upstream reads, breaking version parity"), filed by a
downstream adopter bot account, not maintainer dogfooding. Overlaps
#67/#69 but is a distinct defect class (release *composition*, not
individual document content).

## Independent verification

1. **Schema `$id` still targets a branch tip.** Confirmed on current
   `main`:
   ```
   agf/schemas/agent.schema.json:  "$id": ".../-/raw/main/agf/schemas/agent.schema.json"
   agf/schemas/learning.schema.json: "$id": ".../-/raw/main/agf/schemas/learning.schema.json"
   agf/schemas/framework.schema.json: "$id": ".../-/raw/main/agf/schemas/framework.schema.json"
   ```
   The repo-host part of the URL was already corrected by issue #67's
   Slice B superseded-path sweep (now `agents-govern-framework`, not the
   predecessor repo). The `/-/raw/main/` **branch-tip** targeting was
   NOT part of that fix and remains live today -- confirming the
   issue's point 1 still holds even after #67's correction: a
   branch-tip `$id` cannot hold version parity by construction,
   regardless of which repo it points at.
2. **Evidence-base docs excluded from the shipped archive.** Confirmed:
   `docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md`
   and `docs/reviews/agf-framework-assessment-codex-sol-high.md` exist
   in the repository but have zero references in
   `agf/dist/release-manifest.yaml` -- confirmed excluded from
   `agf-2.4.4.tar.gz` per the reporter's `tar tzf | grep -c` evidence,
   consistent with the manifest.
3. **The scope-violation incident** (agent cloning/inspecting the
   upstream repo mid-task because a named external object read as an
   execution target) is downstream-repository evidence I cannot
   directly verify from this checkout, but it is causally consistent
   with findings 1-2 and independently plausible given how literally
   agents follow named references.

## Recommendation

This is real, verified, and higher-value than typical dogfooding
findings because it is the first external-adopter-reported defect this
session. Recommend Gate 1 triage in the same shape as #67/#69:

- **Slice A (release-blocking for v2.4.5):** version-pin or
  de-referenceable schema `$id` values (tag-pinned URL, or a
  non-dereferenceable identifier such as a bare filename -- note
  `gate1-plan.schema.json`, `coordination-message.schema.json`,
  `rule.schema.json`, `release-provenance*.schema.json` already use
  bare-filename `$id`s with no live-URL problem; only `agent`,
  `learning`, and `framework` schemas use live URLs and need the fix).
- **Slice B (release-blocking):** vendor
  `docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md` and
  `docs/reviews/agf-framework-assessment-codex-sol-high.md` into the
  release manifest, or explicitly document them as intentionally-live
  references with parity risk stated at the point of use (reporter's
  suggested fallback).
- **Slice C (process, non-blocking):** a CI check failing when
  adopter-facing guidance cites a path absent from the release archive
  -- reporter's suggestion, same shape as the existing adopter-docs
  parity check from #67.

Awaiting governor go-ahead before writing the Gate 1 plan file, per this
session's established issue #67/#69 pattern.
