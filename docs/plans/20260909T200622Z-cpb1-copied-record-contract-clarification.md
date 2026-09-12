---
id: 20260909T200622Z-cpb1-copied-record-contract-clarification
status: approved
gate1_approved_by: 20260912T102658Z-governor-reviewer-rqkp
gate1_approved_at: 2026-09-12T10:26:58Z
topic: cpb1-copied-record-contract-clarification
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Appointed semantic-security review mhhk found the target-inventory
      design defect and decisive-read fail-open at 1da3584d. Governor rqkp
      approved this correction; revised-head consultation remains required. This
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

- Governor `20260912T102658Z-governor-reviewer-rqkp` approves the current
  target-inventory amendment proposed in dpga. Original clarification authority
  was governor q862 ratifying planner bridge 771h; that history is retained,
  while rqkp supersedes the explicit merge-base-only restriction.

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
> of any coordination record already present at the resolved merge base or
> pinned current target tip,
> **or** of any coordination record added earlier in the same candidate
> range, is a copy and is rejected as a non-pure addition. The check
> compares actual bytes, so it also catches a mis-authored duplicate that
> reuses an `id:` — it does not rely on the unique-`id`/`timestamp`
> authoring convention. It never misclassifies a genuinely distinct new
> record, because it is identity, not similarity.

### `agov coord publication-check` (`publication_check.rs`)

The contract (per review `1r5a` findings CPB1-COPY-1 / -2):

1. **Source inventories — merge base and pinned current target tip.**
   Resolve the selected target ref once to an immutable commit SHA. Use that
   SHA for merge-base computation and all target tree/blob reads, never a
   subsequently dereferenced mutable branch. Enumerate direct lowercase .md
   coordination records at both merge base and pinned target via git objects;
   accept regular blob modes 100644/100755 only. Do not read working-tree paths.
   Identical inventory entries/content seed the set once; repeated inventory
   presence is not itself a candidate copy. Retain original path provenance.
2. **Source of candidate blobs — the exact Git object at its introducing
   commit.** For each `A`-status `docs/coordination/*.md` path, read the
   added blob with `git cat-file blob <commit>:<path>` at the commit where
   it appears.
3. **Hash is only a candidate index.** Compute a content hash (e.g.
   SHA-256) of each base blob and each candidate blob. On a hash match,
   **compare the two blob objects byte-for-byte** and classify as a copy
   only on exact byte equality. A hash match alone never decides.
4. **Running seen-content set.** Process candidate additions in range
   order; the seen set is `{merge-base records} ∪ {pinned target-tip records}
   ∪ {candidate additions already processed}`. A new addition whose bytes equal anything in the
   seen set is a copy — this catches two identical new records within one
   commit or one candidate range, which neither existed at the merge base.
5. **Fail closed.** Every security-decisive Git read failure fails the check:
   target resolution, merge base, merge/commit enumeration, subjects/full
   messages, per-commit diffs, aggregate diff, tree and blob reads. Do not
   convert errors into empty success evidence. Diagnostics identify failed
   operations/commits/paths without disclosing record bodies.
6. On a copy, fail with
   `"commit <c> adds a byte-identical copy of an existing coordination
   record: <path> == <existing path>"` (existing path is the merge-base,
   pinned target-tip or earlier candidate path).

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
4. **Decisive read failures fail closed** — injected failures for target
   resolution, merge enumeration, commit enumeration, subject, full message,
   per-commit diff, aggregate diff, tree enumeration and blob reads must reject.
   CLI reports nonzero, ok:false, and the failed operation. Include real forbidden
   merge/import and implementation-plus-revert ranges behind injected failures.
5. **Advanced-target copies fail** — target adds A after divergence and the
   candidate adds byte-identical B. A fresh check rejects with both paths while
   a distinct B passes. Copies at merge base, target or prior candidates are equal
   cases; target inventory overlap must not reject a legitimate distinct addition.
6. **Target pinning** — move the mutable target ref after resolution during an
   injected test. Ancestry/tree/blob reads must continue using the original pinned
   SHA, never a mixed target snapshot. The caller still rechecks target freshness
   immediately before merge; pinning makes one invocation coherent, not timeless.

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

Classification: **ADDITIVE / clarifying** for the original copy definition;
this amendment strengthens coverage of the same existing no-copy/full-range
conditions. Genuinely distinct records remain eligible. Copies introduced on an
advanced target and failed decisive reads that previously slipped through now
reject. Error diagnostics identify the failed operation. No similarity heuristic,
new message kind, new tool authority or release permission is introduced.

## Validation and approval record

Run `agov plan validate` on this file, `git diff --check`, and strict projection
validation on the existing CP-B-1 manifest. That manifest covers the amended rule,
Claude projection and all other implementation surfaces; no second manifest is
needed. Original approval was governor q862 ratifying 771h. Governor rqkp approves
the amended target inventory and failure semantics plus expanded test matrix.
Gate 2 remains pending final exact-head evidence and governor decision.

## 2026-09-12 security amendment acceptance

Governor rqkp approved SEC-1's inventory expansion and batching SEC-2's
fail-closed correction in existing !2636. These are amendments of this approved
plan, not a new implementation MR or a waiver of current security findings.
The prior target-tip-pass test is replaced with divergent-target rejection and
pinning coverage. Re-run the affected publication unit/CLI/gate tests, strict
projection validation, clippy and the required exact-head canonical pipeline.
Obtain independent semantic/code review on the final amended candidate before
seeking Gate 2. No unchanged retry of failed paid CI is authorized by this plan.
