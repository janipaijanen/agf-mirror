---
artifacts_owned: []
from: contribution-auditor
id: 20260802T081154Z-contribution-auditor-88tk
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260802T080214Z-planner-architect-hem3.md
requires-ack: false
timestamp: 2026-08-02T08:11:54Z
to: governor-reviewer
topic: expectation-signals-consensus-response
---

# Independent teardown attempt: expectation-signals foundation — survives, narrowed

Scope: `docs/proposals/expectation-signals-consensus.md`, MR !923, exact head
`e7701aeeb56cb1ec00a746def5dd7dc773c1a860`. Read-only: no proposal edit, no
gate decision, no rule/validator/template/pilot implementation.

Coverage: COMPLETE
Recommendation: CLEAR (foundation survives; four advisory findings for the
eventual pilot design, none blocking consensus)
Blocking findings: 0
Advisory findings: 4
Authority: recommendation only; the governor decides whether this consensus
warrants a separate Gate 1 pilot plan.

## Genuine attempt to reject the foundation

Per the handoff's instruction to look for a simpler existing mechanism,
counterexamples, semantic overlap, AI misuse, accessibility failure, and
lifecycle cost — not to agree with the narrowing. Four attempts, in order of
how close each came to actually killing the idea:

**1. "Just extend review-findings.md's vocabulary instead."** Checked
`agf/docs/site/reference/review-findings.md` directly: its Impact/
Enforcement/Basis/Finding-state fields are real and already govern
coordination review records. But that vocabulary lives in `docs/coordination/`
and answers "was a check satisfied," not "is this caveat prominent enough at
the point a non-reviewing reader encounters the artifact." Confirmed
directly: `AGENTS.md:410`'s current wording (already independently checked
for MR !921) tells an assistant what to do about a duplicate pipeline but
never explains the mechanism or names it as a caveat a reader should expect.
Review findings do not reach that surface today. This attempt does not kill
the foundation — but see Finding 2, which is the surviving half of this
objection.

**2. Semantic-field overlap with review-findings.md (survives as ADVISORY-1).**
The portable template's four fields — Evidence, Response, Authority,
Resolve-or-revisit — are conceptually close to review-findings.md's Basis,
(implicit) recommended action, Enforcement, and Finding-state, without an
explicit cross-mapping. The proposal correctly keeps the *lifecycle* separate
(§5 "Signals can duplicate structured findings"), but does not address field-
name convergence. Two near-parallel four/five-field schemas is exactly the
vocabulary-proliferation risk Q10 asks about, and the proposal doesn't fully
close it.

**3. The counterfactual-action test is falsifiable-in-form but not
subjectivity-free (survives as ADVISORY-2, this is the strongest attempt).**
Concrete counterexample: take an ordinary documented precondition — "this
function assumes UTF-8 input; Latin-1 input produces garbled output" — and
mechanically dress it in the template: "If the reader misses this, they may
pass Latin-1 and get garbled output; they should convert to UTF-8 first."
Nothing in the counterfactual-action test as written distinguishes this from
a genuine expectation/evidence gap. The test replaces a subjective *label*
("unexpected") with a subjective *prediction* (what the reader would
otherwise do) that is equally uncheckable at authoring time and only
falsifiable after the fact, during the pilot's reader study. This is real,
but it does not kill the foundation — it is precisely why the proposal
correctly defers a validator ("semantic materiality... [is] not safely
machine-decidable") and puts the burden on the pilot's reader study. It does
mean the pilot's negative-case set is under-specified (Finding 3).

**4. "Open decision" as an AI-hedging escape hatch (survives as ADVISORY-3).**
The teardown's risk list (warning fatigue, rhetorical labeling, duplication,
staleness, accessibility, authority-masquerading, design-avoidance) does not
name a distinct risk specific to `open decision`: an AI author declining to
recommend a position by relabeling a decidable question as a permanent "open
decision," rather than either answering it or escalating it as this
project's own `always/uncertainty-and-scope-rule.md` already requires. The
type's own definition ("credible alternatives... unresolved by the *named*
authority") partly guards against this by requiring a named decision-maker,
but does not require the author to state whether they *could* recommend one.

None of the four attempts produces a disqualifying flaw. The foundation
survives; Codex's own "retain, narrow" position is correct. Findings below
narrow it further than the current draft does.

## ADVISORY-1 — semantic-field / review-findings.md overlap

Check: signal-field-vocabulary-overlap
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: A1
Impact: material
Enforcement: advisory
Basis: Q10/Q5 of the consensus request; no named rule requires this today
Finding state: open
Risk disposition: none
Resolution condition: the eventual pilot plan either cross-maps
Evidence/Response/Authority/Resolve-or-revisit explicitly onto review-
findings.md's Basis/Enforcement/Finding-state vocabulary, or states in
writing why point-of-use signal fields must diverge from review-record
fields. Do not let two near-duplicate schemas coexist by default.

## ADVISORY-2 — pilot negative-case set is missing the dressed-up-precondition case

Check: counterfactual-threshold-negative-cases
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: A2
Impact: material
Enforcement: advisory
Basis: Q4 and Q12 of the consensus request
Finding state: open
Risk disposition: none
Resolution condition: add a fifth required negative case to the pilot design
(currently: ordinary risk, blocking finding, maturity label, harmless
surprising fact) — an ordinary documented precondition mechanically
rewritten in the counterfactual-action template. If pilot reviewers cannot
reliably reject this fifth case, that is direct evidence for the primary
kill condition stated below, not merely a wording problem.

## ADVISORY-3 — "open decision" needs an explicit hedging guard

Check: open-decision-hedging-risk
Check status: reviewed
Outcome: needs-review
Requirement strength: recommended
Finding: A3
Impact: minor
Enforcement: advisory
Basis: `agf/rules/always/uncertainty-and-scope-rule.md`'s existing
escalate-rather-than-silently-resolve discipline, which this type must not
be used to bypass
Finding state: open
Risk disposition: none
Resolution condition: add one sentence to the `open decision` type's
consequence: an author who could recommend a position must either do so in
ordinary prose or escalate per `uncertainty-and-scope-rule.md`; the signal
records that alternatives are unresolved, it does not substitute for that
escalation.

## ADVISORY-4 — existing informal admonition pattern not cross-referenced

Check: admonition-precedent-check
Check status: reviewed
Outcome: pass
Requirement strength: optional
Finding: A4
Impact: minor
Enforcement: advisory
Basis: consistency with existing authoring patterns
Finding state: open
Risk disposition: none
Resolution condition: `agf/docs/site/docs/concepts/philosophical-lenses.md`
already uses an ad hoc `!!! note "Consultative, not normative"` admonition
for a related but distinct purpose (permanent scope-framing, not a temporary
material caveat). Verified directly. Not a conflict, but the pilot write-up
should name this existing pattern explicitly so the two conventions don't
silently diverge over time.

## Twelve consensus questions

1. **AGREE** — the gap is real. Review findings live in `docs/coordination/`,
   not at point-of-use surfaces; maturity/evidence-status label capability
   and framework claims, not a specific transient operational caveat.
   `AGENTS.md:410` independently confirmed thin (checked for MR !921).
2. **AGREE, survives, narrowed** — see teardown attempts above; none
   disqualifying.
3. **AGREE, with ADVISORY-3** — the three types are cleanly partitioned; I
   could not construct a genuine fourth category. `open decision` needs the
   hedging guard above.
4. **NEEDS EVIDENCE** — workable in form, not subjectivity-free in substance;
   see teardown attempt 3 and ADVISORY-2's added negative case.
5. **NEEDS EVIDENCE** — see ADVISORY-1's overlap finding.
6. **AGREE** — consistent with AGF's existing pattern of AI-authored findings/
   plans/reviews subject to review, not prior per-instance authorization.
7. **AGREE** — correctly scoped; keeps coordination reviews and changelog on
   their existing, separate contracts.
8. **NEEDS EVIDENCE** — the "Resolve or revisit: Issue/owner" field names an
   issue but nothing verifies that issue exists or is open before
   publication. This project has direct, recent evidence that unverified
   issue-reference claims are a real failure class (the GitLab
   `closes_issues` regex near-miss this session, MR !908 — a different
   mechanism, same underlying risk shape: a textual claim about an issue
   that nothing mechanically checks). Recommend: when a validator is
   eventually built for this (explicitly deferred, correctly), its first
   required negative self-test per `gate-self-test-rule.md` is a signal
   citing a nonexistent or already-closed issue.
9. **AGREE** — blockquote Markdown degrades safely across GitLab/GitHub/
   terminal/plain-text, unlike MkDocs-only admonition syntax. See ADVISORY-4
   for the one cross-reference gap.
10. **NEEDS EVIDENCE** — same substance as ADVISORY-1; the type names don't
    overlap existing vocabulary, the field names risk drift.
11. **AGREE, valid, with a caveat** — independently and unintentionally
    reproduced this exact race twice more in this session, on MR !921 (push
    pipeline canceled at 74.661622s) and MR !923 (canceled at 75.619253s),
    both confirmed via direct GitLab API inspection, both matching the cited
    figures exactly. This is a live, currently-recurring, non-manufactured
    positive case, not a stale or contrived example. Caveat: it is GitLab-
    hosting-specific, as the proposal's own surface-boundary table already
    implies — the pilot write-up should state this plainly, consistent with
    `verifiable-claims-rule.md` and `marketing-tone-rule.md`'s discipline
    already applied elsewhere in this project (no implied universal AGF
    behavior claim).
12. **Primary kill condition:** if independent pilot readers cannot reliably
    distinguish a published signal from an ordinary risk/limitation
    statement without being told the type in advance — i.e., the taxonomy is
    legible only to its author, not to the reader it exists to help — reject
    the foundation entirely. That would falsify the core premise in the
    opposite direction: not that caveats get buried, but that the proposed
    structure doesn't make them any less buried for an unprimed reader.
    **Secondary kill condition:** if the plain-prose control condition (the
    same caveat, no signal formatting) performs equally well at changing
    reader action as the signal condition, reject — good writing alone
    suffices and the new taxonomy adds authoring and lifecycle cost for no
    behavioral gain.
