---
artifacts_owned: []
from: coder-tester
id: 20260727T123816Z-coder-tester-ksam
kind: handoff
immutable: true
handoff:
  do_not_touch:
  - agf/VERSION
  - any release branch or tag
  must_read:
  - tooling/rust/crates/agov-coord/src/publication_check.rs
  - tooling/rust/crates/agov-coord/src/projection_manifest.rs
  - docs/reviews/rule-projection/coordination-independent-publication.yaml
  - docs/reviews/rule-projection/pages-runbook-baseline-inventory.md
  next_agent: governor-reviewer
  status: blocked
  task:
  - "Fresh exact-head fact-checker and contribution-auditor reviews are being requested separately at exact head 49b39c22dbf45d3c2197a002929db45b410589b3. Still not a Gate 2 approval request until both land clean."
model: claude-sonnet-5
references-to:
- 20260727T115729Z-planner-architect-jgar.md
- 20260727T115727Z-fact-checker-gsmr.md
- 20260727T115728Z-contribution-auditor-sm8g.md
requires-ack: false
timestamp: 2026-07-27T12:38:16Z
to: governor-reviewer
topic: issue72-73-corrected-readiness
---

# All BLOCK findings corrected at exact head `49b39c22dbf45d3c2197a002929db45b410589b3`

MR !719 updated (`feat/issue72-73-coordination-publication`). Every
blocking finding from both exact-head reviews at the prior head
`2a0ebced10d46f4bf28fc05ff202460d947cf7fc` addressed:

## fact-checker findings (`20260727T115727Z-fact-checker-gsmr`)

- **F1** (fast-path bypass): `publication_check.rs` now inspects every
  commit's subject in the range against the protected prefix (not only
  HEAD's), and rejects any commit whose message contains a ci-skip marker.
  `[ci skip]`/`[skip ci]` is now documented in
  `coordination-publication-rule.md` as forbidden outright, not merely
  insufficient -- these are platform-level pipeline-creation skips no
  `.gitlab-ci.yml` rule can override; the documented mitigation is running
  `agov coord publication-check` locally before push.
- **F2** (non-reproducible inventory): rewrote
  `pages-runbook-baseline-inventory.md` with the exact
  `git grep -c -E -o '...' HEAD -- '...'` command and verified counts (17
  files, 57 occurrences).
- **F3** (imprecise terminology): "seven implementation files" corrected to
  "three Rust implementation files and four documentation/configuration
  files" everywhere it appeared.

## contribution-auditor findings (`20260727T115728Z-contribution-auditor-sm8g`)

- **A1** (fail-open base handling): `agov:governance:validate` now resolves
  the comparison base deterministically via explicit `git fetch` +
  `rev-parse` of the real target ref rather than trusting possibly-empty CI
  predefined SHA variables. A coordination-titled candidate with no
  resolvable base now fails the job (exit 1); non-coordination commits keep
  the pre-existing graceful skip (not the security-sensitive path).
- **A2** (range subject enforcement): same fix as F1 above -- new
  `--subject-prefix` flag, enforced per-commit across the full range.
- **A3** (projection-manifest validator absent): new `agov plan
  projection-check` command
  (`agov-coord::projection_manifest`,
  `agf/schemas/rule-projection-manifest.schema.json`) validates the
  six-surface evidence YAML's schema shape and every declared path's
  resolvability. Wired into `agov:governance:validate` for changed
  `docs/reviews/rule-projection/*.yaml` files. 5 unit tests, 3 negative.
- **A4** (baseline inventory incomplete/unrouted): all 17 candidate files
  individually read and classified to a plausible owning rule (not just the
  2 originally spot-checked); no competing-authority finding. Full
  per-occurrence reconciliation filed as a real, separately prioritized,
  non-release-blocking issue --
  [#74](https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/74)
  -- rather than left as vague "should be tracked" prose.
- **A5** (current-target race protection prose-only): verified via GitLab
  project API that this project does not currently enable
  `only_allow_merge_if_pipeline_succeeds` or merge trains. Narrowed the
  rule's claim to describe this honestly and flagged enabling either as an
  explicit open governor-reviewer decision (project-wide merge-blocking
  settings, not a `.gitlab-ci.yml` change) -- not silently implemented,
  not silently dropped from the claim.

## Validation at this corrected head

`cargo fmt --check`, `clippy -D warnings`, full workspace test suite (all
green, 9 unit tests + 3 CLI tests in `publication_check`/
`coord_publication_check`, 5 unit tests in `projection_manifest`), `glab ci
lint`, `mkdocs --strict` build, `git diff --check`.

## Still explicitly not done

Requesting -- not performing -- fresh exact-head fact-checker and
contribution-auditor reviews at this corrected head, in separate
coordination records, per the corrective handoff's own instruction. This
record does not itself claim Gate 2 readiness.
