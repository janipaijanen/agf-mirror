---
id: input-provenance-manifest-49
status: pending-gate1
topic: input-provenance-governance
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No code in this plan; it proposes an additive optional JSON-schema
      field and a new rule file. Given the direct security relevance
      (supply-chain-adjacent drift detection, OWASP LLM03 mapping in the
      companion threat-model plan), recommend an infosec-sentinel pass at
      Gate 2 on the final rule text and the schema diff.
  - role: contribution-auditor
    status: waived
    reason: >-
      New governance rule and schema-field addition, no external
      contribution in this plan itself. Recommend Gate 2 contribution-auditor
      review of the shipped rule text and schema diff.
---

# Plan: Input Provenance Governance — Instruction Supply Chain as a Governed Record

**Date:** 2026-07-20T07:51:31Z
**Status:** PENDING GATE 1 APPROVAL
**GitLab issue:** #49
**Cluster:** declared threat model + governance primitives
**Depends on (conceptually, not mechanically):** `docs/plans/threat-model-stpa-owasp.md`
(#42) — this plan cites UCA-4 from that plan as the threat class it addresses.

---

## Intent

AGF governs and audit-trails **outputs** — code changes, gate decisions,
coordination messages — but currently has no equivalent trail for the
**inputs** that determined agent behavior at the time a governed action was
taken: which version of the rule files, which skills, which MCP server
definitions (and their declared tool surfaces) were actually in effect. If a
rule file, skill, or MCP server changes between one governed action and the
next, there is currently no recorded answer to "which version of the
instructions was actually in effect when this decision was made?"

This directly closes **UCA-4** named in the companion threat-model plan
(#42): "An agent acts under a rule file / skill / MCP server definition whose
version silently differs from what was reviewed, with no record of which
version was actually in effect" — mapped there to OWASP LLM03 (Supply Chain),
currently rated **Not addressed**.

Target alignment: CycloneDX's open "Agent BOM" proposal (an ML-BOM extension
covering MCP servers, tools, and authorizations —
`github.com/CycloneDX/specification/issues/895`), adjacent to OWASP's AIBOM
work. The proposal is still open/unstable — this plan tracks it for eventual
alignment, it does not adopt it wholesale.

---

## Scope

**In scope (first pass — minimal manifest):**
1. Record, per governed action, the versions/hashes of the rule files loaded,
   any skills invoked, and any MCP servers connected with their declared tool
   surfaces.
2. Evaluate CycloneDX Agent BOM as the eventual target export schema, without
   claiming conformance today.
3. Start with a minimal manifest (rule-tree commit hash + skill names + MCP
   server identifiers), not a full BOM implementation.
4. The manifest MUST be a plain, human-readable file under version control —
   no external registry or service (removability constraint, see below).

**Out of scope (explicitly, per the issue):**
- Full CycloneDX Agent BOM conformance — the upstream proposal is open and
  unstable; this plan tracks it, does not implement against it.
- Automatic tooling that populates the manifest at every governed action —
  follow-on implementation work. This plan defines the shape and the schema
  change only.
- MCP server authentication/authorization detail beyond declared tool-surface
  names.

---

## Design: reuse the existing coordination message, do not invent a new file type

The issue's own scope note says "start with a minimal manifest... rather than
a full BOM implementation." The simplest shape that satisfies that, and the
removability constraint, is **not** a new sidecar file format — it is one new
**optional** top-level field on the coordination message schema that already
exists, already lives under version control, and is already the audit trail
for governed actions.

Verified against the current schema (`agf/schemas/coordination-message.schema.json`):
the schema has no top-level `additionalProperties: false`, so a new optional
top-level property is additive and does not invalidate any existing
coordination message. Verified against the Rust validator
(`tooling/rust/crates/agov-coord/src/coord_validator.rs:132-134`): the
validator loads and compiles the JSON schema file directly at validation
time (`CoordMessage.raw` is typed as `serde_json::Value`, "to avoid coupling
the struct to schema evolution" per the module's own doc comment) — so adding
an optional property requires **no Rust code change**, only a schema-file
diff. This is a materially cheaper path than a new manifest file format, and
was checked before proposing it (per the simplicity-rule: prefer an existing
utility over inventing a new one).

### Schema change (additive; `agf/schemas/coordination-message.schema.json`)

```json
"provenance": {
  "type": "object",
  "description": "Optional. Records which versions of the rule tree, skills, and MCP server definitions were in effect when this governed action was taken.",
  "properties": {
    "rules_ref": {
      "type": "string",
      "minLength": 1,
      "description": "Git commit hash of the rules tree (e.g. agf/rules/) in effect at action time."
    },
    "skills": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Names of any skills invoked in producing this action."
    },
    "mcp_servers": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["identifier"],
        "properties": {
          "identifier": { "type": "string", "minLength": 1 },
          "tools": { "type": "array", "items": { "type": "string" } }
        }
      },
      "description": "Identifiers and declared tool surfaces of any MCP servers connected during this action."
    }
  }
}
```

All subfields optional, no `required` block — this is deliberately loose for
the first pass; tightening (e.g. requiring `rules_ref` on `decision-record`
kind messages) is a follow-on decision once adoption experience exists,
matching how `T2-5-advisory-consultation-gate.md` phased in required-role
enforcement only after the shape existed.

### New rule (not created in this session): `agf/rules/always/input-provenance-rule.md`

```markdown
# Input Provenance: Recording the Instruction Supply Chain Behind Every Governed Action
#
# Framework principle: the inputs that shape agent behavior are a governed
# supply chain in their own right, not just the outputs they produce
# See: framework.md — Principle 2 (Artifact Ownership); Section 3 (Cross-Cutting Concerns)
# Threat model: docs/plans/threat-model-stpa-owasp.md — UCA-4 (silent instruction drift)
# Alignment target: CycloneDX Agent BOM proposal (github.com/CycloneDX/specification/issues/895);
#   adjacent: OWASP AIBOM work. Evaluate for alignment once that proposal
#   stabilizes — do not claim conformance today.
# Evidence: Theoretical — no AGF-specific incident yet; ships PREVENTIVE per
#   downstream-impact-rule's classification.

> Provisional status. ...

**Scope:** Any governed decision point in the coordination channel — Gate 1
decision-record, Gate 2 decision-record, review. These SHOULD carry a
`provenance` block. Other message kinds MAY.

## The rule

A governed action's coordination message MAY carry an optional `provenance`
object (see schema above) recording the rule-tree commit hash, invoked skill
names, and connected MCP server identifiers with declared tool surfaces, in
effect when the action was taken.

## Removability constraint (MUST)

The provenance record MUST be a plain file mechanism: it lives inside the
same markdown-with-YAML-frontmatter coordination message already committed
to the repository. No external BOM registry, no service dependency, no state
that is not reconstructible from `git log` and the coordination channel
alone. This satisfies AGF's standing removability constraint (issue #39;
README "Git history is a ledger, not a scratchpad"): AGF must be removable
from a repository at any time without loss of project history or working
capability, and no AGF component may hold state that is not reconstructible
from the repository alone.

## Maturity of the CycloneDX alignment claim (implementation-honesty-rule)

- AGF's own minimal manifest, once the schema field above ships: **Level 2
  (Schema-defined)** — the field exists but no message uses it yet.
- Once a real coordination message carries a `provenance` block: **Level 3
  (Instantiated)**.
- Once `agov coord validate` has fixtures asserting the field's shape
  (positive + negative cases): **Level 4 (Validated)**.
- Once tooling automatically populates the field at every governed action:
  **Level 5 (Deployable)**.
- Alignment with CycloneDX's Agent BOM proposal specifically: **Level 1
  (Planned)** — the upstream proposal is open and unstable; this rule tracks
  it, it does not implement against it.

## Relationship to the existing SPDX SBOM decision — flagged, not silently resolved

AGF already decided to use SPDX, not CycloneDX, for tooling **binary**
distribution SBOMs (`docs/coordination/00275-planner-architect.md`: "SPDX is
an ISO standard [ISO/IEC 5962:2021]; CycloneDX is OWASP-hosted... closer to
what legal reviewers and supply-chain tools expect"). That decision concerns
a **different artifact** — a bill of materials for AGF's own distributed
binaries. This rule concerns **behavioral input provenance** — which
rule/skill/MCP-server versions shaped an agent's decision at the moment it
was made. The two do not technically conflict (different artifact, different
question), but per `always/uncertainty-and-scope-rule.md` ("if a rule
conflicts with what the code already does, point out the conflict explicitly
rather than silently resolving it") this is flagged rather than quietly
adopting CycloneDX-flavored vocabulary in one corner of a project that chose
SPDX in another. A human reconciling this cluster should decide whether
running two BOM-adjacent vocabularies in one project needs a documented
rationale beyond "different domains, different standards bodies" — or
whether the eventual provenance export (if any) should also target SPDX
instead of watching CycloneDX. This plan takes no position beyond flagging it.

## Out of scope

- Full CycloneDX Agent BOM conformance.
- Automatic tooling population of `provenance` — follow-on implementation.
- MCP authentication/authorization detail beyond declared tool-surface names.

## CUSTOMIZE

Adopters MUST customize: which message kinds require (vs. merely allow) a
provenance block; whether `rules_ref` should point at a full-repo commit hash
or a narrower rules-subtree hash if the adopter's tooling can produce one.

## Related rules

- `always/audit-log-completeness-rule.md` — sibling: that rule governs the
  OUTPUT audit trail (who changed what); this rule governs the INPUT audit
  trail (what shaped the decision).
- `always/external-anonymization-rule.md` — MCP server identifiers and skill
  names MUST still be anonymized if a provenance block ever crosses the
  external-destination boundary.
```

---

## Affected files

Nothing is created in this Gate 1 planning session. The table below is what a
later, approved implementation session would create/modify.

| File | Action |
|---|---|
| `agf/rules/always/input-provenance-rule.md` | Create — full rule text per the design above |
| `agf/schemas/coordination-message.schema.json` | Update — add the optional `provenance` object property (additive; no existing message invalidated) |
| `agf/framework.md` | Update — add a row to the §6 "Rule–Principle Mappings" table |
| `agf/docs/site/docs/concepts/coordination-protocol.md` | Update — short subsection describing the optional `provenance` block, linking to the new rule (this page already documents coordination message shape and kinds) |

---

## Migration needed

No. The schema change is additive and backward compatible: existing
coordination messages without a `provenance` field remain valid (verified:
the property is optional, no `required` entry added, no
`additionalProperties: false` at the top level to violate). No Rust code
change needed per the validator design cited above.

---

## Test strategy

- Fixture-based, per `workflow/boundary-shape-testing-rule.md` and
  `workflow/tdd-rule.md`, at implementation time:
  - Positive case: a coordination message with a well-formed `provenance`
    block validates successfully via `agov coord validate`.
  - Positive case: a coordination message with **no** `provenance` field
    (today's shape) still validates successfully (regression guard).
  - Negative case: a coordination message with a malformed `provenance`
    block (e.g. `rules_ref` given as a number instead of a string) is
    rejected by `agov coord validate` with a clear error.
- `agov validate framework-config` — confirm the updated rule-principle
  mapping table in `framework.md` still validates.

---

## Risks

| Risk | Mitigation |
|---|---|
| Dual BOM vocabularies (SPDX for binaries, CycloneDX-flavored for behavioral provenance) read as inconsistent to an external reviewer | Flagged explicitly above rather than resolved silently; human reconciler decides |
| `provenance` becomes another field nobody fills in, functionally decorative | Rule scopes it to SHOULD (not MUST) initially for decision-record/review kinds; tightening to MUST is an explicit follow-on decision once adoption data exists, not assumed now |
| CycloneDX Agent BOM proposal changes shape before AGF aligns to it | Rule text explicitly states Level 1 (Planned) alignment only; no conformance claimed |
| Schema change silently coupled to Rust struct assumptions elsewhere in the tooling | Checked in this plan: `CoordMessage.raw` is `serde_json::Value`, decoupled from schema evolution by design (cited file/line above) |

---

## Rollback

Revert the branch. The schema change, once implemented, is additive; if it
needs to be undone, remove the `provenance` property — no existing message
depends on its presence, so removal does not require a data migration.
