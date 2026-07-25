---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260725T080107Z-governor-reviewer-k4m5
timestamp: 2026-07-25T08:01:07Z
model: human-governor
immutable: true
references-to:
  - 20260725T073230Z-planner-architect-g2k3
  - 20260725T072128Z-planner-architect-3kke
  - 20260725T072657Z-planner-architect-rs4m
topic: gate1-approvals-mrs-504-506
requires-ack: true
artifacts_owned:
  - docs/plans/framework-map-full-size-figure.md
  - docs/plans/agov-day-one-adopter-faq.md
  - docs/plans/split-person-gates-adopter-faq.md
---

# Gate 1 decisions for MRs !504–!506 — APPROVED

The human governor explicitly approved MRs !504, !505, and !506 in the active
coordination session on 2026-07-25. This records the three binding Gate 1
decisions against the submitted plan content and current MR heads.

## MR !504 — all public Mermaid diagrams as openable SVG figures

Gate 1: **APPROVED**.

- plan commit: `121705e99102fb6adea246da04a20425f285696e`
- current MR head: `4694543203326420dfa25089cd53cbf03b343079`
- approved scope: replace the complete five-diagram public Mermaid inventory
  with governed DOT/SVG figures, provide open/full-size access, remove the
  unused Mermaid runtime only after zero inventory, update the gallery, and
  record the bounded learning evidence defined by the plan

## MR !505 — recommend agov from day one in the Adopter FAQ

Gate 1: **APPROVED**.

- plan commit: `ea9db82468a14322f5b7046577024d573e0fc98c`
- current MR head: `c89cb898d760fe443bb21bd8def572533d79593b`
- approved scope: add the plan's bounded FAQ explanation of the likely
  testing/evaluation time and token cost without agov, recommend using agov
  from day one, and preserve the stated evidence-strength limits

## MR !506 — different people at Gate 1 and Gate 2

Gate 1: **APPROVED**.

- plan commit: `f6644a387aa0dc571b0975ee7e24fd271e3bb0c4`
- current MR head: `4ed8bf872c2d36731299f31381a330e4930e3b44`
- approved scope: explain the sequential-authority, handoff, escalation, and
  no-self-approval boundaries for different authorized people at the two gates

Planner-architect may apply the administrative `status: approved` updates and
merge the plan MRs without changing their frozen content, then hand each scope
to a separate implementation branch for Gate 2. Because !505 and !506 both
edit `agf/docs/site/docs/start/adopter-faq.md`, land one implementation first
and rebase/re-verify the second so neither overwrites the other's wording or
rendered evidence.

Any change beyond the approved plan scopes requires renewed Gate 1 review.
