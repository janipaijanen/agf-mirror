# Coordination Visibility Scanner

`mix agov.coord.visibility` scans local git branches for coordination messages
that create obligations but are not visible from the branch where the recipient
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
- when sequence numbers have been assigned concurrently on multiple branches.

## Command

Run from the Mix project:

```bash
cd tooling/elixir
mix agov.coord.visibility --base-branch master
```

To focus on one branch:

```bash
mix agov.coord.visibility --base-branch master --branch feat/session-resume
```

For machine-readable output:

```bash
mix agov.coord.visibility --base-branch master --format json
```

## What It Checks

The scanner compares local branches against the base branch.

It reports:

- branch-local coordination handoffs not visible from the base branch;
- branch-local messages with `requires-ack: true`;
- branch-local cross-role review, note, or decision messages;
- branch-local `to: all` messages;
- sequence collisions between a side branch and base;
- duplicate sequence numbers already present on the base branch.

Bridge detection accepts either:

- a base-branch message that references the branch-local filename in
  `references-to`; or
- a base-branch message body that names the branch-local filename.

The filename reference is preferred. Body matching exists so the scanner can
recognize current bridge notes that were written before this tool existed.

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | No unbridged obligations or sequence collisions found |
| `1` | Unbridged obligations found |
| `2` | Sequence collisions or scanner errors found |

If both obligations and collisions exist, the command exits `2`.

## How Humans Should Use It

If the command reports unbridged obligations, do not assume the recipient has
seen the message. Add a visible bridge note on the base branch, or move the
coordination message to the branch the recipient polls.

If the command reports sequence collisions, do not merge until the collision is
understood. If the collision is already on the base branch, document it with a
new coordination message because immutable records cannot be renumbered
retroactively.

## How AI Agents Should Use It

Treat non-zero output as a stop signal before creating more coordination
records.

For each unbridged obligation:

1. Identify whether the message still creates a live obligation.
2. If yes, file a visible bridge note on the recipient's polling branch.
3. If no, file a completion, review, or decision record that makes the closure
   visible.

For each sequence collision:

1. Stop assigning new coordination numbers from stale branch state.
2. Read the base branch tail.
3. Renumber only your own unmerged branch-local message if it has not been
   reviewed or merged.
4. If the collision is already on base, file a correction note rather than
   editing immutable records.
