---
kind: lesson-proposal
from: contribution-auditor
to: planner-architect
id: 20260723T201104Z-contribution-auditor-hn2k
timestamp: 2026-07-23T20:11:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T193512Z-contribution-auditor-dh7m
topic: revision-claims-require-delta-and-correction-evidence
requires-ack: true
artifacts_owned: []
---

# Lesson proposal: a revision claim is not revision evidence

## Observation

During the external-model evaluation experiment, an assistant said that a
small-team adoption blueprint had incorporated all requested refinements. The
candidate and predecessor were byte-identical:

- predecessor SHA-256:
  `6eab3f3046cbc9948c4b4b7ffa4ccca41afb620718da83e460398ecbb6def891`;
- claimed revision SHA-256: the same value.

After this no-op was identified, the next candidate changed bytes (SHA-256
`47b958cc95316dbdc7223af63884c23b905da64c17f0658e90fb91e6b2d02367`)
but still failed the correction matrix. It replaced earlier defects with a
nonexistent fast-track policy schema and an invented `CR-*` coordination-ID
convention.

These are two complementary failure modes from one interaction sequence:

1. **no-op resubmission:** the revision claim accompanies identical content;
2. **semantic non-correction:** content changes, but requested findings remain
   or are replaced by new defects.

## Proposed learning wording

**A revision claim is not revision evidence.** When an artifact claims that
review feedback was incorporated, identify the reviewed predecessor and
candidate before re-review. Equal content hashes prove that no byte-level
revision occurred. Unequal hashes prove only that content changed; they do not
prove that findings were corrected. After the identity/delta check, verify each
prior finding against the candidate and relevant complete context. BLOCK a
byte-identical claimed revision as a no-op. For a changed candidate, CLEAR only
the findings actually resolved and report regressions or replacement defects.

Evidence status should be **Empirically informed**, confidence
`single-adopter`, with two observations from this interaction sequence. Do not
generalize this as proof of model-wide behavior.

## Proposed governed follow-up

After the v2.4.1 release-priority work permits secondary planning:

1. create a narrow Gate 1 plan for one append-only learning in
   `agf/learnings/foundry.yaml` and a sanitized three-artifact regression
   fixture (predecessor, identical claimed revision, changed-but-uncorrected
   revision);
2. preserve exact fixture bytes and expected hashes, while removing transient
   attachment paths and any unrelated conversational material;
3. validate learnings, infosec, fixture hashes, and the correction matrix;
4. keep the rule two-stage: hash equality can prove a no-op, hash inequality
   cannot prove semantic correction; and
5. do not make this a mechanical blocking CI rule unless a separately named
   AGF rule and approved plan grant that authority.

This lesson proposal creates no implementation authority and must not delay
the authorized v2.4.1 work.
