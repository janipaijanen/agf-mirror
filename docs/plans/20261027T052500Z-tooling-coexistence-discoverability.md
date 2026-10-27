---
id: 20261027T052500Z-tooling-coexistence-discoverability
status: pending-gate1
gate1_approved_by: 20261027T152926Z-governor-reviewer-p47g
gate1_approved_at: "2026-10-27T15:29:26Z"
topic: truthful-coexistence-content-and-discoverability
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Non-author review at exact content head
      ae2e8597e0455fec2b37d4f63964508ff7b4a4fb (room seq 818): CLEAR. The corrected
      consent and opt-out boundaries match the tagged primary documents; no secret,
      private endpoint, executable-install instruction or unsafe command was added.
      A fresh exact-head implementation read is still required before merge.
  - role: contribution-auditor
    status: consulted
    reason: >-
      Non-author review at exact content head
      ae2e8597e0455fec2b37d4f63964508ff7b4a4fb (room seq 818): CLEAR. The material
      amendment corrects the evidence inventory and claim boundaries without adding
      unsound authority, endorsement or precedent claims.
affected_paths:
  - docs/plans/20261027T052500Z-tooling-coexistence-discoverability.md
  - README.md
  - agf/docs/site/docs/adopt/third-party-plugin-coexistence.md
  - agf/docs/build/mkdocs.yml
  - docs/reviews/rule-projection/20261027T052500Z-tooling-coexistence-discoverability.yaml
depends_on:
  - 20261026T151646Z-next-release-scope-intake
  - 20260921T040743Z-assistant-rule-drift-resolution
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
stack, with some evidence behind that statement. Review found that wording
self-contradictory as first drafted; the permitted forms are now exact (below). The earlier README item about
adopting a "Ponytail discipline" is read as superseded by this shape; if the
Human meant adopting lean-change habits in AGF's own guidance, that is a separate
item, outside this plan. Codex split that directive in two: this plan owns the truthful content and
discoverability; a separate Codex-authored plan owns homepage and visual
modernization and will list this plan's ID in `depends_on`. This plan touches no
homepage, CSS, theme or layout.

## Freeze gate (objective start condition)

Implementation MUST NOT start until all of these exist:

1. a Human Governor freeze decision record on the next-release intake plan that
   names the base `main` SHA, the version, and the `IN` scope, and that explicitly
   admits this plan (`20261027T052500Z-tooling-coexistence-discoverability`) either as
   `IN` or as a named, controlled post-freeze documentation slice. This is
   discretionary public-documentation work, not reconciliation of already-frozen
   product facts, so a dependency alone must not silently change frozen scope. The
   current intake plan has `IN` empty and does not name this plan; the next reviewed
   intake refresh must add this plan and the Pages modernization plan before freeze;
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
below that layer.

Permitted wording (exact; nothing else may state or imply compatibility):
- "AGF is designed to sit above tools that work inside the approved change and do
  not edit AGF's governance files."
- "AGF is probably compatible with tools of that kind; some evidence exists (link),
  and it is limited."

Forbidden: any unqualified claim of compatibility, support, certification,
integration or endorsement, and any sentence that says a named tool "works with AGF".
The first permitted sentence is a statement of design intent; the second is a hedged
inference, and both carry the scope condition below.

Conditions that keep it true:
1. The "some evidence" link goes to dated evidence entries that say what was
   observed, over what period, which surface was exercised, in how many
   repositories, and the limits. Only durable records count (see the evidence
   section). The page status line says there is no adopter evidence and only
   bounded foundry evidence.
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
  own Phase A measurement (report on `main` at `9ef652c7e5b1a29f2841b2d26b4fcc9c07b7252f`)
  reports material context cost in the tested Claude and foundry condition and
  identifies user-level hooks as a confound; it does not isolate plugin cost. Nothing
  here may imply AGF or a plugin saves tokens.
- Third-party claims are attributed to the third party's own documents and dated;
  they are not presented as verified, and no sentence implies the plugin code was
  audited. Names are used nominatively and respect each project's trademark terms.
- Evidence labels classify AGF's own coexistence inference, never a third party's
  facts. Licenses, manifests, hooks and telemetry statements are primary-document
  facts, attributed and dated, and are not labelled Theoretical or Empirically
  informed. The inference that AGF coexists with a given tool is Theoretical unless
  an exercised, durable record exists for that tool; observed co-use of Ponytail may
  be Empirically informed, and only with a durable record cited. A claim that rests
  on an unmerged record (for example MR !3271) is omitted until that record lands.
- Tone follows `marketing-tone-rule.md`: no comparative or disparaging statement
  about either project or any other tool.

## Durable evidence for the inference

- **Caveman:** the durable records are the two foundry experiments
  `20261023T051418Z-coder-tester-47qq` and `20261023T103235Z-coder-tester-e1kp`. They
  rewrote local copies of the AGF rule text in a telegraph style and measured token
  and fidelity effects; they concluded no adoption recommendation. They did not run
  the Caveman plugin beside AGF, so they are not evidence that the plugin coexists
  with AGF, and the page must not present them as such. For the plugin, the
  coexistence inference stays Theoretical unless a new durable record of exercised use
  is made before freeze.
- **Ponytail:** the author's Phase A report, now on `main` at
  `9ef652c7e5b1a29f2841b2d26b4fcc9c07b7252f`, is the durable record of the read-only
  package inspection (SDD-SUPPLY-1). It proves package inspection, not generalized
  coexistence; the page may cite it for that only. Co-use in the
  foundry's own agent sessions is an observation by the author and the Human, not yet
  a durable record, and is not claimed until it is.

## Evidence inventory (primary-document claims, to be rechecked at freeze)

Amended 2026-10-27. Codex's row-by-row recheck (room seq 810 and 811) against exact
tagged commits replaced the earlier inventory, which cited moving `main` SHAs and a
local install record. The claims below are the recheck's corrections; the author has
not independently re-verified them, and the recheck itself is cited, not reproduced here.
Each claim must be cited at freeze with its exact immutable-tag URL, the full commit SHA
and the access date (2026-10-27 for this recheck). Vendor claims are attributed to the
vendor's own documents and are not evidence labels.

Exact sources (immutable commits):
- Ponytail 5.3.2: commit `16352ac229eb08a9598dcae48fe9f2b342567d86`
  (`https://github.com/DietrichGebert/ponytail/commit/16352ac229eb08a9598dcae48fe9f2b342567d86`).
- Caveman 4.0.1: peeled commit `d262ab3882952a59312ff3537cd86d894523c5ab`
  (`https://github.com/JuliusBrussee/caveman/commit/d262ab3882952a59312ff3537cd86d894523c5ab`).

| Item | Ponytail 5.3.2 | Caveman 4.0.1 |
|---|---|---|
| License | MIT (repository license at the tagged commit) | Apache-2.0 for the whole repository from 3.0.0; the MIT text is kept only as a historical notice for older parts |
| Claude Code hooks | three manifest event groups (SessionStart, SubagentStart, UserPromptSubmit). The vendor prose calls them two lifecycle hooks; this inconsistency is stated, not resolved | four manifest event groups (SessionStart, SubagentStart, UserPromptSubmit, SessionEnd) |
| Local data | a per-turn local task ledger is on by default; the optional webhook is empty by default; task-name sending defaults on when a webhook is configured | the skill runs locally except compress; native SessionStart can emit telemetry after an interactive `yes` was persisted. Separately, the default CLI telemetry can send install and account data, locale and timezone, session, command and outcome, token, proxy and history data, MCP-tool metadata and the IP address; account and cloud paths are separate |
| Provenance | none asserted beyond the tagged commit. The earlier install-record SHA and no-signature claim is removed because it has no tagged primary support | the vendor's SECURITY.md at the tagged commit says git tags are not protected and immutable releases are not enabled, and that runtime binaries are checked against a signed manifest. That is a tagged-document snapshot, not the current repository setting and not a verified operation |
| Benchmarks | vendor-reported; not reproduced; not cited as evidence | vendor-reported; not reproduced; not cited as evidence |
| Not reviewed | hook code paths, and the code behind the data statements | the engine, proxy, mem, mcp, shrink and browse code, and the hook code behind the data statements |

Telemetry wording: the vendor's SECURITY.md at the tagged commit describes broader
opt-in and default telemetry than the counts-plus-IP summary the earlier draft used. The
page must quote the tagged wording, not a summary of it.

## Projection screen (six rows)

| Surface | Disposition |
|---|---|
| Normative owner | Unchanged: authority stays with `agf/agents/MANIFEST.yaml`, gate discipline in `AGENTS.md`, and `agf/rules/always/tool-authority-boundary-rule.md`. The page restates and explains them and must not become a second owner, and no rule file is changed. Recorded in the manifest. |
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
- A fresh exact-head infosec read of the final implementation, covering every
  telemetry, network, installer and hook statement, before the implementation MR
  merges. The plan-stage consultation does not stand in for it.
- A reviewer greps the added text for the forbidden words in the claim discipline,
  and confirms that `README.md` and the landing text contain no third-party tool
  name (the names appear only in the evidence entries on the coexistence page).
- The "some evidence" link resolves to an evidence entry that states the number of
  repositories, the period, and the limits.

## Collision disposition

The collision gate reports one overlap on `README.md` with the approved plan
`20260921T040743Z-assistant-rule-drift-resolution`, now acknowledged in `depends_on`.
That plan is long approved and its work is not part of this plan. Before
implementation the owner MUST fetch current refs, confirm whether that plan still has
unmerged work on `README.md`, and rebase onto its merged result; a conflicting
counterpart is a stop for coordination, not permission to overwrite. This
acknowledgment approves nothing.

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

The Human's earlier conditional Gate 1 approval was for head 686f06bf and did not carry
to this revised head; this head needs a new binding Gate 1 decision. Gate 1 and Gate 2
are governor-reviewer decisions. Because the text makes public
statements about third-party tools, the author recommends the Human Governor decide
Gate 1; a delegated decision by codex-foundry under the cross-approval delegation is
possible only if the Human agrees. The author does not approve this plan.
