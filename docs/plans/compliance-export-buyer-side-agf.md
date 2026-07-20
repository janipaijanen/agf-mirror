---
id: compliance-export-buyer-side-agf
status: pending-gate1
topic: compliance-export-generator-and-buyer-side-verification
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      plan reads/exports existing coordination + learning artefacts and
      defines a new evidence-pack export format; no new secrets, auth,
      or tenant-isolation surface, but export format design and the
      external-anonymization boundary (docs/plans/ vs external evidence
      packs) are infosec-adjacent enough to warrant explicit review
      before Gate 2.
  - role: contribution-auditor
    status: consulted
    reason: >-
      new artefact-consuming tooling and a new cross-repo verification
      command surface; contribution-auditor should confirm the claims
      made about "what an evidence pack proves" do not overclaim.
---

# Plan: Compliance-export generator (#40) and buyer-side AGF verification (#41)

**Date:** 2026-07-20T07:50:58Z
**Status:** PENDING GATE 1 APPROVAL
**Cluster:** compliance-export / buyer-side AGF / CRA entity-structure decision
(this file covers #40 and #41; the CRA decision is a separate, much
lighter note — see `docs/plans/cra-entity-structure-decision.md`)
**Author's role in the split:** one of seven parallel Gate-1 planning
agents working separate clusters from the same triage pass over
`docs/sources.md` and an internal roadmap draft. A human reconciles
all clusters after Gate 1. Where this plan touches another cluster's
territory, the assumption is stated explicitly rather than resolved
unilaterally.

---

## Why these two issues are one plan

Issue #40 (compliance-export generator) and issue #41 (buyer-side AGF
verification) are two ends of the same pipe:

```
AGF repo (docs/coordination/, learnings/, docs/plans/, agents/)
        │
        ▼  #40: compliance-export generator
   Evidence pack (plain files: Markdown/PDF + a machine-readable index)
        │
        ▼  #41: buyer-side verification command
   Pass/fail + human-readable checklist result
```

If these ship as independently-designed formats, the second one will
either (a) re-derive its own notion of "what a well-formed evidence
pack looks like" and drift from what #40 actually emits, or (b) become
a rubber stamp that only checks the pack "looks like a folder of
Markdown files" without checking anything AGF-specific. Planning them
together fixes the schema once.

This plan is intentionally scoped to **first-pass MCC-AI only**, per
the issue's own scope note. AI Act Art. 26, ISO/IEC 42001, and NIST SP
800-218A are named as follow-on adapters against the same internal
representation — not built in this pass.

---

## Regulatory grounding (with recheck caveat)

Per `always/verifiable-claims-rule.md` and the implementation-honesty
rule, every date below is cited from `docs/sources.md` (this repo's
2026-07 source survey) and is marked for recheck — regulatory
implementation dates move, and AGF must not launder a snapshot into a
standing claim.

| Source | Claim | Recheck before citing |
|---|---|---|
| EU MCC-AI (upd. 2025-03-05), public-buyers-community.ec.europa.eu | Public buyers get contractual rights to technical documentation, logs, record-keeping, audits from AI vendors. Clause structure sourced via secondary source in `docs/sources.md` — not read from the primary clause text directly. | Yes — read primary clause text before the export schema claims clause-level conformance. |
| AI Act Art. 26, artificialintelligenceact.eu/article/26/ | Deployer duties: named-human oversight, monitoring with duty to suspend, auto-generated logs retained ≥6 months (Art. 26(6)). High-risk applicability 2026-08-02; a provisional "Omnibus VII" deferral (to 2027-12/2028-08) is agreed but not yet law per the source note. | Yes — `docs/sources.md` itself flags "[dates: verify before citing]". |
| ISO/IEC 42001 | AI management systems standard; AGF produces operational evidence a 42001 audit consumes. | Standard text is paywalled; cite only the publicly documented clause structure (management-system audit evidence categories), not internal clause numbers, until a primary copy is reviewed. |
| NIST SP 800-218A (SSDF GenAI profile), csrc.nist.gov | Explicitly addresses producer / integrator / acquirer roles — acquirer framing is the direct citation for #41's buyer-side framing. | Final version at time of `docs/sources.md` writing; re-verify no superseding revision before citing a section number. |

**What this plan does NOT claim:** that producing an MCC-AI-shaped
evidence pack, or passing #41's verification command, means a vendor
"is MCC-AI compliant" or "satisfies AI Act Art. 26." Compliance
determinations are a legal/audit judgment made by a human governor,
auditor, or contracting authority who consumes the evidence pack — the
generator's job stops at producing accurate, traceable evidence
artifacts. This distinction must appear verbatim (or by direct link)
in every doc page and CLI help text the generator ships with; see
"Maturity classification" below and `agf/docs/adopt/compliance-profile.md`'s
own layer-4 boundary ("the framework does not enforce runtime
behaviour... framework's contract is to produce verifiable runtime
artefacts, not to enforce runtime behaviour"), which this plan
inherits directly for the "artifact vs. determination" split.

---

## Integration with the existing three-tier compliance vocabulary

`agf/docs/adopt/compliance-profile.md` already defines Artefact-only /
Process-aware / Full tiers mapped to the five-layer verifiability
model (`agf/docs/contracts/integration-contract.md`). This plan does
**not** invent a parallel vocabulary. Instead:

- **The evidence pack's completeness claim is phrased in terms of the
  adopter's declared tier**, read from `framework.yaml`'s
  `compliance_target` field (or absent-file default, per the existing
  validator: `agov validate framework-config` "passes when no
  `framework.yaml` exists").
- **Each evidence-pack section states which layer it draws from**:
  - Layer 1 (artefact/schema validation) → sourced from `agov validate`
    output (pass/fail per artefact class).
  - Layer 2a (gate execution) → sourced from CI pipeline status
    (where available; Process-aware+ only).
  - Layer 2b (decision recording) → sourced from gate-approval
    artefacts in `docs/coordination/` (review comments, Gate 2
    sign-offs, escalation closures).
  - Layer 3 (binding) → **not populated**. Binding is planned/deferred
    per `agf/docs/contracts/binding-contract.md`; the export MUST say
    so explicitly rather than silently omitting the section. A
    Full-tier adopter reads "Layer 3: not available in current AGF v2
    tooling" rather than inferring absence from a missing section.
  - Layer 4 (runtime behaviour) → **never populated, by design**. The
    pack states this as the framework's declared hard boundary, quoting
    (or linking) `compliance-profile.md`'s existing sentence rather
    than paraphrasing it, per the marketing-tone-rule's message-
    integrity requirement (state a claim once, canonically; other
    surfaces link to it).
- **An Artefact-only adopter's evidence pack is honest about its own
  ceiling**: the export explicitly states "this adopter has not
  declared Process-aware or Full; layers 2a/2b/3 are not claimed" —
  the generator does not upgrade an adopter's claim past what they
  declared just because more data happens to exist in the repo.

This keeps #40's evidence pack from becoming a fourth vocabulary that
readers have to reconcile against the three-tier model documented
elsewhere — a direct instance of the marketing-tone-rule's message-
integrity clause ("every factual or value claim is stated once, in its
canonical form... other documents reference or link to it").

---

## Maturity classification (per implementation-honesty-rule)

| Capability | Level | Artifact that justifies it |
|---|---|---|
| AGF→MCC-AI artefact-type mapping (which AGF file maps to which MCC-AI evidence category) | 2 — Schema-defined (this plan defines the mapping; no code yet) | This plan document itself, pending Gate 1/2 |
| Evidence-pack export command (`agov compliance export --target mcc-ai`) | 1 — Planned | Named in this plan; not yet in `agov --help` |
| Evidence-pack schema (shared with #41) | 2 — Schema-defined, once the schema doc lands in Gate 2 | The schema doc committed alongside the implementation MR |
| ISO 42001 / Art. 26 / SSDF adapters | 1 — Planned | Named as follow-on scope in this plan; no schema work started |
| Buyer-side verification command (`agov compliance verify`) | 1 — Planned | Named in this plan |
| Buyer-facing checklist document | 1 — Planned | Named in this plan |

No claim in this plan or its eventual doc pages may assert a level
higher than what's in this table without updating the table in the
same commit (per the rule's "update maturity levels when the
implementation changes").

---

## Scope

### #40 — Compliance-export generator (first pass: MCC-AI only)

1. **Artefact-to-evidence-category mapping** (design doc, not code):
   for MCC-AI's named evidence categories (technical documentation,
   logs, record-keeping, audit support — exact clause-level names to
   be confirmed against primary MCC-AI text per the recheck caveat
   above), map to the specific AGF artefact class that satisfies it:
   - "Technical documentation" → plan files (`docs/plans/*.md`),
     agent definitions (`agents/*.yaml`)
   - "Logs" → `docs/coordination/*.md` append-only trail
   - "Record-keeping" → learning records (`learnings/*.yaml`), Gate 2
     approval entries
   - "Audit support" → the combination of the above plus the
     `agov validate` pass/fail report for the export's own repo
     snapshot
2. **Export command**: `agov compliance export --target mcc-ai
   --since <ref> --out <dir>` — reads `docs/coordination/`,
   `learnings/`, `docs/plans/` history for a chosen ref range, emits a
   static Markdown pack (one file per evidence category, cross-linked)
   plus a machine-readable index (`index.json` — see shared schema
   below).
3. **Explicit non-claim banner**: every exported Markdown file
   carries a fixed disclaimer paragraph (defined once in the schema
   doc, reused verbatim) stating the artifact-vs-determination split
   above.

### #41 — Buyer-side AGF verification tooling

1. **Verification command**: `agov compliance verify <path-to-pack-
   or-repo>` — read-only, works against either a live repo (running
   the same coordination-chain checks live) or an exported pack's
   `index.json` alone (no live repo required, per the removability
   constraint below). Confirms:
   - Coordination chain is well-formed (append-only, `seq` ordering,
     no gaps) — reuses the existing `agov coord validate` logic rather
     than re-implementing it.
   - Referenced Gate 2 approvals exist and are not dangling references.
   - No gate in the chain was skipped for merged work (i.e. no merged
     branch entry lacking a preceding review/approval entry it claims).
2. **Buyer-facing checklist document**
   (`agf/docs/site/docs/adopt/buyer-verification-checklist.md` or
   equivalent site path — final path decided against the existing
   site nav structure at Gate 2, not invented here): "what does 'an
   AGF trail exists' mean," phrased for a reader who has never run
   AGF and is not expected to install it. Written to
   documentation-audience-rule's "system administrators / business
   users who deploy and verify" audience, not the developer audience.
3. Exit codes reuse the existing taxonomy in
   `agf/docs/contracts/integration-contract.md`'s "Exit codes" table
   (0 pass / 1 validation failure / 3 invariant violation) rather than
   inventing new ones.

### Explicitly out of scope for this plan

- ISO 42001 / Art. 26 / SSDF adapters (named as follow-on issues once
  MCC-AI ships and is validated against real repo data).
- Legal review of whether an export actually satisfies any named
  framework's audit requirements.
- Any hosted/live verification service — the removability constraint
  (below) forbids it by construction.
- Cryptographic integrity of the pack (signing, hash-chaining) — this
  is explicitly out of scope per `integration-contract.md`'s own "Out
  of scope" list ("cryptographic integrity guarantees... belong to the
  infrastructure layer the framework explicitly delegates").

---

## Shared evidence-pack schema (the #40/#41 contract)

This is the one artifact both issues must agree on. Sketch (finalized
at Gate 2, not frozen here):

```json
{
  "agf_evidence_pack_version": "1.0.0",
  "generated_at": "2026-07-20T07:50:58Z",
  "source_repo_ref": "<git ref or range the pack was generated from>",
  "target_framework": "mcc-ai",
  "adopter_declared_tier": "process-aware",
  "layers_claimed": ["1", "2a", "2b"],
  "layers_not_available": ["3"],
  "layers_out_of_scope": ["4"],
  "sections": [
    {
      "evidence_category": "technical-documentation",
      "source_artefact_class": "docs/plans",
      "file": "technical-documentation.md",
      "item_count": 42
    }
  ],
  "non_claim_disclaimer": "This pack documents AGF process artefacts. It does not itself determine compliance with any named framework; that determination is made by a human reviewer or auditor consuming this pack."
}
```

Design constraints on this schema (binding on both #40 and #41):

- **Plain JSON + plain Markdown only.** No database, no hosted index,
  no proprietary pack format — direct instance of the removability
  constraint (issue #39): the pack is reconstructible from the repo
  and readable by a human with a text editor, with no AGF tooling
  installed.
- **`index.json` is the only file #41 needs.** The verification
  command must not require re-reading every Markdown file's prose to
  do its structural checks — the index carries every fact the verifier
  checks (item counts, source refs, layer claims). Markdown remains
  the human-readable layer; JSON is the machine-readable layer. This
  mirors the existing "schema-of-record hierarchy" pattern in
  `integration-contract.md` (validators/schema as machine contract,
  Markdown as consumer-facing prose).
- **Versioned.** `agf_evidence_pack_version` follows the same
  semver-no-carry-rule as the rest of the framework; #41's verifier
  must refuse (not guess) if it sees a major version it does not
  recognize.

---

## Assumptions about other clusters' ownership

Stated explicitly per the parallel-planning brief, not resolved here:

- **`agov compliance` as the CLI subcommand namespace** assumes no
  other cluster is already claiming a `compliance` or `export`
  subcommand under the Rust `agov` binary. If another cluster's plan
  (e.g. the coord-cli-ci-distribution cluster) has already reserved
  CLI namespace conventions, this plan's command names are subject to
  Gate 1 reconciliation, not a hard commitment.
- **Reuse of `agov coord validate` internals** for #41's chain-
  well-formedness check assumes that validator's current shape is
  stable; if a concurrent cluster is mid-refactor of the coordination
  validator, #41's implementation MR will need to rebase onto whatever
  lands first — this is a cross-branch dependency to surface at
  MR-open time per `branch-and-worktree-rule.md`, not something this
  plan can resolve in advance.
- **CRA entity-structure decision (#44)** is planned separately in
  this same branch (`docs/plans/cra-entity-structure-decision.md`)
  and is intentionally not merged into this plan — the two are related
  only by "same regulatory source survey," not by shared schema or
  shared code.
- **Binding contract (Layer 3) status** is treated as authoritative
  from `agf/docs/contracts/binding-contract.md` as of this writing
  (planned/deferred). If another concurrent cluster ships binding
  validation before this plan's implementation phase, the evidence
  pack's "layers_not_available" list must be revisited — flagged here
  so Gate 2 review catches it if the assumption goes stale.

---

## Affected files (implementation phase — not touched in this planning session)

| File | Action |
|---|---|
| `docs/plans/compliance-export-buyer-side-agf.md` | This file (planning only) |
| `agf/docs/adopt/evidence-pack-schema.md` (or similar; final path at Gate 2) | Create — canonical home for the shared JSON schema above |
| `tooling/rust/crates/agov-validators/src/compliance_export.rs` | Create — #40 generator |
| `tooling/rust/crates/agov-validators/src/compliance_verify.rs` | Create — #41 verifier |
| `tooling/rust/crates/agov/src/main.rs` | Modify — wire `agov compliance export` / `agov compliance verify` subcommands |
| `agf/docs/site/docs/adopt/buyer-verification-checklist.md` (path TBD at Gate 2) | Create — buyer-facing checklist |
| `agf/docs/adopt/compliance-profile.md` | Modify (small) — cross-link to the new evidence-pack schema page once it exists |
| Test fixtures (real exported evidence packs, both well-formed and deliberately malformed for #41's negative tests) | Create |

**Migration needed:** No. No database, no schema migration — plain
files only, consistent with the removability constraint.

---

## Test strategy (for the implementation phase)

| Tier | What | Notes |
|---|---|---|
| Unit | `compliance_export`: given a fixture repo snapshot (coordination + learnings + plans), emits the expected `index.json` shape | No DB; pure file-tree → JSON transform |
| Unit | `compliance_verify`: given a fixture `index.json` (well-formed and several deliberately-broken variants — missing approval reference, out-of-order seq, unknown major version), returns correct pass/fail | Per `boundary-shape-testing-rule`: fixtures must use the *real* shape #40 emits, not an invented simplification — the two implementations should literally share a test fixture directory |
| Integration | `agov compliance verify` against a real exported pack from a real (test) repo, round-tripped through both commands | Confirms #40's output is actually what #41 accepts — the core risk this joint plan exists to prevent |
| Visible-artifact | Per `visible-artifact-verification-rule`: since the pack's Markdown files are meant for human/auditor reading, a Gate 2 reviewer must open at least one generated Markdown file and confirm it reads sensibly (not just that the JSON validates) | One-time per Gate 2 review of the implementation MR |

---

## Risks

| Risk | Mitigation |
|---|---|
| Evidence pack read as a compliance guarantee by a buyer or vendor | Fixed non-claim disclaimer embedded in every exported file (see schema); doc pages state the artifact-vs-determination split verbatim |
| MCC-AI clause structure cited incorrectly (sourced via secondary source per `docs/sources.md`) | Recheck against primary clause text before Gate 2 approval of the mapping table; flagged explicitly in this plan |
| #40 and #41 schemas drift after initial implementation | Shared fixture directory (test strategy above) makes drift a failing test, not a silent divergence |
| Evidence pack overstates an adopter's tier (e.g. claims Process-aware layers for an Artefact-only adopter) | Generator reads `framework.yaml`'s declared `compliance_target` and never claims more than declared, per "Integration with the existing three-tier vocabulary" above |
| CLI namespace collision with a concurrent cluster's plan | Flagged explicitly under "Assumptions about other clusters' ownership"; resolved at Gate 1 reconciliation |
| Regulatory dates cited go stale before implementation ships | Every date in this plan is marked with its source and a recheck instruction; the eventual doc pages must carry the same caveat, not a laundered firm date |

---

## Rollback

Planning-only artifact; no code shipped yet. If Gate 1 rejects this
plan's shape, no rollback beyond discarding this branch is needed.
