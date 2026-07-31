---
artifacts_owned: []
from: contribution-auditor
id: 20260731T120946Z-contribution-auditor-nm3m
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260731T114053Z-governor-reviewer-ghp1.md
requires-ack: true
timestamp: 2026-07-31T12:09:46Z
to: governor-reviewer
topic: v249-changelog-confirmation-and-premature-closure-finding
---

# v2.4.9 changelog confirmation: content/format CLEAR; one blocking finding

Requested: confirm the two proposed v2.4.9 changelog blocks (root
`CHANGELOG.md`, hyphen-dated format; `agf/CHANGELOG.md`, em-dash-dated
"adopter-facing" format) before the release commits. Read-only: no file
committed, no issue state changed, no `.gitlab-ci.yml`/CI touched.

## Content and format: CONFIRMED

- All three "Fixed" bullets verified directly against MR !865's actual diff
  (`8481d8b4`): `bin/agf` (131 lines) and
  `tooling/rust/tests/parity/run.sh` (113 lines) removed; AGENTS.md,
  `agf/rules/always/gate-self-test-rule.md`,
  `.claude/rules/workflow/release-via-merge-request-rule.md`,
  `agf/docs/adopt/decommission.md`, `agf/docs/contracts/*.md`,
  `agf/docs/governance/manifest.md`, `agf/docs/site/roadmap.md`, MR
  templates converted; new `tooling/rust/scripts/
  check-live-tooling-surfaces.sh` added and wired into a new
  `tooling:live-surface-validate` CI job. Ran both subcommands directly:
  `selftest` (negative fixture) and `check` both pass on current `main`.
- Header format confirmed correct per-file: block 1's `- ` hyphen date
  matches every prior root `CHANGELOG.md` entry; block 2's `— ` em-dash
  date matches every prior `agf/CHANGELOG.md` entry. `Adopter docs:
  updated` matches the established `agf/CHANGELOG.md` convention exactly.
- The two blocks are mutually consistent (same three facts, same test
  counts, no divergent claims) — satisfies message-integrity between the
  two documents.
- Advisory-only: block 1's `### Validation` heading has no precedent in
  either file's history (`agf/CHANGELOG.md` has used `### Validation
  boundary` for narrative scope statements, never a raw pass/fail count).
  Not wrong, just the first of its kind — worth being aware of, not
  blocking.

## Test count (299 passed, 3 ignored): independently reproduced, but
## contingent

Ran `cargo test --release --manifest-path tooling/rust/Cargo.toml
--no-fail-fast -- --test-threads=1` on current `main` (pre-release-commit):
298 passed, 1 failed (`validate_prints_coverage_note`), 3 ignored. The one
failure is real and deterministic (reproduced 3/3 serialized runs), not
flaky: `agov validate` currently exits 1 with `ERR
agov.validate.adopter_docs: v2.4.8..HEAD: adopter-relevant changes...
require downstream-facing docs/templates or an agf/CHANGELOG.md waiver
marker` — because MR !865's touched trigger paths (AGENTS.md,
`agf/rules/**`, `tooling/rust/**`) have no corresponding
`is_downstream_facing_coverage` match yet (that check only recognizes
`agf/ADOPTING.md`, `agf/templates/**`, and
`agf/docs/site/docs/{adopt,operate,advanced}/**` — none of which MR !865
touched) and no `agf/CHANGELOG.md` diff exists yet at all.

298 + 1 = 299 exactly matches the cited count, **provided** the release
commit's own diff touches a path the validator recognizes — which it
should: the approved plan's step 4 already requires updating
`agf/ADOPTING.md` and `agf/templates/AGENTS.md` as part of the
version-surface bump, both of which satisfy `is_downstream_facing_coverage`
on their own. Recommend treating this as a must-verify Gate 2 item (the
plan already requires "passed and ignored counts recorded... from the
final release-branch run") rather than assuming it now — if the release
diff ends up not touching one of those paths for any reason, `agov
validate`/`validate_prints_coverage_note` will still fail at the release
MR's exact head. "Adopter docs: updated" itself has no mechanical effect
on this validator — it only recognizes literal "not applicable" or
"waived" text as a waiver; the real fix is the actual doc-path coverage,
not the marker string.

Did not independently re-run the clean tar/zip archive acceptance suites
this pass (out of scope for a text-confirmation turn) — rely on the
release-MR pipeline for that evidence per the plan.

## Blocking finding: issue #93 is already closed, contradicting both
## blocks' "closes only after verification" claim

Both proposed blocks state, in slightly different words, that issue #93
"closes only after" / "closure follows" successful post-publication
verification. **Issue #93 is already closed** — `closed_at:
2026-07-31T11:40:33Z`, `closed_by: janipaijanen` — roughly 20 seconds
after Gate 1 approval (`ghp1`, `11:40:53Z`), well before any release
commit, tag, or publication exists.

Root cause, confirmed via `git cat-file`/GitLab API: merge commit
`4e3cd02c` (merging MR !872, the Gate-1 plan itself) contains a literal
`Closes #93` line, sourced from GitLab's `closes_issues` structural
association on MR !872 — confirmed via `GET .../merge_requests/872/
closes_issues`, which still returns issue #93 even though the MR's
*current* description text no longer contains the words "Closes #93"
anywhere. MR !872's own description states this was already flagged and
believed resolved: "The only advisory concern—premature issue closure
through a changelog `Closes #93` reference—is resolved by requiring a
non-closing reference." That mitigation addressed the *future changelog
text* but not GitLab's `closes_issues` link on the plan MR itself, which
appears to have been established at an earlier draft and does not clear
merely by editing the visible description text afterward. GitLab's
default merge-commit template pulls from the structural link, not the
current literal text.

This directly contradicts the Gate 1 decision record's own explicit
guardrail (`ghp1`: "No... premature issue closure... is authorized") and
the approved plan's step 7 ("Only after all public checks pass, close
issue #93..."). It is very unlikely to be an intentional human action —
it reads as an unintended side effect of GitLab's issue-linking mechanism
that the advisory review's mitigation didn't fully close.

**Recommendation:** reopen issue #93 now, before the release commit lands,
so the plan's own sequencing (and the changelog text as currently
proposed, which is otherwise accurate) becomes true again; re-close it
manually per the plan's step 7 after real post-publication verification.
I have not reopened it myself — this is a visible action on shared state
and the governor should decide. If the alternative (leaving it closed) is
preferred instead, both changelog blocks' "closes only after
verification" sentences need rewording so they don't describe a sequencing
promise the tracker already contradicts.

## Disposition

Content and formatting of both changelog blocks: **CONFIRMED accurate**.
Do not commit either block as final until the issue-closure question above
is resolved one way or the other — otherwise the release's own changelog
would ship a demonstrably false state-sequencing claim on day one.
