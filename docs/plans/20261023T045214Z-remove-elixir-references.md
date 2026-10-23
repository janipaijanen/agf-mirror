---
id: 20261023T045214Z-remove-elixir-references
status: pending-gate1
topic: remove-retired-elixir-references-from-guides
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >
      Review requested. No credentials, learning records or security boundary
      are touched; the one gate-script change keeps its check and inverts it.
  - role: contribution-auditor
    status: pending
    reason: Review requested before Gate 1.
affected_paths:
  - AGENTS.md
  - docs/reference/foundry-polling-and-bridge-procedure.md
  - docs/reference/foundry-agents-md-moved-text.md
  - tooling/rust/scripts/check-live-tooling-surfaces.sh
  - agf/docs/site/docs/adopt/upgrade-from-v1.md
  - agf/docs/site/reference/release-artifacts.md
  - agf/docs/adopt/decommission.md
  - agf/docs/governance/learning-system.md
  - agf/docs/adr/20260505-lean-agile-test-fly.md
depends_on:
  - 20261022T205113Z-instruction-context-reduction
  - 20260913T085123Z-work-item-steward-gate1
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260915T075705Z-issue130-pending-advisory-consultation-status
  - 20260921T040743Z-assistant-rule-drift-resolution
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: remove retired Elixir references from the guides

Tracks issue #139. Draft by planner-architect. Nothing here is approved.

## Decision requested

Gate 1 approval to remove prose that describes the retired Elixir
implementation from foundry and shipped guides, and to invert one CI gate so it
no longer requires such a sentence. The governor stated on 2026-10-23: "remove
the elixir stuff from the guides. it was historical, but now git history holds
it", and chose the scope "foundry guides plus shipped agf docs". Bridge note to
follow; provisional until a governor-reviewer record references it.

## Facts (grep of main, 2026-10-23)

- `AGENTS.md`: tree line `elixir/ # Reference/development implementation and
  parity oracle` (stale: it says the opposite of the residue paragraph), one
  "do not fall back to Elixir/Mix" clause, and the paragraph "No Elixir
  implementation is tracked. An ignored `tooling/elixir/` directory ...".
  Phase 1 of issue #138 (MR !3126) moves two of these into
  `docs/reference/`; this plan depends on that plan's text.
- `tooling/rust/scripts/check-live-tooling-surfaces.sh` lines 50-58 require
  exactly one `AGENTS.md` line containing the ignored-directory wording. This
  blocks removal until the gate changes. That is gate mechanics
  (`gate-self-test-rule`, STRICT).
- Shipped guides: `upgrade-from-v1.md` (7 mentions), `release-artifacts.md`
  (3), `decommission.md` (1), `learning-system.md` (1), the ADR below (1),
  `use-cases.md` (1, kept), `retrospective.md` (1, kept).

## Scope

In:
1. `AGENTS.md` and the two moved reference docs: delete the Elixir tree line,
   the "Elixir/Mix" clause (keep "report the gap"), and the residue paragraph.
2. `check-live-tooling-surfaces.sh`: replace the "exactly one residue line"
   rule with "no `tooling/elixir` mention in `AGENTS.md`". Existing checks on
   tracked files and `mix agov` patterns stay.
3. Shipped guide prose that describes Elixir tooling as a current or fallback
   option. Per file: `release-artifacts.md` drop the "no Elixir/ERTS package"
   statements or reduce to the Rust-only fact; `decommission.md` drop the
   legacy tooling cell; `learning-system.md` drop the E4 Elixir deferral note;
   one sentence in `agf/docs/adr/20260505-lean-agile-test-fly.md` (D4) that
   falsely says current validator guidance lives in Elixir Mix tasks under
   `tooling/elixir/`: correct it to the Rust `agov` validators, nothing else in
   that ADR changes.

Intent reading (governor to correct if wrong): "Elixir stuff" means the retired
AGF tooling, not Elixir as an example language. So `use-cases.md` ("across
Elixir and Python") and `retrospective.md` (an adopter's historical stack
evidence) stay as kept residuals below.

Out (kept on purpose):
- `check-no-elixir-artifacts.sh`, the `release:elixir-exclusion-selftest` job,
  `agf/dist/release-manifest.yaml` forbidden entries, `.gitignore`,
  `.dockerignore`: these are release-payload and checkout guards, not guide text.
  Removing them is a separate gate change.
- Stack examples in rules and templates (`migration-generation-rule`,
  `testing-environments-rule`, `coord-gate2-handoff` `mix test`): Elixir as an
  example language, not the retired tooling.
- `mise.toml` / `mise.lock` toolchain pin: build config, separate decision.
- Coordination records, plans, learnings, audits, proposals, CHANGELOG: immutable
  or historical.

Expected residual `git grep -i elixir` hits after the change, each deliberate:
the release-payload guard sentences in `release-artifacts.md`
that name what the payload must not contain, `use-cases.md` and
`retrospective.md` (language/stack evidence, not tooling), and the guard
scripts and manifests listed above.

## Governor decision on `upgrade-from-v1.md` (live, 2026-10-23)

The governor chose option (b): "Delete every elixir/mix mention." Every
Elixir/Mix mention is removed from `upgrade-from-v1.md`, including the `mix
agov.*` command names; the page says only "v1 commands" where it must refer to
them. Bridge note `20261023T061220Z-planner-architect-2gpd` (provisional until a
governor-reviewer record references it). The earlier recommendation to keep
those names as search targets is withdrawn.

## Classification and impact

STRICT: gate mechanics, shipped docs, `AGENTS.md` loader.
```
Impact:
  Foundry:          AGENTS.md and the live-tooling gate change; CI stays green by test.
  Direct adopters:  EDITORIAL for docs that only drop retired-tooling prose.
  Downstream:       none; no command, schema or artifact shape changes.
  Migration:        not required.
  Evidence:         governor instruction 2026-10-23; issue #139.
```
Adopter docs parity: updated (shipped guides edited in the same MR).

## Test strategy

1. Gate self-test (`gate-self-test-rule`): run `check-live-tooling-surfaces.sh
   check` on a scratch tree where `AGENTS.md` still names `tooling/elixir`:
   it must exit 1. Re-run on the edited tree: exit 0. Paste both transcripts in
   the MR `## Negative test` section.
2. `check-live-tooling-surfaces.sh selftest` extended with that fixture.
3. `git grep -i elixir -- AGENTS.md agf/docs agf/templates docs/reference`
   lists only the expected residuals above; the MR pastes the output.
4. `agov validate adoption`, `agov plan projection-check-all`, pipeline green
   at the exact head.

## Risks and rollback

- R1: a hidden consumer of the AGENTS.md residue line. Mitigation: grep of
  scripts and CI shows only the one script.
- R2: over-deleting a v1-upgrade hint. Mitigation: the open question above.
Rollback: revert the MR. No tag or release is involved.
