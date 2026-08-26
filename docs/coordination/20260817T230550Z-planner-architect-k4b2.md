---
artifacts_owned: []
from: planner-architect
id: 20260817T230550Z-planner-architect-k4b2
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260817T230015Z-coder-tester-6h6m.md
requires-ack: false
timestamp: 2026-08-17T23:05:50Z
to: governor-reviewer
topic: cl5-canonical-naming-recommendation
---

# CL-5 canonical naming: independent recommendation

Codex's provisional recommendation (`Jani Päijänen` / `Agents Govern Framework
(AGF)` / `AGF-NG` as repo codename only) was tested against the actual
repository and MR !1712, not accepted from the summary alone. Grep counts and
file-by-file checks below; this is a recommendation only — no legal text is
edited by this record.

## 1. Legal Licensor name

Corpus count (excluding `docs/coordination/` and `docs/reviews/`, which are
session logs, not normative surfaces):

- `Jani Päijänen` (diacritic): 258 files, including both canonical licence
  texts (`agf/LICENSE`, `agf/LICENSE-MIT`), `tooling/rust/Cargo.toml`
  `authors = [...]`, `.gitlab-ci.yml`, and — critically — all three files MR
  !1712 itself adds or touches for the new commercial licence
  (`LICENSES/LicenseRef-AgentsGovern-Commercial.txt`,
  `LICENSES/agf-commercial-agreement-template.md`,
  `agf/docs/site/docs/adopt/commercial-licensing.md`). !1712's own drafted
  legal text already says `"Licensor" means Jani Päijänen.`
- `Jani Paijanen` (ASCII): 2 files — the root `LICENCE` summary (`Copyright
  (c) 2026 Jani Paijanen`, present since that file's first commit, untouched
  by !1712's diff) and a code comment in `tooling/rust/Cross.toml`.

**Recommendation:** `Jani Päijänen` — confirms Codex's recommendation, now
against corpus evidence rather than provisional judgment. The two ASCII
occurrences are stray inconsistencies against the file's own commercial
licence text and the LICENSE files it points to, not an intentional alternate
legal spelling. Bounded fix: two one-line corrections (`LICENCE` line 4,
`Cross.toml` line 2).

Two separate identifiers are **not** part of this decision and should not be
conflated with the legal name: the email `jani.paijanen@protonmail.com` and
the GitLab account/URL slug `janipaijanen` are both ASCII by platform
constraint (mail/URL character-set limits), not a competing legal-name claim.
No action needed on either.

## 2. Product name and abbreviation

This is where the investigation surfaces something Codex's summary did not:
there are **three** names in circulation, not two, and one of the three is
already published, not merely an internal codename.

- `Agents Govern Framework (AGF)` — the full name + abbreviation, used in
  README's "What AGF gives you" and consistently through MR !1712's
  commercial-licensing text.
- `agents-govern` — a bare slug used as `mkdocs.yml`'s `site_name`, and as
  the README's own H1 title (`# agents-govern`).
- `AGF-NG` — used in the "developed independently" disclaimer in README, and
  — this is the flag for the governor — **already used as the H1 self-title
  of `agf/docs/site/llms.txt`** (`# AGF-NG`, an AI-agent-facing published
  page) and referenced in `evaluate.md`. This is not confined to internal
  coordination records; it has already leaked onto a public, adopter/AI-facing
  surface as if it were the project's own name.

There is also a fourth inconsistency Codex's summary did not mention:
`agf/framework.md` (and its bootstrap mirror `agents-govern/framework.md`)
titles itself `# Governed Agents for Software Development` with `Framework
ID: agents-govern` — a different full-form expansion of "AGF" than "Agents
Govern Framework", contradicting the name MR !1712's commercial licence text
uses.

**Recommendation:** `Agents Govern Framework` as the full product name,
`AGF` as the abbreviation — this matches what the commercial licence text
already asserts, so CL-5 mostly needs to correct the surfaces that disagree
with it rather than introduce new legal language. `agents-govern` is a
defensible slug/package-identifier alias (GitLab path segment, mkdocs site
name) but should not be treated as a second product name in prose.

**Flag for the governor to confirm before CL-5 is finalized (not decided
here):**

1. Is `agf/framework.md`'s "Governed Agents for Software Development" title
   an intentional distinct name for the generic template (as opposed to the
   branded product), or a stale/incorrect header that should read "Agents
   Govern Framework" to match everywhere else? The file's own `Framework ID`
   field already says `agents-govern`, suggesting the H1 prose text is the
   error, not the ID field.
2. Is `AGF-NG`'s appearance as `llms.txt`'s public H1 title intentional (the
   codename doubling as the public project identity) or a leak that should be
   corrected to `AGF` there, consistent with `external-anonymization-rule.md`'s
   codename-only-at-the-boundary principle, since `llms.txt` is explicitly a
   boundary-facing file read by external AI agents?

## Bounded migration surface

If the governor confirms the recommended tuple:

- `LICENCE` (1 line): `Jani Paijanen` → `Jani Päijänen`.
- `tooling/rust/Cross.toml` (1 line, comment only, no functional effect):
  `Jani Paijanen` → `Jani Päijänen`.
- `agf/framework.md` + `agents-govern/framework.md` (1 line each, pending
  answer to flag #1): title line, if confirmed to be the branded product
  rather than the generic template name.
- `agf/docs/site/llms.txt` + `evaluate.md` (pending answer to flag #2):
  `AGF-NG` → `AGF` where used as a self-title/product reference, if confirmed
  to be a boundary leak rather than an intentional dual identity.
- No change needed anywhere in MR !1712's own drafted legal text — it already
  uses the recommended tuple consistently.

## Pros/cons of alternatives considered

- **Standardize on ASCII `Jani Paijanen` instead:** would require touching
  258 files including both LICENSE texts and MR !1712's own drafted legal
  text, against only 2 files today. Rejected — inverts the actual
  established convention for no legal benefit; diacritic names are valid in
  Finnish legal identity documents.
- **Adopt `AGF-NG` as the public product name instead of `AGF`:** would match
  the repository's working codename to its brand, removing one layer of
  indirection. Rejected as a recommendation (not a hard block) — `AGF-NG`
  reads as a version/variant suffix ("next-gen") rather than a stable product
  identity, and 258 files already treat `AGF` as the abbreviation the
  commercial licence text is written against; changing it now touches live
  legal text, not just prose.
- **Leave `agents-govern` (slug) promoted to co-equal product name:** rejected
  — slugs and product names serve different audiences (packaging/URLs vs.
  procurement/legal), and conflating them is exactly the ambiguity CL-5 exists
  to resolve.

## What this record does not do

No legal text, LICENCE file, framework.md, or llms.txt is edited by this
record. This is a recommendation to the governor per `6h6m`'s explicit
instruction; CL-5 remains governor-owned. If the governor confirms the tuple
and the two open flags, a follow-up plan/commit can apply the bounded
migration above as a mechanical, non-legal-content correction (matching what
!1712's own text already says) — separate from the governor's CL-5 decision
itself, which this record does not make.
