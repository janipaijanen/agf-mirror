# Poll and coordinate

*Status: Released. Audience: operators and AI assistants responsible for advancing already-authorized work. A poll is complete only when all shared surfaces were checked and the highest-priority obligation was advanced, monitored, or concretely blocked.*

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
4. Inspect every relevant change request/MR: source and target SHA, draft and
   merge state, approvals, exact-head pipeline and jobs, discussions, and
   comments/notes.
5. Inspect relevant tickets/issues: new issues, state and label changes, and
   issue comments.
6. Classify new items by urgency, current gate, role authority, ownership, and
   whether they create a downstream obligation.
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

## Terminal outcomes

End a cycle only when the obligation is completed and recorded, actively
running with an identified monitor, or blocked with evidence and ownership.
“Found work” is not a terminal outcome when the current role is authorized to
advance it.
