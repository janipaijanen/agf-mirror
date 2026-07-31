# v2.4.9 tooling-reference classification

Observed against `main` after merged Gate 1 plan !863. This inventory applies
the classification matrix in `docs/plans/v249-live-tooling-surface-cleanup.md`.

| Reference group | Classification | Action and evidence |
|---|---|---|
| `bin/agf` | Broken supported-looking executable | Deleted. It invoked only the absent Mix project; `tooling/agov` remains the single repository wrapper. |
| `tooling/rust/tests/parity/run.sh` | Dead executable test helper | Deleted. It required the removed Elixir oracle and fixtures and had no live CI reference. |
| AGENTS.md | Current operator guidance | Corrected to state that no Elixir implementation is tracked; one explicit ignored-residue reference remains. |
| Current GitLab MR templates | Current contributor guidance | Replaced Mix tasks with exact Rust `agov validate` commands. |
| Foundry `.claude/rules/` copies | Current local assistant guidance | Corrected the two remaining legacy Python CLI examples; unrelated drift remains governed by the existing assistant-drift advisory. |
| `agf/framework.md`, governance manifest, contract docs, decommission guide, gate-self-test rule | Current adopter/framework guidance | Replaced AGF Mix commands and removed claims that the deleted implementation is current. Generic removal language remains technology-neutral. |
| `agf/docs/site/roadmap.md`, `docs/TODO.md` | Current project guidance | Updated implementation status and Rust tooling terminology. |
| `agf/docs/site/docs/adopt/upgrade-from-v1.md` | Explicit legacy migration guidance | Preserved. Its purpose is to identify and remove v1 Elixir/Python syntax. |
| `agf/CHANGELOG.md`, `agf/docs/adr/**`, `agf/learnings/**` | Historical evidence | Preserved verbatim. |
| `docs/plans/**`, `docs/coordination/**`, `docs/academic/**`, post-v2.4.8 disposition ledger | Historical/governed evidence | Preserved verbatim; generated academic build artifacts describe their dated snapshot. |
| `docs/audits/**` and `docs/proposals/**` | Dated audit/proposal evidence | Preserved verbatim; these describe superseded implementation directions rather than current operator instructions. |
| `agents-govern/**` | Bootstrap reference | Preserved verbatim as required by AGENTS.md. It is the historical 1.1.50 governance distribution, not the AGF-NG product or operator path. |
| `agf/ADOPTING.md`, `agf/templates/coord-gate2-handoff.md` | Generic adopter stack examples | Preserved where `mix` refers to an adopter's own Elixir/Ecto tests, not AGF tooling. |
| `.gitignore`, `.dockerignore`, `.gitlab-ci.yml`, `agf/dist/README.md`, `agf/dist/release-manifest.yaml`, `check-no-elixir-artifacts.sh` | Local-residue, regression, and release exclusions | Preserved or extended. Negative paths and fixtures intentionally contain forbidden artifact names. |
| Rust comments, tests, fixtures, and token-ROI snapshots | Implementation history/test evidence | Preserved when the text is not presented as a supported operator command. |

`tooling/rust/scripts/check-live-tooling-surfaces.sh` enforces the current live
surface allowlist, absence of the two dead executables and tracked Elixir
sources, and the single unambiguous AGENTS.md residue reference. Its self-test
proves that supported Rust syntax passes and stale Mix syntax fails.

The final tracked-file scan classified 1,433 matching lines: 1,411 governed
historical/bootstrap lines, 21 protective or test-evidence lines, and the one
explicit AGENTS.md local-residue line. No match remained unclassified.
