# Evidence and learnings

*Status: Released (v2.1 documentation). Audience: developers and
governors. After this page you know how operational incidents become
rules, who owns which records, and how anonymization protects your
project.*

!!! note "AGF v2 layout"
    This page uses the v2 default layout: framework-owned learning records live
    under `agf/learnings/`; project working evidence lives under root `docs/`.

## The learning loop

The framework grows from evidence, not opinion. Figure 1 shows the
learning loop: governed work creates evidence, evidence becomes a
learning record, learning records propose changes, and project use feeds
the next cycle.

![Learning loop](../../reference/figures/learning-loop.svg)

Rules shipped on single-adopter evidence carry a **Provisional** header
naming the evidence and asking adopters who hit the rule to file a
learning record. That header is the honesty mechanism: you can always
tell how much evidence stands behind a constraint. The escape hatch is
irreversibility — rules preventing unrecoverable damage (anonymization
leaks, moved git tags) may ship at full strength on single-source
evidence, with the reasoning stated in the rule.

## Who owns which records

Table 1 separates upstream learning records from adopter-owned learning
records and shows what happens to each during framework upgrades.

| Record | Owner | On upgrade |
|---|---|---|
| `agf/learnings/<framework-codenames>.yaml` | Upstream | Replaced |
| `agf/learnings/<your-codename>.yaml` | **You** | Never touched |

The replace-on-upgrade boundary is explicit so that your operational
knowledge survives every framework release. Your file accumulates your
project's incidents; upstream files carry the corroborated evidence base
the rules cite.

## Anatomy of a learning record

A learning record states what happened, what it cost, what was changed,
and what the framework should learn — in schema-validated YAML so the
contribution-audit tooling can process it. Severity is graded; a
critical-severity learning (an incident that produced an irreversible
loss or a near-miss) gets priority in rule formation.

Your first useful learning can be the commissioning run itself. Record whether
the assistant understood the install prompt, kept AGF-owned files inside
`agf/`, created only a small tool-specific loader if needed, and left the
governor with clear decisions. That evidence is valuable upstream because it
tests the adoption path before the first governed code change.

Do not add a made-up global learning ID just because an example looks like it
needs one. Use the fields in the shipped template. If a downstream project
adds IDs later, use `YYYYMMDDThhmmssZ-<4-char-random>`, or equivalent local
tooling that cannot collide. Preserve existing IDs when updating old learning
records; use a new suffixed ID only for a new or superseding entry.

## When evidence becomes a learning

Not every change deserves a learning record. Use learning records for
operational evidence that should change future behavior, not as a diary of
normal work.

A single typo, self-corrected compile failure, or routine test failure is
usually normal iteration. Record it only when it exposes a rule gap, repeats
often enough to change how agents should work, or nearly becomes bad merge
evidence.

Table 2 gives the default threshold.

| Situation | Default action | Example |
|---|---|---|
| Normal iteration | Do not record | A test failed, the agent fixed the implementation, and no governance rule or review path changed. |
| First commissioning run | Record | The assistant installed `agf/`, created the loader, and left decisions for the governor clearly enough to repeat. |
| Repeated friction | Record | Two branches in one week needed the same manual correction to coordination metadata or assistant rule loading. |
| Rule boundary violation | Record and consider upstream | An assistant wrote outside the approved `agf/` boundary, used an unapproved branch, or skipped a required coordination entry. |
| Rule helped | Record | A validator, checklist, or governor gate caught a risky migration before merge. |
| Rule hurt | Record | A rule was so heavy that the team routed around it or used fast track where full review was warranted. |
| Near-miss | Record and consider upstream | A malformed decision record almost became merge evidence, but review caught it. |
| Irreversible loss, secret exposure, moved release tag, or boundary breach | Escalate immediately, then record | Stop work, notify the governor, preserve evidence, and write the learning after containment. |

If you are unsure, ask: **Would the next agent, reviewer, or adopter make a
better decision if this were written down?** If yes, record it. If the answer
is only "we completed routine work", keep it in the normal plan, review, or
merge request trail.

## Anonymization: codenames at the boundary

Inside your repository, your project's natural vocabulary is correct and
expected. At the **export boundary** — filing a learning upstream, an
issue on an external tracker, anything leaving your infrastructure — the
only project handle that may appear is your codename. Domain-revealing
schema names, file paths, and scenario names are replaced with neutral
generics. The anonymization rule treats AI-assistant prompts sent to a
provider API as external content too: the editor feels local; the prompt
is not.

This is what makes upstream contribution safe: the evidence flows, the
identity does not. Codenames in this documentation (*foundry*,
*silver-birch*, *copper-rapids*, *blue-marlin*) are exactly this
mechanism in operation.

## Why evidence discipline matters

A governance framework that invents constraints faster than it validates
them becomes ceremony. The evidence discipline is the brake: every rule
cites the incident or learning that motivated it (the verifiable-claims
rule applies to the framework's own documentation), Provisional status is
visible, and rules that fail to corroborate are candidates for
retirement, not permanence.

## Next

- The daily rhythm that produces these records:
  [day-to-day operation](../operate/day-to-day.md)
- Wiring the validators that check them:
  [contracts and validators](../advanced/contracts-and-validators.md)
