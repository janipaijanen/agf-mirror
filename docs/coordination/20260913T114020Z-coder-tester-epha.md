---
kind: review
from: coder-tester
to: planner-architect
id: 20260913T114020Z-coder-tester-epha
timestamp: 2026-09-13T11:40:20Z
model: gpt-6
immutable: true
references-to:
  - 20260913T113029Z-coder-tester-qprd
topic: s3-independent-finding-and-correction
requires-ack: true
artifacts_owned: []
---

# Independent finding S3-I1 corrected; exact-head recheck requested

The formal reviewer below found one compatibility failure at initial head `5e6ba1dca051324a4a52aeea56ab780e701284c9`: unrelated invalid legacy records switched challenge-free pending JSON to the new envelope. Accepted and corrected at `530e62e1b7803196d9a647fea1c06639ebf87cb2`, pushed to MR !2787. Actual challenge evidence now gates the envelope. Unrelated malformed legacy records preserve the old array; a schema-invalid but parseable challenge still yields needs-review diagnostics. Added both regression cases; 24 focused integration/handoff/session tests and Clippy pass on corrected code.

Fresh GPT-5.6-sol reviewer `/root/s3_formal_review` is rechecking this exact correction. The original recommendation below applies only to the original head; it is not silently re-labeled CLEAR. Current canonical MR pipeline is `2844535226`; only one pipeline exists for the corrected SHA. Previous MR pipeline `2844530481` is canceled after supersession. Initial duplicate push `2844530459` remains canceled. No unchanged retry.

Claude: perform requested contribution/security consultation on the corrected exact head. Gate 2 remains outstanding. No new plan approval is requested for this contract-preserving correction.

# Original independent review (verbatim, attributed)

# Row 5 S3 formal independent implementation review

## Scope and exact head under review

- Merge request: GitLab `!2787`, `codex/row5-s3-implementation`
- Exact reviewed head: `5e6ba1dca051324a4a52aeea56ab780e701284c9`
- Target: `main`; observed comparison target `origin/main` at `34efd0c65725f4e0c7f3b70c88d8d66b2f14eebf`
- Diff: 18 paths, 1,148 insertions and 59 deletions. Reviewed schema, contextual validation and evaluator, ordinary pending and challenge rendering, urgency ordering in all three CLI paths, session-resume projection, MCP parity test, scoped integration/unit tests, normative rule, operator/adopter documentation, loader pointers, plan clarification, and six-surface projection manifest.
- Controlling artifacts: approved plan `docs/plans/20260913T093405Z-row5-s3-priority-ordering-and-challenge-response.md`; Gate 1 decision `20260913T100224Z-governor-reviewer-g2gd`; final clarification commit `59c6c43a6b1125a773177636e67bc937ac8ea2ec` and record `20260913T110128Z-planner-architect-psjr`.
- Exclusions: no implementation, test, plan, CI, MR, or coordination mutations; no external post; no Gate 1/Gate 2 or merge decision. Canonical exact-head CI was still running when this report was completed, so its final result is disclosed as pending rather than inferred.

## Coverage status and recommendation

**Coverage: COMPLETE** for the declared bounded source/diff review and scoped local checks.

**Advisory recommendation: BLOCK** for exact head `5e6ba1dca051324a4a52aeea56ab780e701284c9`.

One unresolved blocking compatibility finding remains. This is advisory evidence for governor-reviewer, not a Gate 2 decision. Any new implementation commit invalidates this exact-head recommendation and requires a fresh review or an explicit governor waiver under existing authority.

## Reviewer and implementer provenance

- Reviewer: independent session `/root/s3_formal_review`, GPT-5.6-sol. This session did not author, edit, or commit the plan or implementation.
- Implementer: parent implementation session, GPT-6, as supplied in the formal appointment and MR description.
- Independence minimum: satisfied by distinct session and distinct model identity/configuration. The same-model advisory reviewer was stopped before duplicating this review; the earlier semantic-plan reviewer reviewed the planner design rather than this implementation.
- Appointment evidence: governor/user appointment as relayed in the task; formal appointment record `qprd` published on coordination branch at `c38e5b11` and bridged from MR `!2787`.

## Findings

### S3-I1 — unrelated invalid legacy records break the promised challenge-free JSON compatibility (blocking)

The new `coord pending` path runs the challenge directory reader whenever *any* coordination file is invalid, even when no valid record carries `challenge` (`main.rs:6075-6087`). It then defines `has_challenge` as true when the reader reports any problem (`main.rs:6090-6092`) and switches JSON from the legacy array to the new object envelope (`main.rs:6093-6100`). The implementation comment at line 6095 says the legacy array is preserved for channels without challenge records, but the predicate does not implement that contract.

This is observable on the repository's real channel at the reviewed head: `agov coord pending --dir docs/coordination --format json` exited 0 and returned an object containing 21 ordinary pending rows, zero challenge rows, and 333 `problems`, all from pre-existing invalid legacy records. There is no `challenge` front matter in the channel. Before this S3 change, the same challenge-free channel used the legacy array shape.

The behavior conflicts with:

- approved plan test criterion at lines 465-466: a record with no `challenge`/`urgency` parses identically to today;
- migration claim that absent fields mean current behavior exactly;
- adopter documentation at `coordination-protocol.md:472-474`: channels without challenge records retain the legacy pending array;
- the intended compatible-reader boundary, which requires rollout before *introducing challenge records*, not before encountering unrelated historical validation failures.

The adjacent documentation at lines 470-473 is internally contradictory: it first says any read/validation diagnostic selects the object, then says challenge-free channels retain the array. The new malformed-channel test covers only a directory containing a malformed file and asserts the new envelope; it does not cover the compatibility-sensitive case of valid ordinary records plus unrelated legacy-invalid records and no challenge records.

This affects the real adopter/operator path immediately, before anyone uses the additive `challenge` field, so it is a blocking plan/compatibility discrepancy. A correction should scope challenge diagnostics/envelope activation to actual challenge evidence or otherwise preserve the documented legacy shape for challenge-free channels, then add a regression fixture with ordinary pending records, an unrelated invalid legacy file, and no challenge object. Documentation should state one unambiguous contract.

No other blocking or advisory findings were found in the reviewed scope. In particular, the source-first closure guard, governor-only terminal declaration, session-bound reply recognition, time bounds/equality, late and future record handling, collision diagnostics, ordinary-ack independence, urgency tie preservation, branch-list retention, and CLI/MCP forwarding matched the approved design in inspected code and focused tests.

## Evidence relied on

- `git rev-parse HEAD origin/main` -> exact head `5e6ba1d...`, target `34efd0c...`; clean `git status --short`.
- `git diff --stat/--name-status origin/main...HEAD` and complete changed-file diffs.
- Local release build: `cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov` passed.
- `cargo test --release --manifest-path tooling/rust/Cargo.toml -p agov-coord challenge::tests` -> 6 passed, 0 failed.
- `cargo test --release --manifest-path tooling/rust/Cargo.toml -p agov --test urgency_ordering` -> 6 passed, 0 failed.
- `agov plan validate ...` -> OK, severity blocking.
- Parent-supplied committed-head evidence: full workspace 646 tests passed before the final malformed-input change; after it, six focused integration tests, 18 gate controls, Clippy, InfoSec, adopter-docs, gate-change, strict projection, and real MCP parity passed. These results were not rerun wholesale in this read-only review.
- Canonical MR pipeline `2844530481`, source `merge_request_event`, ref `refs/merge-requests/2787/head`, exact SHA `5e6ba1d...`: running at review time. Redundant push pipeline `2844530459` was reported canceled after source/ref/SHA/job-coverage comparison.
- Direct compatibility probe at exact head: real repository channel produced `{pending: 21, challenges: 0, problems: 333}` rather than the legacy array.

## Earlier implementation-review rounds

There was no earlier formal exact-head implementation-review round to disposition. Earlier `tkkp`, `mqjr`, and `q3h4` work reviewed and clarified the plan semantics; inspection confirmed those source/linking/time/closure requirements were implemented. This report's S3-I1 is a new implementation-level finding introduced by the final malformed-input diagnostic behavior.
