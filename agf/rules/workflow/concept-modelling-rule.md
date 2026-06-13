<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Concept Modelling: Intrinsic vs Derived Attributes
#
# Framework principle: Verification over Assumption — the data model
# is a claim about the domain that needs explicit checking
# See: framework.md — Pattern 4 (Tester Discipline)

> **Provisional status.** Single-adopter evidence (silver-quill,
> 2026-04-18, framework v0.30.2 — issues #28 and #29). The original
> proposal was for a new `concept-modeller` agent definition; this
> rule downgrades the artifact form to a Gate 1 / Gate 2 review
> checklist item per the framework's calibration: rules ship on
> single-adopter evidence with a Provisional header; new agents wait
> for 2+ adopter evidence (heavier long-term cost — jurisdiction,
> communication channels, MANIFEST entries). The same finding can
> still be reported by any reviewer at any gate. Adopters hitting
> this rule (any project where data models survive code review and
> domain reviewers but encode wrong-axis attributes): please file a
> learning record so we can promote from provisional to recommended,
> or convert to a dedicated agent if the operational pain warrants
> it.

**Scope:** Any task that introduces, modifies, or moves attributes
on persisted data. Examples: a new column on an existing table; a
new field on a schema/model class; a new dimension on an analytics
event; a new label on a classification scheme; a new index key.

## The rule

For every new or moved attribute, a reviewer (Planner-Architect at
Gate 1, or any reviewer at Gate 2 — see CUSTOMIZE) MUST classify it
as one of:

1. **Intrinsic** — the value is a property of the entity itself,
   independent of how the entity was collected, queued, processed,
   or discovered. Example: a publication's *venue* is intrinsic to
   the publication; the publication has the same venue regardless
   of which search engine surfaced it.

2. **Derived from collection / processing path** — the value
   depends on how the entity was obtained, not on the entity itself.
   Example: which search engine returned this result; which queue
   the job was pulled from; which session triggered the import.

3. **Composite** — the attribute encodes both. Composite attributes
   are usually a smell: they conflate two axes that downstream code
   will eventually need to separate.

The classification MUST be written down — in the plan, the ADR, or
the changeset comment — and the storage location MUST match the
classification. An intrinsic attribute belongs on the entity's own
record; a derived attribute belongs on a join record (entity +
collection-context); a composite attribute belongs nowhere and
should be split.

## Failure mode this catches

Intrinsic and derived attributes routinely look interchangeable in
isolation. Both are scalar fields; both can be filled in at the
same code site; both type-check the same way. The error becomes
visible only when:

- A second collection path reaches the same entity through a
  different route (silver-quill: a journal article reached through
  Google Scholar vs. through a publisher's RSS feed had different
  `source_category` values stored, even though the venue was the
  same peer-reviewed journal in both cases).
- A query asks "what's the X of all entities of type Y?" and
  returns inconsistent results because the field was per-collection,
  not per-entity.
- A migration / dedup job needs to reconcile multiple records for
  the same logical entity and discovers that intrinsic-looking
  fields disagree.

## Concrete example (silver-quill, 2026-04-18)

A `source_category` attribute was implemented as a property of the
collection method (which provider queued the article). The bug:
*category* is intrinsic to the venue (the article's publication
URL), not derived from the discovery path. An article from a
peer-reviewed journal queued via a general web search was being
classified as "grey literature" because the queue-provider was
classified that way, even though the journal itself was clearly
"white" (peer-reviewed).

The bug existed from feature introduction. It was **not caught by**
code review, the infosec assessment, or the data-quality assessment
— neither of those agents' mandates covers semantic correctness of
the data model. It was surfaced only when a domain-modelling
question was asked conversationally: *"if a general search engine
points to a peer-reviewed journal, is the article white or grey?"*

The fix added a domain → category map (intrinsic to the venue URL)
and removed the queue-provider keying. Three call sites changed.

## Review checklist (per-change, Gate 1 or Gate 2)

For any new or moved attribute on a persisted entity:

- [ ] Is this attribute **intrinsic** to the entity (property of the
      entity itself), or **derived** from how the entity was collected
      / queued / processed?
- [ ] If intrinsic: is it stored on the entity's own record (not on
      a collection-context record)?
- [ ] If derived: is it stored on a join record that explicitly
      ties entity + collection-context, with the collection-context
      identifier captured?
- [ ] If composite: can it be split? If not, is the composite-ness
      documented and the downstream split-cost accepted?
- [ ] Pay extra attention to **classification / categorisation /
      tier / source-type / quality-grade** fields — these are the
      most common site of this error.

## Anti-patterns

- **"It's just a string field, who cares which axis it's on."** —
  Until the second collection path arrives, then it matters
  irrecoverably. Migrations of misaligned attributes are expensive.
- **"The infosec / data-quality reviewer will catch it."** — They
  will not. Their mandates cover information leakage and data
  *integrity*, not data *modelling*. silver-quill confirmed this:
  two assessment passes missed the bug.
- **"We'll classify it correctly when we have to."** — The
  classification is cheap at write time and expensive at migration
  time. Always-classify is cheaper.
- **"Composite is fine; we'll split it later."** — Composite
  attributes accumulate downstream consumers that all silently
  assume one of the two axes. Splitting later means reconciling
  every consumer.

## CUSTOMIZE

Adopters MUST customize:

- **Which gate runs the check** — Gate 1 (Planner-Architect names
  the classification in the plan) or Gate 2 (reviewer verifies
  storage matches classification) or both. Default: both.
- **Which roles run the check** — Planner-Architect, Coder-Tester,
  Governor-Reviewer, or a project-specific data-quality reviewer.
  silver-quill found a project-originated `data-quality` agent
  surfaced this gap when none of the upstream agents did.

Adopters MAY customize:

- **Domain-specific intrinsic-vs-derived examples** — every domain
  has its own canonical confusion (e.g., e-commerce: *price* is
  derived from offer-context, not intrinsic to the product;
  publishing: *category* is intrinsic to the venue, not derived
  from the discovery path; logistics: *weight* is intrinsic to the
  shipment, *priority* is derived from the customer-tier).
- **Periodic audit cadence** — for projects with many attributes,
  schedule a per-milestone review of the data model.

## Related rules

- `workflow/systematic-code-review-rule.md` — Gate 2 reviewers
  apply this checklist as part of their review when the change
  touches schema or persisted data.
- (single-adopter open candidate) future agent: a dedicated
  *concept-modeller* agent definition is deferred until 2+ adopter
  evidence shows that rule-only enforcement is insufficient. See
  silver-quill #28 / #29 for the original proposal.
