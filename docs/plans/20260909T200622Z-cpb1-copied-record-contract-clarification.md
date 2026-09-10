---
id: 20260909T200622Z-cpb1-copied-record-contract-clarification
status: approved
gate1_approved_by: 20260910T052328Z-planner-architect-771h
gate1_approved_at: 2026-09-10T05:23:28Z
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

> A copy is detected by **exact byte identity**, not by similarity: an
> added `docs/coordination/*.md` file whose full byte content equals that
> of any coordination record already present at the resolved merge base,
> **or** of any coordination record added earlier in the same candidate
> range, is a copy and is rejected as a non-pure addition. The check
> compares actual bytes, so it also catches a mis-authored duplicate that
> reuses an `id:` — it does not rely on the unique-`id`/`timestamp`
> authoring convention. It never misclassifies a genuinely distinct new
> record, because it is identity, not similarity.

### `agov coord publication-check` (`publication_check.rs`)

The contract (per review `1r5a` findings CPB1-COPY-1 / -2):

1. **Source of base records — pinned to the merge base, not the tip.**
   Enumerate the coordination records present at the **already-resolved
   merge base** via `git ls-tree -r <merge-base> -- docs/coordination`
   (the same merge base condition 1 resolves). Do not read the target
   branch tip (avoids target-tip drift) and do not read working-tree
   paths (avoids working-tree races).
2. **Source of candidate blobs — the exact Git object at its introducing
   commit.** For each `A`-status `docs/coordination/*.md` path, read the
   added blob with `git cat-file blob <commit>:<path>` at the commit where
   it appears.
3. **Hash is only a candidate index.** Compute a content hash (e.g.
   SHA-256) of each base blob and each candidate blob. On a hash match,
   **compare the two blob objects byte-for-byte** and classify as a copy
   only on exact byte equality. A hash match alone never decides.
4. **Running seen-content set.** Process candidate additions in range
   order; the seen set is `{merge-base records} ∪ {candidate additions
   already processed}`. A new addition whose bytes equal anything in the
   seen set is a copy — this catches two identical new records within one
   commit or one candidate range, which neither existed at the merge base.
5. **Fail closed.** Any `ls-tree` or `cat-file` enumeration/object-read
   error fails the check (never pass-open), consistent with condition 1.
6. On a copy, fail with
   `"commit <c> adds a byte-identical copy of an existing coordination
   record: <path> == <existing path>"` (existing path is the merge-base
   path or the earlier candidate path).

### Gate-self-test additions (the CP-B-1 implementing MR, MR !2636)

Add to `publication_check.rs` tests (per `1r5a` CPB1-COPY-2):

1. **Copy of a merge-base record fails** — merge base has `A.md`; the
   candidate range adds `B.md` whose bytes equal `A.md`. Must fail with a
   reason naming **both** paths.
2. **Candidate-internal duplicate fails** — neither exists at the merge
   base; the candidate range adds `B.md` and `C.md` with identical bytes.
   Must fail (the seen-content set catches the second).
3. **Template-similar distinct record passes** — base has a valid record;
   the candidate adds a new record sharing the SPDX header, frontmatter
   keys, and section headings but with a distinct `id:`, `timestamp:`,
   `topic:`, and body. Must **pass** — identity, not similarity.
4. **Object/enumeration read failure fails closed** — a simulated
   `ls-tree` / `cat-file` failure must fail the check, not pass it.

The rule's "Required negative self-tests" list gains an explicit
copied-record item, matching condition 3's full wording.

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
coordination MR becomes invalid (two genuinely distinct records differ in
`id:` / `timestamp:` / body, so they are never byte-identical; the check
compares bytes, so it neither depends on that convention nor flags a
distinct record). `agov coord publication-check` output is unchanged for
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
