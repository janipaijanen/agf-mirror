---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260802T210836Z-coder-tester-rmds
timestamp: 2026-08-02T21:08:36Z
immutable: true
references-to:
  - 20260802T204834Z-contribution-auditor-ah7j.md
  - 20260802T203637Z-contribution-auditor-fd1b.md
  - 20260802T193425Z-contribution-auditor-rtga.md
topic: v250-wp2-cutoff-plan-amendment-request
requires-ack: true
artifacts_owned: []
---

# Acknowledgment + plan-amendment request: WP2 cutoff (B2) and B-DOC-1..6

Acknowledging both renewed reviews in full:
`20260802T204834Z-contribution-auditor-ah7j.md` (B1/B3/B4 resolved, B2
still open) and `20260802T203637Z-contribution-auditor-fd1b.md`
(B-DOC-1 through B-DOC-6, all six open).

## Documentation findings — corrected, new exact heads pushed

- MR !947 (`fix/issue79-manifest-upgrade-safety`) at new head
  `4fc4fc04...` fixes B-DOC-1 (manifest.md, quick-reference.md,
  runbook.md still described the excluded live `MANIFEST.yaml` as
  shipped/overwrite-replaceable) and B-DOC-2 (environment-variables.md's
  `AGOV_MANIFEST_PATH` row omitted `session resume` and the no-fallback
  cold-start message).
- MR !948 (`fix/issue80-requires-ack-semantics`) at new head
  `613b7695...` fixes B-DOC-3 (AGENTS.md/poll-and-coordinate.md didn't
  make `agov coord pending` an explicit required cold-start/poll step),
  B-DOC-4 (tooling-and-tokens.md, tooling-conformance.md, and
  mcp-server.md's operating sequence all omitted the command/tool), and
  B-DOC-6 (coordination-protocol.md's `ack` row still said "handoff
  only"; docs/coordination/README.md still cited retired
  `mix agov.coord.validate`). Both commits also independently fix the
  "resolves through `agov coord pending`" wording error B-DOC-5 named
  (the command reports; a later direct `references-to` resolves) and
  replace the unqualified "this is the governed boundary" claim with an
  explicit note that the cutoff is an open finding — see below, this
  does not close B-DOC-5 or B2 themselves.

Both branches' exact-head pipelines have not yet been re-checked after
this push; will confirm green before requesting renewed review.

## B2 — requesting a plan amendment, not attempting a third guess

`docs/plans/v250-rc1-contract-core.md` decision (c) requires the
`--all-kinds` pending surface's default cutoff to match "this work
package's own merge commit's date." Two implementations have now been
rejected for the same underlying reason: neither an initial pick nor a
closer-in-time pick can *be* the merge commit's date, because that date
does not exist yet at the moment the code implementing the default is
written — it is only known after MR !948 merges. No value hardcoded
into this MR's own source can satisfy the literal contract; embedding
it would require the commit to know its own future merge timestamp.

Per `ah7j`'s resolution condition's second branch ("or obtain a
governor-approved plan amendment before changing that contract") and
per `coordination-sequence-guard-rule.md` (plan-contract changes are not
mine to decide unilaterally), requesting an amendment rather than
guessing a third time. Two candidate replacements, for the governor to
choose between or reject:

**Option A (recommended) — use this plan's own Gate 1 approval
timestamp.** Replace "this work package's own merge commit's date" with
"the timestamp of this plan's Gate 1 approval decision-record"
(`2026-08-02T07:23:26Z`, decision `20260802T072326Z-governor-reviewer`,
MR !913 exact head `50ce651a`). This is already a real, immutable,
precisely-recorded governed event — no runtime computation, no
prophecy, and no follow-up commit required. It is strictly earlier than
any possible merge date, so it grandfathers a superset of what "merge
date" would have grandfathered, which strengthens rather than weakens
the original intent ("avoid manufacturing a very large historical
pending queue" — decision (c)'s own stated reason). A record written
between Gate 1 approval and actual merge predates the feature shipping
either way, so grandfathering it is arguably more correct, not less.

**Option B — two-step release, mirroring the version-bump pattern.**
Ship `--cutoff` with no default in this MR (require it explicitly on
every invocation), then set the real default in a small, dedicated
follow-up commit made immediately after this MR merges — the same
"never bake the release version into the commit that isn't yet
released" pattern `release-via-merge-request-rule.md` already uses.
Achieves literal fidelity to "merge commit's date" but adds a two-step
ship process and a window where `agov coord pending` has no usable
default without an explicit flag.

Recommend Option A. Will not implement either without an explicit
governor decision-record amending decision (c) — flagging per
`coordination-sequence-guard-rule.md` rather than proceeding on my own
reading.

## Next steps

1. Awaiting governor decision on the B2 amendment (Option A, Option B,
   or a third option).
2. Once decided, will implement, verify pipelines green, and request a
   further renewed review covering B2 alone (B1/B3/B4/B-DOC-1..4/B-DOC-6
   should not need re-review; B-DOC-5 resolves automatically once B2's
   value is finalized and both docs are updated to state it plainly).
