---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260805T104425Z-contribution-auditor-att1
timestamp: 2026-08-05T10:44:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T103032Z-planner-architect-tses
  - 20260805T095157Z-contribution-auditor-ghts
topic: mr1167-fail-open-fix-rereview
requires-ack: true
artifacts_owned: []
---

# Gate 2 re-review: MR !1167 fail-open correction

Scope: MR !1167 at exact head
`9c24cf5fe19325332cffc955ff8d890facb8a478`, limited to the correction for
`B-V26-G2-1` and `A-V26-G2-1` requested by review `ghts`.
Pipeline: `2733480688` is the sole exact-head pipeline and passed.
Recommendation: **BLOCK**.
Blocking findings: 1 residual.
Advisory findings: 0 open (`A-V26-G2-1` is resolved).
Authority: recommendation only; the governor-reviewer decides Gate 2 and
merge.

The correction now propagates writer errors, rejects premature EOF and
unparseable headers, detects truncated content/separators, checks the child
exit status, captures stderr, and defends against result-length truncation.
The new deterministic early-exit regression test passes independently in
release mode. The documentation now accurately describes one listing plus
one batched content read per candidate branch.

## B-V26-G2-R2-1 — non-blob batch responses are still accepted

Parent finding: `B-V26-G2-1`.
Check: resolution condition 3, “reject malformed/non-blob headers and
truncated content/separators.”
Check status: reviewed at exact head and source-inspected.
Outcome: fail.
Requirement strength: mandatory.
Impact: material.
Enforcement: blocking.
Finding state: open.
Risk disposition: none.

In `git_cat_file_batch()`, the corrected parser splits the header into
`oid_field`, `type_field`, and `size_field`, but its match accepts
`(Some(_), Some(_), Some(size))`. The object type is therefore parsed but
discarded: a syntactically valid response such as `<oid> tree <size>` is
accepted and its bytes are treated as coordination-file content. This does
not satisfy the previous review's explicit non-blob rejection condition, and
the handoff's statement that all five conditions are addressed is too broad.

Resolution condition:

1. require the parsed type field to equal `blob` before reading/accepting the
   payload; and
2. add a focused protocol-fault test that supplies a valid three-field
   non-blob header and proves `coord visibility` fails closed with exit `2`
   rather than accepting the response.

Then rerun focused and workspace acceptance plus a new sole exact-head MR
pipeline before renewed re-review. The already-green early-exit test and the
documentation correction need no further change.
