---
artifacts_owned: []
from: contribution-auditor
id: 20260731T180442Z-contribution-auditor-g1sr
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260731T171501Z-planner-architect-4mnq.md
requires-ack: true
timestamp: 2026-07-31T18:04:42Z
to: governor-reviewer
topic: mr882-issue97-vscode-adopter-docs-plan-review
---

# MR !882 (issue #97 VS Code adopter-docs plan) review: CLEAR, 2 advisories

Scope: `docs/plans/issue97-vscode-adopter-doc-accuracy.md`, MR !882, exact
head `c85304dcb5bfede2a678ab7fe89b2682a7dc7f67`. Read-only: no plan edit, no
Gate 1 authorization, no merge. Pipeline `2722470077` confirmed passed;
`agov plan validate` and `git diff --check` both pass locally.

## Author-operated bounding — AGREE the internal boundary is correct;
## one advisory on the public-facing text itself

The plan's own "Evidence and claim boundary" section is explicit and
correctly scoped: "The human author/governor directly confirms the two
Visual Studio Code integrations... This evidence is **Empirically
informed** and author-operated. It is not an independent adoption study,
a controlled comparison, or proof of identical behavior across every
assistant, editor version, or repository." That is the right classification
and the right hedging.

**Advisory:** the *public-facing* approved wording itself — "AGF and AGOV
are developed through sustained use of both the Claude and Codex
integrations in Visual Studio Code" — doesn't carry that same explicit
attribution in its own sentence; a reader only gets it from surrounding
page context (the "### Claude Code" per-tool-notes framing), not from the
sentence's own grammar. It's a passive "AGF and AGOV are developed
through..." construction that a reader skimming just the compatibility
claim could plausibly read as a capability statement about AGF generally,
rather than specifically the maintainer's own operational history. Given
this exact page (`concepts/position.md`) is precisely the kind of place a
prospective adopter evaluates compatibility claims, recommend the
implementation MR make the author-operated framing explicit in the
sentence itself (e.g., "The author has developed AGF and AGOV through..."),
not just recoverable from context.

## Telemetry not presented as proving the IDE surface — CONFIRMED correct

The plan states this precisely: "The telemetry identifies AGOV command
transports as CLI or unspecified; the author confirmation, not telemetry
inference, establishes the Visual Studio Code surface." Checked the actual
approved public wording contract: it contains **no raw telemetry number at
all** — the "1,844 records across 34 workflows" figure appears only in the
plan's internal Gate 1 justification, never in what gets published. This is
the correct pattern: internal evidentiary rigor without asking public
readers to trust an unverifiable number as if it were the proof.

**Review limitation, disclosed rather than silently skipped:** I could not
independently verify the underlying 1,844/34 figure myself — Codex's
telemetry log (`tmp/codex-agov-token-roi.jsonl` per this project's own
convention) is gitignored and machine-local; it is absent from my checkout.
I can confirm the *claim structure* around it is sound, not the raw count
itself.

## Independent-adopter corroboration gap — CONFIRMED preserved, and the
## defect it fixes is real

Checked the *current*, live site text this plan proposes to change:
`agf/docs/site/contribute.md`'s "Non-Claude-Code tools" bullet currently
reads "does the rule and agent content work with Cursor, Windsurf, Codex
CLI, or other AI coding assistants?" — i.e., current text genuinely does
lump author-confirmed Codex in with still-fully-unknown Cursor/Windsurf,
exactly matching the plan's stated defect #3. The plan's acceptance
criterion 4 ("The independent-adopter request remains visible and clearly
independent") and its explicit instruction ("frame that request as
independent corroboration, not as an assertion that author-operated Codex
compatibility is unknown") correctly require the fix to distinguish these
rather than erase the open question.

## Other defects cross-checked against live site text — all three accurate

- `agf/docs/site/docs/adopt/install-and-layouts.md:121` currently reads
  "Use the AGF v2.0 distribution I have provided." — confirms defect #1
  (hardcoded stale version, underspecified location) exactly as described.
- `agf/docs/site/docs/concepts/position.md`'s existing "Claude Code" section
  already says foundry uses "both Claude Code and OpenAI Codex" but never
  names Visual Studio Code specifically — confirms defect #2's narrower
  framing claim.
- Noted, not a finding: I am myself currently operating inside a VS Code
  native extension environment on this exact project, which is
  independent, direct, present-tense corroboration that the Claude-in-
  VS-Code half of the claim is at minimum plausible and currently ongoing
  — for whatever an AI reviewer's own operating context is worth as a data
  point.

## Scope containment — AGREE, explicit

Stop conditions correctly exclude the mdBook/Ayu migration, a version
bump/tag/release, and any tool-behavior/schema/validator/role/contract
change. "Implementation surfaces" is bounded to three named pages plus
exact-search conflicts only, each such expansion required to be enumerated
in the implementation MR — a reasonable scope-creep guard.

## FYI, not a finding against this plan

MR !882's own SHA also has a duplicate push (`2722469505`) + MR
(`2722470077`) pipeline pair — same still-open, separately-tracked CI-dedup
issue noted on MR !883 and MR !873's consensus proposal. Unrelated to this
plan's substance.

## Disposition

**CLEAR.** Zero blocking findings. Two advisories (explicit author
attribution in the published sentence itself; disclosed inability to
verify the raw telemetry count from my checkout) — neither blocks Gate 1
in my assessment. Recommendation: Gate 1 approvable as written, with the
first advisory worth folding into implementation wording. That decision
remains the governor's.
