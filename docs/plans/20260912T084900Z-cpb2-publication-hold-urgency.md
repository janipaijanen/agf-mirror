---
id: 20260912T084900Z-cpb2-publication-hold-urgency
status: pending-gate1
topic: cpb2-publication-hold-and-urgency
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Governor-appointed semantic security consultation smmp completed at 9913892e with six blockers and one advisory (q82p). This revision addresses them; bounded re-review remains pending, not a waiver or clearance.
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
release work-order/decision, and expire after its own timestamp and no later
than 24 hours after that timestamp. This proposed operational maximum is a
child Gate 1 choice; over-limit controls are invalid, not silently clamped. An end must
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

Resolve the named target branch once to an immutable target SHA. Accept only
unambiguous branch references, not a raw SHA, tag, revision expression or symbolic
alias such as HEAD/origin/HEAD. A full refs/heads/<name> identifies a local branch;
refs/remotes/<remote>/<name> identifies a remote-tracking branch for one configured
remote. Short forms are accepted only when exactly one of those full branch refs
matches; reject ambiguous local/remote aliases and ambiguous remote prefixes.
Emit the resolved full `target_ref`, canonical `target_branch` equal to
refs/heads/<name>, selected remote (or null for local), and `target_sha` separately.
Hold controls store only this canonical logical target_branch form. Thus a hold
for refs/heads/main matches main and origin/main when both resolve unambiguously
to logical main; a same-SHA feature ref never matches. Different remotes may name
the same logical branch but retain distinct target_ref/remote provenance; this
is not proof they are the authorized publication remote. Operator verification
must check the repository's configured publication target and fresh fetch.
A stale local main is reported at its actual SHA, never replaced by origin/main.
Use the pinned SHA for all Git reads; no independent base override is accepted.

Resolve the repository root once. Read direct lowercase .md records from the
pinned target's configured channel and the current filesystem channel. One
optional pending directory is explicitly named, nonrecursive, under the root.
Reject traversal, all symlink directory components and files (even within the
root), and nonregular entries. Git channel records must be regular blobs with
mode 100644 or 100755; reject symlinks/submodules/nonregular channel entries.
Do not implicitly enumerate archives, every branch, GitLab or the network.
Label each source and immutable Git SHA. Canonical repository-relative paths
are evidence identity: same path plus identical bytes is deduplicated with all
source labels retained; same path plus different bytes is a conflict regardless
of embedded IDs. Same ID plus different bytes is also a conflict. Equal ID/raw
bytes at distinct paths is one logical declaration retaining all provenance.
This read-side deduplication never permits publishing a copied new record;
CP-B-1 publication-copy rejection remains separately governed.

For each filesystem source, capture the direct entry inventory (path, type and
stable file identity), open regular files without following links, read bytes,
and check descriptor/path identity and metadata before/after reading. Re-enumerate
all requested filesystem inventories and re-read/compare the captured bytes
before returning. Any observable add/delete/rename/replace/type/identity/content
change is churn: exit 2, coverage INCOMPLETE, with no positive delay state. No
automatic retry loop. Deterministic test hooks exercise each walk/read boundary.
COMPLETE means these bounded stability checks completed for explicit sources;
it is not a transactional filesystem snapshot or a guarantee against changes
after return. Output and runbook state this limit; an actual delay decision
requires fresh independent verification, never reliance on a saved COMPLETE row.

The current filesystem can show a newly published branch's hold which is not
merged into main. Output must distinguish base-visible from local-only evidence;
local-only is not proof of remote publication. ALL status rows are declarations,
never assertions of a published or usable hold: `publication_verified: false`
and `authority_verified: false` are fixed at top level and on every hold/record
row. Neither a role label, source visibility, matching SHA nor local file can
flip these fields. External operator verification remains necessary even for
base-visible evidence. Required external checks are explicit below.

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

1. `publish-now`: urgent, hold control, requires-ack:true, or kind handoff or
   review, regardless of omitted/normal urgency or deferred_by. These conservative
   structural overrides do not need to prove an obligation is still live.
2. `publish-now`: any requested source is incomplete or has semantic evidence
   problems that could affect the selected target. Malformed/unknown-target
   evidence is potentially relevant. Preserve every reason and apply this
   override globally, not only to the record with the problem. Valid controls
   proven to concern a different canonical branch may be isolated; never infer
   unrelatedness from malformed fields or a different SHA.
3. `unassociated`: no deferred_by; ordinary publication rules apply. This is
   not a determination that the record is safe to delay.
4. `publish-now`: association missing, invalid, future/not-yet-active, ended,
   expired, stale or conflicting.
5. `review-required`: normal association to the one active matching declaration
   in a complete, conflict-free stability-checked scan. Reports author intent
   and its unresolved verification requirements, never a positive deferral.

There is no `deferred-intent` or automatic allow-delay output. The optional
stored deferred_by remains a declaration, not permission. Before any actual
batching delay, an independent human/operator must verify that exact record's
urgency and absence of security/gate findings, escalations and live obligations,
as well as hold publication, governor authority, selected publication remote,
current target SHA and expiry. If any check is unavailable, publish immediately.
The command neither accepts a verification flag nor records an approval; it
always outputs the external-check requirements and fixed false verification
fields. A later ordinary record revision requires fresh verification. Urgent
records must remain independently visible even when main integration is held.

Machine-readable `required_external_checks` is the stable enum list
`[publication, governor_authority, record_urgency_and_obligations,
publication_target, current_target_and_expiry]`, on the result and each row.
Text and MCP expose the same list, not only a prose disclaimer. This avoids
using a review of an unpublished record as an assumed future safeguard.

### Machine interface and status

JSON version 1 object: `schema_version`, `target_ref`, `target_branch`, `target_remote`,
`target_sha`, `now`, `publication_verified`, `authority_verified`,
`required_external_checks`,
`scope`, `sources`, `coverage`, `holds`, `records`, `problems`. Arrays are stable
sorted by timestamp, ID and path; each evidence item retains its source identity.
`coverage` is COMPLETE or INCOMPLETE for explicit sources only. Text is a compact
projection of the same result, with diagnostics deduplicated and referenced.

Exit 0: complete requested-source stability checks and no semantic problems,
whether a hold declaration is active or not; review-required is not permission.
Exit 1: conflicting/invalid semantic evidence; every potentially affected row
is publish-now. Exit 1 never coexists with a positive delay state.
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
run strict projection validation for the final implementation. The companion manifest is prepared with this draft. It records planned
implementation effects, not completed source edits. Actual consultations and
allocation acknowledgement still precede the final Gate 1 submission.

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
- Same fixture through real CLI and MCP stdio: equivalent JSON after removing only the independently sampled `now` value,
  statuses and isError for complete, invalid and incomplete cases. Assert each
  now is valid UTC and lies within its invocation bounds; semantic expiry
  boundary tests use the internal fake clock. Deterministic fake-clock tests.
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


## Consolidated semantic security corrections (q82p)

The governor-appointed reviewer reported CPB2-SEC-1 through SEC-6 and advisory
A1 at 9913892e. This revision addresses all seven as one batch; it is not cleared
until the reviewer checks the new exact head. In addition to the earlier matrix:

- SEC-1: omitted/normal urgency plus handoff, review or requires-ack:true always
  yields publish-now. Security/gate/escalation examples not structurally detected
  never yield automatic deferral; normal active associations are review-required
  with per-record external urgency/obligation verification required.
- SEC-2: test local/remote/full refs, main/origin-main equivalence, feature at the
  same SHA, stale local main, multiple configured remotes with retained provenance,
  ambiguous short aliases, HEAD, tags, SHAs and ref expressions. Assert one pinned
  target object for all reads and canonical branch matching independent of SHA.
- SEC-3: test in-root/escaping symlinks, symlinked source directories, Git
  symlink/submodule/nonregular modes, and same-path changed bytes with both equal
  and unequal IDs. Reject aliases; preserve cross-source provenance on dedup.
- SEC-4: deterministic add/delete/rename/replace and type/identity/byte changes
  during source enumeration/read/recheck return exit2/INCOMPLETE/publish-now.
  Document bounded stability checks and post-return freshness limits honestly.
- SEC-5: malformed YAML/schema, unknown-target controls, same-target conflicts
  and future starts cannot coexist with a positive delay row. Assert exit/row
  precedence globally across requested sources, not only on associated records.
- SEC-6: forged governor labels and both local/base-visible controls keep
  publication_verified:false and authority_verified:false at result and row
  level. Check required_external_checks and identical semantics in JSON/text/MCP.
- A1: proposed 24-hour maximum, positive-duration check, exact maximum accepted,
  over-limit/year-9999 rejected. This policy is subject to child Gate1 approval.

No implementation is included. No automatic trusted verification store, new
service, network discovery or new CI gate is introduced by these corrections.
