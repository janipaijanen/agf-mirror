# Where v2 could evolve

*Status: **Hypothesis** — everything on this page. Nothing here is a
commitment, a roadmap date, or a shipped capability. Items graduate off
this page only by the framework's own evidence discipline: operational
learning, corroboration, then promotion. Audience: anyone evaluating
long-term direction.*

## Why this page exists

The implementation-honesty rule forbids presenting planned work as
capability. The inverse failure is hiding direction entirely, leaving
evaluators to guess. This page is the middle path: hypotheses, labelled
as such, each with the evidence that would justify building it.

## Hypotheses under consideration

**Coordination-channel semantic validator.** The channel is the audit trail.
Rust `agov` already validates coordination-message schema, reports channel
status, finds stale claims, checks branch visibility, discovers handoffs, and
renders graph and graph-index views. That is useful structural automation, but
it does not prove the full governance chain: whether a referenced approval was
the right one, whether a review finding was actually resolved, whether a
handoff's policy meaning matches the work, or whether a cross-message story is
semantically complete. That deeper consistency layer remains the framework's
most concrete known validator gap. *Evidence that would promote it:* recurring
channel-integrity incidents in adopter learning records that current schema,
visibility, status, and graph tooling did not catch.

**Multi-project / multi-governor federation.** Today's validated core is
one project, one governor. Organisations running several governed
projects may want shared learnings, shared rule customizations, or a
governor hierarchy. *Evidence that would promote it:* multiple adopters
independently reporting cross-project coordination pain — none has yet.

**Richer programmatic visual verification.** The visible-artifact rule
already sketches render-and-describe loops with image-capable models as
a complement to human inspection. A shipped harness (render, snapshot,
describe, diff) is plausible. *Evidence that would promote it:* adopters
beyond the current single-project evidence reporting the same defect
class and ad-hoc harness.

**Broader assistant-tool enforcement.** The rule content is
tool-agnostic, but enforcement depth varies by assistant. Deeper
first-class integration for non-reference tools depends on adopter
demand visible in learning records.

**Deputy-governor maturation.** The delegation mechanism ships
deliberately empty (no delegable categories) on single-project evidence.
Adopter audits could corroborate which categories are routinely safe to
delegate — or show that none are, retiring the pattern.

**Delegated AI work validation.** v2.x documents principal-bound AI work as an
audit convention: a human, team, or service principal may grant bounded scope,
while the AI session and AGF role remain visible. The current convention uses
ordinary `kind: note` coordination records and is not semantically enforced by
tooling. A future version could add schema-backed delegation and contestation
objects, validate grant authorship, expiry, scope references, and contested
records, and report expired or disputed grants through `agov`. *Evidence that
would promote it:* adopters using the convention repeatedly and reporting that
manual grant review is either too error-prone or too costly for the audit value
it provides.

## What would *not* change

The invariants are not up for evolution: separated authority, immutable
audit trails, evidence-gated rule strength, the adopter-owned/
upstream-owned boundary, and the documented exit path. A hypothesis that
required weakening any of these would be rejected on principle, not
evidence.

## Following along

Watch the changelog and the learning records in each release. Anything
moving from this page into the shipped framework will arrive classified
under the downstream-impact discipline, with migration notes if it
touches you.
