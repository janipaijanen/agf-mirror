<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Coordination Publication: Independent, Mechanically-Bounded Fast Path
#
# Framework principle: Audit-trail timeliness without weakening Gate 2
# See: framework.md — Principle 2 (Artifact Ownership), Pattern 1 (Pipeline)
# Evidence: MR !692 (2026-07-27) — a branch titled and described as
#   coordination-only was rooted on an implementation commit; merging it
#   carried 7 implementation/product files onto `main` under a CI path that
#   both the `coordination:`-title `when: never` workflow rule and `[ci skip]`
#   were independently capable of suppressing. Issue #72.

> **Provisional status.** Single-project evidence (this repository,
> 2026-07-27). Adopters hitting this rule: please file a learning record so
> we can promote it from provisional to recommended.

**Scope:** Every merge request whose commit title(s) claim to be
coordination-only (matching the project's coordination fast-path
convention, e.g. a `coordination:` title prefix) and any tooling that grants
such a claim reduced CI validation.

## The rule

A coordination record MUST be able to publish (merge to the protected
branch) **independently of, and often ahead of**, the Gate 2 approval of the
implementation artifact it describes. Coordination visibility MUST NOT be
gated behind that artifact's own review latency — see
`agf/docs/site/docs/concepts/coordination-protocol.md` for why this is a
correctness property, not a convenience.

That independence is only safe if the fast path is **mechanically incapable
of carrying non-coordination history**. A title or commit-message
convention is not a safety mechanism — it is an unverified claim. This rule
grants the fast path real, narrow, tool-enforced authority in place of that
unverified claim.

### Multi-record publication

A coordination-only merge request MAY add any number of new coordination
records. The fast path remains valid only when every change in the full commit
range is a pure addition under `docs/coordination/*.md`; all six blocking
conditions below apply unchanged to the whole range.

A copy is an added record whose full raw bytes equal a coordination record
at the resolved merge base, the pinned current target tip, or an earlier
addition in the candidate range
(including the same commit). Identity is checked byte-for-byte, never by
similarity or by trusting distinct IDs; genuinely distinct records pass.

Records produced in the same session or work cycle SHOULD be published
together when no individual record has an independent visibility obligation.
Batching reduces merge-request churn without weakening validation. A record
that another agent or human is waiting on to act — including a live
obligation, security or gate finding, escalation, or blocking review — MUST
publish independently and immediately rather than wait for a batch.

This permission changes neither coordination lifecycle and supersession
semantics nor the protected title convention or the `[ci skip]` prohibition.

### What the fast path may mechanically verify (blocking authority)

A coordination-publication check MAY block a merge request when, and only
when, one of these objectively decidable conditions holds for the full
commit range between the merge base and the candidate head (not merely the
tip diff):

1. **Base ancestry is unresolvable.** No valid merge-base exists on fetched
   target history, or the target could not be fetched.
2. **A commit outside `docs/coordination/*.md` is present anywhere in the
   range.** This is the exact condition MR !692 violated: its tip diff was
   coordination-only, but its range was not.
3. **A change under `docs/coordination/` is not a pure addition.**
   Modifications, deletions, renames, or copies of existing coordination
   files are rejected; coordination records are append-only by design (see
   `immutable-tags-rule.md`'s sibling principle for tags — this rule is the
   append-only analogue for coordination records). Copies are detected by exact
   byte identity against the resolved merge base, pinned current target tip,
   and prior candidate additions,
   including earlier additions in the same commit, not by similarity.
4. **A merge commit or imported history is present in the range.**
5. **A newly added coordination record fails `agov coord validate`.**
6. **The commit range does not use the protected fast-path commit
   convention** (the project's `coordination:` title prefix, or equivalent),
   when the branch is being routed through the lightweight path.

Resolve the selected target ref and candidate HEAD once each per invocation to
immutable commit SHAs before ancestry/enumeration; use only pinned objects for
ancestry, range, aggregate diff and content reads. Report candidate_sha and
target_sha in text/JSON so a result is bound to the checked candidate. Inventory direct regular
coordination blobs at both the merge base and pinned target tip. Repeated
inventory entries seed the comparison set without becoming candidate additions.
Every Git read needed to establish conditions 1-6 must fail closed on error,
including merge/commit enumeration, message, per-commit/aggregate diff and object
reads. A failed read is unavailable evidence, not a successful empty result.
This is a prerequisite for enforcing the six existing conditions, not a new
semantic gate. A caller must still revalidate after a subsequent target advance.

### What the fast path MUST NOT decide (stays human/role authority)

- Whether prose is normative and needs a rule owner, or is merely
  explanatory.
- Whether a projection to an adopter page, runbook, or AI loader is
  semantically adequate (see `documentation-rule-projection-rule.md`).
- Whether a claimed "ready", "approved", or "complete" state in a
  coordination record's body is actually true. See lifecycle semantics
  below — that remains an authorized-role decision, not a tool inference.
- Whether a Gate 2 decision itself is correct. This rule governs
  **publication of records about work**, never the work's own approval.

Mechanical blocking authority is exactly the six conditions above. Anything
else that looks suspicious is a finding for a human or role reviewer to
raise, not a block the tool issues on its own.

## Lifecycle and supersession semantics

Coordination records make time-sensitive claims. Precision here is what
makes independent, fast publication safe rather than misleading:

- **Claims, notes, reviews, and ready handoffs** publish promptly when true.
  A handoff record means "ready for the named next action, addressed to the
  named next agent" — it does not mean the action is approved or complete.
- **Approval and completion records** (Gate 1 approval, Gate 2 approval,
  task completion) may only be published **after** the event they describe
  actually occurred. A tool or agent MUST NOT pre-publish an approval or
  completion record speculatively, even under this rule's fast path.
- **Only an authorized role or principal** may issue a binding decision
  record for that decision's gate (e.g., only the human governor issues a
  binding Gate 2 approval under this project's delegation model).
- **When readiness, an artifact's identity, or a prior claim changes**, the
  correction is a **new, append-only record** that references and
  supersedes the earlier one for the same audience. Coordination records
  are never mutated to fix a stale claim — see
  `docs/coordination/20260727T080733Z-governor-reviewer-jgp5.md` for a
  worked example of superseding an invalidated Gate 2 approval record
  without rewriting it.
- **A changed implementation head invalidates an earlier readiness claim**
  for that exact head. A Gate 2 handoff or review naming an exact commit SHA
  stops being current the moment that SHA changes; a new record naming the
  new exact head is required before Gate 2 proceeds.

A false lifecycle claim (e.g., a record asserting completion before the
event happened) is a blocking finding at whatever review stage catches it,
corrected by a superseding record — never by editing the false one.

## CI implementation shape

- Do not rely solely on commit-title pattern matching
  (`$CI_COMMIT_TITLE =~ /^coordination:/`) to skip validation for a
  candidate claiming to be coordination-only. A title is an unverified
  claim about the *branch*, and does not by itself inspect the *commit
  range*. The title MAY still be used to *select* which candidates run the
  stricter check (routing, not trust) — `agov coord publication-check`
  independently re-verifies the range regardless of title.
- **`[ci skip]` / `[skip ci]` is forbidden in coordination-only commit
  messages, full stop — not merely "insufficient."** These are platform-
  level markers (GitLab, GitHub, and others): when present, the forge skips
  pipeline *creation* entirely, before `.gitlab-ci.yml` or any equivalent
  CI configuration is evaluated. No workflow/job `rules:` can override this
  — the check literally never runs. A coordination-only commit carrying
  this marker is unverifiable by construction, not merely unverified.
  `agov coord publication-check` rejects any commit in the range whose
  message contains one, so the check still catches it whenever it *does*
  run (a later non-skipped commit in the same range, a manual pipeline
  trigger, or a local pre-flight run) — but the marker's main effect,
  silently preventing CI from running at all, cannot be caught after the
  fact by CI itself. Run `agov coord publication-check --base-ref
  origin/<target>` **locally, before pushing**, as the actual safety net
  for this case.
- Route coordination-publication candidates to a real, lightweight pipeline
  stage that runs the mechanical check above. Expensive implementation
  jobs (full test suites, cross-platform builds, distribution packaging) MAY
  still be skipped for a candidate that passes the check — that is the
  legitimate performance benefit this rule preserves.
- Revalidate when the target branch advances before merge. A pipeline that
  passed against a stale target is not evidence the check would still pass;
  use the forge's merge-train/merged-results support or an equivalent
  immediate pre-merge check against the current target tip.

  **Current enforcement state (this project, verified via GitLab project
  API) — a partial mitigation, not full closure of this requirement.**
  `only_allow_merge_if_pipeline_succeeds` is enabled at the project level
  (governor-reviewer decision, 2026-07-27,
  `docs/coordination/20260727T152325Z-coder-tester-rrtp.md`), together with
  `allow_merge_on_skipped_pipeline: false` — a merge request with no
  pipeline result, a failed pipeline, or a *skipped* pipeline (zero jobs
  matched, as happened transiently on this MR's own corrected head) cannot
  be merged. **This does not force a re-check when the target branch
  advances after the candidate's pipeline last passed** — the plan's
  "successful source-head pipeline alone is insufficient" condition is not
  met by this setting alone. `agov coord publication-check` always
  evaluates the *current* fetched target tip when it runs, so a fresh
  invocation gets a live answer, but nothing on this project *forces* that
  fresh invocation immediately before merge.

  Merge trains and merged-results pipelines — the mechanism the plan names
  as the primary way to close this gap — were investigated and found
  **unavailable on this project's GitLab plan tier**: `merge_pipelines_
  enabled` and `merge_trains_enabled` are absent from the GitLab project
  API response entirely (not merely `false`), confirming the feature
  cannot be enabled here regardless of permissions, not merely that it is
  currently off. A governor-reviewer decision (2026-07-27, same record)
  accepted this as the reason full current-target enforcement is not
  implemented: this is a **governed narrowing of the plan's requirement**
  for this project's specific hosting constraints, not a silently dropped
  claim and not a substitute implementation pretending to be equivalent. A
  future adopter or plan tier upgrade with merge-train/merged-results
  support available should revisit this and close the gap for real.
- The named Rust command implementing this check (provisionally
  `agov coord publication-check`) is the forge-neutral canonical
  implementation. CI wiring (GitLab, GitHub Actions, etc.) is a removable
  adapter that invokes it — do not duplicate the check's logic into
  CI-vendor-specific script.

## Required negative self-tests

Per `always/gate-self-test-rule.md`, this gate does not ship without proving
it fires on known-bad input:

1. **The exact MR !692 shape** — an implementation commit followed by a
   commit that only touches `docs/coordination/`, such that the tip diff
   alone looks coordination-only but the range does not. Must fail.
2. **Implementation-then-revert-then-coordination** — an implementation
   commit, a revert of that commit, and a coordination commit. The tip
   *aggregate* diff may look coordination-only, but the range still
   contains non-coordination commits. Must fail (the rule inspects every
   commit in the range, not only the net diff).
3. **Modification, deletion, or rename of an existing coordination record.**
   Each must fail as a non-pure addition.
4. **A multi-record batch with any non-coordination file.** Must fail even
   when all added coordination records are valid.
5. **A clean range adding two or more valid coordination records against
   current target history.** Must pass.
6. **A stale or unfetchable base ref.** Must fail closed, not pass open.
7. **A `[ci skip]` or `[skip ci]` marker in any coordination commit.** Must
   fail locally even though the forge would suppress pipeline creation.

8. **An exact copy of a merge-base record, pinned target-tip record, or earlier candidate addition
   (including the same commit).** Must fail with both paths; template-similar
   but byte-distinct records must pass. Include divergence where the target
   acquires the copied record after the merge base. Target pinning must prevent
   ref drift during reads, and overlapping inventories must not create false copies.
9. **Any decisive Git read fails.** Inject errors for target/candidate resolution, merge
   and commit enumeration, subject/full message, per-commit and aggregate diffs,
   and tree/blob reads. Every case must fail closed, including otherwise hidden
   merge/import and implementation-plus-revert histories.

## Why this exists

See the evidence header. The concrete cost of the gap this rule closes was
seven implementation/product files (`agov-core/src/roi.rs`,
`agov/src/main.rs`, `agov/tests/token_roi.rs`, and four documentation files)
entering `main` through a merge request titled and described as
coordination-only, with a title-suppressed CI pipeline. No malicious intent
was involved — the branch was simply built on top of the implementation
branch rather than on `main`, and nothing checked the difference. The fix is
not "write better MR descriptions" — it is removing the ability for a title
or commit message to make an unverified safety claim that a mechanical
check should be making instead.

## Related rules

- `always/coordination-message-rule.md` — establishes that coordination
  entries must exist and reference the action they record; this rule
  governs *how fast and how safely* those entries reach the protected
  branch.
- `always/coordination-sequence-guard-rule.md` — confirms before breaking
  the coordination sequence; this rule is what makes the sequence
  trustworthy to break-guard in the first place.
- `workflow/merge-request-workflow-rule.md` — all changes go through an MR;
  this rule narrows what "coordination-only" is allowed to mean within that
  requirement.
- `workflow/immutable-tags-rule.md` — the append-only principle for
  coordination records mirrors the immutability principle for tags: fix
  forward with a new record, never rewrite the old one.
- `always/gate-self-test-rule.md` — the negative-test requirement above is
  a direct application of this rule.
- `documentation-rule-projection-rule.md` — sibling rule governing where
  this rule itself (and any other normative rule) must be discoverable
  from adopter-facing Pages, runbooks, and AI loaders.

## Channel membership and bounded candidate discovery

Channel membership consists of lowercase `.md` files directly inside
`docs/coordination`. Both the local reader and Git visibility reader MUST use
this depth-one rule without filename exemptions; `README.md` is validated like
other channel files. Membership itself does not prove validity, resolution,
approval or gate authority.

Visibility additionally inspects Markdown within the channel subtree, direct
`docs` siblings whose names start with `coordination`, and the
`docs/coordination-archive` subtree. It reports schema-valid out-of-channel
records as misplaced candidates with acknowledgement/handoff metadata as
unassessed evidence. Recognizable malformed coordination front matter is an
invalid candidate; ordinary prose is not automatically coordination.

Exact path/blob entries already in the fetched target archive are inventoried
as lifecycle/closure **unassessed**. Identical candidate-ref entries at that path
are suppressed as duplicate baseline inventory, never treated as cleared.
New, changed or moved archive entries warn; a prefix alone is not an exemption.
Nested nonarchive records remain misplaced even on target. No migration or
archive mechanism is authorized by this contract.

The scan MUST disclose its ref SHAs and bounded scopes. Missing targets,
unavailable schemas and incomplete blob/ref reads produce incomplete coverage,
never a clean-empty claim. Diagnostics use existing advisory warning/error
exit conventions and confer no new MR-blocking or gate authority.

## Advisory recipient session addressing

`handoff.for_session` optionally names a recipient session using the same
nonempty-string schema shape and UUIDv7 convention as `session`. Top-level
`session` remains author provenance. `handoff next <role> --session <token>`
and `coord visibility --session <token>` annotate mismatched handoff recipients
as addressed elsewhere. They MUST NOT hide, reorder, deprioritize or change
ready/blocked state, lifecycle, exit codes, claims, temporary allocations,
independent review requirements or governor authority. Without the flag,
output remains unchanged. An absent `for_session` remains role-wide; a matching
recipient retains its original state. Notes/reviews remain role-wide and their
author session never becomes a recipient address.

## Advisory publication holds and urgency

`urgency: normal|urgent` is optional; omission means normal without changing
stored bytes. A governor-reviewer urgent `decision-record` may declare
`publication_hold: {action: start, target_branch, target_sha, expires_at}`.
The canonical branch is `refs/heads/<name>` and SHA is 40 lowercase hex digits.
A start references a release work order/decision, expires after its timestamp,
and lasts at most 24 hours. An urgent governor end instead declares
`{action: end, target_branch, hold_id}` and references that exact start. It must
match the target and not precede the start. Unknown or incompatible fields are
invalid. Controls cannot be deferred. Ordinary records may declare `deferred_by`
with a hold ID; that is author intent, never permission to delay.

A hold concerns main integration, never recipient visibility. Publish controls
on a shared coordination branch and bridge them independently of main. Urgent
records, controls, reviews, handoffs and requires-ack records publish immediately.
Do not merge controls just to make them visible: target drift still requires
release preflight again. A hold does not authorize a tag, merge or release.

`agov coord hold-status --target-branch origin/main --candidate-ref <ref>`
reads explicit pinned local Git snapshots only. Target selection accepts an
unambiguous local or remote-tracking branch, not HEAD, tags or raw SHAs.
Candidates accept commit-resolving refs, default HEAD, with no implicit fetch,
branch enumeration or filesystem drafts. `--dir` is a normalized repository-relative
Git tree path, default docs/coordination. Incomplete sources, malformed evidence,
missing/invalid associations, current overlaps and future, ended, expired or
stale associations cannot support delay. Other-target valid controls remain
evidence. Historical and future overlaps are advisory; simultaneous active
selected-target declarations conflict even when their SHAs differ. End intervals
are half-open and use the earliest valid end or expiry, whichever comes first.

Complete conflict-free matching active declarations can yield `review-required`;
this NEVER grants permission to delay. `unassociated` means ordinary rules apply.
All result and hold/end/action rows keep publication_verified and authority_verified
false. Before batching each exact record, independently verify all six checks:
publication, governor_authority, record_urgency_and_obligations,
publication_target, current_target_and_expiry, release_hold_authorization.
Check actual published start/end bytes, the governor's decision and referenced
release authorization, current remote/target/expiry, and absence of security or
gate findings, escalations and live obligations in the exact candidate. If any
check is unavailable, publish immediately. Source presence and role strings are
not authentication; candidate-only ends cannot automatically clear real holds.

The tool is advisory with explicit-sources coverage, not a whole inbox or atomic
cross-ref snapshot. Exit 0 means complete readable sources without semantic
problems; 1 means semantic problems; 2 means unavailable/incomplete evidence
and takes precedence. Affected candidates fail conservatively to publish-now.
These statuses add no blocking authority; the existing named coordination
schema validator remains the field-validation gate.

Rollout requires compatible framework schemas/rules and CLI/MCP artifacts before
any immutable record uses these fields. The intended first release is v2.11.0;
release preflight must verify actual artifacts and minimum supported versions.
Older readers reject fields or lack the command; complete partial upgrades
without stripping immutable fields. Before first use a gated revert is possible.
After first use preserve field recognition: a separately gated forward correction
may disable advisory behavior, reporting publish-now/hold-behavior-disabled for
all candidates while retaining error precedence and external checks. No runtime
rollback flag or automatic migration is introduced here.
