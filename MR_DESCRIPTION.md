# plan: Readable coordination IDs

## Summary

Adds the approved Gate 1 plan for human-readable coordination message IDs and
tooling-first ID generation.

The target readable ID format is:

```text
20260620T183914Z-coder-tester-r7k2
```

Existing 26-character ULID coordination IDs remain valid forever. This MR does
not rename historical coordination records and does not implement the readable
ID generator yet.

## Scope

- Adds `docs/plans/readable-coordination-ids.md`.
- Defines readable coordination ID and filename policy.
- Preserves ULID backward compatibility.
- Requires tooling-first ID generation for AI agents, with manual fallback when
  tooling is unavailable and validation passes.
- Scopes RFC 2119/8174 keywords to explicitly declared normative surfaces.
- Defines an interim branch-watch ownership protocol for multi-agent polling.
- Specifies validator work for the implementation phase:
  - accept both ULID and readable IDs;
  - block duplicate frontmatter IDs;
  - warn on filename/frontmatter timestamp or sender drift;
  - keep references and handoff tracking backward compatible.
- Records the branch-watch learning in `agf/learnings/foundry.yaml`.
- Records coordination proposals, Gate 1 routing, approval, supersession of
  invalid records, and coder-tester ack.

## Governance Trail

- Tooling-first readable ID proposal:
  `docs/coordination/01KVK62WGAZAMKHDQD3DAXPM9R-coder-tester.md`
- Scoped RFC 2119/8174 proposal:
  `docs/coordination/01KVK6AFC7PRYDTH4DNGGYWJK7-coder-tester.md`
- Branch-watch ownership learning:
  `docs/coordination/01KVK6P77H34DASXBVGRMPD9CV-coder-tester.md`
- Planner direction:
  `docs/coordination/01KVK6VK01RF18HHK69W-planner-architect.md`
- Plan handoff:
  `docs/coordination/01KVK77ACVM6TF8VKHZ5ZHJQ0T-coder-tester.md`
- Branch watcher record:
  `docs/coordination/01KVK7A38Z63ST2DTG0FKNTXXN-coder-tester.md`
- Valid Gate 1 routing:
  `docs/coordination/01KVK7TFMY6DQNPRH7SAS5BSHP-planner-architect.md`
- Valid Gate 1 approval:
  `docs/coordination/01KVK7TFMYC725T6V5CQDDASK5-governor-reviewer.md`
- Valid implementation handoff:
  `docs/coordination/01KVK7TFN1ADA2WBZDWKFRPBXR-planner-architect.md`
- Remediation ack for invalid earlier Gate 1 records:
  `docs/coordination/01KVK7TFN3S02ZFJ9WJJF8YQ6F-planner-architect.md`
- Coder-tester handoff ack:
  `docs/coordination/01KVK82P4N9PQZB1JMPZSEPY0T-coder-tester.md`

## Validation

- `agov plan validate docs/plans/readable-coordination-ids.md` passed.
- `agov coord validate` passed for the valid superseding Gate 1 records:
  - `docs/coordination/01KVK7TFMY6DQNPRH7SAS5BSHP-planner-architect.md`
  - `docs/coordination/01KVK7TFMYC725T6V5CQDDASK5-governor-reviewer.md`
  - `docs/coordination/01KVK7TFN1ADA2WBZDWKFRPBXR-planner-architect.md`
  - `docs/coordination/01KVK7TFN3S02ZFJ9WJJF8YQ6F-planner-architect.md`
- `agov coord validate docs/coordination/01KVK82P4N9PQZB1JMPZSEPY0T-coder-tester.md`
  passed.
- `git diff --check` passed.

## Review Notes

- This is a Gate 1 planning and coordination MR, not the implementation MR.
- Implementation is explicitly gated on MR !31 (`feat/rust-agov-phase1`)
  merging first.
- Three earlier Gate 1 coordination files in this branch used invalid
  20-character IDs:
  - `docs/coordination/01KVK7DYM21GGQ7ZZ5RR-planner-architect.md`
  - `docs/coordination/01KVK7J9MZSH6RJA7TKS-governor-reviewer.md`
  - `docs/coordination/01KVK7J9N2AW8DJZQAG2-planner-architect.md`
- Those invalid records are immutable and intentionally superseded by valid
  26-character ULID records listed in the governance trail.
- If MR !31 has not merged to `main` yet, this MR should either wait or be
  reviewed with that dependency in mind.

## Out of Scope

- Implementing readable ID generation.
- Making readable IDs the default immediately.
- Adding `agov watch start/status/stop`.
- Renaming historical coordination files.
- Removing ULID support.
- Extending readable IDs to plan files or other governance artifacts in this
  phase.
