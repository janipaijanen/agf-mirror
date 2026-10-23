---
id: 20261023T083302Z-remove-elixir-leftovers
status: pending-gate1
topic: remove-remaining-elixir-leftovers
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: Review requested. Package B removes a release-payload guard; no credentials or schemas change.
  - role: contribution-auditor
    status: pending
    reason: Review requested before Gate 1.
affected_paths:
  - mise.toml
  - mise.lock
  - .gitignore
  - .dockerignore
  - .gitlab-ci.yml
  - agf/dist/release-manifest.yaml
  - agf/dist/README.md
  - agf/docs/site/reference/release-artifacts.md
  - docs/reviews/rule-projection/20261023T083302Z-remove-elixir-leftovers.yaml
  - tooling/rust/scripts/check-no-elixir-artifacts.sh
  - tooling/rust/scripts/check-live-tooling-surfaces.sh
  - tooling/rust/crates/agov/src/dist.rs
  - tooling/rust/crates/agov/tests/coord_brief.rs
  - tooling/rust/crates/agov-coord/src/coord_validator.rs
  - tooling/rust/crates/agov-core/src/roi.rs
depends_on:
  - 20261023T045214Z-remove-elixir-references
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: remove the remaining Elixir leftovers

Tracks issue #140. Follows #139 (guide prose, merged as `f13f1524`). Draft by
planner-architect. Nothing here is approved. Governor instruction, 2026-10-23:
"Elixir leftovers: remove." Bridge note to follow; provisional until a
governor-reviewer record references it.

## Facts (grep of main eb70adb8, outside immutable history)

No Elixir implementation is tracked and none exists in any local worktree. What
remains is configuration, guards and names:

| Item | Where |
|---|---|
| Toolchain pin `elixir = "1.20.1-otp-29"` | `mise.toml`, `mise.lock` |
| Ignore entries | `.gitignore` (`tooling/elixir/`), `.dockerignore` (3 lines) |
| Release-payload guard script and selftest | `tooling/rust/scripts/check-no-elixir-artifacts.sh` |
| Guard invocations | `.gitlab-ci.yml` 454, 491, 528, 531, 1572, 1576; job `release:elixir-exclusion-selftest` (795-816) |
| Forbidden release entries | `agf/dist/release-manifest.yaml` 65-69 (`tooling/elixir/**`, `mix.exs`, `mix.lock`, `*.ex`, `*.exs`); prose in `agf/dist/README.md` 18-19 |
| Live-surface check parts | `check-live-tooling-surfaces.sh`: tracked-file check, `tooling/elixir` pattern, dead-parity message |
| Rust names and comments | `dist.rs` test `rejects_selected_mix_and_elixir_sources`; `coord_brief.rs` two "no retired elixir path" tests; comments in `coord_validator.rs:137` and `roi.rs:8`; test `roi.rs:1659` |
| Prose | `.gitlab-ci.yml:165` comment (mix.lock); `agf/docs/site/reference/release-artifacts.md:41` ("Elixir build directories" in the payload exclusion list) |

## Scope

Package A (inert leftovers, low risk): remove the `mise.toml` elixir pin and
refresh `mise.lock` only for that tool (no unrelated tool-version churn; the diff
of `mise.lock` is reviewed line by line); remove the Elixir entries from `.gitignore` and
`.dockerignore`; reword the Rust comments to drop the Elixir reference; rename
the `roi.rs` mixed-records test to "legacy records" (fixture data untouched);
reword the CI comment to generic wording. `agf/agents/build-engineer.yaml:89` lists `mix.exs` as one valid adopter dependency-manifest example; it stays (Elixir as an example language is out of scope, review F1), which also keeps this plan off the agent-definition red-list surface.

Package B (guards, removes a safety net): delete `check-no-elixir-artifacts.sh`,
its six CI invocations and the `release:elixir-exclusion-selftest` job; remove
the five Elixir entries from the release manifest `forbidden` list, the README
sentence, and the now-false phrase "Elixir build directories" in
`agf/docs/site/reference/release-artifacts.md:41`; in `check-live-tooling-surfaces.sh` drop the tracked-`tooling/elixir`-files check,
the `tooling/elixir` term from the scan pattern and the `bad-residue` selftest
fixture, reword "dead Elixir parity harness remains" to "dead parity harness
remains", and add selftest fixtures for the `uv run agov-` and `bin/agf` patterns
that the script scans but does not yet test; drop the two `coord_brief.rs` tests; switch
the `dist.rs` forbidden-list test to a neutral forbidden pattern so the generic
mechanism stays tested.

Out of scope: Elixir as an example language in rules and docs
(`migration-generation-rule`, `use-cases.md`, `retrospective.md`, stack
examples), immutable history, fixture data, release changelog text.

## Decision requested

Gate 1 for Package A and, separately, an explicit Governor decision on Package B:
once its guards are gone, nothing stops a future commit or release payload from
re-adding `tooling/elixir/`, `mix.*` or `*.ex[s]` files. If the Governor accepts
that, B proceeds; if not, only A ships. Both touch gate or release mechanics
(`high-risk-operations-rule`: release and publication mechanics), so Gate 1 and
Gate 2 are the Governor's; `agf/agents/*.yaml` is also red-listed.

## Classification and impact

STRICT: release mechanics, gate removal, agent definition text.
```
Impact:
  Foundry:          CI loses the Elixir exclusion selftest and six guard calls; builds unchanged.
  Direct adopters:  release-manifest `forbidden` list shrinks (no such files ship); BREAKING under Parnas (set of forbidden actions narrows), no practical effect.
  Downstream:       none; release payload membership unchanged. Source archive bytes and checksums change because `agf/dist/README.md` and the manifest ship.
  Migration:        not required; no shipped file matches the removed patterns.
  Evidence:         governor instruction 2026-10-23; issue #140; grep above.
```
Adopter docs parity: updated (`agf/dist/README.md`).

## Test strategy

1. Gate self-test (`gate-self-test-rule`, Package B modifies gates): after the
   removal, `dist.rs` forbidden-list test with a neutral pattern still fails closed
   on a forbidden file; show the transcript. `check-live-tooling-surfaces.sh`
   `selftest` still rejects its bad fixtures: the existing `mix agov` one plus the
   new `uv run agov-` and `bin/agf` ones added by this plan.
2. `cargo test` for `agov`, `agov-coord`, `agov-core`; `agov dist validate`.
   Projection manifest `docs/reviews/rule-projection/20261023T083302Z-remove-elixir-leftovers.yaml`
   (normative release guidance in `agf/dist/README.md` and the Pages page
   `release-artifacts.md`; six surfaces disposed) passes `agov plan projection-check`.
3. Pipeline: all jobs green at the exact head; the removed job no longer appears.
4. Final `git grep -i elixir` outside history lists only the out-of-scope items.

## Risks and rollback

- R1: reintroduced Elixir files go undetected. Accepted only by explicit Governor
  decision on Package B; A ships alone otherwise.
- R2: a CI rule or `needs:` reference breaks. Mitigation: `ci_needs_implication`
  test and pipeline at the exact head.
Rollback: revert the MR. No tag or release is involved.
