# feat: ship AGENTS.md adopter template + assistant_loader validator

Branch: `feat/agents-md-template`

## Summary

Implements both actions from learning `20260624T065743Z-ag7x`:

**Action 1 — AGENTS.md adopter template** (`agf/templates/AGENTS.md`)
Copy to project root. Cold-start commands, version reference comment
(`<!-- AGF: X.Y.Z -->`), rule orientation, coordination channel guidance,
gate/branch discipline, key agov commands. CUSTOMIZE markers throughout.

**Action 2 — `agov validate` assistant_loader advisory**
`agov.validate.assistant_loader` runs as part of the umbrella `agov validate`.
Advisory (WARN) only — never blocks. Checks:
- At least one loader exists (`AGENTS.md` or `.claude/CLAUDE.md`)
- Each loader's `<!-- AGF: X.Y.Z -->` comment matches `agf/VERSION` (stale → WARN)

A green validate result now catches both missing and stale loader files.

## Files changed

| File | Change |
|---|---|
| `agf/templates/AGENTS.md` | New — adopter AGENTS.md loader template |
| `agf/templates/README.md` | Added AGENTS.md row to the copy-to table |
| `agf/ADOPTING.md` | §1.4 step 2 names specific templates per assistant surface |
| `agf/learnings/foundry.yaml` | ag7x status → `implemented`; both actions noted |
| `tooling/rust/crates/agov-validators/src/assistant_loader.rs` | New — validator module |
| `tooling/rust/crates/agov-validators/src/lib.rs` | Added `pub mod assistant_loader` |
| `tooling/rust/crates/agov/src/main.rs` | Wired into `run_all_validators` |

## Gate self-test (negative test evidence)

Required per `always/gate-self-test-rule.md`:

```
$ cargo test -p agov-validators assistant_loader 2>&1 | grep -E "ok|FAILED"
test assistant_loader::tests::extract_rc_version ... ok
test assistant_loader::tests::extract_version_from_tight_comment ... ok
test assistant_loader::tests::extract_version_from_spaced_comment ... ok
test assistant_loader::tests::returns_none_when_no_comment ... ok
test assistant_loader::tests::no_warning_when_no_agf_version_file ... ok
test assistant_loader::tests::ok_when_only_claude_md_exists_no_version_comment ... ok
test assistant_loader::tests::warns_when_no_loader_exists ... ok
test assistant_loader::tests::ok_when_agents_md_exists_with_no_version_comment ... ok
test assistant_loader::tests::warns_when_agents_md_version_is_stale ... ok
test assistant_loader::tests::warns_when_claude_md_version_is_stale ... ok
test assistant_loader::tests::ok_when_version_matches ... ok
test result: ok. 11 passed; 0 failed
```

Negative cases: `warns_when_no_loader_exists` and `warns_when_agents_md_version_is_stale`
both create a temp directory with a known-bad state and assert the WARN fires.

The binary's `repo_root()` resolves via git and does not support an env override,
so live binary negative testing requires a full worktree. Unit tests are the
authoritative artifact.

## Smoke test on the foundry repo

```
OK agov.validate.assistant_loader
```

Foundry has `AGENTS.md` (no AGF version comment — template not used for the
foundry's internal loader). No false positive.

## Review checklist

- [ ] Template cold-start section is complete and commands are correct
- [ ] `assistant_loader.rs` advisory messages are clear and actionable
- [ ] Learning ag7x status updated to `implemented`
- [ ] All 11 tests pass; both negative cases fire as expected
- [ ] No `Co-authored-by` trailers in commits
