---
kind: handoff
from: planner-architect
to: planner-architect
id: 20260912T085315Z-planner-architect-kkjh
timestamp: 2026-09-12T08:53:15Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260912T084321Z-planner-architect-n5g8
  - 20260912T084635Z-coder-tester-2s6m
topic: CP-B-2 draft ready for consultation alongside CP-B-1 implementation
requires-ack: true
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Claude to consult on CP-B-2 draft below and acknowledge planning allocation; retain estate and release integration.
  must_read:
    - docs/plans/20260909T133651Z-cpb-coordination-publication-efficiency.md
  do_not_touch:
    - CP-B-1 implementation branch is owned by Codex pending exact-head review.
---

The complete local CP-B-2 draft and independent preliminary review are reproduced
below for durable shared consultation. This is not a Gate 1 submission/approval;
projection manifest and actual security/governor consultation remain required.
No new plan MR is opened. Codex can complete the final submission after the
planning allocation is acknowledged; Claude need not draft this from scratch.
The draft status validator passed. The reviewer authored none of the draft and
resolved D1-D4 on recheck; this is not an InfoSec or gate attestation.

In parallel CP-B-1 !2636 correction is pushed at
b6d5dce70cd5458e654af8306f7fcad9ffa84638. Tests: 18 publication unit,
4 CLI publication, 17 gate controls, clippy, release build and strict projection
passed. Distinct independent implementation review and exact-head CI are in
progress. Existing governor q862 authorizes the correction, not its Gate 2 merge.

## CP-B-2 draft (complete)

---
id: 20260912T084900Z-cpb2-publication-hold-urgency
status: pending-gate1
topic: cpb2-publication-hold-and-urgency
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: Consultation requested; this is not an approved waiver. Review hold authority, expiry, ambiguous records and the never-delay-security boundary before Gate 1.
  - role: contribution-auditor
    status: waived
    reason: Independent consultation requested; no author clearance claimed. Review parent fidelity, advisory authority, source coverage and the release-target boundary before Gate 1.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B-2: read-only publication holds and explicit urgency

## Scope and authority

Child of approved `20260909T133651Z-cpb-coordination-publication-efficiency`.
CP-B-1's approved plan expressly leaves the hold wire shape, deferred-record
source, CLI/JSON/exit-code/MCP contracts and urgency normalization to this child.
Governor's 2026-09-12 instruction permits independent RUW progress and planning;
it does not approve this new implementation. Status stays pending-gate1 until
an actual governor decision. CP-B-1's copied-record correction is a separate
already-approved implementation in !2636. Claude retains estate/release integration.

The problem is unnecessary main advancement during a release freeze. This
surface reports hold and publication evidence; it does not publish, merge,
sleep, schedule, suppress CI, or grant release authority. Claims of token/build
savings remain empirically informed hypotheses until measured. This plan is
Level 1 Planned; successful branch implementation will be Level 4 Validated.

## Concrete contract proposed for approval

### Backward-compatible record fields; no new message kind

1. `urgency: normal|urgent`, optional, semantic default normal. Omission remains
   omission in immutable stored bytes. Any other value, type or null is invalid.
2. Optional `publication_hold` object on `kind: decision-record`, with exact
   fields `action: start|end`, `target_branch`, and either:
   - start: `target_sha` (40 lowercase hex), `expires_at` (RFC3339 UTC), or
   - end: `hold_id` (ID of a start record).
   Unknown object keys and action-incompatible fields are rejected.
3. Optional `deferred_by` string on an ordinary record explicitly associates it
   with a hold ID. It is a declaration of batching intent, not evidence that
   the record has or has not been pushed. Existing records are never edited to
   add this field. Hold controls themselves cannot be deferred.

A start must be from governor-reviewer, have urgency urgent, reference the
release work-order/decision, and expire after its own timestamp. An end must
also be a governor-reviewer urgent decision-record, reference the same start
in both hold_id and references-to, and name the same target. A hold cannot be
extended in place. A later start is a separate hold; overlapping starts are
reported as conflict, never silently reduced to the newest timestamp.

The role label is channel evidence, not cryptographic authentication. These
checks do not authenticate the governor or elevate an authored role string into
binding authority. Operators still verify the actual governor decision and
published record under existing gate rules. Explicitly limiting creation to the
governor avoids introducing an unspecified delegated release role in this slice.

A hold delays integration into main, not visibility to a waiting actor. A local
unpublished record is still a draft. Urgent records publish independently at
once; the hold control is itself urgent and must be published before relying on
it. A frozen target SHA is evidence for the release lane, not permission to
ignore target changes: any target drift requires release preflight re-evaluation.
CP-B-3 wires the release procedure later; this child never weakens that rule.

### Operator command and input coverage

`agov coord hold-status --target-branch <local-or-remote-tracking-branch>
[--dir <coord-dir>] [--pending-dir <repo-relative-dir>] [--format text|json]`

Resolve the named target branch once to an immutable target SHA; no independent
base override is accepted, so evidence cannot silently relabel a feature SHA as main.
Use that same target SHA as the base inventory.
resolve the repository root once. Read coordination records at that base's
configured top-level channel and in the current filesystem channel. One optional
pending directory is explicitly named, nonrecursive Markdown-only evidence under
the repository root; reject traversal or symlinks escaping that root. Do not
implicitly enumerate archives, every branch, GitLab or the network. Label every
source and exact base SHA. Same path+bytes across base/filesystem is deduplicated;
different bytes under one ID are a conflict, not a last-writer-wins update.
Equal ID+raw bytes across different evidence paths is one logical declaration
with all source labels retained. This read-side evidence deduplication never
permits publishing a copied new record; CP-B-1 copy rejection remains unchanged.

The current filesystem can show a newly published branch's hold which is not
merged into main. Output must distinguish base-visible from local-only evidence;
local-only is not proof of remote publication. ALL status rows are declarations,
never assertions of a published or usable hold: `publication_verified: false` is
fixed in output. External operator verification remains necessary even for
base-visible evidence; role labels do not prove governor authenticity.

Operator path: publish the start on the shared coordination branch independently
of main, bridge it to the release lane, and verify recipient visibility and
governor authority using existing rules. Run the command in that checkout with
`--target-branch origin/main` after an explicit fetch. Both start and end must be
verified before the operator changes its batching decision; an unpublished local
end cannot clear a real hold merely because this command shows ended-declaration.
Do not merge the start into main merely to make it visible. That would advance
the frozen SHA and require release preflight again.

Scope is always `explicit-sources`,
never `whole-inbox`; completeness concerns only the requested readable sources.

Use one UTC now value per invocation, injectable only through an internal clock
in tests. Active interval is start.timestamp <= now < expires_at. A valid end must have timestamp >= start.timestamp; when it is at or before
now it ends that exact declaration. Earlier ends are invalid; future ends do
not apply. Multiple valid ends of the same start are idempotent; retain every
end ID as evidence rather than silently losing records. Expired/ended holds remain
visible evidence. Missing starts, contradictory controls, invalid timestamps,
malformed records, unreadable directories/Git objects and invalid base references
are explicit problems, never an empty clean result. Unavailable evidence never
creates permission to hold or merge.

For the selected target, return all active-declaration IDs, ended/expired controls and
conflicts. A start whose target_sha differs from the resolved target is `stale`
and cannot support treating the target as frozen. Multiple live starts are
`conflict` even when their SHA matches. Report records declaring deferred_by
with their paths/IDs, hold resolution, urgency and the following ordered states
(first match wins):

1. `publish-now`: urgent or hold control, with or without deferred_by. This
   remains visible even when unrelated source evidence is incomplete.
2. `unassociated`: no deferred_by; ordinary publication rules apply.
3. `publish-now`: association missing, invalid, ended, expired, stale, conflicting
   or any requested-source coverage incomplete. Preserve every reason. No safe
   delay is inferred from a readable subset.
4. `deferred-intent`: normal association to the one active matching declaration
   in a complete, conflict-free requested-source scan. This reports the author's
   intent only; independently verified publication/authority is still required
   before any real delay. It is never a command to hide, publish or merge.

Mandatory urgency categories remain normative, not reliably inferable from text:
security/gate findings, escalations and live obligations with waiting actors MUST
publish immediately. Explicit urgent always overrides deferred_by. Absence of
urgency never proves the absence of a waiting actor. Tool output includes this
limitation and lists unassociated records; it does not automatically classify
content as safe to delay. A suspicious normal classification is a review finding.

### Machine interface and status

JSON version 1 object: `schema_version`, `target_branch`, `target_sha`, `now`, `publication_verified`,
`scope`, `sources`, `coverage`, `holds`, `records`, `problems`. Arrays are stable
sorted by timestamp, ID and path; each evidence item retains its source identity.
`coverage` is COMPLETE or INCOMPLETE for explicit sources only. Text is a compact
projection of the same result, with diagnostics deduplicated and referenced.

Exit 0: complete requested-source scan, whether a hold is active or not.
Exit 1: conflicting/invalid semantic evidence; no usable hold asserted.
Exit 2: unavailable/incomplete requested-source evidence (takes precedence over 1).
None of these statuses is a push, merge or release gate. The existing named
coordination schema validator remains the only field-validation gate; this
command does not add CI blocking authority. MCP `agov_coord_hold_status` mirrors
arguments/output/status/isError exactly, local-only with no implicit forge mode.

## Implementation paths and projections

Implementation will touch the coordination schema; agov-coord parsing and new
hold-status module; agov CLI dispatch; agov-mcp schema/dispatch; relevant telemetry
allowlists; integration and stdio tests. Locate exact owning modules from the
frozen implementation base before mutation; no unrelated refactor or new crate.

Normative owner: `agf/rules/workflow/coordination-publication-rule.md` and its
Claude projection. Explain hold-vs-visibility, mandatory urgency, declaration
fields, expiry, conflicting controls and lack of new tool authority there.
Adopter concept: coordination-protocol page. Operator: poll-and-coordinate and
CLI/MCP command reference. Loaders: AGENTS.md and agf/templates/AGENTS.md concise
pointer only. Distribution: schema, rule, docs and binaries use existing packaging.
Create the standard six-surface projection manifest before Gate 1 submission and
run strict projection validation for the final implementation. This local draft
is not yet a complete Gate 1 submission without that manifest and consultations.

## Required acceptance evidence

- Old valid record without any new fields remains valid and normal; malformed
  urgency/hold/deferred types and forbidden control combinations rejected.
- Start/end, future start/end, exact expiry boundary, different targets, stale SHA,
  overlapping holds, unknown/mismatched end and duplicate conflicting ID fixtures.
- Urgent+deferred association always publish-now, controls cannot be held; explicit
  normal classification never advertised as proof that delaying is safe.
- Real Git base plus current/pending directory integration with source labels,
  identical-source dedup, changed bytes, unreadable/missing source, malformed
  record and symlink/traversal rejection. No implicit network call.
- Same fixture through real CLI and MCP stdio: byte-equivalent JSON, statuses and
  isError for complete, invalid and incomplete cases. Deterministic fake-clock tests.
- Preserve publication-check MR !692 mixed range, implementation/revert range,
  modification/deletion/rename/copy rejection and batch-smuggling negatives;
  two-or-more valid new records still pass. Include CP-B-1 exact-byte copy tests.
- Gate companion and strict projection checks; affected crate tests, workspace
  clippy, canonical exact-head CI and independent implementation/InfoSec reviews.
  No unchanged paid retry; no duplicate runnable initial push/MR pipelines.

## Sequencing, risk and rollback

Draft and consult independently now. Before implementation, reconcile exact paths
with CP-B-1 and any #122/!2027 work, use merged CP-B-1 as the base and retain the
S2/S3 additions already on main. Bounded InfoSec and contribution review precede
Gate 1; build-engineer reviews command/test/CI footprint. No purchase or release
permission is requested. Gate 2 remains separate after implementation evidence.

The major risk is a hold mistaken for permission to suppress evidence. Controls
are deliberately explicit and advisory; urgent obligations stay independently
visible and errors cannot enable a safe effective hold. Another risk is adding
ceremony: one start/end per release hold, no per-record MR requirement, no
persistent service, no background polling and no coordination archive introduced.

Before merge rollback is abandoning the branch. After merge do not rewrite old
records; a correction/revert is forward-only through gates. Expiry ensures a
missed end does not produce an indefinite hold.

## Draft review corrections and migration

Independent draft review `cpb2-draft-review.md` identified D1-D4. Revised contract
never equates declaration state with verified publication; gives classification
precedence and conservative partial-source behavior; removes the independent
base override; defines chronology and equivalent evidence deduplication. Added
one-directory/nonrecursive limit rather than a new discovery subsystem.

Acceptance additionally requires unpublished start and unpublished end fixtures
with publication_verified false, urgent unassociated precedence, partial-source
row states, earlier-than-start end rejection, duplicate end idempotence and
same-ID/same-byte cross-source provenance. Distinct actual target branches must
produce independently labeled SHAs; unsupported base override is rejected.

Downstream impact: BREAKING for the adopted hold workflow as classified by the
parent; ADDITIVE for optional schema fields/CLI/MCP. Existing records require no
rewrite and no hold is automatically enabled. Adoption instructions distinguish
intent/status from actual independently visible publication. The governor-only
control policy is proposed here for explicit child approval, not claimed as a
previously settled parent choice.

## Independent draft review (complete)

# CP-B-2 draft review

Read-only draft review, 2026-09-12 UTC. This is neither binding Gate 1 clearance nor independent InfoSec clearance. Reviewed `/home/dev/s/tmp/ci-five-day-audit/20260912T084900Z-cpb2-publication-hold-urgency.md` against the approved CP-B parent and CP-B-1 exclusion on origin/main `4fe0f389a36d9234c218d95f4182cdfcdc239a6d`.

**Recommendation: revise before Gate 1 submission.** The scope belongs in CP-B-2; optional urgency, append-only control records, explicit source coverage and an advisory command fit the parent. The main defects are in source trust/usable hold semantics and exact result precedence, not the choice to implement a hold.

## D1 — Local-only control evidence can apparently activate or terminate a usable hold

Lines 64–67 require publication before relying on a hold; lines 85–88 admit filesystem evidence is not proof of publication. But lines 90–106 derive active/end state and `deferred` from valid records without excluding local-only controls. A local unpushed start can thus appear to authorize delay, and a local draft end can clear a published hold. Merely printing a source label does not reconcile these contracts.

There is also an operator-path problem: merging a start into the target advances the target SHA, making the start's recorded pre-merge `target_sha` stale. The useful start is normally on a separately published coordination branch, yet the default input surfaces cannot establish that publication.

Minimal fix: expressly call local evidence a *candidate/declaration*, require independent publication verification before reporting a hold as usable, and make a clear operator recipe for the shared coordination branch. Either keep this verification outside the tool and never call its local result a safe/effective hold, or accept an explicitly named pinned evidence ref (read-only, no fetch/network) with carefully limited provenance claims. Do not infer publication from a tracked file, branch name or author role string. Include unpublished start and unpublished end tests.

## D2 — Disjoint record-state predicates overlap and error precedence is incomplete

An urgent record with no `deferred_by` matches both `publish-now` and `unassociated`; a hold control also normally lacks `deferred_by`. The plan calls these states disjoint without an evaluation order. In addition, the exit contract says semantic problems produce 'no safe effective hold', but the per-record contract can still emit `deferred` whenever a valid single hold is found in the readable subset, even when another requested source cannot be read.

Specify ordered classification: (1) urgent/control routes immediately, (2) otherwise no association is unassociated, (3) otherwise resolve the association and classify it. Define whether any incomplete/conflicting relevant control evidence suppresses all usable hold assertions for that target; if yes, route affected candidates conservatively and retain diagnostic uncertainty. Do not erase `publish-now` for urgent evidence because unrelated files failed. Tests should assert result rows as well as exit status for partial-source cases.

## D3 — Target identity and evidence-base identity are conflated

The command takes both `--target-branch` and `--base-ref`, then resolves 'it' once and emits `target_sha`. With `--target-branch main --base-ref some-feature`, it is unclear whether the feature SHA is falsely labeled main's current SHA. That can make a stale hold seem current or a current hold seem stale.

Define `--base-ref` as an explicit mapping for the selected target (and validate/document the mapping), or resolve the actual target separately and label base/evidence SHA independently. The simplest surface may omit the override if it has no distinct operator requirement. Add a mismatched target/base fixture.

## D4 — End chronology and duplicate evidence need precise rules

The only chronology stated for an end is end.timestamp <= now. An end timestamp before its referenced start can therefore cancel that later start. Explicitly require end.timestamp >= start.timestamp; an earlier end is invalid, not a valid terminal event. State whether duplicate matching end declarations are idempotent or contradictory.

Dedup handles same path+bytes and conflicts for different bytes under one ID, but not the same ID+bytes at different paths across explicitly supplied sources. Decide whether those are equivalent provenance copies (retain all source labels but one logical control) or invalid duplicates; do not accidentally count them as two overlapping starts. Distinguish copies already admitted as explicit evidence from permission to publish copied records: CP-B-1 still rejects byte-identical new record publication.

## Scope and simplicity observations

- The approved CP-B-1 plan explicitly delegates hold wire shape, deferred sources, CLI/JSON/exit/MCP contracts and urgency normalization to CP-B-2. These are legitimate child-design decisions. CP-B-3 owns release procedure integration, so this child should provide an operator example without rewriting release-cut authority.
- `deferred_by` is defensible as the minimal explicit deferred-record source; it avoids guessing from all normal messages. Retain the declaration-only caveat and state old records remain governed by ordinary visibility rules.
- Governor-only starts/ends narrow the parent's unspecified 'release lane' and require explicit approval in this child. They do not require an invented additional role. Do not describe the narrowing as already decided by the parent.
- Recursive multiple pending directories plus root/symlink policy may be unnecessary complexity for the first version. One explicit staging directory may meet the need; retain multiple directories only with an actual operator case. Reuse existing channel parsing/path behavior where possible rather than inventing a second discovery subsystem.
- The detailed JSON field names are listed, but nested enum/field shapes are not pinned. Provide a small example of success, conflict and incomplete results before implementation so CLI/MCP parity is testable beyond a shared implementation.
- Add a downstream-impact classification and migration note. The parent explicitly calls hold workflow BREAKING and optional tooling/schema ADDITIVE; if this child chooses a different classification, justify the narrower final contract rather than silently omitting it.
- The draft honestly says its projection manifest and consultations are incomplete. Keep this as a draft until they exist. `waived` placeholders are not completed independent consultations or governor waivers, as the draft itself correctly discloses.

No implementation changes, publication, tests, gate decisions or role activation were performed by this review.

## Corrected draft recheck — final draft verdict

**CLEAR for onward Claude consultation as a draft. D1–D4 resolved; no remaining concrete blocker from this bounded recheck.** This verdict supersedes the initial “revise before submission” recommendation only for draft readiness. It is not Gate 1 readiness, implementation clearance or an InfoSec consultation. The explicitly pending projection manifest and specialist consultations are not demanded at this draft stage.

Reviewed corrected draft SHA-256: `69042447b8b1d9a458acd91914f6311917deffa2d7de191a9dd0f6bca3d63718`.

- **D1 resolved:** all statuses are declarations; publication_verified is always false. The operator must separately verify both start and end authority/publication. The shared-branch recipe avoids advancing the frozen target just to publish the start. Local ended-declaration cannot itself clear the operational hold.
- **D2 resolved:** first-match routing prioritizes urgent/control records, then unassociated records, then unusable associations, finally deferred-intent. Incomplete sources prevent intent from being represented as support for safe delay; urgent routing survives unrelated failures.
- **D3 resolved:** base override removed; the named local/remote-tracking target resolves to the single labeled target/base SHA. Explicit fetch remains an operator prerequisite, honestly outside this local-only command.
- **D4 resolved:** earlier-than-start ends are invalid; multiple valid ends are idempotent with evidence retained. Equal ID/raw bytes across paths become one logical declaration with every source label, without weakening CP-B-1 publication copy rejection.

The one-directory, nonrecursive input limit and explicit impact/authority qualifications also address the earlier complexity and parent-fidelity advice.

Minor implementation-specification note, not a draft blocker: real CLI and MCP invocations acquire separate wall-clock now values, so literal byte-equivalent JSON may differ in `now` even on the same fixture. Define parity under a shared controlled test clock or compare the time field separately; do not add an operator-visible clock override merely to satisfy the test. Pin nested JSON examples before implementation as previously suggested.

Read-only recheck; no draft edits, publication, role activation or binding decision performed.
