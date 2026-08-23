---
name: poll-and-coordinate
origin: upstream
description: >
  The full pause/poll/act/rearm lifecycle for coordination work: pause any
  armed wake-up so cycles never overlap, load a durable cursor, fetch
  shared refs, inspect coordination records/MRs/issues/pipelines from the
  fetched state (not a stale local checkout or a same-SHA duplicate
  pipeline), classify findings, act within role/gate authority or record
  the concrete blocker, persist the cursor, then rearm or stop.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Poll and coordinate

Run the complete pause/poll/act/rearm lifecycle for coordination work in
one invocation. This skill operationalizes
[`agf/docs/site/docs/operate/poll-and-coordinate.md`](../../docs/site/docs/operate/poll-and-coordinate.md)
into an invokable procedure — it follows that page's scope and semantics
exactly; it does not restate them. Read that page first if this is your
first time running the cycle; it defines what "poll" and "coordinate"
mean and the terminal-outcome contract this skill's Step 6 enforces.

<!-- CUSTOMIZE: adjust the GitLab-specific commands in Step 3 if your
     project hosts on GitHub or another forge. Adjust the cursor path
     and default cadence/idle-window values in Inputs for your project. -->

## Inputs

| Input | Default | Meaning |
|---|---|---|
| `cadence_seconds` | 1800 | Gap between cycles while work may still be pending |
| `idle_window_minutes` | 240 | Stop rearming once this much wall-clock time has passed with nothing new found |
| `cursor_path` | `.agov/poll-cursor.json` (project-local, must be git-ignored) | Durable record of last-seen state across cycles and across sessions |

If the invoking surface passes explicit values for any input, use those.
Otherwise use the defaults above and state which you used.

Runtime polling state must never enter version control. Before the first
write to `cursor_path` in a given repository, verify it is actually
ignored (`git check-ignore -v <cursor_path>`). If it is not, add an
ignore rule for it (this project ships one for the default path, under
`.gitignore`) before writing — do not write an unignored cursor file and
do not silently fall back to committing it.

## Step 0 — Pause any armed wake-up before starting

If a wake-up, timer, or scheduled continuation for this same poll loop is
already armed on the current assistant surface, cancel or update it
*before* running this cycle's checks. Two cycles racing against the same
coordination/MR state can both act on the same finding.

**Safe fallback when the surface has no programmatic way to inspect or
cancel an existing wake-up:** proceed anyway, but treat every action in
Steps 4–5 as required to be idempotent and safe to run twice (claiming an
already-claimed item, or re-publishing an already-published record, must
be a harmless no-op detectable by checking current coordination state
before acting — never a blind re-post). State explicitly, in the cycle's
own record or summary, that no pause was possible and idempotency was
relied on instead.

## Step 1 — Load the durable cursor

Read `cursor_path` if it exists: last-seen coordination IDs, timestamps,
watched MR/issue numbers with their last-seen SHA and pipeline ID. If it
does not exist (first run, or a fresh clone), start from conservative
defaults (e.g. all coordination records and open MRs/issues in full —
Step 3's paginated-total discipline applies regardless of cursor state)
and note that no prior cursor was found.

## Step 2 — Fetch, then work from the fetched state in a disposable worktree

1. `git fetch --prune origin`.
2. Run every local-file governance check (`agov coord status`, `agov
   coord pending`, `agov coord stale`, `agov coord visibility`, `agov
   coord graph-index`, `agov session resume`) from a **freshly created,
   detached, disposable worktree checked out at the fetched ref**, never
   from the operator's persistent checkout. **A fetch alone does not
   update a persistent checkout's files** — reading local-file state from
   an unreset checkout silently reads stale or wrong-branch content even
   immediately after a successful fetch.

   ```bash
   git worktree add <project-worktree-root>/poll-<unique-id> origin/<branch> --detach
   ```

   Use a fresh, uniquely-named directory under the project's designated
   disposable-worktree root every cycle — never reuse a path that might
   already hold uncommitted work, and never target the operator's primary
   checkout path. Remove the worktree with `git worktree remove` once the
   cycle's local-file checks are done.

   **Do not** run `git reset --hard` on any checkout this skill did not
   itself just create as a disposable worktree. A reset on a shared or
   persistent checkout can silently discard uncommitted work — see
   `agf/rules/workflow/destructive-op-preflight-rule.md`. If a
   disposable worktree cannot be created (no spare path, no worktree
   support on the surface), state that explicitly as a blocker for Step 2
   rather than falling back to resetting an existing checkout.

## Step 3 — Inspect every shared surface at true scope

1. Merge requests: fetch the **true total open count**, not a
   default-paginated or recency-sorted slice.

   ```bash
   glab api "projects/<owner>%2F<repo>/merge_requests?state=opened&per_page=100" --paginate
   ```

   `glab mr list` alone is not sufficient for this step — its default
   page size (commonly 30) silently truncates, and sorting the result by
   `updated_at` and only reading the top few rows is structurally blind
   to a large, slow-moving backlog. For each MR in scope: source/target
   SHA, draft/merge state, approvals, exact-head pipeline and jobs,
   discussions, and comments/notes.
2. Issues: same paginated-total discipline. New issues, state/label
   changes, issue comments.
3. Coordination records: `agov coord status --recent 30 --roles`,
   `agov coord pending`, `agov coord stale`, `agov coord visibility
   --base-branch main`, `agov coord graph-index` — all from the
   Step 2 disposable worktree. A merged coordination record's carrier MR
   disappears from every "open MR" view the moment it merges — checking
   `docs/coordination/` commit history on the fetched ref directly
   (`git log --since <last-seen> -- docs/coordination/`) catches what an
   MR-only scan cannot.
4. Pipelines: before triggering or waiting on a new pipeline for an MR,
   check whether a pipeline already exists for the exact same source SHA.
   Reuse or wait on that one — do not trigger a duplicate paid run against
   an unchanged head. Record the pipeline ID and job IDs you relied on.

## Step 4 — Classify

For each new item since the cursor: urgency, current gate, role
authority, ownership, and whether it creates a downstream obligation.
Distinguish items already inside your role/gate/plan authority from items
that need a decision only the human governor or another role can make.

## Step 5 — Act inside the current gate

Advance the highest-priority obligation your role can already act on:
acknowledge/claim it and publish the causal coordination record;
continue an approved implementation; answer review findings within
their accepted scope; correct an owned MR description or handoff. For
anything outside current role/gate authority, record the concrete
blocker — do not leave "found work" unrecorded and unaddressed. Every
action here must be safe to repeat (see Step 0's fallback) — check
current state immediately before acting, not just at the start of the
cycle.

## Step 6 — Persist the cursor, then rearm or stop

1. Write the updated cursor to `cursor_path`: new last-seen coordination
   IDs/timestamps, and for every actively-watched MR/issue its current
   SHA and pipeline ID.
2. Terminal outcome required before ending the cycle: the obligation is
   completed and recorded, actively running with an identified monitor,
   or blocked with evidence and ownership recorded. "Found work" alone,
   with nothing acted on or recorded, is not a terminal outcome when your
   role is authorized to advance it.
3. If more work remains, or `idle_window_minutes` has not yet elapsed
   since the last cycle that found something new, rearm: schedule the
   next cycle after `cadence_seconds` using your assistant surface's real
   wake-up mechanism — never a background shell timer, `sleep`, cron job,
   or detached process, none of which reliably resume an assistant
   session.
4. If nothing new was found and `idle_window_minutes` has elapsed since
   the last cycle that found something, stop rather than rearm.
