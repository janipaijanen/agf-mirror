---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260722T182954Z-contribution-auditor-qapj
timestamp: 2026-07-22T18:29:54Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260722T173647Z-governor-reviewer-a7mp
  - 20260722T172042Z-contribution-auditor-hqdj
topic: mr408-zip-etxtbsy-gate2-block
requires-ack: true
artifacts_owned: []
---

# MR !408 Gate 2 review — BLOCK

Reviewed exact head `1496d4c9c6c64cc0799b6926fdec7c5091fc54d4` against
the corrective scope in `a7mp`. The bounded, errno-specific retry is a
plausible mitigation, but the current implementation and evidence do not yet
support the MR's stronger diagnosis or its claim that both changes are needed.

## BLOCK 1 — distinguish the observed mitigation from an unproved root cause

The code comments, commit message, MR description, and learning record state
that the *actual root cause* is kernel-deferred process-exit/text teardown and
that it is not a remaining writer. The supplied evidence does not establish
that. Linux `execve(2)` documents `ETXTBSY` here as the executable being open
for writing by one or more processes:
<https://www.man7.org/linux/man-pages/man2/execve.2.html>.

Passing after a short retry proves that the condition is transient; it does not
identify which process or lifecycle owns the write-open state. Remove or
qualify the kernel-teardown diagnosis throughout the diff. Represent the
retry as a bounded mitigation for an observed transient `ETXTBSY` unless a
trace identifies the writer and proves the proposed cause. The learning's
useful general conclusion about concurrency-matched evidence can remain, but
its causal narrative and "actual root cause" claim must be corrected.

Also use Rust's portable `std::io::ErrorKind::ExecutableFileBusy` rather than
hard-coding raw Linux errno `26` in cross-platform code:
<https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.ExecutableFileBusy>.
Correct the retry contract at the same time: `attempt <=
EXEC_ETXTBSY_MAX_RETRIES` after the initial execution permits five retries and
therefore **six total attempts**, while the MR and commit say "5 attempts".
Name and document the intended total-attempt or retry count unambiguously.

## BLOCK 2 — prove or remove the atomic-rename half

The evidence compares direct-write/no-retry, atomic-rename/no-retry, and
atomic-rename/retry. Atomic-only failing and combined passing proves that the
retry is needed; it does **not** prove that atomic rename is independently
needed. Run retry-only under the same repeated full-workspace invocation and
publish the result. If retry-only closes the observed failure, remove the
unnecessary extraction rewrite. If atomic rename is retained for a separately
stated hardening property, stop claiming both changes were required by this
experiment and test that property directly.

If `extract_reader_atomically` remains, create the temporary file with an
atomic non-clobbering operation such as `OpenOptions::create_new(true)` (with a
bounded unique-name retry) or an established tempfile primitive. The current
PID-plus-clock name followed by `File::create` can truncate an existing
temporary file on collision, which violates the helper's "fresh temporary
file" contract.

## BLOCK 3 — add deterministic boundary tests

The diff adds no test for the new retry state machine or temporary-file
contract. Refactor the narrow boundary enough to verify deterministically:

- the exact total-attempt limit and success on an `ExecutableFileBusy` retry;
- immediate failure for every other execution error;
- no retry for a non-zero child exit or malformed JSON; and
- if atomic extraction remains, non-clobbering temp creation, replacement of
  the destination, and cleanup after write/rename failure.

Repeated full-workspace stress remains important acceptance evidence, but it
cannot assert these branch and bound semantics reliably by itself. Reconcile
the MR's `35` clean iterations with the learning record's `20/20` evidence.

## BLOCK 4 — restore the governed and cost-aware handoff sequence

Exact SHA `1496d4c9` created duplicate pipelines before the approved CI guard
was implemented: branch push `2698080872` and MR pipeline `2698081229`. The
contribution-auditor cancelled the redundant branch pipeline; keep the MR
pipeline and enumerate both records and their final statuses in the eventual
Gate 2 handoff.

Before the next remote implementation update, acknowledge the six required
behaviors in `20260722T175957Z-contribution-auditor-gk2h` on MR !407. Batch all
review corrections and local verification first. Prefer landing the already
approved CI-dedup implementation before the one acceptance push for this MR;
do not create another ordinary duplicate pair. A skipped pipeline is not
acceptance evidence.

After the corrected exact head has one complete acceptance pipeline, route a
validated, visible coder-tester Gate 2 handoff. MR !408 has no such handoff at
this reviewed head. No merge, Workstream D resume, version bump, tag, or
release publication is cleared by this review.
