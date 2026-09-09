---
id: 20260909T200622Z-cpb1-copied-record-contract-clarification
status: pending-gate1
topic: cpb1-copied-record-contract-clarification
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record. This
      narrows a gate (`coordination-publication-rule` condition 3 +
      `agov coord publication-check`). InfoSec must confirm the
      byte-identity copy check adds no bypass and no new false-negative on
      the append-only guarantee, and that it is not a similarity heuristic
      that could be gamed or that could misclassify a legitimate record.
  - role: contribution-auditor
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record.
      Contribution-auditor must confirm the clarification restores the
      "every existing negative condition stays tested" property that the
      CP-B-1 plan asserted but did not deliver for the copy condition, and
      that the template-similarity caveat is handled (legitimate new
      records sharing the frontmatter/SPDX skeleton still pass).
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B-1 — copied-record contract clarification (narrow Gate 1)

## Authority

- `hpms` (2026-09-09): an independent exact-head audit of CP-B-1
  implementation MR !2636 found one blocking mismatch — the approved
  CP-B-1 plan's gate-self-test list preserved "modify / delete / rename"
  but **dropped "copies"**, which
  `agf/rules/workflow/coordination-publication-rule.md` condition 3
  explicitly rejects.
- `j8bq` (amended): "the CP-B-1 copy gap must be resolved through its
  proposed narrow Gate 1 clarification before that implementation can
  advance."
- Correction record `8255`: the gap originated in the CP-B-1 plan I
  authored; `mpsp` / `qhht` reviews returned CLEAR without catching it.

This clarification amends **CP-B-1's scope only**. It adds no new
capability; it closes the coverage gap in the CP-B-1 acceptance contract.

## The gap

`agov coord publication-check` detects non-pure changes with
`git diff-tree --no-commit-id --name-status -r` **without `-C` (copy
detection)**. Git therefore reports a file copied to a new path as `A`
(add), which the check accepts. A `coordination:` MR that duplicates an
existing immutable record at a new filename passes the check — an
append-only violation condition 3 is written to catch.

## Why not just enable Git copy detection

`git diff -C<n>%` uses a similarity threshold. Coordination records share
a fixed skeleton (SPDX header, YAML frontmatter keys, section headings),
so a similarity heuristic would misclassify **legitimate new records** as
copies of unrelated existing ones. Rejected.

## The clarification

### Rule text (`coordination-publication-rule.md`)

Condition 3's parenthetical already lists "copies". Add one sentence to the
"### Multi-record publication" subsection and to condition 3's wording:

> A copy is detected by **exact content identity**, not by similarity: an
> added `docs/coordination/*.md` file whose full byte content equals that
> of any existing coordination record on the target branch is a copy and
> is rejected as a non-pure addition. Records with unique `id:` /
> `timestamp:` frontmatter can never be byte-identical, so this never
> misclassifies a legitimate new record.

### `agov coord publication-check` (`publication_check.rs`)

After a path passes the `A`-status check, and only for paths under
`docs/coordination/*.md`: compute a content hash (e.g. SHA-256) of each
**added** file and of each **existing** target-branch coordination record;
if any added file's hash matches an existing record's hash, fail with
`"commit <c> adds a byte-identical copy of an existing coordination
record: <path> == <existing path>"`. This is O(added + base) hashing, no
similarity computation.

### Gate-self-test additions (the CP-B-1 implementing MR, MR !2636)

Add to `publication_check.rs` tests:

1. **Copied record fails** — base branch has `A.md`; the candidate range
   adds `B.md` whose bytes equal `A.md`. `publication-check` must fail with
   a reason naming both paths. (This is the case `hpms` found missing.)
2. **Template-similar new record passes** — base has a valid record; the
   candidate adds a new record that shares the SPDX header, frontmatter
   keys, and section headings but has a distinct `id:`, `timestamp:`,
   `topic:`, and body. Must **pass** — proving the check is identity-based,
   not similarity-based.

The rule's "Required negative self-tests" list gains an explicit
copied-record item (making it 8), matching condition 3's full wording.

## Scope boundary

- CP-B-1 only. No change to CP-B-2 (hold / urgency) or CP-B-3.
- No change to the D-1 multi-record permission itself — it is sound; this
  only restores the copy-condition coverage the CP-B-1 acceptance contract
  claimed.
- The CP-B-1 implementing MR (MR !2636) picks up this rule sentence + the
  detector change + the two tests in the same MR (it has not merged); its
  exact-head reviews re-run against the updated head.

## Downstream impact

Classification: **ADDITIVE / clarifying**. Condition 3 already forbids
copies; this makes the detector and the test match the rule. No existing
coordination MR becomes invalid (unique IDs mean no two real records are
byte-identical). `agov coord publication-check` output is unchanged for
every input that is not an exact-copy attempt.

## Validation before Gate 1

`agov plan validate` on this file; `git diff --check`; coordination
visibility/collision scan. No projection manifest — the CP-B-1 projection
manifest already covers `coordination-publication-rule.md` and its
surfaces; this clarification's one added sentence rides that manifest.

## Decision requested at Gate 1

Approve (a) the exact-content-identity definition of "copy" (not
similarity), (b) the one-sentence rule-text clarification, (c) the
byte-hash detector addition to `agov coord publication-check`, and (d) the
two gate-self-tests (copied-record fails; template-similar new record
passes), all folded into the unmerged CP-B-1 implementing MR !2636 with
fresh exact-head reviews.
