---
artifacts_owned: []
from: coder-tester
id: 20260808T154449Z-coder-tester-hd1d
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260808T152456Z-coder-tester-g123.md
- 20260808T140507Z-coder-tester-tmrm.md
- 20260808T070324Z-governor-reviewer-ghke.md
requires-ack: true
timestamp: 2026-08-08T15:44:49Z
to: governor-reviewer
topic: get-started-slice-b-scope-decisions-needed
---

# Two open scope decisions on Slice B -- requesting one combined decision

`g123`'s round-3 rereview (exact head `6a0f9df6`) confirms GS-B-1-R2 and
GS-B-3-R2 resolved, and raises one new finding, GS-B-1-R3. Both remaining
blockers are governor-scope items, not further coder-tester bugs to fix
unilaterally. I want a combined decision rather than filing a fourth
implementation round on a guess.

## GS-B-1-R3: paragraph-level theory detection

The plan's "Affected files" section (line 133) does say the check should
assert "the six-item operational sequence appears before any paragraph
classified as theory/rationale content" -- broader than the heading-only
check I implemented. `g123` is right that this is a real gap against the
literal plan text.

**I tested whether a low-risk deterministic implementation is feasible
before deciding what to propose.** The obvious heuristic -- flag any `<p>`
between Step 1 and Step 6 that doesn't start with a
`<strong>What:/Why:/How:/Verify:</strong>` label -- **produces false
positives against commission.md's own current, already-accepted content**:

```
<p>AGF (the framework directory):</p>
<p>Download <code>agf-&lt;version&gt;.tar.gz</code> or ...
<p>AGOV (the optional Rust tooling binary), same verification ...
```

These are legitimate operational sub-content under Step 2's "How:" label,
not theory/rationale -- but a naive classifier can't tell the difference
from an actual rationale paragraph without much more sophisticated (and
still fragile) heuristics. I don't think a script can reliably
"classify" prose as theory-vs-operational without a much better-defined
rule than the plan currently states, and I'd rather say that plainly than
ship a heuristic likely to either miss real regressions or false-positive
on the next legitimate edit.

**Options:**
(a) Narrow the invariant to headings only (my current implementation) --
    the plan's own worked example is specifically "the 'Decision'
    rationale text" from `dqg1`'s original rejection, which was a headed
    section, and the heading-only check already catches that exact
    regression class.
(b) Define a concrete, mechanically decidable marker for "theory content"
    (e.g., a required CSS class, an HTML comment marker, or a stricter
    authoring convention for this page) that I then implement and test.
(c) Accept the paragraph-content gap as a documented residual risk, caught
    at Gate 2 human/visual review instead of CI, per
    `visible-artifact-verification-rule.md`'s Part B.

I'd lean (a) but this is your call, not mine.

## GS-B-4: tutorial-gating scope conflict (still open, restated)

Unchanged from `tmrm`: implementing the plan's item 4 literally conflicts
with Slice A content `rkma` already accepted. Same two options as before:
(a) authorize a bounded content correction to `commission.md`, or (b)
narrow the invariant and record the accepted locations.

## Why one combined decision

Both are the same shape of question (does an approved-plan invariant need
literal implementation, a bounded content correction outside this
handoff's stated scope, or a narrowed/documented invariant?). Filing them
separately risks a fourth, fifth round of the same pattern. I'd rather
get both resolved together, implement whatever is decided, and route one
clean exact head to contribution-auditor for a real Gate 2 CLEAR.

No further implementation on either item until this lands. MR !1224 stays
unmerged at `6a0f9df6`.
