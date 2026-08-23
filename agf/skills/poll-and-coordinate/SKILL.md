---
name: poll-and-coordinate
origin: upstream
description: >
  One coordination-poll cycle: fetch shared refs, inspect coordination
  records/MRs/issues/pipelines from the fetched state (not a stale local
  checkout), classify findings, act within role/gate authority or record
  the concrete blocker, then continue or stop the cycle.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Poll and coordinate

Run one complete coordination-poll cycle. This skill operationalizes
[`agf/docs/site/docs/operate/poll-and-coordinate.md`](../../docs/site/docs/operate/poll-and-coordinate.md)
into an invokable procedure — it follows that page's scope and semantics
exactly; it does not restate them. Read that page first if this is your
first time running the cycle; it defines what "poll" and "coordinate"
mean and the terminal-outcome contract this skill's Step 5 enforces.

<!-- CUSTOMIZE: adjust the GitLab-specific commands in Step 2 if your
     project hosts on GitHub or another forge. -->

## Step 0 — Record the starting point

Note last-seen coordination IDs, timestamps, branch SHAs, pipeline IDs,
and the specific MRs/issues you are actively watching. This is what lets
a later cycle say precisely what changed, not just "nothing new."

## Step 1 — Fetch, then work from the fetched state, not a stale checkout

1. `git fetch --prune origin`.
2. Run every local-file governance check (`agov coord status`, `agov
   coord pending`, `agov coord stale`, `agov coord visibility`, `agov
   coord graph-index`, `agov session resume`) from a working tree
   freshly reset to the fetched ref for this cycle — a disposable
   worktree checked out from `origin/<branch>`, or `git reset --hard
   origin/<branch>` on a dedicated poll checkout. **A fetch alone does
   not update a persistent checkout's files** — reading local-file state
   from an unreset checkout silently reads stale or wrong-branch content
   even immediately after a successful fetch.

## Step 2 — Inspect every shared surface at true scope

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
   Step 1 freshly-reset working tree. A merged coordination record's
   carrier MR disappears from every "open MR" view the moment it
   merges — checking `docs/coordination/` commit history on the fetched
   ref directly (`git log --since <last-seen> -- docs/coordination/`)
   catches what an MR-only scan cannot.

## Step 3 — Classify

For each new item: urgency, current gate, role authority, ownership, and
whether it creates a downstream obligation. Distinguish items already
inside your role/gate/plan authority from items that need a decision
only the human governor or another role can make.

## Step 4 — Act inside the current gate

Advance the highest-priority obligation your role can already act on:
acknowledge/claim it and publish the causal coordination record;
continue an approved implementation; answer review findings within
their accepted scope; correct an owned MR description or handoff. For
anything outside current role/gate authority, record the concrete
blocker — do not leave "found work" unrecorded and unaddressed.

## Step 5 — Terminal outcome, then continue or stop

End the cycle only when the obligation is completed and recorded,
actively running with an identified monitor, or blocked with evidence
and ownership recorded. "Found work" alone, with nothing acted on or
recorded, is not a terminal outcome when your role is authorized to
advance it.

If more work remains or the cadence calls for another check, schedule
the next cycle using your assistant surface's real wake-up mechanism —
never a background shell timer, `sleep`, cron job, or detached process,
none of which reliably resume an assistant session. If nothing is
pending and the cadence's idle window has elapsed, stop rather than
reschedule.
