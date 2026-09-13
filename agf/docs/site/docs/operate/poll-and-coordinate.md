<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Poll and coordinate

*Status: Released. Audience: every active operator or AI development
agent/session in an AGF-governed project — not only the agent that opened a
particular MR, and not only when someone types "poll". A poll is complete
only when all shared surfaces were checked and the highest-priority
authorized obligation was advanced, monitored, or concretely blocked.*

This page explains and situates the obligation. The normative contract — who
must poll, the complete observation surface, collision-safe selection,
temporary role-allocation consensus, the exhaustive stop boundary, and the
empty-queue behavior — is owned by
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`.
Where this page and that rule differ, the rule governs.

**Shared surface** means repository/worktree state, published
coordination records, and forge issues/MRs/pipelines — anything visible
to concurrent AI assistants or operators sharing one repository. The
term is used as shorthand for that set throughout this page and the
`poll-and-coordinate` skill that operationalizes it. Observation covers the
**whole** shared surface inside the governed project scope; work selection
and execution stay role-scoped — every active agent observes everything, but
they do not all claim everything.

## One complete cycle

1. Record last-seen coordination IDs, timestamps, branch SHAs, pipeline IDs,
   and monitored MRs/issues.
2. Fetch shared refs with `git fetch --prune origin`.
3. Inspect coordination with the supported Rust commands:

   ```bash
   agov session resume --base-branch main
   agov session resume --role <role> --base-branch main
   agov coord status --recent 30 --roles
   agov coord stale
   agov coord visibility --base-branch main
   agov coord graph-index
   ```

   `agov coord list --since <UTC timestamp>` may be used when the installed
   release exposes it. Check `agov coord list --help` rather than assuming
   version parity. `coord watch` is not a shipped command.

   Run `agov coord pending` every cycle when the installed release exposes
   it — it is the only surface that reports unresolved non-`handoff`
   `requires-ack: true` records (claim, review, note, decision-record,
   fast-track-waiver, lesson-proposal). Neither `session resume` nor
   `coord graph-index` reports this class; skipping this command risks
   missing an obligation none of the other listed commands surface.
   `handoff` obligations remain `agov coord graph-index`'s and `agov
   handoff next`'s surface, unchanged. The command only *reports* pending
   obligations — a later coordination record's direct `references-to` is
   what resolves one, not the `pending` command itself.
4. Inspect **every** open change request/MR — regardless of author or apparent
   relevance: source and target SHA, draft and merge state, approvals,
   exact-head pipeline and jobs, discussions, and comments/notes. A
   cursor-based delta narrows how many records transfer; it never narrows
   which authors, roles, or classes are observed. APIs without reliable
   changed-since semantics are paginated in full.
5. Inspect **every** open item in tickets/issues, regardless of assignee or
   apparent relevance: new issues, state and label changes, and issue comments.
6. Recheck every exact external state (branch, tag, release, runner,
   pipeline, job) the agent started or awaits.
7. Classify new items by urgency, current gate, role authority, ownership, and
   whether they create a downstream obligation.

A full paginated reconciliation of every currently open MR/issue and each
monitored exact head is mandatory on cold start, post-compaction resume,
cursor loss, evidence of missed events, and at least once per UTC day while
the session stays active. Between those, the delta cycle bounds routine cost
at the number of changed items.

### Common pitfalls (both cause a poll to look complete while missing real work)

- **Reading a stale local checkout instead of the fetched ref.**
  `git fetch --prune origin` updates only remote-tracking refs; it does
  not update a persistent working directory's checked-out files. A
  local-file tool (`agov coord status`, `agov coord pending`, etc.) run
  from that persistent checkout after a fetch still reads whatever
  branch and commit the checkout happens to be on — which may be
  unrelated to, or older than, what was just fetched. Run local-file
  governance checks from a freshly created, detached, disposable
  worktree checked out at the fetched ref for that cycle — never assume
  a fetch alone is sufficient, and never `git reset --hard` an existing
  checkout to catch up, since a reset can silently discard uncommitted
  work on a checkout that cycle did not create. Remove the disposable
  worktree once the cycle's local-file checks are done.
- **Trusting a default-paginated MR/issue listing.** Many CLI tools
  (e.g. `glab mr list`) cap results at a default page size (commonly 30)
  with no truncation warning. Combined with sorting by recency and only
  looking at the first few rows, this is structurally blind to a large,
  slow-moving backlog — exactly the shape a stale-MR backlog takes.
  Always fetch the true total count (an explicit large `--per-page`, or
  `--paginate` against the API) before concluding "nothing new," and
  never substitute a recency-sorted slice for a full count.
- **Triggering a duplicate pipeline against an unchanged head.** Before
  triggering or waiting on CI for an MR, check whether a pipeline already
  exists for the exact same source SHA and reuse or wait on that one.
  Retriggering against an unchanged head burns paid CI minutes for no new
  evidence.
8. Select and act inside the current gate, using the priority order in the
   canonical rule (screened security → in-role obligation → gate-ready item →
   next-release readiness → existing-feature quality → approved features).
   Split independent work into non-overlapping ownership, communicate material
   progress, and publish validated handoffs or blockers where the recipient
   can see them. Advance the durable cursor only after the selected action is
   completed, actively running and monitored, or concretely blocked.
9. Monitor running work until it reaches a reviewable terminal point or a
   concrete blocker. Update last-seen markers.

**Poll** means observe. **Coordinate** means act, assign, record, and make the
result visible. **Approve, merge, and release** remain reserved decisions;
polling never grants them.

### Collision-safe selection, role consensus, and stopping

- **Before the first mutation**, check for an existing claim or an
  overlapping plan/MR on the same item or artifacts. Publish a `kind: claim`
  naming the exact item, branch/lane, and file ownership when one is needed to
  prevent collision; an existing current handoff or claim to that lane
  satisfies it, and small related mutations may share one bounded batch claim.
  Work under a trivial-change waiver or the `simplicity-rule.md`
  research-maintenance fast path keeps its own branch/MR collision scan but
  acquires no separate claim ceremony.
- **Concurrent sessions with an ambiguous or mismatched role** reach durable
  consensus on a temporary role allocation before either mutates — a proposal,
  explicit acknowledgements from every affected session, and a resume role.
  Consensus cannot activate a suspended role or create authority; an
  unreachable peer is routed to the governor, not waited on indefinitely.
- **Stopping** means stop autonomous mutation and request or await a named
  authority: Gate 1 or Gate 2 approval; tag/release/deploy authorization;
  scope expansion or conflicting instructions; destructive/irreversible action;
  new role or permission authority; secrets/security/privacy/legal ambiguity;
  or a concrete external blocker after one bounded diagnostic pass. Ordinary
  uncertainty, waiting CI, an empty chat inbox, or the absence of another
  prompt are not stop conditions. Name the boundary and cite the missing
  decision; if you cannot, continue the next authorized action.

The full definitions of each of the three are in the canonical rule.

**Publishing a coordination-only handoff:** publish it promptly — do not wait
for the implementation artifact's Gate 2 to merge first. Branch it from the
target branch, not from an implementation branch's tip, and use the
`coordination:` commit title only when the branch's entire commit range is
additions under `docs/coordination/`. See
[Coordination protocol § Publication is independent of Gate 2](../concepts/coordination-protocol.md#publication-is-independent-of-gate-2-but-mechanically-verified)
for why, and `agf/rules/workflow/coordination-publication-rule.md` for the
mechanically-enforced conditions.
Batch same-cycle records into one coordination-only merge request when no
individual record is holding up another actor; publish an urgent or waiting
obligation alone immediately.

## Splitting work safely

Before delegating, identify the approved plan, role, owned files or artifacts,
dependencies, expected output, validation, and return channel. Parallel slices
must not edit the same files unless an explicit integrator owns reconciliation.
Progress messages should name exact branches, commits, MRs, tests, blockers,
and the next check—not unverifiable percentages.

A request to another role is not complete as a chat or MR note. First publish
the canonical coordination record; then use a comment only as a bridge naming
its coordination ID and location.

## GitLab adapter

For GitLab, scan merge requests **and** MR discussions/notes, issues **and**
issue notes, exact-head pipelines/jobs, releases, and remote branches. `glab`
or the API may automate the scan, but GitLab output is observation evidence,
not gate authority. Preserve IDs and SHAs in progress reports.

### First-push pipeline race

This is a GitLab configuration concern, not behavior created by AGF. Projects
that create both branch-push and merge-request pipelines, then suppress branch
pipelines with `$CI_OPEN_MERGE_REQUESTS`, can still see an initial pair: the
branch push may begin before GitLab has registered the MR, and MR creation then
starts its own pipeline for the same commit. GitLab documents both the
[`CI_OPEN_MERGE_REQUESTS` predefined variable](https://docs.gitlab.com/ci/variables/predefined_variables/)
and [merge request pipelines](https://docs.gitlab.com/ci/pipelines/merge_request_pipelines/).

Do not infer redundancy from a matching SHA alone. Compare pipeline source,
ref, status, and required-job coverage first. Each project must name its
canonical acceptance source; this framework's own foundry preserves the MR
pipeline. Default-branch, tag, schedule, API, web, trigger, and downstream
pipelines can intentionally share a SHA and should not be canceled merely
because another pipeline exists.

## Polling cadence

A cadence such as `15mins/180min` means scan every 15 minutes until 180 minutes
of inactivity, resetting the idle window after relevant activity. Use a real
assistant/orchestrator wake-up. A background shell timer, `sleep`, cron job, or
detached process does not ensure an assistant will resume. If the surface
cannot wake itself, say polling continues when the session is active.

Scheduled polling is **non-reentrant**. A wake that becomes due while the same
session is executing an authorized work item does not interrupt that work or
start a concurrent poll — it sets one pending-poll latch, and repeated ticks
coalesce into it. Exactly one complete poll runs immediately after the active
item reaches its next safe completion, handoff, gate, or blocker boundary, and
cadence then continues from that cycle. "Deferred while busy" is not
"skipped." The 180-minute idle calculation uses observed relevant activity,
not the number of coalesced ticks.

## Terminal outcomes

End a cycle only when the obligation is completed and recorded, actively
running with an identified monitor, or blocked with evidence and ownership.
“Found work” is not a terminal outcome when the current role is authorized to
advance it.

## Inspect a release hold without hiding coordination

After an explicit fetch, publish the governor's urgent start on a shared
coordination branch and bridge it to the release lane. Keep main frozen only
under the independently verified release decision; do not merge the control
merely to make it visible. Inspect the actual shared ref:

```sh
git fetch --prune origin
agov coord hold-status --target-branch origin/main --candidate-ref origin/coordination/release-hold --format json
```

Substitute the existing shared branch. Repeat `--candidate-ref` for explicit
sources; omission defaults to HEAD. `--dir` selects a normalized Git tree path,
not working-tree drafts. Targets must be unambiguous branches; candidates may
be commit-resolving refs. Sources are pinned separately, with no implicit
network, enumeration or atomic cross-ref snapshot. Complete coverage covers
only requested sources and is not fresh publication evidence or a whole inbox.

Text/JSON show publication_verified=false, authority_verified=false and these
required_external_checks: publication, governor_authority,
record_urgency_and_obligations, publication_target, current_target_and_expiry,
release_hold_authorization. Verify each exact record and control externally.
If any check is unavailable, publish immediately. Review-required never permits
delay; unassociated follows ordinary rules. Publish urgent records, controls,
reviews, handoffs and requires-ack records independently at once. Verify end
publication/authority before treating a real hold as ended. Target drift still
requires release preflight again.

Exit 0 means complete readable evidence without semantic errors, 1 semantic
errors, 2 unavailable/incomplete sources (precedence over 1). These are advisory
statuses, not merge or release gates. Malformed evidence and incomplete reads
force potentially affected action rows to publish-now.

### Reading urgency in the v2.11 development CLI

Urgent pending records lead the queue, and session resume highlights them.
Use this ordering to select attention, then check role, ownership and gates as
usual: urgency is self-declared and does not supply permission. Normal and
omitted urgency share the same tier. This behavior is Level 4 (validated in
development), pending inclusion in a supported release.

### Handling challenge evidence

Read the separate challenge section in `coord pending`. Investigate unavailable
sources before interpreting closure; a future record reports when it becomes
effective. A party reply remains decision-pending. For an expired window,
route the evidence to the governor through normal coordination; the display
itself never decides a gate. When raising a review, link one disputed source
explicitly with `challenge.disputed_record_id`, and keep any unrelated
requires-ack request independent. See the
[protocol](../concepts/coordination-protocol.md#challenge-response) for record
fields and the conditional JSON envelope for compatible consumers.
