<!-- Radar carrier. Retained verbatim from MR !923 (source branch proposal/expectation-signals-consensus, head 8bb1efc2), file docs/proposals/expectation-signals-consensus.md.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
# Proposal: expectation signals for material caveats

Status: consensus reached; Level 1 Planned and Theoretical. This is not Gate 1,
rule, template, validator, or implementation authority. Tracking issue: #100.
Related evidence questions: #53 and #60. Independent teardown: MR !924 at
exact head `25b2688363393fe67bffc6913e215b817370827d` (CLEAR; zero
blocking and four advisory findings incorporated below).

## Decision to reach

Decide whether AGF needs a small, portable convention that prevents a
load-bearing caveat from remaining technically truthful but practically buried
in ordinary prose. The convention must improve reader action without becoming
warning decoration, an AI rhetorical weapon, or a parallel finding system.

Codex's initial position after attempting to reject the idea was **retain the
foundation, narrow the mechanism**. Claude independently tried to reject it
through existing-mechanism substitution, semantic-field overlap, a concrete
counterexample to the threshold, and AI-hedging misuse. The foundation
survived, subject to the pilot controls recorded here. No implementation
follows directly from consensus.

## Current mechanisms and the uncovered gap

AGF already has several adjacent mechanisms:

- `verifiable-claims-rule.md` governs whether factual prose is supportable;
- evidence status calibrates framework-level claims;
- capability maturity says what exists and at what operational level;
- review-finding fields carry outcome, impact, enforcement, state, and risk
  disposition in review records;
- plan risks and coordination handoffs route work and authority; and
- Pages admonitions can visually emphasize selected text.

None answers this narrower reader-interface question consistently:

> Is this caveat prominent enough when missing it would materially change what
> a reasonable target reader does or believes about the artifact?

The first-push pipeline race is a concrete example. It is accurately described
in plans and a consensus proposal, while assistant and contributor entry points
say little about why the first duplicate pair can still happen. The prose is
true; the expectation-setting is weak.

## Teardown: reasons to reject the naive idea

### 1. “Unexpected” is subjective

An assistant's intuition about what surprises a human is not evidence. Different
readers have different experience, and labeling behavior “unexpected” can imply
the system is defective before that judgment is established.

**Consequence:** reject a free-form `UNEXPECTED` badge. The candidate becomes
an **expectation gap**, which must name both the expectation invited by the
artifact/context and the verified actual state.

### 2. “Controversial” is rhetorical

Calling a choice controversial can manufacture conflict, amplify a minority
objection, or stigmatize a legitimate decision. An AI must not infer social
controversy from its own reaction.

**Consequence:** reject `CONTROVERSIAL`. The narrower type is **open decision**:
credible alternatives remain unresolved by the named authority. Evidence is
the alternatives and unresolved decision state, not a tone judgment.
An author who can recommend a position must do so in ordinary prose or
escalate it under `uncertainty-and-scope-rule.md`; an open-decision signal
records unresolved authority and must not become an AI hedging substitute for
analysis or escalation.

### 3. Warnings consume scarce attention

If assistants add banners defensively, important signals become indistinguishable
from boilerplate. A “signal everything” rule directly worsens issue #60's
reviewer-attention problem.

**Consequence:** use a counterfactual-action threshold, not a list of topics.
The author must complete: “If the target reader misses this, they may ___;
after seeing it, they should instead ___.” If the action or interpretation does
not materially change, ordinary prose is enough.

### 4. A signal can masquerade as evidence or authority

Visual prominence can make an AI-authored caution appear more authoritative
than a sourced paragraph. It could also be mistaken for a blocking finding.

**Consequence:** every signal states evidence (or missing evidence), response,
and authority. It explicitly does not block unless a named rule, contract, CI
policy, or governor decision independently grants blocking authority.

### 5. Signals can duplicate structured findings

Review records already have a finding contract. Copying a finding into a new
signal vocabulary creates divergent status and resolution histories.

**Consequence:** coordination reviews and Gate decisions keep their existing
structured findings. A signal may link to a finding from reader-facing prose,
but does not reproduce or supersede its lifecycle.

### 6. Stale warnings become misinformation

A limitation banner that survives the fix is worse than a buried accurate
caveat. Temporary warnings often outlive the issue that motivated them.

**Consequence:** every non-permanent signal names a resolution/revisit
condition and an issue/decision owner. The change that resolves the condition
must remove or rewrite the signal atomically.

### 7. Visual callouts can be inaccessible and renderer-specific

Color, icons, or MkDocs-only admonition syntax can disappear in raw Markdown,
GitLab, GitHub, terminals, screen readers, or assistant context.

**Consequence:** the semantic form is plain text and blockquote Markdown.
Renderers may enhance it, but the type and fields remain understandable without
CSS, color, or an icon.

### 8. A new vocabulary can conceal bad design

Repeatedly labeling surprising behavior may normalize it instead of fixing it.

**Consequence:** signals do not satisfy remediation. A verified harmful or
avoidable condition still requires an issue, plan, finding, or decision. The
signal only protects readers during the interval when the condition exists.

## Foundation that survives the teardown

An **expectation signal** is a sparse, evidence-linked, non-authoritative
reader-attention block for a material caveat that passes the
counterfactual-action threshold.

Candidate types:

| Type | Required factual basis | What it tells the reader |
|---|---|---|
| **Expectation gap** | The artifact/context invites expectation X; verified behavior is materially different Y | Do not operate on X; use the named response for Y |
| **Evidence gap** | A load-bearing boundary has not been tested, observed, or independently verified | Do not infer coverage; use the bounded fallback or obtain evidence |
| **Open decision** | Named credible alternatives remain unresolved by the authorized decision-maker | Do not present one alternative as settled; follow the interim boundary |

“Blind spot” remains useful explanatory prose, but **evidence gap** is the
proposed formal type because it names the observable deficiency without
claiming that every participant is unaware of it.

## Threshold: the counterfactual-action test

A signal is warranted only when all are true:

1. The target audience and artifact scope are named.
2. The caveat is material to correctness, safety, cost, authority, privacy,
   evidence interpretation, reversibility, or the reader's next action.
3. Missing the caveat plausibly leads to a different action or materially false
   interpretation.
4. The signal can provide an actionable response or explicit stop boundary.
5. The claim is evidenced, or the absence of evidence is itself stated
   precisely.

Ordinary risks, optional advice, background limitations, TODOs, and facts that
do not change reader action stay in ordinary prose. There is no arbitrary
numeric signal quota; reviewers consolidate duplicates and reject signals that
cannot pass the threshold. The test remains a prediction about reader action,
not an objective materiality oracle. The pilot must therefore compare it with
negative controls and unformatted prose rather than treating an author's
completed template as proof that a signal is warranted.

## Portable semantic template

```markdown
> **Expectation signal — evidence gap**
> **Scope and audience:** Release operator validating a tag.
> **What changes your action:** The integration boundary has not been exercised;
> a green unit-test job is not evidence for it.
> **Known / unknown:** Unit behavior is verified; external-service behavior is unknown.
> **Evidence:** Pipeline/job IDs, source link, or “no representative execution yet.”
> **Response:** Stop publication or run the named representative check.
> **Authority:** Attention signal only; blocking authority comes from <rule/decision>.
> **Resolve or revisit:** Issue/owner and observable condition.
```

Surface-specific compression is allowed only when every applicable semantic
field remains recoverable. A Pages theme may render the block as an admonition;
plain Markdown is canonical.

AGF already uses an informal MkDocs admonition in
`concepts/philosophical-lenses.md` for “Consultative, not normative.” That is
permanent scope-framing, not a temporary material caveat with a resolution
lifecycle. A pilot must name and preserve this distinction so the two
conventions do not silently converge.

## Author and reviewer responsibilities

- Any human or AI author may **propose** a signal within the artifact they are
  authorized to draft. Requiring a separate human decision merely to flag a
  possible caveat would discourage candor.
- The artifact's normal reviewer checks factual basis, threshold, response,
  authority wording, accessibility, and lifecycle link.
- The governor decides only the underlying gate/policy question, not whether an
  author was allowed to raise attention.
- An AI may not label social controversy, infer reader expectations without
  context, or use a signal to expand its authority.
- An AI may not use **open decision** to avoid making a supportable
  recommendation or the escalation already required by
  `uncertainty-and-scope-rule.md`.
- Removing a still-current signal without resolving or explicitly accepting
  its condition is a reviewable change, not editorial cleanup.

## Proposed surface boundary

| Surface | Candidate use |
|---|---|
| README and public documentation | Yes; full portable form for material reader-facing caveats |
| Plans and proposals | Yes; for load-bearing assumptions/gaps that change approval interpretation |
| AGENTS/instruction files | Yes; concise operational response plus evidence/revisit link |
| Release information | Yes, only for a current release limitation that changes install/operate action |
| Coordination reviews and gate decisions | No parallel signal; use the existing finding/decision contract and link from reader-facing prose |
| Changelog | Normally no; record what changed, then link to the affected documented signal if needed |
| Source-code comments | Out of scope for the initial convention |

## Interaction with existing vocabulary

- **Evidence status** says how strong support for a framework claim is. An
  evidence-gap signal says a specific missing boundary changes reader action.
- **Capability maturity** says what operational level exists. An expectation
  gap may warn that the surrounding page otherwise invites a higher-level
  interpretation.
- **Review findings** route evaluated nonconformance and enforcement. Signals
  route reader attention and carry no independent enforcement.
- **Risk sections** help decision-makers evaluate a plan. Signals are used only
  when a risk also changes how the artifact's target reader should act now.

Point-of-use signals and coordination review findings have separate audiences
and lifecycles, but their evidence fields must not drift into competing
meanings:

| Signal field | Closest review-finding concept | Required distinction |
|---|---|---|
| Scope and audience | Check / artifact under review | Names the reader-facing context, not a compliance check |
| What changes your action | Outcome / finding | States the reader consequence, not pass/fail status |
| Evidence | Basis | Same evidence discipline; a signal may link to, but not copy, a finding lifecycle |
| Response | Resolution condition or recommendation | Immediate reader action, not finding closure |
| Authority | Enforcement | Explicitly attention-only unless separate authority is cited |
| Resolve or revisit | Finding state / resolution condition | Names external ownership and observable removal/rewrite condition; it is not a second finding state |

This mapping is a pilot acceptance item. Divergence is allowed only where the
reader-facing purpose above requires it; near-duplicate semantics must be
consolidated rather than named twice by default.

## Four-Way-Test screen

1. **Truth:** the signal separates verified state, missing evidence, and open
   authority rather than blending them.
2. **Fairness:** affected readers receive material caveats at the point of use.
3. **Goodwill:** neutral type names avoid alarmist “controversial” labels and
   blame attribution.
4. **Benefit:** the response field must change or protect reader action; a
   non-actionable banner fails the threshold.

This screen supports the foundation but does not prove the mechanism works.

## Pilot before rule promotion

Because consensus retains the foundation, a later Gate 1 plan should run a small
documentation-only pilot before adding an always-on rule or validator:

1. **Positive expectation-gap case:** current GitLab-specific first-push
   duplicate-pipeline limitation under issue #53. Do not imply that AGF or all
   CI hosts exhibit this behavior.
2. **Positive evidence-gap case:** one existing public capability boundary
   with a verified untested surface, selected by fact-checker.
3. **Positive open-decision case:** one genuinely unresolved plan choice with a
   named decision owner.
4. **Negative cases:** an ordinary implementation risk, a blocking review
   finding, a maturity label, a harmless surprising fact, and an ordinary
   documented precondition mechanically dressed in the counterfactual-action
   template must all remain outside the signal mechanism. Example control:
   “UTF-8 required; convert Latin-1 first” does not become a material
   expectation signal merely because an author fills every field.
5. Verify manually that every issue/owner named by **Resolve or revisit**
   exists and has the claimed open lifecycle state. If syntax validation is
   proposed later, its first required negative self-tests are nonexistent and
   already-closed issue references.
6. Ask unprimed independent readers to state expected action before and after
   both (a) a plain-prose control and (b) the signal form, whether they can
   identify the signal without being told its type, whether they mistake it
   for blocking authority, and whether any field is redundant.

Reject the foundation—not merely its wording—if either primary kill condition
occurs:

1. Unprimed independent readers cannot reliably distinguish a signal from
   ordinary prose/negative controls without first being told its type.
2. The plain-prose control changes reader action as effectively as the signal
   form; in that case good writing is sufficient and the taxonomy adds cost
   without behavioral value.

Also reject or redesign the mechanism if reviewers cannot distinguish positive
from negative cases (including the dressed-up precondition), readers infer
false blocking authority, signals do not change the intended action, or
lifecycle ownership cannot be maintained.

Do not build a validator in the pilot. Semantic materiality and reader
expectation are not safely machine-decidable. A later validator may check only
stable syntax, required fields, broken lifecycle references, or explicitly
forbidden visual-only forms after usage supplies evidence.

## Consensus outcome

MR !924 answered all twelve questions after an independent attempt to reject
the idea:

- **AGREE:** the point-of-use reader-attention gap is real; the three types,
  author boundary, surface boundary, portable blockquote form, and
  GitLab-qualified first-push pilot case survive.
- **NEEDS EVIDENCE:** the action threshold, field proportionality, lifecycle
  references, and vocabulary interaction must be tested in the pilot rather
  than asserted.
- **No disagreement or blocking finding:** the foundation survives, narrowed
  by the cross-map, fifth negative control, anti-hedging guard, admonition
  distinction, issue-state check, and two explicit kill conditions above.

The original questions are retained below as the audit frame for any future
pilot report.

## Original consensus questions

Return `AGREE`, `DISAGREE`, or `NEEDS EVIDENCE` for each, with blockers,
advisories, and rejected alternatives separated:

1. Does a reader-attention gap remain after accounting for verifiable claims,
   evidence status, maturity, risks, and review findings?
2. Does the foundation survive, or should AGF improve existing mechanisms
   instead of naming expectation signals?
3. Are `expectation gap`, `evidence gap`, and `open decision` sufficiently
   distinct and non-rhetorical? Merge, rename, or remove any that are not.
4. Is the counterfactual-action test a workable threshold against warning
   fatigue? What concrete counterexample defeats it?
5. Are the mandatory semantic fields sufficient and proportionate?
6. May AI authors propose signals without prior human authorization, given
   ordinary review and unchanged gate authority?
7. Is the surface boundary correct, especially excluding parallel signals from
   coordination review records and changelogs?
8. Does the lifecycle rule adequately prevent stale banners? If not, what
   observable control is needed without premature semantic automation?
9. Is portable blockquote Markdown the right canonical representation?
10. Does the mechanism interact cleanly with evidence/maturity/finding
    vocabulary, or create an unacceptable second taxonomy?
11. Is the first-push race a valid pilot, or too project-specific/easy to make
    the pilot informative?
12. What result from the pilot would cause you to reject the foundation rather
    than merely tune its wording?

## Stop condition

No rule, schema, validator, template, public claim, or first-push signal is
implemented from this proposal. Claude's independent critique is incorporated;
the next authorized step, if the governor chooses it, is a separate Gate 1
plan for the documentation-only pilot at this proposal's exact revised head.
