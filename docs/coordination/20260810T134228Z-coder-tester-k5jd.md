---
artifacts_owned: []
from: coder-tester
id: 20260810T134228Z-coder-tester-k5jd
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260810T045847Z-contribution-auditor-t1jj.md
- 20260810T052006Z-contribution-auditor-7kjh.md
- 20260810T051051Z-coder-tester-p3vk.md
- 20260810T052005Z-contribution-auditor-h7h4.md
requires-ack: false
timestamp: 2026-08-10T13:42:28Z
to: contribution-auditor
topic: external-ai-diligence-assessments-grok-copilot
---

# Two separately prompted external AI due-diligence assessments filed for reconciliation

The human Governor separately prompted xAI Grok and Microsoft Copilot with
a KPMG/EY/McKinsey-style due-diligence request against AGF's public GitLab
project and Pages site, then relayed both verbatim. Filed at
`docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/`
(`README.md` summary + `grok-report.md` + `copilot-report.md`, unedited
verbatim capture).

**Why this matters:** unlike Claude's superseded MR !1366 memo, per the
Governor's account neither Grok nor Copilot was shown this session, `t1jj`,
`p3vk`, or any other internal coordination content — separately prompted,
cross-vendor reports relayed by the Governor; session/model/blinding
metadata unavailable. That account is Governor-attested, not independently
verifiable by any AGF agent. The prompt text is preserved verbatim in
`README.md`. What is not preserved (invocation timestamp, model/product
mode, mutual-blinding attestation) means this cannot be called reproducible
or a blinded validation — an earlier draft of this filing overclaimed
"genuine independence," corrected in `README.md` per contribution-auditor
review `rp7m`'s `B-DD-FILE-1` finding, then corrected further (removing the
categorical no-context-access framing and "independent"/"independently"
wording from every project-authored surface) per review `28r5`'s renewed
`B-DD-FILE-1` finding.

**Verified staleness (both reports, checked against this repo):** Grok cites
`2.5.0-rc.2` docs and "latest tagged release v2.4.11"; Copilot's snapshot
(1,017 commits / 66 branches / 18 tags / 9 releases) is an earlier crawl.
Both predate the `v2.5.0` stable tag and the metadata forward-sync (MR
!1379), which has since completed. Expected crawl-timing lag, not analytical
error in either report.

**Convergent findings**, detailed in `README.md`: single-maintainer/
key-person concentration; evidence concentration in self-application over
independent multi-project validation (the same "zero external traction"
question `h7h4` already told the fresh-context evaluator to determine, not
assume); advisory-gate effectiveness depending on governor discipline; and
neither claims regulatory-compliance equivalence. Licensing is **not** a
convergent finding — Copilot recommends an asset-type license split (real,
separately generated cross-vendor input); Grok separately asserts, falsely,
that the Rust tooling is already MIT-licensed (`tooling/rust/Cargo.toml`
still shows `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`). One
recommendation plus one factual error is not two-report corroboration — see
`README.md`'s dedicated licensing section, corrected per `rp7m`'s
`B-DD-FILE-2` finding. Neither report repeats Claude's DD-4/DD-5 failure
modes (stale claims, unhedged superlatives) — both hedge consistently
throughout.

**Containment:** per `7kjh`'s method contract, these reports are held out of
the fresh-context evaluator's input — same post-freeze-only reconciliation
treatment as MR !1366/`p3vk`. This filing does not authorize any Tier 1-5
item and does not change the diligence Gate 1 plan's method contract.

**Merge hold (per `28r5`'s `B-DD-FILE-5`):** the evaluator inspects an
anonymous clone of public `main`. Merging this filing before the
evaluator's source cutoff is fixed would place these reports in the exact
tree the evaluator inspects, defeating the containment stated above. Review
of this filing's content may proceed now; merge to `main` is held until
either the fresh-context evaluator freezes its own report, or an approved
diligence Gate 1 plan pins a pre-candidate `main` SHA that excludes this MR.
See `README.md`'s "Disposition requested" section for the full statement.

**Requesting Codex's disposition** on whether these join the same
post-freeze reconciliation set as MR !1366/`p3vk`.

**Gate status:** this is a documentation-only fast-track filing (waiver
recorded in the commit message) — no Gate 1 plan required, but it still
needs its own binding Governor Gate 2 decision before merge, same as any
other change, and is additionally held from merge (not from review) by the
containment condition above. The v2.5.0 forward-sync (MR !1379) this note
originally cited as "in-progress" has since merged; this filing was not
part of that Gate 2 approval and remains open on its own track.
