# Review and release

*Status: Released (v2.1 documentation). Audience: governors and
developers operating gates and releases. After this page you can run a
Gate 2 review that earns its keep, and a release that cannot ship broken
twice the same way.*

!!! note "AGF v2 layout"
    Gate evidence is recorded in the adopter-owned coordination channel,
    `docs/coordination/` by default. Plan and coordination validation uses `agov`.

## Gate 2 in practice

A Gate 2 review is evidence-based, not vibes-based. The submission
arrives as a coordination handoff carrying:

- the branch and the plan (or fast-track waiver) it implements;
- test results — green, with the suite named;
- the self-review checklist outcome;
- for visible artifacts (drawings, PDFs, dashboards, generated docs):
  confirmation a human or visual reviewer **opened the output and
  looked at it**. Automated invariants catch what someone thought to
  assert; the eyeball catches the rest. The framework's evidence base
  includes consecutive iterations where green tests missed what thirty
  seconds of looking found.

The reviewer checks scope first — does the diff answer exactly the
yes/no question the branch was sized for? Foreign commits, drive-by
refactors, and unrelated files are findings even when the code is fine.
Findings are filed as a review entry; blocking findings return the work;
approval is a decision record, possibly with conditions the merge must
honour, and the merge commit references it.

Figure 1 places Gate 2 in the full governed pipeline. The reviewer is checking
the implemented branch against the approved plan before the merge, not after
the fact.

### Gate 2 review checklist

Copy this into your review entry or run through it before approving:

- [ ] Approved plan or fast-track waiver identified in submission
- [ ] Branch name matches the sized work; diff answers exactly one yes/no question
- [ ] All named tests green; no deleted or weakened tests without replacement
- [ ] Visible outputs inspected where applicable (drawings, PDFs, generated docs)
- [ ] No security-boundary changes, secret exposure, or unreviewed infrastructure
- [ ] Findings recorded (blocking findings return the work; non-blocking noted)
- [ ] Decision record references branch and any merge conditions

### Evidence quality labels

The governor may label a Gate 2 decision with evidence quality. This is not a
new maturity level; it is a local review aid for spotting weak approvals later.

| Label | Meaning |
|---|---|
| Complete | Plan or waiver, branch, diff scope, tests, visible-output review, findings, and decision chain are present. |
| Adequate | Minor omissions exist, but no material uncertainty affects the merge decision. |
| Weak | Approval relies partly on assertion, incomplete evidence, or missing review detail. Use only with an explicit residual-risk note. |
| Failed | Evidence chain is missing or unverifiable; do not merge until repaired or rejected. |

The labels above apply at one specific point in the pipeline Figure 1
shows: the Gate 2 review that sits after implementation and before
merge. A "Weak" or "Failed" label is a claim about the state of the
evidence chain at that single point, not about the plan reviewed
earlier at Gate 1 or the release step that follows merge — those are
separate stages in the same figure, with their own review discipline.
The point to retain: evidence quality is scoped per gate, so a strong
Gate 1 does not offset a weak Gate 2, and neither substitutes for the
release discipline covered next.

![Gate pipeline](../../reference/figures/gate-pipeline.svg)

## Releases: the immutability discipline

Two rules interlock:

1. **Tags are immutable once pushed.** Never deleted, never moved. A
   broken release is superseded by the next version number, never
   repaired in place. Version numbers are free; moved tags break caches,
   images, and trust.
2. **Releases go through a merge request.** Branch `release-X.Y.Z` from
   main; commit *only* the version bump and changelog there; review the
   MR (the diff is tiny by construction); merge; tag the merge commit;
   push the tag.

The order is load-bearing: bump-on-main-first produces an empty,
unreviewable MR; tag-before-merge produces an ungated release.

Before a release MR is approved, run `agov validate adopter-docs`. The check
blocks releases where adopter-relevant framework or tooling changes have only
reference/internal documentation and no downstream-facing docs, templates, or
governor-visible waiver. Release changelog entries for adopter-relevant
changes should include one of:

- `Adopter docs: updated`
- `Adopter docs: not applicable`
- `Adopter docs: waived <coordination-id>`

Gate 2 review remains a human decision. Current tooling does not prove
plan-to-diff binding or semantic completeness of the evidence trail. If the
release diff changes scope, omits adopter-facing documentation, or relies on a
weak coordination record, the reviewer should require a plan update,
coordination repair, or explicit waiver before approval.

## Gates must be proven to fire

Any new gate — hook, validator, CI check — ships with a **negative
test**: a deliberately bad input, demonstrated to be blocked, with the
transcript in the MR. The framework adopted this after three separate
incidents in its own history where a gate shipped broken (a skill nobody
ran, a check nothing invoked, a hook that wasn't executable) and the
failure mode it guarded recurred while everyone assumed it could not.
A gate that ships unproven is worse than no gate: it buys false
confidence at the price of the discipline it replaced.

## Upgrading the framework

Framework releases replace upstream-owned files and never touch
adopter-owned ones (your codename's learnings, your customized rules) —
the manifest marks which is which. The upgrade is an ordinary governed
change: branch, replace, diff-review what upstream changed, Gate 2,
merge. Pin the version; read the changelog for BREAKING entries — the
downstream-impact discipline requires upstream to classify every change
and provide migration notes for breaking ones.

## Audit cadence

Per-change review catches per-change problems. Some gaps are
structurally invisible per-change — missing audit-log calls, drifted
claims between documents, accumulating fast-track waivers — and only a
periodic cross-cutting sweep finds them. Set a cadence (per milestone or
per release), and let the governor's audit of deputized approvals ride
the same rhythm.

Useful anti-ceremony metrics:

- coordination messages per merged governed change;
- fast-track waivers per milestone or release;
- corrective coordination records after missing or malformed evidence;
- learning records promoted to rules, docs, validators, or retired as local-only;
- rules retired or deleted as inapplicable;
- governor review time by change class;
- Gate 2 decisions labelled weak or failed.

The point is not to optimize the numbers downward. The point is to notice when
the trail becomes noise, when fast track becomes the default for everything, or
when evidence records stop changing decisions.

## Next

- The machinery underneath: [contracts and validators](../advanced/contracts-and-validators.md)
