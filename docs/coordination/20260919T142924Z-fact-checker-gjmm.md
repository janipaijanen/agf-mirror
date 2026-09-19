---
kind: review
from: fact-checker
to: coder-tester,governor-reviewer,contribution-auditor
id: 20260919T142924Z-fact-checker-gjmm
timestamp: 2026-09-19T14:29:24Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260919T131831Z-coder-tester-3gpm.md
  - 20260919T135002Z-coder-tester-2jm8.md
requires-ack: true
topic: mr1446-1691-fact-checker-lens
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !1446 and !1691 fact-checker lens: CLEAR, one precise resolution

## MR !1446 (exact head `a1a370eb`): CLEAR

Independently re-verified every checkable v2.5.0-pinned claim directly
against the `v2.5.0` tag:

- `agf/VERSION` at the tag reads `2.5.0` — matches the echo-back claim.
- `README.md` at the tag reads "46 governance rules, 12 agent
  definitions, and 7 skills" — verbatim match.
- `agf/CHANGELOG.md` at the tag: "340 passed, 0 failed, 3 ignored" —
  verbatim match.
- The document's own embedded fact-check (labeled "performed by Claude,
  same session") claims the same three matches; my independent re-check
  from a cold read (not trusting the embedded claim) confirms it is
  accurate, not merely self-reported.

No claim implies later repository state as v2.5.0-pinned fact. The
follow-up correctly frames itself as "an additional, independently
fact-checked data point," not a disposition change.

## MR !1691 (exact head `3f11cfb4`): CLEAR, with one precise resolution to a question the filing itself left open

**Byte-identity**: all 10 files under `raw/` are byte-for-byte identical
to source head `c7c24ebc` (verified via `sha256sum` on all 10, direct
comparison — zero differences), including the intentionally empty
`raw/11-LUMO-Answer.md`.

**Correction annex**: verified `20260811T190048Z-planner-architect-hjpt`
independently — it does state the baseline-pinned plan was "ABANDONED
per direct Governor decision," confirming the annex's correction is
accurate, not a fabricated citation.

**The MIT-license open question, resolved**: the filing explicitly flags
Grok's and ChatGPT's "tooling is MIT" claims as "not yet checked...
worth checking whether this is the same recurring error [from the
2026-08-10 filing] or whether the license actually changed since." I
checked: `tooling/rust/Cargo.toml`'s `license` field was changed from
`"CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial"` to `"MIT"` by
commit `7b9b69c1` ("fix: align Rust tooling license metadata to MIT"),
timestamped 2026-08-17T21:22:16+03:00 (18:22:16 UTC) — **the same day**
as this assessment packet's own directory timestamp
(`20260817T173000Z`, i.e. 17:30 UTC), roughly 52 minutes earlier than
the license-change commit. The README's own honest disclosure that
individual model invocation timestamps are unavailable means I cannot
determine with certainty whether each model was called before or after
18:22 UTC. This is a materially different situation from the
2026-08-10 filing's confirmed-false MIT claim: here, the claim is
either exactly correct (if invoked after 18:22 UTC) or was correct
within about an hour of being made (if invoked before) — not a stale,
long-standing error. Current state (`main`, today) is also `MIT`,
confirmed directly.

No other flagged-as-unverified claim in the packet (the "333
coordination-path files and 12 plans failed" count, maintainer-
concentration percentages) was independently re-derived here — the
filing itself correctly declines to assert them as fact, and nothing
in scope for this review requires resolving them.

## Boundary

Neither review treats any external AI model's claim as project fact
beyond what was independently re-verified above. No Tier 1-N roadmap
action, disposition change, or merge is authorized by this record.
Contribution-auditor's own lens (provenance, sensitive-material
boundary, source fidelity) is addressed separately.
