# Coordination Visibility Scanner

`agov coord visibility` scans git branches for coordination messages that
create obligations but are not visible from the branch where the recipient
is expected to poll.

It is a read-only diagnostic for L009-class failures. It does not create bridge
notes, renumber files, merge branches, or approve work.

## When To Run It

Run it before waiting on a human or agent to act on a coordination message that
was written on a feature branch.

Also run it:

- before Gate 2 review or merge;
- after discovering a branch-local coordination message;
- when a receiving agent says it cannot see a handoff;
- when sequence numbers have been assigned concurrently on multiple branches;
- after `git fetch`, to check obligations on a branch you have not checked out
  locally.

## Command

Run from the repository root:

```bash
agov coord visibility --base-branch main
```

The default scan (no `--branch`) covers **local branches and
remote-tracking branches** (`refs/heads/*` and `refs/remotes/<remote>/*`),
deduplicated by commit -- a branch checked out locally and its
remote-tracking alias are one candidate, not two. This scan is batched
(one file listing plus one batched content read per candidate branch --
a bounded O(branches) number of Git subprocesses, not one per
coordination file), so the default scan is fast enough for everyday use
without needing
`--branch` purely for performance reasons. Prefer a targeted `--branch`
invocation when you already know which branch you care about, or when
you want output scoped to a single review.

To focus on one branch:

```bash
agov coord visibility --base-branch main --branch feat/session-resume
```

An explicit `--branch` (or list of branches) is used exactly as given --
no enumeration or deduplication is applied to an explicit selection.

For machine-readable output:

```bash
agov coord visibility --base-branch main --format json
```

## What It Checks

The scanner compares candidate branches against the base branch.

It reports:

- branch-local coordination handoffs not visible from the base branch;
- branch-local messages with `requires-ack: true`;
- branch-local cross-role review, note, or decision messages;
- branch-local `to: all` messages;
- sequence collisions between a side branch and base (blocking);
- sequence collisions confined entirely to base-branch history, unrelated
  to any candidate branch (reported as a summary, non-blocking -- see
  "Legacy base-history collisions" below).

Bridge detection accepts either:

- a base-branch message that references the branch-local filename in
  `references-to`; or
- a base-branch message body that names the branch-local filename.

The filename reference is preferred. Body matching exists so the scanner can
recognize current bridge notes that were written before this tool existed.

## Base-ref freshness

`--base-branch main` (the default) resolves to local `main` unless a
configured upstream (e.g. `main` tracking `origin/main`) has a different
OID -- in that case the upstream's OID is used as the effective
comparison base instead, so `git fetch` followed by `agov coord
visibility --base-branch main` is correct without requiring you to
separately fast-forward local `main`. Both text and JSON output state
which OID was actually used:

```
effective-base: <oid> (local|upstream)
```

Every ref that resolves to the effective base OID -- the local base
branch itself and any remote-tracking alias at that same OID -- is
excluded from the candidate branch list; the base is the comparison
point, never a candidate obligation source.

## Legacy base-history collisions

A sequence collision confined entirely to base-branch history (both
colliding messages already exist on the base, unrelated to any candidate
branch) does not block. It is reported as a single summary line and does
not affect `exit_code`:

```
WARN <N> legacy sequence collision group(s) confined to base branch history; run agov coord validate for detail
```

A collision that involves a candidate branch (a real conflict you need to
know about before merging) still blocks, with full per-collision detail,
exactly as before.

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | No unbridged obligations or branch-relevant sequence collisions found |
| `1` | Unbridged obligations found |
| `2` | Branch-relevant sequence collisions or scanner errors found |

Legacy base-history-only collisions never drive the exit code by
themselves. If a scan has both base-history-only legacy noise and a
genuine unbridged obligation, it exits `1` -- the legacy noise never
escalates that to `2`, and never suppresses the `1`. If both a genuine
obligation and a branch-relevant collision exist, the command exits `2`.

## JSON fields

| Field | Meaning |
|---|---|
| `base_branch` | The requested base branch name, exactly as given via `--base-branch` |
| `effective_base_oid` | The commit OID actually used as the comparison base (see "Base-ref freshness") |
| `effective_base_source` | `"local"` or `"upstream"` -- which ref supplied `effective_base_oid` |
| `scanned_branches` | Deduplicated candidate branch identifiers actually scanned |
| `unbridged_obligations` | Branch-local obligations not visible from the base |
| `sequence_collisions` | Branch-relevant collisions only (blocking; drives `exit_code`) |
| `legacy_base_collisions` | `{"count": <int>, "seqs": [<int>, ...]}` -- always present, never `null` |
| `errors` | Scanner-level errors |
| `exit_code` | The process exit code, mirrored into the JSON body |

## How Humans Should Use It

If the command reports unbridged obligations, do not assume the recipient has
seen the message. Add a visible bridge note on the base branch, or move the
coordination message to the branch the recipient polls.

If the command reports sequence collisions, do not merge until the collision is
understood. If the collision is already on the base branch, document it with a
new coordination message because immutable records cannot be renumbered
retroactively. A legacy base-history-only collision (reported as a summary,
not a per-collision block) does not need to block a merge on its own, but
is worth a look with `agov coord validate` if the count seems unexpected.

## How AI Agents Should Use It

Treat exit `1` or `2` as a stop signal before creating more coordination
records. Exit `0` in the presence of a nonzero `legacy_base_collisions.count`
is not an error -- the legacy noise was tolerated, not a false negative.

For each unbridged obligation:

1. Identify whether the message still creates a live obligation.
2. If yes, file a visible bridge note on the recipient's polling branch.
3. If no, file a completion, review, or decision record that makes the closure
   visible.

For each sequence collision reported in `sequence_collisions` (branch-relevant,
blocking):

1. Stop assigning new coordination numbers from stale branch state.
2. Read the base branch tail.
3. Renumber only your own unmerged branch-local message if it has not been
   reviewed or merged.
4. If the collision is already on base, file a correction note rather than
   editing immutable records.
