---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260908T041457Z-contribution-auditor-hg52
timestamp: 2026-09-08T04:14:57Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T200210Z-coder-tester-8j2h.md
  - 20260907T192603Z-planner-architect-rkp3.md
  - 20260907T191930Z-governor-reviewer-qqbh.md
  - 20260907T172745Z-issue120-standard-path-namespace-absence.md
topic: ruw4-namespace-implementation-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent contribution audit of MR !2539 (RUW-4 implementation)

Scope: all 13 changed files at exact head
`0b71e17900c5b8555c1c2803837fb04f8e72062b`, against `origin/main`
(`d1f0094b`). Non-author contribution-auditor lens, routed by the governor
per `8j2h`.

**Independence disclosure:** this auditing Claude session **authored the
approved child plan** (`3g2g` / `qk8p` / `gnqd`) that MR !2539 implements. I
did **not** author the implementation — Codex tasks `01a07d2f` (primary) and
`01a06b4e` (auth-phase correction) did. The plan itself received three
independent non-author CLEAR reviews (`gpm5` / `5bqa` / `rm1g`). The governor
should weigh whether a reviewer independent of *both* the plan and the
implementation is preferable; this review is offered on that disclosed basis.

Coverage: COMPLETE
Recommendation: **MODIFY** — 1 medium finding to resolve before Gate 2, 1
low advisory. No blocking correctness, security, or scope-integrity defect.

## What is clean

| Dimension | Assessment |
|---|---|
| Faithful implementation of the approved plan | Yes. Four-namespace contract, fail-closed semantics, positive-authenticated-access-before-a-404-is-absence, runtime-derived identity (`$CI_PROJECT_ID` + canonical `path_with_namespace` verification; package name from `agf/dist/release-manifest.yaml`; OCI path/tag from `agov-oci-image.sh image-ref` + `verify-ref` + `$CI_REGISTRY_IMAGE`), both mandatory operator runs (`--phase preflight` / `--phase pre-push`), detached-MR SHA binding (not a future merge commit), post-inspection recheck of target + checkout. |
| Governance relaxation | None — this adds a gate. Blocking authority for `release:namespace-check` is explicitly granted by a named clause in `release-via-merge-request-rule.md` (`tool-authority-boundary-rule` satisfied). |
| Plausibility | The failure mode is the v2.8.0 `q5mm` incident exactly. Sound. |
| Rule soundness | Mechanical enforcement (`allow_failure: false`) + mandatory operator runs + `gate-self-test` negative evidence. Exceptions explicitly preserved ("does not replace the stricter exception checklists"). The check-to-push race is acknowledged, not papered over. |
| CI job | `release:namespace-check`: `alpine:3.22` pinned, `allow_failure: false`, `needs: rust:tooling:preview` (no second build), `rules:` = `merge_request_event` + `changes: agf/VERSION`, non-DinD, `JOB-TOKEN: $CI_JOB_TOKEN` only, no shell tracing (`set +x`). Matches the plan. |
| Scope creep | None. The `roi.rs` `COMMAND_PATH_TOKENS`/`KNOWN_FLAG_NAMES` additions, the `ci-needs/mandatory-edges.tsv` edge, and the `ci_needs_implication.rs` / `gate_change_check.rs` cases are **required accompaniment** — the existing CLI-parity test, CI-topology checker, and gate-change-check would fail without them. Verified, not unrelated refactoring. |
| Dependencies | No `Cargo.toml` / `Cargo.lock` change — `no-auto-package-update` satisfied. |
| Six-surface projection | Manifest updated (`validation_drift_evidence` gains `paths`); strict `agov plan projection-check --base-ref origin/main` (no `--allow-planned`) passes. |
| `gate-self-test-rule` | MR description `## Negative test` has a reproducible transcript: occupied git-tag / release / generic-package(pending_destruction) / oci → BLOCKED, `publication_invoked=false`; wrong-project → identity mismatch; hidden-404 → ambiguity; positive absent / fuzzy-package / no-oci-repository → ABSENT. Command given. Embedded + integration tests cover pagination loops, duplicate repos, malformed JSON / redirects without echoing bodies, credential/origin injection, dirty pre-push tree, CI/operator credential separation. |
| Scope exclusions | Confirmed: no `agf/VERSION`, no `CHANGELOG`, no tag/release-metadata in the 13-file diff. |
| Security | Read-only (GET only), token via `curl --config -` on stdin (never argv), `--proto =https`, timeouts + filesize cap + request budget, stderr suppressed, errors never render HTTP bodies or credentials, non-200/404 always fails. |

`agov validate contributions` OK (the two WARNs are the pre-existing
unchanged `agf/learnings/foundry.yaml` baseline, not this contribution).
Canonical MR pipeline `2827510732` is green at this head.

## RUW4-IMPL-1 — medium — `downstream-impact-rule` Impact statement missing

Basis: `always/downstream-impact-rule.md`. The commit changes
`agf/rules/workflow/release-via-merge-request-rule.md`, `.gitlab-ci.yml`,
and `tooling/**` — all trigger paths — and **restricts previously-permitted
behavior**: a standard-path release could previously proceed with no
required namespace-absence gate; now it cannot. Per the rule that is a
**BREAKING** classification (PREVENTIVE "unless it restricts
previously-permitted behavior (then BREAKING)").

The commit message (`feat: enforce standard-path release namespace
absence`, 4 lines) carries no three-circle Impact block (Foundry / Direct
adopters / Downstream / Migration / Evidence), and the MR adds no
`agf/CHANGELOG.md` entry. The substance (rollback, risks, evidence = issue
#120 + the v2.8.0 `q5mm`/`gs18` incident) lives in the approved plan, but
the rule wants it in the commit message, a plan file *for this change*, or
`CHANGELOG.md`.

Resolution before Gate 2 (documentation only, no code change): add the
Impact block to the commit message, **or** add an `agf/CHANGELOG.md` entry
under a `Changed` (breaking) heading, stating that adopters on the standard
release workflow gain a new mandatory pre-tag four-namespace gate
(`release:namespace-check` + two operator runs), citing issue #120 and the
v2.8.0 evidence, and noting why no deprecation cycle applies
(`deprecation-policy-rule` Phase 0, pre-customer).

## RUW4-IMPL-2 — low — advisory — skill preflight example is CI-centric

`agf/skills/release/SKILL.md` shows the operator preflight as
`agov release namespace-check --phase preflight --api-url "$CI_API_V4_URL"
--project-id "$CI_PROJECT_ID" …`. A pure-local operator run has those
`CI_*` vars unset. The operator page text is correct ("Use the configured
release project's ID/path … not a registry repository ID"), but the skill's
literal `$CI_*` placeholders could mislead an operator into thinking the
command self-configures locally. Consider explicit placeholder values or a
one-line "export these from the release project first" note.

## Disposition

Faithful, well-tested, security-conscious implementation of an approved
plan. **MODIFY**: resolve RUW4-IMPL-1 (Impact statement) before Gate 2;
RUW4-IMPL-2 is advisory. Neither requires a re-review of the command logic.
Build-engineer and infosec exact-head lenses remain separate prerequisites;
Gate 2 is the governor's.
