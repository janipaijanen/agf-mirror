# Draft Learning Entry: Polling Baseline Tail

Status: draft for governor-reviewer / planner-architect review.

This is a proposed entry for `agents-govern/learnings/foundry.yaml`. It has not
been appended to the learning record yet.

```yaml
  - id: L011
    date: "2026-06-13"
    category: gap
    agents_involved: [coder-tester, planner-architect]
    summary: >
      A coordination polling timer that only reports changes after it is armed
      can miss important recently-created messages that already existed at
      startup.
    detail: >
      Codex armed a 180-second coordination polling timer after planner-architect
      had already written recent coordination files, including seq 202-205. The
      timer captured the existing files as its initial baseline and then reported
      "no new messages", which made it look as if the trigger was broken. In
      practice, the timer was technically watching for future file changes, but
      it failed the operator need: a newly armed watcher must orient the agent
      to recent coordination state, not only future deltas.

      The immediate correction was to restart the timer so that arming prints
      the current file count, maximum numeric sequence, and the last 20
      coordination files before entering the 180-second polling loop. Subsequent
      polls compare filename set, file size/mtime, and max sequence number.

      This is related to L009 but distinct. L009 is about messages hidden on
      branches. This issue is about messages that are visible in the watched
      directory but were already present before the polling baseline was taken.
      A cold-start or rearmed poller must show enough recent context to avoid a
      false "nothing happened" interpretation.
    framework_impact: medium
    status: proposed
    recommendation: >
      Coordination polling tools and session timers should print a startup
      baseline that includes at least the current max sequence and the last 20
      coordination files. Polling should then report future changes by filename
      set, file size/mtime, and max sequence number. A "no new messages" poll
      should always include the current max sequence so humans and agents can
      see which baseline is being held.
```

## Notes

- The current ad hoc timer was restarted with `tail_n=20`.
- A future durable implementation should make the baseline tail count
  configurable, with 20 as the default.
- This draft should be reconciled with any future `mix agov.session.*` or
  coordination polling tool rather than becoming only chat memory.
