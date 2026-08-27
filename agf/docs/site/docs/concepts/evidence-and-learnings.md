# Learning loop and evidence

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: developers and
governors. After this page you know how operational incidents become
rules, who owns which records, and how anonymization protects your
project.*

!!! note "AGF v2 layout"
    This page uses the v2 default layout: framework-owned learning records live
    under `agf/learnings/`; project working evidence lives under root `docs/`.

## The learning loop

The framework grows from evidence, not opinion, and Figure 1 exists to
make that claim checkable: it draws the loop as a closed cycle rather
than a one-way pipeline — governed work creates evidence, evidence
becomes a learning record, system-level analysis turns a pattern across
records into a proposal, governed approval precedes implementation, and
observed project use feeds the next cycle rather than ending the process.
The point to retain: because the loop closes, a rule that stops matching
observed practice is itself evidence for the next cycle, not a fixed
requirement exempt from the process that produced it.

![Recurring learning cycle from governed work through system change and observed use](../../reference/figures/learning-loop.svg)

*AI-generated figure. Publication remains the responsibility of the human maintainer.*

## Established foundations

AGF's loop adapts established learning and safety traditions. These sources
help explain the design; their existence does **not** empirically validate AGF
or prove that an adopter's process is effective.

- **Double-loop learning.** Argyris and Schön distinguish correcting an action
  within existing assumptions from questioning the governing assumptions,
  policies, or objectives themselves. Routine implementation correction is the
  first kind. A learning that proposes changing an AGF rule, authority
  boundary, or gate is the second. See
  [*Organizational Learning: A Theory of Action Perspective*](https://books.google.com/books?vid=ISBN0201001748)
  (1978).
- **PDSA.** The Deming Institute describes Plan–Do–Study–Act as an iterative
  method for generating knowledge: predict, act, study observed results, then
  adjust the theory or method. In AGF, governed implementation supplies the
  action and project-use evidence supplies the study; a completion record alone
  does not close the learning cycle. See the
  [PDSA Cycle](https://deming.org/explore/pdsa/).
- **System-level causal analysis.** Leveson's CAST method asks how a control
  structure allowed an undesired loss and what controls should change, rather
  than stopping at an individual “root cause.” AGF's analysis step likewise
  asks whether a failure belongs in implementation, role boundaries, a gate,
  tooling, or the rule set. See the MIT
  [CAST Handbook](https://sunnyday.mit.edu/CAST-Handbook.pdf).
- **Continual AI-management improvement.** ISO/IEC 42001 specifies continual
  improvement of an AI management system and uses a Plan–Do–Check–Act model.
  AGF's records can contribute evidence to such a management process, but AGF
  does not by itself implement ISO/IEC 42001 or confer conformity. See
  [ISO/IEC 42001:2023](https://www.iso.org/standard/42001).

These relationships are **theoretical and prior-art informed**. Evidence that
AGF's particular loop improves outcomes must come from observed adopter use,
independent evaluation, and learning records—not analogy alone.

## Design provenance: NASA LLIS

NASA's [Lessons Learned Information System](https://llis.nasa.gov/),
particularly public [Lesson 30101](https://llis.nasa.gov/lesson/30101),
prompted AGF's review of learning-record structure and lifecycle. AGF
subsequently adopted its own append-safe `supersedes` field on the newer
record, derives the reverse relationship instead of editing historical
entries, and formalized its existing optional `proposed | implemented` status
convention.

This acknowledgement describes design influence—not protocol equivalence,
NASA endorsement, or comparable assurance stakes. AGF did not adopt NASA's
overall governance body, review workflow, classification model, or lessons
protocol. See [Influences and acknowledgements](../../acknowledgements.md) for
the project's credit policy and other material foundations.

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
- Design provenance and credit boundaries:
  [Influences and acknowledgements](../../acknowledgements.md)
