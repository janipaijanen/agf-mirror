---
id: 20261025T192020Z-supersedes-adopter-docs
status: pending-gate1
topic: supersedes-adopter-docs
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      Requested at publication. The plan adds documentation prose only and names no credential, secret or
      cross-project link; `agov validate infosec` is mechanical evidence, not a verdict.
  - role: contribution-auditor
    status: pending
    reason: >-
      Requested at publication. An independent (non-author) review is needed: the plan adds normative wording
      to a rule (`coordination-publication-rule.md`) and must not relax or widen the six existing blocking
      conditions or the supersession authority checks.
affected_paths:
  - agf/rules/workflow/coordination-publication-rule.md
  - agf/docs/site/docs/concepts/coordination-protocol.md
  - agf/CHANGELOG.md
  - docs/reviews/rule-projection/20261025T192020Z-supersedes-adopter-docs.yaml
depends_on:
  - 20260913T195450Z-row5-s2-self-announcing-supersession
  - 20260910T065635Z-coordination-lifecycle-semantics-work-order
  - 20261025T183939Z-v211-changelog-and-scope-reconciliation
  - 20260921T040743Z-assistant-rule-drift-resolution
  - 20261021T220000Z-retire-windows-support
  - 20261023T155559Z-agov-strict-args-ref-list
  - 20261024T070435Z-timer-reality-polling-alignment
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: document the coordination `supersedes` field for adopters

Requested by codex-foundry (ACS seq 487) to resolve the `Adopter docs: GAP` finding on the `78fc52a7` changelog
entry in MR !3215. Draft by planner-architect. Nothing here is approved; no rule, page or changelog is edited
before Gate 1. Gate 1 is decided by a governor-reviewer record, not by me.

## Problem

`78fc52a7` (Row 5 S2) added the optional coordination field `supersedes` and the read-time derivation that
retires a superseded record's live-obligation status. Evidence, all on `origin/main` at `eb571217`:

- The schema field description (`agf/schemas/coordination-message.schema.json`) says the semantics are in
  `coordination-publication-rule.md`'s "lifecycle and supersession semantics".
- That rule says only that a correction is a new record that "references and supersedes the earlier one".
  It never names the field or the validity conditions. The schema description therefore points at text that
  does not exist.
- `agf/docs/site/docs/concepts/coordination-protocol.md` (line ~135) repeats the same prose, without the field.
- The conditions live only in `tooling/rust/crates/agov-coord/src/supersession.rs`
  (`is_valid_direct_edge`, `valid_superseder`), and the effect shows in `agov coord list`
  ("superseded - see <id>") and in pending-acknowledgement and handoff views.
- The real handoff pair `8e7m` and `e82e` is a case where an adopter would have wanted the field: `e82e` only
  referenced `8e7m`, so `8e7m` stayed a live obligation.

An adopter cannot learn from shipped documentation that the field exists, when it takes effect, or why an
invalid edge is silently ignored.

## Scope and changes (documentation only, after Gate 1, one implementation MR)

1. **Normative owner, `agf/rules/workflow/coordination-publication-rule.md`**: add one short subsection under
   "Lifecycle and supersession semantics" that names the field and states the contract in words (no code):
   - A later record MAY declare `supersedes: [ids]`; each id MUST also appear in that record's `references-to`.
   - An edge is honored only if all hold: the target exists; the superseder is strictly later (compared as
     instants); it is not the same record; it has the same `to` audience; its `from` equals the target's `from`
     or is `governor-reviewer`.
   - Chains resolve to the current leaf. A correction must name the leaf, not the original. Among several valid
     candidates the earliest wins, then a `governor-reviewer` record, then the smallest id.
   - An invalid or absent edge is ignored without error: the earlier record simply stays live. The field never
     changes the earlier record, never approves or completes anything, and grants no authority.
2. **Adopter explanation, `agf/docs/site/docs/concepts/coordination-protocol.md`**: replace the single prose
   bullet's wording with one that names `supersedes`, gives a four-line example (a head-change correction that
   both cites and supersedes the earlier handoff), says where the effect shows (`agov coord list`, pending and
   handoff views), and points to the rule as owner. It restates no condition list beyond one sentence.
3. **Changelog, `agf/CHANGELOG.md`**: after MR !3215 merges, change the `78fc52a7` entry's `Adopter docs: GAP`
   to `Adopter docs: updated` with the two paths. This plan edits no changelog text while !3215 is open.

The schema file is **not** changed (it is a high-risk surface). No tooling, test or code changes.

## Classification and impact

Documentation of existing behavior; the observable contract (what the tooling requires, permits, forbids) is
unchanged, so it is EDITORIAL under the Parnas boundary. Impact: Foundry: none. Direct adopters: they gain the
explanation; nothing to do. Downstream: none. Migration: none. Evidence: the `Adopter docs: GAP` finding in
MR !3215 and handoff `20261025T191011Z-planner-architect-k6sp`.
Adopter docs parity: updated (the plan's purpose).

## Rule-projection screen (`documentation-rule-projection-rule.md`)

The plan adds normative wording to a rule and to an adopter page, so the six-row screen applies. The manifest
is `docs/reviews/rule-projection/20261025T192020Z-supersedes-adopter-docs.yaml`: owner = the rule (updated);
explanation = `coordination-protocol.md` (updated); runbook = not applicable (no runbook step covers record
correction, and adding one would be a second statement); AI loaders = existing pointer sufficient
(`agf/templates/AGENTS.md` already points to the rule); validation = advisory only; distribution = all
changed files ship under `agf/`.

## Tests and acceptance

- `agov plan validate` on this plan; `agov plan projection-check --allow-planned` at Gate 1, and the strict
  `agov plan projection-check` (no flag) against the final manifest before Gate 2.
- Fidelity check by a reviewer: each condition in the new rule text matches `is_valid_direct_edge` and
  `valid_superseder` in `supersession.rs` (five conditions, chain-to-leaf, three-step tie-break). A claim that
  the code does not support is a blocking finding.
- The `agov validate adopter-docs` run passes; `site-html-build` link checks pass; the new text states the
  conditions once (rule) and the page only summarises and links.
- A worked check: the `8e7m`/`e82e` pair, rewritten with `supersedes`, meets all five conditions (later,
  different record, same `to`, same `from`, cited in `references-to`). Recorded in the MR, not committed.

## Collision and sequencing disposition

Scan at 2026-10-25T19:28Z: no open MR changes the rule, the Pages page, the schema or the loaders. Declared
`affected_paths` overlap only the approved plan `20261025T183939Z-v211-changelog-and-scope-reconciliation`
on `agf/CHANGELOG.md`; it is acknowledged in `depends_on`, and the implementation must merge after !3215. The
parents `20260913T195450Z-row5-s2-self-announcing-supersession` and
`20260910T065635Z-coordination-lifecycle-semantics-work-order` are cited, not edited.

CI `agov:governance:plan-collision-gate` (pipeline 2987029092, head d2563a4a) reported four more plan overlaps
that the first draft did not acknowledge. They are now listed in `depends_on` as deliberate and sequenced:
`20260921T040743Z-assistant-rule-drift-resolution` (glob `agf/rules/**` vs the rule file),
`20261021T220000Z-retire-windows-support` (`agf/CHANGELOG.md`), and
`20261023T155559Z-agov-strict-args-ref-list` and `20261024T070435Z-timer-reality-polling-alignment`
(`coordination-protocol.md`). All four are approved plans whose implementations are already on `main`; this
plan edits those files only after Gate 1 and does not reopen them. My earlier local check missed them because
it ran the main checkout's release binary of 2026-10-22; CI's current-build result is the authority.

## Risks and rollback

Risk: the new rule wording drifts from the code. Mitigation: the fidelity check above; the rule describes the
contract, not the implementation. Risk: it reads as new authority. Mitigation: the text states the field grants
none. Rollback: revert the implementation MR; no code, schema or data moves.

## Explicitly out of scope

Schema, tooling and test changes; a waiver; the `learning-record` `supersedes` field (a different field);
`VERSION`, release, tag, deploy; MR !3215 content.
