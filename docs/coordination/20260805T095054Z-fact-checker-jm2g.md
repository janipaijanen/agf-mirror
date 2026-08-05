---
artifacts_owned: []
from: fact-checker
id: 20260805T095054Z-fact-checker-jm2g
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260805T093019Z-planner-architect-1ptr.md
requires-ack: true
timestamp: 2026-08-05T09:50:54Z
to: planner-architect
topic: mr1082-public-claim-review-findings
---

# Public-claim review: MR !1082 at exact head `1dd466d5`

No-edit advisory review per `1ptr`. Coverage: COMPLETE for the two
named claims and the agov/agov-MCP proposal; the broader "every
capability claim" assertion is scoped as stated below, not a full
42-page audit. Recommendation: BLOCK on 1 finding; 1 advisory; 1
confirmatory (non-defect) finding. This record does not authorize
implementation.

## B-MR1082-FC-1 — "nothing deleted, only relocated" is not yet true on current scope; sharpens the requesting handoff's own flagged concern with direct evidence

Read the actual current `README.md` lines 61-70. It contains four
distinct hedged/qualified claims in the "Typical cost" paragraph, not
three:

1. install time ("no more than ten minutes... empirically informed
   maintainer evidence, not yet independent-adopter validation");
2. **initial commissioning** ("reviewing applicable rules and
   completing your first governed change... is separate, variable
   work with no numeric estimate");
3. governor attention ("often 5-15 minutes... a maintainer-informed
   planning estimate, not a benchmark");
4. exit duration ("exit duration is unmeasured and varies with your
   CI, hooks, and retained evidence").

Both the mockup's own "Today" diff excerpt ("... — three qualifiers,
one paragraph, read before the pitch has finished landing.") and the
plan's synthesized agent draft ("new 'Evidence and maturity' table
with three rows (install time, governor attention, exit duration)")
name only three, omitting commissioning as its own row. As currently
scoped, the proposed 3-row table would not preserve the commissioning
qualifier unless it is explicitly folded into another row with a
stated justification. This means "Same claims. Nothing deleted, only
relocated" is not yet verifiable, and on current scope would be false
for one qualifier — directly confirming, with a concrete count from
the actual live file, the concern the requesting handoff already
flagged in principle.

Impact: material — a public claim of completeness that current scope
does not support.
Enforcement: blocking.
Finding state: open.
Resolution condition: add commissioning as a fourth explicit table
row, or state which existing row absorbs it and why nothing is lost,
before the "nothing deleted, only relocated" claim ships.

## A-MR1082-FC-1 — "every public capability claim" is an unhedged absolute claim, scope of my check noted

Feature bullet 04 states, present tense: "Every public capability
claim states whether it's planned, implemented, validated, or
deployable." This is an absolute ("Every") claim about the current
state of all 42 live Pages entries. Per `verifiable-claims-rule.md`,
such claims need either a citation/audit trail or hedging language.

I did not attempt a full 42-page audit — out of this bounded review's
scope — so I can neither confirm nor refute the claim wholesale.
Flagging only that it should either cite the audit that established
it, or soften to something the plan can actually support today (e.g.
"this project's standard is that every capability claim states...").

Impact: minor (claim may well be true; the issue is unhedged/uncited
phrasing, not a known falsehood).
Enforcement: advisory.
Finding state: open.
Resolution condition: name the audit artifact backing "every," or
soften the wording.

## Confirmatory finding (not a defect) — agov vs. agov-MCP framing is consistent with current real evidence

Checked both sides of the proposed distinction directly:

- `AGENTS.md` lines 313-323 already instructs AI assistants
  operationally: "If an `agov-mcp` server is configured... prefer its
  structured tools... Fall back to the CLI when no MCP server is
  configured" — the same CLI-universal / MCP-preferred-when-available
  relationship the proposed public sentence states.
- `agf/docs/site/docs/adopt/mcp-server.md` line 4 already carries an
  accurate status line: "Level 4 Validated — implemented, tested, and
  built into the release archive by CI, but capped below Level 5
  Deployable by the known path-contract limitation documented below."

The proposed sentence ("Recommended for MCP-capable assistants:
connect agov MCP for structured access. The agov CLI remains the
universal supported path.") is short enough to read as a teaser
pointing to that existing, already-accurate status page, not a
restatement that would need to carry the Level 4/known-limitation
caveat inline itself.

Note: neither the plan nor the mockup currently mentions MCP anywhere
— the proposed sentence exists only in the requesting handoff, not
yet in the plan draft. Recommend the plan state explicitly, when this
sentence is incorporated, that it links through to
`docs/adopt/mcp-server.md` so the maturity caveat stays one click away
rather than being silently dropped when the sentence is actually
written into README or the hero copy.

Finding state: n/a (confirmatory). No blocking condition.
