---
id: coordination-visibility-scanner-s1
status: pending-gate1
topic: coordination-visibility-scanner
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: read-only local git and coordination-channel scanner; no credentials, auth, network, or secret handling
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry tooling slice; no upstream distribution contract change before Gate 2
---

# Plan: Coordination Visibility Scanner S1

## Intent

Add a read-only Mix task that catches L009-class failures: coordination messages
that are committed on a side branch but invisible on the branch where the
recipient is expected to poll.

`mix agov.session.resume` already warns about branch-local coordination files
on the **current** branch. This slice adds the complementary scan: find buried
coordination obligations on **other local branches** before humans or agents
wait on a message that the recipient cannot see.

## Command

```bash
mix agov.coord.visibility --base-branch master
```

Options:

| Option | Purpose |
|---|---|
| `--base-branch <branch>` | Polling branch to compare against. Defaults to `master`. |
| `--branch <branch>` | Scan one branch. Repeatable. Defaults to all local branches except base. |
| `--dir <path>` | Coordination directory. Defaults to `../../docs/coordination`. |
| `--format text|json` | Text output for humans/agents; JSON for future CI. Defaults to text. |

## Scope

S1 is local and read-only:

- scans local git branches under `refs/heads`;
- compares `docs/coordination/*.md` files on each branch against the base
  branch;
- parses branch-local coordination message front matter;
- flags branch-local messages that create obligations;
- checks whether the base branch contains a later visible bridge/reference;
- reports sequence collisions where two branches contain different files with
  the same `seq`;
- exits non-zero when unbridged obligations or sequence collisions are found.

S1 does not:

- fetch remotes;
- scan remote-only branches;
- write bridge notes;
- auto-renumber coordination files;
- install pre-commit/pre-push hooks;
- change schemas or coordination message kinds.

## Obligation Detection

A branch-local coordination file is treated as an obligation when any of these
are true:

- `kind: handoff`;
- `requires-ack: true`;
- `kind` is `review`, `decision-record`, or `note` and `to` is not the same as
  `from`;
- `to: all`.

This is deliberately conservative. False positives are cheaper than invisible
work routing.

## Bridge Detection

A branch-local obligation is considered visible when the base branch contains a
later coordination message that names either:

- the branch-local filename in `references-to`; or
- the branch name and filename in the message body.

The filename reference is preferred. Body matching is an S1 compatibility
escape hatch for existing bridge notes such as seq 182, 184, 186, 188, and 190.

## Output Shape

Example text output:

```text
WARN branch-local coordination obligations not visible on master
  branch: feat/session-resume
  file: docs/coordination/00189-coder-tester.md
  seq: 189
  kind: handoff
  to: governor-reviewer
  topic: session-continuity-s1-one-command-two-steps-docs
  bridge: missing

ERR sequence collision
  seq: 185
  master: docs/coordination/00185-planner-architect.md
  branch: feat/session-resume
  branch-file: docs/coordination/00185-coder-tester.md
```

Success output:

```text
OK coordination visibility clean relative to master
```

## Affected Files

| File | Action | Purpose |
|---|---|---|
| `tooling/elixir/lib/agov/coord_visibility.ex` | Create | Pure-ish scanner and report builder around a git adapter |
| `tooling/elixir/lib/mix/tasks/agov.coord.visibility.ex` | Create | CLI wrapper and exit codes |
| `tooling/elixir/test/agov/coord_visibility_test.exs` | Create | Unit tests with injected git state |
| `docs/reference/session-resume.md` | Update | Link L009 visibility scanner as the next diagnostic when buried messages may exist on other branches |
| `AGENTS.md` | Update | Add quick-reference command |

No schema changes. No changes to `agents-govern/`.

## Design Notes

Use a thin git adapter, like `Agov.GitStatus`, so tests do not need real branch
mutation. The adapter should expose:

- local branch names;
- coordination file names for a given ref;
- file content for a given `ref:path`;
- base-branch coordination file names/content.

The scanner should parse Markdown front matter from git-provided content rather
than requiring files to exist in the working tree.

## Test Strategy

TDD first. Required RED scenarios:

1. **Unbridged handoff on side branch** — reports warning/error with branch,
   file, seq, kind, `to`, and topic.
2. **Bridged handoff** — side-branch obligation is not reported when base
   contains a later message referencing the filename.
3. **Body-only existing bridge** — side-branch obligation is not reported when
   base bridge body names branch and filename.
4. **Non-obligation branch-local note** — reported as informational or ignored,
   but does not fail.
5. **Sequence collision** — same seq on base and branch with different filename
   is reported as an error.
6. **Missing base branch** — returns a clear error, not a false clean result.
7. **JSON format** — stable machine-readable fields for future CI.

Manual verification:

```bash
cd tooling/elixir
mix agov.coord.visibility --base-branch master
mix agov.coord.visibility --base-branch master --branch feat/session-resume
```

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | No unbridged obligations or sequence collisions found |
| `1` | Unbridged obligations found |
| `2` | Sequence collisions or scanner errors |

If both unbridged obligations and collisions exist, exit `2`.

## Risks And Mitigations

| Risk | Mitigation |
|---|---|
| False positives from branch-local notes | Conservative obligation rules; only blocking for obligation-shaped messages |
| Existing bridge messages do not use `references-to` | S1 body matching accepts branch+filename text |
| Local scan misses remote-only branches | Document S1 as local-only; CI/remote scanning is a later slice |
| Shelling out to git is brittle in tests | Use injected adapter for unit tests |
| Output becomes noisy with many old branches | Support `--branch` for targeted scans; future slice may add age/merged filters |

## Rollback

Remove the two new Elixir files, the test file, and the two documentation
additions. No persisted data or schema changes need cleanup.

## Gate 1 Questions

**Q1 — Exit code severity:** Should unbridged obligations exit `1` immediately
in S1, or should S1 start warning-only and become blocking later? Recommendation:
exit `1`; the command is opt-in and exists specifically to catch L009.

**Q2 — Body bridge matching:** Should S1 accept bridge messages whose body names
the branch-local file, or require `references-to` only? Recommendation: accept
body matching in S1 for compatibility with the current bridge notes, then prefer
`references-to` in future process guidance.

**Q3 — Scan scope:** Should the default scan all local branches or only the
current branch? Recommendation: all local branches except base. Current-branch
coverage already exists in `session.resume`; this tool's value is finding
buried messages elsewhere.
