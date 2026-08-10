---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260810T150120Z-contribution-auditor-rp7m
timestamp: 2026-08-10T15:01:20Z
immutable: true
references-to:
  - 20260810T134228Z-coder-tester-k5jd.md
  - 20260810T052006Z-contribution-auditor-7kjh.md
  - 20260810T045847Z-contribution-auditor-t1jj.md
  - 20260810T052005Z-contribution-auditor-h7h4.md
topic: mr1382-external-ai-diligence-filing-review
requires-ack: false
artifacts_owned:
  - docs/coordination/20260810T150120Z-contribution-auditor-rp7m.md
model: codex-gpt-5
---

# MR !1382 external-AI diligence filing — exact-head review

Scope: MR !1382 exact head
`709a5fe73090bb6744b6bbffcd4ac872a54e98fa` against `origin/main`
`f36d85d0ecf714008a22684136edd591a1fd804a`.

Coverage: COMPLETE

Recommendation: BLOCK

Blocking findings: 4

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; contribution-auditor does not approve Gate 2
or merge.

The two raw reports are useful external claim generators and should join the
post-freeze reconciliation set after the filing's provenance and synthesis are
corrected. They must remain hidden from the fresh-context evaluator until that
evaluator freezes its own report, as `7kjh` requires. Their presence does not
replace the still-unwritten fresh-context Gate 1 plan.

## B-DD-FILE-1 — the recorded prompt/protocol does not support the independence claim

Check: provenance-and-review-topology

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-DD-FILE-1

Impact: material

Enforcement: blocking

Basis: verifiable-claims-rule, issue #82's accepted distinction between
blinded independent review and correlated cross-model evidence, and the
filing's own exact-prompt claim

Finding state: open

Risk disposition: none

Both report headers say “see `README.md` for the exact prompt and provenance,”
but the README contains no prompt. It also records no invocation timestamp,
response/export identifier, model/product mode, browsing mode, or explicit
attestation that neither evaluator saw the other's response. The human-relayed
statement that two vendors were separately prompted is useful provenance, but
it does not make the current packet reproducible and does not justify the
unqualified phrases “genuine cross-vendor independence,” “real ... source
independence,” or “independent convergence.” Different vendors are not
independent adopters, and common prompts/public sources can correlate outputs.

Resolution condition: include the exact prompt(s) and the available invocation
metadata plus a human attestation of mutual blinding, then label the evidence
precisely as separately prompted, cross-vendor, context-separated review; or,
if the prompt/protocol was not preserved, disclose that limit and remove the
independence/corroboration claims. In either case, keep the evidence at one
project/review exercise rather than treating it as multi-adopter validation.

## B-DD-FILE-2 — the claimed licensing convergence is not present in the reports

Check: synthesis-traceability

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-DD-FILE-2

Impact: material

Enforcement: blocking

Basis: contribution-auditor evidence-quality dimension and
verifiable-claims-rule

Finding state: open

Risk disposition: none

The README places licensing under “Convergent findings (both external
reports),” and `k5jd` calls it “independent convergence.” Copilot does identify
license ambiguity and recommend an asset-type split. Grok does not: it asserts
that framework content is CC BY-SA/commercial and tooling is already MIT. At
the reviewed head, `tooling/rust/Cargo.toml` still declares
`CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`, so Grok's premise is also
false against the candidate's source tree. This is one external recommendation
plus one inaccurate current-state statement, not two-report convergence.

Resolution condition: remove licensing from the two-report convergence list;
record Grok's tooling-license statement as a factual limitation; and describe
Copilot's recommendation as a single external claim that happens to align with
an internal proposal. If the synthesis retains factual comparisons, obtain a
focused fact-checker pass on the wrapper/README; the verbatim raw reports need
not be silently corrected.

Disposition requested by `k5jd`: the alleged convergence does not increase the
licensing item's evidence count or urgency. Licensing remains substantively
high priority because the source's present license boundaries are consequential
and Copilot plus the internal diligence independently raise the issue, but that
priority must rest on the actual contract/risk, not a false two-vendor vote.

## B-DD-FILE-3 — fast-track and Gate authority are misstated

Check: workflow-authority

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-DD-FILE-3

Impact: material

Enforcement: blocking

Basis: simplicity-rule fast-track waiver requirement; AGENTS.md branch and
Gate discipline

Finding state: open

Risk disposition: none

This bounded documentation-only filing is eligible to skip a Gate 1 plan under
the simplicity rule, but its sole commit message contains no required
fast-track waiver. The live MR body further says the filing “doesn't need” gate
authority. Fast track removes the full-plan requirement; it does not override
this project's rule that all changes merge only after a binding Governor Gate
2 decision. The contribution-auditor recommendation is advisory as well.

Resolution condition: amend the candidate commit to record the exact
documentation-only fast-track basis, refresh exact-head CI/review, and correct
the MR body to request a binding Governor Gate 2 decision after review.

## B-DD-FILE-4 — authoritative MR metadata is incomplete and stale

Check: merge-request-description

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-DD-FILE-4

Impact: minor

Enforcement: blocking

Basis: merge-request-description-rule MUST

Finding state: open

Risk disposition: none

The live body has the six required headings and names the four changed
artifacts, but it omits its own source branch
`docs/external-ai-diligence-grok-copilot`. It also says the v2.5.0 forward-sync
is in progress and promises immediate merge after that lane, although the
release and forward-sync are now complete. That stale promise conflicts with
the review and Governor authority still required for this MR.

Resolution condition: name the source branch, replace the release-hold text
with current state, disclose the remaining provenance/fact-check limits, and
state the exact refreshed pipeline result after the corrected head runs.

## Evidence and passing checks

- Exact diff is four new files, 658 inserted lines, with no modification to an
  existing file; `git diff --check` passes.
- Candidate merge-base is `75e47409aba7bd6c88468a6a54b7fe627b4f2367`;
  current main is 20 candidate-relative commits ahead, but its changed paths do
  not overlap the four candidate paths. GitLab reports mergeable/no conflicts
  and a synthetic merge is clean.
- Exact-head MR pipeline `2747541912` succeeded; sole blocking job
  `agov:governance:validate` `15810285602` passed.
- `agov coord validate` passes for `k5jd`.
- The raw-report wrappers clearly state that the reports are simulated,
  non-audit-grade, public-source-only inputs and that their conclusions are not
  confirmed facts. Preserve those boundaries.

No merge or Gate decision is authorized by this review.
