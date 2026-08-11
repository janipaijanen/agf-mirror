# External simulated due-diligence assessments: Grok and Copilot

## Provenance

The human Governor separately prompted xAI Grok and Microsoft Copilot,
then relayed both responses verbatim into this session. The exact prompt, as
given to both systems:

> Perform fresh, simulated, due diligence to agents govern framework using
> its public web pages, and repository, like KPMG, EY, McKinsey or other
> recognised firms would do.
> https://gitlab.com/janipaijanen/agents-govern-framework
> https://janipaijanen.gitlab.io/agents-govern-framework/
>
> Current released version is 2.5.0

**What is and is not known about this packet.** The prompt text above is
known and reproducible. Not preserved or available: invocation timestamp,
response/export identifier, model or product mode (e.g. which Grok/Copilot
variant, whether a "deep research" or browsing-enabled mode was used), or an
explicit attestation that neither system's response was shown to the other
before both were generated. Absent that metadata, this packet cannot be
called reproducible, and claims of "genuine independence" or "real source
independence" are not supportable from what is actually on record — a prior
version of this filing made exactly that overclaim, flagged as
`B-DD-FILE-1` by contribution-auditor review `rp7m`. A second, more subtle
form of the same overclaim (categorically asserting no-context-access as an
established project fact, rather than what is actually attested) was flagged
by contribution-auditor review `28r5`, corrected here.

What can be said accurately, and no more: this is **separately prompted,
cross-vendor reports relayed by the Governor; session/model/blinding
metadata unavailable.** Per the Governor's account, only the public prompt
above was given to each system — neither was shown this session, MR !1366,
`p3vk`, `t1jj`, or other internal coordination content. That account is not
independently verifiable by any AGF agent (no AGF agent has access to
either system's own session), so it is recorded as Governor-attested, not
as an AGF-observed fact. It is meaningfully different from Claude's own
same-session attempt (MR !1366), but it is not equivalent to a blinded
multi-adopter validation, and this filing does not claim it is.

Full verbatim text relayed by the human is preserved in `grok-report.md` and
`copilot-report.md` in this directory. Neither relayed response body was
edited, trimmed, or corrected before filing — this directory is a verbatim
capture, not a re-authored summary. Each file's project-authored wrapper
header (the few lines before the `---` separator) may carry provenance
corrections over time, as it did per `28r5`'s `B-DD-FILE-1`; the response
body beneath the separator does not change.

**Known staleness in both reports, verified against this repository:**

- Grok cites live docs showing `2.5.0-rc.2` and "latest tagged release
  observed v2.4.11" — both predate the `v2.5.0` stable tag/release
  (2026-08-10T04:48:20Z). As of this correction, the metadata forward-sync
  to `main` (MR !1379) has also completed (merged as
  `93a340b126d092cc4689a94ed60a977da6d3b87e`, Governor Gate 2 approval
  `20260810T143410Z-governor-reviewer-22j2.md`), so `main` itself no longer
  shows the staleness either report observed at crawl time.
- Copilot's repository snapshot (1,017 commits / 66 branches / 18 tags / 9
  releases, "created 14 June 2026") is an earlier crawl than either report's
  "August 2026" review date suggests — commit/branch/tag counts are
  substantially higher at time of filing.

Both reports state they relied on public metadata only, without
authenticated inspection, so this staleness is expected, not a defect in
either report — it reflects the crawl timing, not analytical error.

## Convergent findings (both external reports, cross-checked against `p3vk`/`t1jj`)

1. **Single-maintainer / key-person concentration.** Both reports
   separately flag this. Matches Tier 3 of `p3vk`'s (unauthorized) proposal
   list.
2. **Evidence concentration in self-application, not independent
   multi-project validation.** Both note this explicitly using the
   project's own maturity/evidence-status vocabulary correctly. This is the
   same "zero external traction" question `h7h4` told the fresh-context
   evaluator to determine rather than assume.
3. **Advisory (not purely structural) gate effectiveness** depends on
   governor discipline — both flag this as the main operational risk axis,
   which also aligns with `p3vk`'s framing.
4. **Neither claims regulatory-compliance equivalence** — both explicitly
   warn against overclaiming NIST AI RMF / ISO 42001 alignment as
   certification, consistent with `verifiable-claims-rule.md` and
   `implementation-honesty-rule.md`.

## Licensing: one recommendation, one factual error — not convergence

A prior version of this filing listed licensing as a fourth convergent
finding. It was not: the two reports do not agree, and one contains a false
statement about this project's current state. Flagged as `B-DD-FILE-2` by
`rp7m`; corrected here rather than silently rewritten in the raw report.

- **Copilot** identifies licensing ambiguity (CC BY-SA 4.0 spanning docs,
  code, and templates) and recommends splitting by asset type — a real,
  separately generated cross-vendor recommendation. It happens to align
  with the Rust-tooling-license question already open between the human
  Governor, Claude, and Codex via `p3vk`/`h7h4`.
- **Grok** asserts, as an observed fact, that framework content is under
  CC BY-SA/commercial terms and tooling is *already* MIT. That is false
  against this project's actual source at the time of this filing:
  `tooling/rust/Cargo.toml` declares
  `license = "CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial"` for the
  Rust workspace — no MIT split exists yet. Grok's statement describes a
  proposed/discussed direction as settled current state.

Licensing remains a substantively high-priority open item — the project's
actual license boundaries are consequential, and both Copilot's
recommendation and the internal `p3vk` discussion also raise it — but that
priority rests on the real contract and risk, not on a false two-vendor
vote. This finding's evidence count is one external recommendation, not
two-report corroboration.

## Where they diverge from Claude's superseded memo (MR !1366)

Neither report repeats the DD-4 (stale-claim) or DD-5 (unhedged
superlative) failure modes `t1jj` found in Claude's first attempt — both
consistently hedge comparative claims ("many platforms...", "this
assessment is simulated, not audit-grade") throughout.

## Disposition requested

Per `7kjh`'s method contract, these reports must **not** be shown to the
genuinely fresh-context evaluator before it freezes its own findings — the
same containment already applied to MR !1366 and `p3vk` applies here.
Requesting contribution-auditor's (Codex's) view on whether these two
reports should be added to the same post-freeze reconciliation set as MR
!1366/`p3vk` (compare-after, not feed-before).

**Merge/publication hold (per contribution-auditor review `28r5`,
`B-DD-FILE-5`).** The evaluator is required to inspect an anonymous clone of
public `main`. Merging this filing to `main` before the evaluator's source
cutoff is fixed would put these two reports and their synthesis directly in
the tree the evaluator inspects, defeating the containment this section
already commits to. Content/Gate 2 *review* of this filing may proceed
independently of that timing question, but *merge* to `main` is held until
either (a) the fresh-context evaluator freezes its own report, or (b) an
approved diligence Gate 1 plan pins an exact pre-candidate `main` SHA (one
that excludes this MR) as the evaluator's evidence cutoff. This hold is
narrower than, and does not replace, the plan-writing dependency already
tracked via `7kjh`/`h7h4` — it is specifically about this filing's merge
timing relative to the evaluator's source of truth.

This filing is informational and requires its own binding Governor Gate 2
decision before merge, same as any other change to this repository — filing
under a documentation-only fast-track waiver (see commit message) removes
the full Gate 1 plan requirement, not the Gate 2 merge requirement. It does
not authorize any Tier 1–5 item, does not change the diligence Gate 1 plan's
method contract, and does not treat either external report as confirmed
fact — both are simulated assessments based on public metadata, same
limitation class as any other AI-authored due-diligence memo.
