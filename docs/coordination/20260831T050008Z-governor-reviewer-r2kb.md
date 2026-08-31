---
kind: decision-record
from: governor-reviewer
to: all
id: 20260831T050008Z-governor-reviewer-r2kb
timestamp: 2026-08-31T05:00:12Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260831T041020Z-coder-tester-1mqg.md
topic: plans-must-carry-candidate-release
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Every plan from now on carries a candidate release

Binding governance decision, given by the governor-reviewer in session and
transcribed by Claude.

## Decision

**Every Gate 1 plan authored from this record forward MUST declare a candidate
release version in its front matter.**

```yaml
candidate_release: v2.9.0
```

The value is a **candidate**, not a commitment. It may change as scope moves,
and changing it is an ordinary plan revision, not a governance event. What is
forbidden is having no answer at all.

A plan whose work genuinely ships in no release states `candidate_release:
none` **with a reason** — that is a deliberate, visible choice rather than an
omission.

## Why

The governor's words: *we want release version, not internally baked gate
documents as we have done for the last two months.*

The record supports that plainly. The last released version is **v2.8.0**.
Since then `main` has accumulated 48 non-coordination commits, of which a
large share are plans, learnings and governance documents rather than
shipped capability. In one session on 2026-08-30/31 this project produced
seven review rounds on a single internal diagnostics plan, four rounds on a
figure-grammar pilot, and over one hundred CI pipelines — and exhausted its
CI minutes quota — without moving a release.

Governance artifacts are not the product. They exist to make the product
shippable. A plan that cannot name a release it is aiming at is a plan whose
connection to shipping nobody has checked.

## What this changes in practice

- At Gate 1, a reviewer can ask "which release?" and get an answer that is
  either credible or visibly not. That question was previously unaskable.
- Work that keeps deferring acquires a visible symptom: its candidate release
  slips, repeatedly, in the plan's own history.
- Aggregating plans by `candidate_release` yields a real release scope
  without a separate roadmap document.

## Scope and limits

- **No schema change is required.** `agf/schemas/gate1-plan.schema.json` does
  not set `additionalProperties: false`, so the field validates today. Verified
  before this record was written.
- **This is not a gate.** No validator, CI check, or merge policy is modified
  by this record. It is an authoring obligation enforced at review. Making it
  mechanically enforced is a separate change under
  `always/gate-self-test-rule.md` and needs its own Gate 1.
- **Not retroactive.** Existing plans are not invalidated and need no
  backfill. They may add the field when next revised.
- **A candidate release is not Gate 1 approval, a release commitment, or
  authority to tag.** It is a stated intent.

## Follow-up, not blocking

Projecting this into `agf/rules/`, the plan template, and the adopter-facing
documentation requires the six-row screen in
`documentation-rule-projection-rule.md`. That work is separate and does not
gate this decision taking effect.
