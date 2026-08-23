# Poll and coordinate

*Status: Released. Audience: operators and AI assistants responsible for advancing already-authorized work. A poll is complete only when all shared surfaces were checked and the highest-priority obligation was advanced, monitored, or concretely blocked.*

**Shared surface** means repository/worktree state, published
coordination records, and forge issues/MRs/pipelines — anything visible
to concurrent AI assistants or operators sharing one repository. The
term is used as shorthand for that set throughout this page and the
`poll-and-coordinate` skill that operationalizes it.

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
4. Inspect every relevant change request/MR: source and target SHA, draft and
   merge state, approvals, exact-head pipeline and jobs, discussions, and
   comments/notes.
5. Inspect relevant tickets/issues: new issues, state and label changes, and
   issue comments.
6. Classify new items by urgency, current gate, role authority, ownership, and
   whether they create a downstream obligation.

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
7. Act inside the current gate. Split independent work into non-overlapping
   ownership, communicate material progress, and publish validated handoffs or
   blockers where the recipient can see them.
8. Monitor running work until it reaches a reviewable terminal point or a
   concrete blocker. Update last-seen markers.

**Poll** means observe. **Coordinate** means act, assign, record, and make the
result visible. **Approve, merge, and release** remain reserved decisions;
polling never grants them.

**Publishing a coordination-only handoff:** publish it promptly — do not wait
for the implementation artifact's Gate 2 to merge first. Branch it from the
target branch, not from an implementation branch's tip, and use the
`coordination:` commit title only when the branch's entire commit range is
additions under `docs/coordination/`. See
[Coordination protocol § Publication is independent of Gate 2](../concepts/coordination-protocol.md#publication-is-independent-of-gate-2-but-mechanically-verified)
for why, and `agf/rules/workflow/coordination-publication-rule.md` for the
mechanically-enforced conditions.

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

## Polling cadence

A cadence such as `15mins/180min` means scan every 15 minutes until 180 minutes
of inactivity, resetting the idle window after relevant activity. Use a real
assistant/orchestrator wake-up. A background shell timer, `sleep`, cron job, or
detached process does not ensure an assistant will resume. If the surface
cannot wake itself, say polling continues when the session is active.

## Terminal outcomes

End a cycle only when the obligation is completed and recorded, actively
running with an identified monitor, or blocked with evidence and ownership.
“Found work” is not a terminal outcome when the current role is authorized to
advance it.

