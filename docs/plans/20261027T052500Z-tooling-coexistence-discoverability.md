---
id: 20261027T052500Z-tooling-coexistence-discoverability
status: pending-gate1
topic: truthful-coexistence-content-and-discoverability
advisory_consultation:
  - role: infosec-sentinel
    status: pending
    reason: >-
      The plan describes third-party installers, hooks, telemetry and update paths
      as primary-document claims. Needs a non-author read that no claim implies
      verification of third-party code and that the plan introduces no credential,
      endpoint or install instruction that would be unsafe to publish.
  - role: contribution-auditor
    status: pending
    reason: >-
      Needs a non-author audit that the claim discipline below cannot be read as
      endorsement, support or integration, and that the freeze gate is objective.
affected_paths:
  - docs/plans/20261027T052500Z-tooling-coexistence-discoverability.md
  - README.md
  - agf/docs/site/docs/adopt/third-party-plugin-coexistence.md
  - agf/docs/build/mkdocs.yml
  - docs/reviews/rule-projection/20261027T052500Z-tooling-coexistence-discoverability.yaml
depends_on:
  - 20261026T151646Z-next-release-scope-intake
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: truthful coexistence content and discoverability

Authored by planner-architect (claude). Version-neutral. This plan requests Gate 1
only. It authorizes no edit to any file below, and it grants no merge, release or
version authority.

## Intent and source

The Human Governor directed (relayed by codex-foundry, room seq 746, not seen
first-hand by the author) that, for the next release once contents are stable, the
main README and the Pages site are updated with truthful content on using AGF
alongside other tooling such as Caveman and Ponytail. In a live session the Human
then chose this shape (the author's summary, recorded when this plan was revised):
do not praise other tooling directly or by name in the main text; state AGF's
position in the stack, and say that AGF is probably compatible with the reader's
stack, with some evidence behind that statement. The earlier README item about
adopting a "Ponytail discipline" is read as superseded by this shape; if the
Human meant adopting lean-change habits in AGF's own guidance, that is a separate
item, outside this plan. Codex split that directive in two: this plan owns the truthful content and
discoverability; a separate Codex-authored plan owns homepage and visual
modernization and will list this plan's ID in `depends_on`. This plan touches no
homepage, CSS, theme or layout.

## Freeze gate (objective start condition)

Implementation MUST NOT start until all of these exist:

1. a Human Governor freeze decision record on the next-release intake plan that
   names the base `main` SHA, the version, and the `IN` scope;
2. both `[Unreleased]` changelogs reconciled to that scope, with no unresolved
   release-content disposition for landed changes; and
3. a recheck, at that SHA, of every primary-document claim recorded in the
   evidence table below (versions, licenses, telemetry statements may have changed).

Drafting and review of this plan may proceed now. A changed base SHA after the freeze
record invalidates the recheck and requires it to be repeated.

## Position statement (the shape of the text)

The README and the page lead with AGF's place in the stack, without naming any
tool: AGF is the governance layer, meaning who may decide what, the gates and the
coordination record. Coding-style assistant tools work inside an approved change,
below that layer. The reader-facing sentence is of the form: "AGF is probably
compatible with your stack, because tools that work inside the approved change and
do not edit AGF's governance files have coexisted with it in our own use; some
evidence exists (link)". It is deliberately hedged. It says "no conflict was
observed in our own use", never "compatible" or "supported".

Conditions that keep it true:
1. The "some evidence" link goes to dated evidence entries that say what was
   observed, in how many repositories, and the limits. Today that is one
   repository (this one) with two tools active beside AGF and no conflict seen,
   labelled Empirically informed at most.
2. "Your stack" is scoped to tools that work inside the approved change and do not
   edit AGF's governance files. An installer that overwrites the AGF loader, or a
   hook that writes records, is outside that scope and the text says so.
3. Names appear only in the evidence entries on the coexistence page, attributed
   to the third party's own documents, dated, and labelled vendor-stated where they
   are vendor claims. They do not appear in the README or in the landing text.

## Scope (implementation, after the gate)

| Path | Change |
|---|---|
| `agf/docs/site/docs/adopt/third-party-plugin-coexistence.md` | Single canonical owner of the content. Rewrite layer-first (the position statement and its scope). Keep the Ponytail example and add a Caveman example as dated evidence entries, updated against the recorded evidence. Keep the status line honest (see claims). |
| `agf/docs/build/mkdocs.yml` | Expose the existing page in the Adopt navigation. It is currently not listed, so it is undiscoverable from the site menu. One nav line. |
| `README.md` | One short section with the generic position statement and a link to the page. It names no third-party tool, copies no rules and cites no benchmark figure. |
| `docs/reviews/rule-projection/20261027T052500Z-tooling-coexistence-discoverability.yaml` | The six-row projection screen manifest required because the page carries normative precedence guidance. |

Not included: `llms.txt` (add only if a discoverability need beyond README plus
navigation is shown during review), the Pages homepage, CSS, theme, or any visual
change; any rule, schema, template, tooling, or CI change; the page
`docs/adopt/tooling-and-tokens.md`, which describes AGF's own `agov` tooling and is
not edited here (the new text only links to it if needed).

## Claim discipline (applies to every sentence added)

- Use "coexists with" or "can be used alongside"; in the main text, "probably
  compatible, some evidence exists". Never "official integration", "supported",
  "certified", "endorsed" or "recommended by AGF", and no praise of any tool, by
  name or otherwise.
- AGF keeps authority: gates, role authority and the coordination record are
  unchanged by any plugin. Plugin output and reviews are advisory evidence only.
- Installers, hooks, MCP servers and loader edits are supply-chain surfaces and the
  text says so; each needs the adopter's own review and a human trust decision.
- No token-savings, cost, speed or quality claim, and no vendor benchmark figure
  unless it is labelled vendor-stated, attributed, and not reproduced. The author's
  own Phase A measurement (MR !3271, once merged) shows always-on instructions and
  hooks add context, so nothing here may imply AGF or a plugin saves tokens.
- Third-party claims are attributed to the third party's own documents and dated;
  they are not presented as verified, and no sentence implies the plugin code was
  audited. Names are used nominatively and respect each project's trademark terms.
- Evidence labels: Caveman stays Theoretical unless an exercised, durable record
  exists; Ponytail at most Empirically informed, and only with durable evidence
  cited; the co-use observation is one repository and is not a general claim.
- Tone follows `marketing-tone-rule.md`: no comparative or disparaging statement
  about either project or any other tool.

## Evidence inventory (primary-document claims, to be rechecked at freeze)

These were read from locally installed packages on 2026-10-26 and 2026-10-27, read
only, with nothing executed. They are claims from the vendors' own documents, not
verification. The notes are local; the durable record is the Phase A report once it
merges.

| Item | Ponytail 5.3.2 | Caveman 4.0.1 |
|---|---|---|
| Package | git sha `09c434ab…` | git sha `bad48a0e…` |
| License | MIT | Apache-2.0 since 3.0.0; MIT text kept for older parts |
| Hosts | per-host manifests for Claude Code, Codex and others, vendor-listed | manifests for Claude Code, Codex, Cursor, Gemini; vendor claims many more |
| Hooks | three Claude Code lifecycle hooks, small Node scripts | four Claude Code lifecycle hooks, Node scripts |
| Data | per-turn local ledger on by default; optional webhook off by default | vendor states skill and hooks make no service calls; CLI usage telemetry (content-free counts with IP) default-on, opt-out; account-based services separate |
| Provenance | install record carries a git sha; no signature seen | vendor states tags are not yet protected and immutable releases are off; runtime binaries use a signed manifest |
| Benchmarks | vendor-stated, not reproduced | vendor-stated, not reproduced |
| Not reviewed | hook code paths | hook code against the data statements; engine, proxy, mem, mcp, shrink, browse code |

## Projection screen (six rows)

| Surface | Disposition |
|---|---|
| Normative owner | The precedence rule already lives in the coexistence page itself and `agf/rules/` is not changed; existing pointer sufficient. Recorded in the manifest. |
| Adopter-facing explanation | `third-party-plugin-coexistence.md`, updated. |
| Runbook/situation map | Not applicable unless review finds a situation-map row needed; reasoned in the manifest. |
| AI landing/loaders | `README.md` pointer updated; `AGENTS.md` and `agf/templates/AGENTS.md` unchanged because the page already is what they would point to; reasoned in the manifest. |
| Validation/drift evidence | Not mechanically decidable for third-party claims; the independent fact-check is the evidence. `mkdocs build` and `agov validate` cover structure only. |
| Distribution | The page and nav ship in the Pages build; the README is project-local. |

## Test strategy and acceptance

- Independent fact-check by a non-author (codex-foundry) of every sentence that
  names a third party, against the evidence table at the freeze SHA, with each
  claim marked primary-document or removed.
- `mkdocs build` for the Pages site passes with the new nav line and no broken links.
- `agov validate`, `agov plan validate` and `agov plan projection-check` (strict)
  pass for the manifest; `git diff --check` is clean on the current-target merge tree.
- A reviewer greps the added text for the forbidden words in the claim discipline,
  and confirms that `README.md` and the landing text contain no third-party tool
  name (the names appear only in the evidence entries on the coexistence page).
- The "some evidence" link resolves to an evidence entry that states the number of
  repositories, the period, and the limits.

## Risks and rollback

- **Staleness:** third-party packages change quickly. Control: the freeze recheck,
  attributed and dated claims, and a review-by statement in the page.
- **Read as endorsement:** control: no names in the main text, the hedged wording,
  the claim discipline and the non-author fact-check.
- **"Probably compatible" read as a promise:** control: the scope condition on
  "your stack", the explicit statement that no conflict was observed in our own use,
  and the evidence label.
- **Hidden supply-chain advice:** control: the text names installers and hooks as
  supply-chain surfaces and tells the adopter to review and decide.
- **Scope creep into modernization:** control: the explicit exclusions above and the
  separate plan.

Rollback: revert the implementation MR; no data, schema or tag is involved.

## Classification and impact

Documentation only. No `agf/rules`, schema, template or tooling path is touched, so
the downstream-impact trigger paths are not hit.

```
Impact:
  Foundry:          README gains a short pointer; docs only.
  Direct adopters:  ADDITIVE: a discoverable, bounded coexistence page.
  Downstream:       none.
  Migration:        not required.
  Evidence:         Phase A report (MR !3271) and local read-only package notes.
```

Adopter docs parity: updated (the changed files are adopter-facing).

## Authority

Gate 1 and Gate 2 are governor-reviewer decisions. Because the text makes public
statements about third-party tools, the author recommends the Human Governor decide
Gate 1; a delegated decision by codex-foundry under the cross-approval delegation is
possible only if the Human agrees. The author does not approve this plan.
