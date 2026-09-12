---
id: 20260912T084900Z-cpb2-publication-hold-urgency
status: approved
gate1_approved_by: 20260912T132544Z-governor-reviewer-jhaq
gate1_approved_at: 2026-09-12T13:25:44Z
topic: cpb2-publication-hold-and-urgency
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: Governor-appointed semantic security review and the narrowed independent plan-review trial are COMPLETE/CLEAR at bf88c256 in sqrn (20260912T110246Z-independent-implementation-reviewer-sqrn). The current change records governor approval only; reviewed implementation scope is unchanged.
  - role: contribution-auditor
    status: consulted
    reason: Independent contribution review rr8g and bounded recheck bnkg at246f57e5 cleared source simplification, external release authorization and staged rollout; citation advisory resolved. This revision only clarifies end provenance and stable problem identity; those three consulted areas remain unchanged. Consultation is not governor approval.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B-2: read-only publication holds and explicit urgency

## Scope and authority

Child of approved `20260909T133651Z-cpb-coordination-publication-efficiency`.
CP-B-1's approved plan expressly leaves the hold wire shape, deferred-record
source, CLI/JSON/exit-code/MCP contracts and urgency normalization to this child.
Governor decision `20260912T132544Z-governor-reviewer-jhaq` approves this child
implementation plan at reviewed head `bf88c256`. Implementation begins only
after CP-B-1 merges; that hard predecessor remains in force. CP-B-1's copied-record correction is a separate
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

A start must be from governor-reviewer, have urgency urgent, declare at least one release work-order/decision ID in references-to, and expire after its own timestamp and no later
than 24 hours after that timestamp. This proposed operational maximum is a
child Gate 1 choice; over-limit controls are invalid, not silently clamped. An end must
also be a governor-reviewer urgent decision-record, reference the same start
in both hold_id and references-to, and name the same target. A hold cannot be
extended in place. A later start is a separate hold; effective interval overlap
is evaluated by the precise current-state rules below, never by choosing the
newest timestamp. The command validates a nonempty references-to list for starts
but does not resolve or authenticate release authorization. Emit these declared
reference IDs and require the external release_hold_authorization check: the
operator must resolve the actual approved release work order/decision, verify
its identity, kind, scope, publication and governor authority. An absent list is
invalid semantic evidence; an unresolved, wrong-kind, conflicting or local-only
reference never becomes verified through this command. An externally verified
reference still produces fixed false verification fields in this read-only result.

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
[--dir <repo-relative-coord-path>] [--candidate-ref <commit-ref>]...
[--format text|json]`

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

Resolve the repository root once. `--dir` defaults to docs/coordination and
selects one normalized repository-relative Git tree path in every source.
Reject absolute paths, empty/dot/dot-dot components, traversal and symlink tree
components. It never names a filesystem source. Candidate inputs are explicit
commit refs, default HEAD when none is given; resolve each exactly once to a
commit SHA before inventory reads. Accept commit-resolving branches, tags or
object IDs for candidates (unlike the branch-only publication target), reject
ambiguous names and malformed/non-commit refs, and retain the requested ref plus
its immutable SHA. Subsequent reads use only pinned objects even if any input
ref moves. Deduplicate identical candidate SHAs while retaining all requested
ref labels. No implicit branch enumeration, network, archive or working-tree read.

Read direct lowercase .md regular blobs (100644 or 100755) from the selected
channel at the target and every candidate SHA. Reject Git symlinks, submodules
and other nonregular channel entries. A missing/unreadable requested tree/blob
or failed decisive Git read makes coverage INCOMPLETE; an existing empty tree
is complete. Sources are snapshots pinned per ref, not one atomic cross-ref
transaction. Report this observation limit and require fresh target/publication
verification before acting; COMPLETE does not prove freshness or publication.
No new runtime crate, no platform-specific filesystem protocol, and no retry loop.

Evidence identity is normalized repository-relative path plus exact raw bytes:
same path/bytes deduplicates with all source labels; same path/different bytes
conflicts regardless of IDs. Same ID/different bytes conflicts across paths.
Same ID/bytes at different paths is one logical declaration with all provenance.
This read-side deduplication never permits publication of copied new records;
CP-B-1's copy rejection remains independently governed.

Separate the complete evidence inventory from publication action candidates.
Target records resolve hold controls, references and conflicts but never alone
create action rows. An action candidate is a logical record present at an explicit
candidate SHA with no byte-identical logical record in target inventory, regardless
of path. Candidate/target-identical records remain evidence only. Candidate records
with conflicting bytes remain action rows and are publish-now; a malformed
candidate with no ID has a path-addressed action row. Include candidate controls
and ordinary records both with and without deferred_by. Retain every provenance
entry when multiple candidate refs contain the same logical record. This excludes
already integrated historical review/handoff/urgent records from new work routing.

Every hold and action row has fixed publication_verified:false and
authority_verified:false. Target-visible versus candidate-only is source evidence,
never authenticated publication or authority. A local commit can contain an
unpublished draft; the command cannot distinguish it from a pushed shared commit.
Neither a role label, source visibility, matching SHA nor an external operator
claim can flip these fields. All rows retain required external checks.

The first version deliberately excludes arbitrary uncommitted filesystem drafts
and --pending-dir. A draft may be committed locally on an appropriate branch and
supplied by candidate ref without pushing it. A published but unmerged shared
coordination branch is fully supported by explicit candidate ref after fetching;
it does not require a pending directory. This is the simpler choice from trial
8435/build consultation pkgg, not a promise or approval of a filesystem follow-up.

Operator path: publish the start on the shared coordination branch independently
of main, bridge it to the release lane, and verify recipient visibility and
governor authority using existing rules. Run the command in that checkout with
`--target-branch origin/main --candidate-ref origin/codex/ruw-followthrough-coordination`
after an explicit fetch (substitute the actual shared branch). Both start and end must be
verified before the operator changes its batching decision; an unpublished local
end cannot clear a real hold merely because this command shows ended-declaration.
Do not merge the start into main merely to make it visible. That would advance
the frozen SHA and require release preflight again.

Scope is always `explicit-sources`,
never `whole-inbox`; completeness concerns only the requested readable sources.

Use one UTC now value per invocation, injectable only through an internal clock
in tests. A valid end names its exact start and has timestamp >= start.timestamp;
earlier ends, missing starts and target mismatches are semantic errors. The
effective half-open interval of a start is [start.timestamp, min(expires_at,
earliest valid end timestamp)); retain all valid end IDs. Ends after expiry do
not extend the interval. At now before start the declaration is future; at or
after the effective end it is ended if an end is <= expiry, otherwise expired;
otherwise it is active. An end exactly at expiry gives ended. Multiple valid ends
are idempotent and the earliest one controls, including equal-time ends.

Conflict is current-state only: two or more intervals for the selected canonical
target containing now are conflicting, regardless of SHA. Past overlap is retained
as a nonblocking historical-overlap diagnostic and does not poison a later unique
active declaration. Future overlap is advisory until it becomes current. Other
malformed or contradictory evidence remains a semantic problem. Explicitly
report temporal_state separately from target_state (matching|stale|other-target)
and conflict state; a SHA mismatch must never obscure an active overlap. A stale
selected-target start cannot support review-required. Valid other-target controls
remain evidence but cannot affect selected-target classification.

For the selected target, return all observed starts and temporal/target states,
including future, ended and expired declarations, with conflicts and their evidence.
Report only the candidate action population defined above, including unassociated
records, with their paths/IDs, hold resolution, urgency and the following ordered states
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
   in a complete, conflict-free pinned-source scan. Reports author intent
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
publication_target, current_target_and_expiry, release_hold_authorization]`, on the result and each row.
Text and MCP expose the same list, not only a prose disclaimer. This avoids
using a review of an unpublished record as an assumed future safeguard.

### Machine interface and status

The following is the normative JSON v1 contract (all keys required, no additional
keys; `?` below means JSON null is allowed, not key omission). Non-null strings
are UTF-8; SHA is a full lowercase Git object ID for the supported repository
format; timestamps are canonical RFC3339 UTC. Stored target_sha remains the
40-hex schema requirement above, so unsupported object formats fail explicitly.

```text
Result {
 schema_version: 1,
 target_ref: string?, target_branch: string?, target_remote: string?,
 target_sha: SHA?, now: timestamp,
 publication_verified: false, authority_verified: false,
 required_external_checks: Check[], scope: "explicit-sources",
 sources: Source[], coverage: "COMPLETE"|"INCOMPLETE",
 holds: Hold[], records: Record[], problems: Problem[]
}
Check = "publication"|"governor_authority"|"record_urgency_and_obligations"|
        "publication_target"|"current_target_and_expiry"|"release_hold_authorization"
Source {
 source_id: string, kind: "target"|"candidate", requested_refs: string[],
 sha: SHA?, channel: string, status: "read"|"unavailable", problem_ids: string[]
}
Provenance { source_id: string, path: string, blob_oid: SHA }
Hold {
 id: string, timestamp: timestamp, target_branch: string, target_sha: SHA,
 expires_at: timestamp, effective_end: timestamp, ends: End[],
 declared_release_references: string[],
 temporal_state: "future"|"active"|"ended"|"expired",
 target_state: "matching"|"stale"|"other-target",
 conflict: boolean, provenance: Provenance[], problem_ids: string[],
 publication_verified: false, authority_verified: false,
 required_external_checks: Check[]
}
End {
 id: string, timestamp: timestamp, hold_id: string, target_branch: string,
 provenance: Provenance[], problem_ids: string[],
 publication_verified: false, authority_verified: false,
 required_external_checks: Check[]
}
Record {
 id: string?, timestamp: timestamp?, provenance: Provenance[],
 urgency: "normal"|"urgent"|"invalid", deferred_by: string?,
 hold_id: string?, state: "publish-now"|"unassociated"|"review-required",
 reasons: Reason[], problem_ids: string[],
 publication_verified: false, authority_verified: false,
 required_external_checks: Check[]
}
Reason = "urgent"|"hold-control"|"requires-ack"|"handoff"|"review"|
         "incomplete-source"|"semantic-problem"|"unassociated"|"missing-hold"|
         "invalid-hold"|"future-hold"|"ended-hold"|"expired-hold"|"stale-hold"|
         "conflicting-hold"|"external-review-required"|"hold-behavior-disabled"
Problem {
 problem_id: string, code: "invalid-input"|"git-read-failed"|"nonregular-entry"|
   "invalid-record"|"path-conflict"|"id-conflict"|"invalid-control"|
   "missing-start"|"active-overlap"|"historical-overlap"|"future-overlap"|
   "hold-behavior-disabled",
 severity: "unavailable"|"semantic"|"advisory",
 operation: "resolve-target"|"resolve-candidate"|"read-tree"|"read-blob"|
   "validate-record"|"resolve-hold"|"classify-record"|"disabled-fallback",
 subject: { ref: string?, sha: SHA?, path: string?, record_id: string? },
 message: string, source_ids: string[], paths: string[], record_ids: string[]
}
```

Every valid end, including target-only and future ends, is a structured End
inside the matching Hold. Keep its own timestamp and every source/path/blob
provenance; never attach an end's provenance to its start. Equal ID/bytes across
sources deduplicates that End while retaining provenance; distinct end IDs remain
separate even when timestamps match. External end publication/authority checks
can therefore identify exact bytes and sources independently of the start.
Target-only ends are evidence here even though they create no action Record row.
Missing-start/invalid ends remain attributable via Problem source/path/record IDs,
without fabricating a Hold. Golden cases cover target-only end with candidate start,
candidate-only end, base/candidate identical end, and multiple ends with distinct
timestamps, paths and SHAs; assert effective_end and each End's fixed false flags.

Malformed controls are represented by problems, never invented valid Hold rows;
malformed candidate records additionally retain their path/provenance action row.
An unresolved association has hold_id:null; a resolved stale/ended/conflicting
association keeps its actual ID. Invalid urgency is never coerced to normal.
Target identity keys are null only when resolution fails; remote null also means
an explicitly local target. A resolution failure retains a Source with sha:null
and unavailable status. Invocation argument syntax errors remain ordinary CLI
usage errors; once a valid invocation starts evaluation, failures use this envelope.

Source IDs are `target` or `candidate:<SHA>`; an unresolved candidate source uses
`unresolved:<requested-ref>`. Sort sources by source_id, provenance by source_id
then path then blob_oid, holds/records by timestamp (null first), ID (null first),
then first provenance path/source. Sort ends by timestamp, ID and first provenance
path/source. Sort all reference/check/reason/problem-ID arrays
lexically and remove duplicates. Problems sort by code, severity, operation, subject.ref/sha/path/record_id
(null first), source_ids, paths and record_ids, then receive deterministic IDs
p000001 onward. Their identity is precisely that tuple, excluding message.
Subject ref identifies a requested ref for resolution, sha/path the pinned Git
object/channel/blob for reads, and record_id/path the semantic evidence when
available; unused fields are null. Never use Git stderr as subject identity.
Repeated occurrences with the same tuple coalesce into one problem; retain
unique diagnostic messages sorted lexically in message, separated by newline.
Changing platform Git error wording cannot renumber problems or row references.
All six external checks appear exactly once at result and every row. Dynamic
Git error prose is diagnostic only; tests assert stable code, identity and status.

Acceptance must use independent checked-in golden JSON for empty history,
active normal candidate, candidate urgent control, ended/expired/stale/future,
active conflict and unavailable source, plus malformed candidate/no-ID and
base-identical exclusion. Inject different platform/error prose for the same stable operation/subject and
assert identical problem IDs, ordering and row references. Golden files pin
every nested field, null, enum,
reason, provenance and sort order. Deserialize against a checked-in JSON schema
implementing this contract. CLI/MCP shared-backend parity supplements these
independent expected outputs and cannot replace them. Text is a compact projection
of this result with problem references and the same verification requirements.

Exit 0: complete requested pinned-source reads and no semantic problems,
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
- Real pinned target plus candidate Git refs with source labels, base-only urgent
  and normal history, base/candidate identical exclusion, local-only normal/urgent
  candidates, mixed large history/small action set, changed-byte conflicts,
  unavailable tree/blob and nonregular Git entries. Moving target/candidate refs
  after resolution cannot change observed objects. No implicit network call.
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
with CP-B-1 and the merged work order
`docs/plans/20260910T065635Z-coordination-lifecycle-semantics-work-order.md`
and correction `20260910T070211Z-planner-architect-p2rk`. CP-B-2 alone owns urgency;
Row-5 S3 is a hard successor and cannot introduce competing priority fields.
The older open !2027 plan is historical/superseded for this implementation, not
authority to reconcile a competing schema. Repeat the open-MR/path collision scan
immediately before coding, use merged CP-B-1 as base and retain S2/S3 on main. Bounded InfoSec and contribution review precede
Gate 1; build-engineer reviews command/test/CI footprint. No purchase or release
permission is requested. Gate 2 remains separate after implementation evidence.

The major risk is a hold mistaken for permission to suppress evidence. Controls
are deliberately explicit and advisory; urgent obligations stay independently
visible and errors cannot enable a safe effective hold. Another risk is adding
ceremony: one start/end per release hold, no per-record MR requirement, no
persistent service, no background polling and no coordination archive introduced.

Before merge rollback is abandoning the branch. Before any new-field record is
used, a gated revert remains possible. Rollout must first ship schema recognition,
parser/validator support, CLI/MCP, docs and compatible framework/tooling artifacts;
only then may CP-B-3 or an operator author the first immutable new-field record.
The intended first compatible release is v2.11.0; release preflight must verify
both actual framework and tooling artifacts include this support and pin their
real minimum versions in adoption instructions (no capability claim from a plan).
Older schema/tools reject unknown fields or lack hold-status; they are not a
supported reader once a channel uses these fields. Partial upgrades must be
completed before use, never handled by stripping immutable record fields.
After first use, rollback preserves schema/parser acceptance and forward-disables
advisory hold behavior through a gated forward correction to a conservative
reader: every candidate row becomes publish-now with hold-behavior-disabled
reason, fixed false verification fields and external checks; add an advisory
problem with that code. Existing source/semantic failures retain their exit
precedence. Exercise this fallback evaluator in a migration fixture; activating
it in a shipped correction remains separately gated. There is no new CLI mode
or automatic rollback mechanism, and the pre-field schema cannot be restored.
A migration fixture must validate immutable new-field records with the compatible
reader and its disabled behavior. Expiry bounds missed-end effects.

## Draft review corrections and migration

Independent draft review embedded in
`20260912T085315Z-planner-architect-kkjh` at commit `e964a024` identified D1-D4. Revised contract
never equates declaration state with verified publication; gives classification
precedence and conservative partial-source behavior; removes the independent
base override; defines chronology and equivalent evidence deduplication. The trial revision replaces mutable filesystem input with explicit immutable
candidate refs; it adds no discovery subsystem.

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
- SEC-3: test Git symlink/submodule/nonregular modes, invalid channel paths,
  and same-path changed bytes with both equal and unequal IDs; retain provenance.
- SEC-4: resolve every target/candidate ref once; inject ref movement/deletion
  after resolution and verify all reads remain on pinned objects. Inject every
  decisive Git read failure: exit2/INCOMPLETE/publish-now. Uncommitted filesystem
  edits cannot affect the result. Document per-ref observation/freshness limits.
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


## Narrowed trial disposition (8435, build consultation pkgg)

IPR-1 defines evidence versus action rows and base-identical exclusion. IPR-2
chooses declared references plus mandatory external release_hold_authorization,
with absent/unresolved/wrong-kind/conflicting/local-only/externally verified cases.
IPR-3 defines effective intervals, earliest ends and current versus historical
conflicts; fixtures cover exact start/end/expiry equality, equal/different multiple
ends, A ending before B starts and B starting before A ends, evaluated before,
during and after overlap. IPR-4 replaces the stale dependency. IPR-5 specifies
rollout and safe post-use disablement. IPR-6 pins nested JSON plus independent
goldens. A1 selects explicit pinned Git candidate inputs; no filesystem increment
is authorized or assumed. pkgg's filesystem-loss comparison is corrected here:
unmerged published branch records remain accessible through candidate refs; only
uncommitted drafts are excluded. No platform crate recommendation in that advisory
is treated as verified dependency evidence. Recheck this changed plan with the
trial reviewer and affected security/contribution consultations before Gate 1.


The bounded 246f57e5 re-review additionally required attributable end evidence
and stable problem identity. This revision nests structured End objects with
independent timestamps/provenance/verification requirements and sorts problem
identities by stable operation/subject evidence, never dynamic diagnostic prose.
These are contract clarifications within IPR-6, not new implementation authority.
