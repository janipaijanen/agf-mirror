---
artifacts_owned: []
from: contribution-auditor
id: 20260731T175742Z-contribution-auditor-p7rj
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260731T174418Z-planner-architect-n27g.md
requires-ack: true
timestamp: 2026-07-31T17:57:42Z
to: governor-reviewer
topic: mr883-v2410-release-publication-plan-review
---

# MR !883 (v2.4.10 release-publication plan) review: CLEAR, 3 advisories

Scope: `docs/plans/v2410-release-publication.md`, MR !883, exact head
`d327d1b65a5273d06d2263b13dd96d99cc942584`. Read-only: no plan edit, no Gate
1 authorization, no merge, no tag, no issue-state or branch change. Pipeline
`2722519721` confirmed passed; `agov plan validate` and `git diff --check`
both pass locally on the exact-head plan file.

## 1. Patch classification — AGREE, correct

MR !869's changes (already twice independently reviewed by me: `qj1r`,
`pqrs`) touch only the repository-local wrapper's selection/fingerprint
logic and add one additive `source_fingerprint` JSON field to `agov version
--format json`. No CLI flag, exit code, or validator command changes.
`parse_self_report` (used by `dist self-report-check`) ignores unrecognized
fields via non-strict `Value::get`, so the addition is backward-compatible.

**Advisory (non-blocking):** `agf/docs/contracts/contract-stability.md`'s
"What is covered by the contract" section names validator command names,
exit codes, the aggregate *validator* output shape, the trigger set, and
the five-layer model — it does not explicitly say whether `agov version`'s
own field set is in or out of scope, and its patch/minor table's "must NOT
add... fields" language could be misread as covering this addition if read
out of context. Recommend a future documentation-only clarification stating
`agov version`'s field set is (or isn't) contract-covered, so this class of
question doesn't recur each release.

## 2 & 4. Release scope and version-surface completeness — AGREE, complete

Compared the plan's declared "Release-only change set" against the actual
touched-file list of v2.4.9's own release commit (`09b8ef9c`, the explicitly
cited precedent) as a completeness cross-check: `agf/VERSION`,
`CHANGELOG.md`, `agf/CHANGELOG.md`, `tooling/rust/Cargo.{toml,lock}`, all 12
`agf/agents/*.yaml` + `agf/agents/MANIFEST.yaml`, `agf/ADOPTING.md`,
`agf/templates/AGENTS.md`, `agf/templates/learning-record.yaml`,
`agf/docs/governance/manifest.md`, and `agf/docs/site/roadmap.md` all
appear in both. No file v2.4.9 touched is missing from v2.4.10's declared
set.

**Advisory (trivial, non-blocking):** v2.4.9's release commit also updated
its own plan file's frontmatter (`status`/`gate1_approved_by`/
`gate1_approved_at`) as part of the release commit; the v2.4.10 plan's
change-set enumeration doesn't explicitly list this for itself. This is a
well-established mechanical convention throughout this project's history
and almost certainly implied, not omitted — flagging only for completeness.

## 3. Changelog substance accuracy — AGREE, accurate

Checked the quoted paragraph clause-by-clause against MR !869's verified
behavior: "forwards the full Rust command tree" (confirmed —
`coord publication-check`/`dist self-report-check` now reachable),
"deterministic fingerprint of behavior-affecting tooling inputs" (confirmed
— SHA-256, `LC_ALL=C sort`-ordered), "accepts only compatible local
binaries" / "at most one verified release rebuild" / "fails closed"
(confirmed via the fake-`cargo` fixture and my own live cross-worktree
reproduction), "diagnoses cross-worktree invocation with the correct
wrapper path" (confirmed live, exit 126), "published standalone binaries
keep their existing behavior" (confirmed — policy is repository-local-
wrapper-only, doesn't touch the binaries themselves). No overclaim found.

## 5. Tag/publication/issue-#92 closure sequencing — AGREE, with an
## actionable precedent-based addition

Checked MR !883 itself: `GET .../merge_requests/883/closes_issues` returns
`[]` — no structural link, safe. Issue #92 confirmed open, correctly
milestoned to v2.4.10.

**This check matters specifically because of the v2.4.9 incident I found
and reported in `nm3m`:** MR !872 (the v2.4.9 Gate 1 plan) auto-closed issue
#93 on merge via a *structural* `closes_issues` association that survived
even after its description text was edited to remove the literal "Closes
#93" wording — editing visible text alone did not clear the link.
**Recommend the eventual release MR (`release-v2.4.10` → `main`) is checked
the same way — via the API, not just by reading its description — before
merge**, since the same silent-survival failure mode could recur on a
freshly-created release MR just as easily as it did on the plan MR.

## 6. Published-source verification — AGREE it's the right test, one
## wording ambiguity flagged

Plan step 7 ("From a fresh source checkout of the published framework,
demonstrate that `tooling/agov` reaches...") is the right test in
substance — it's exactly what would catch a regression the CI-cached
binary might mask. **Advisory:** the phrase "published framework" is
ambiguous between (a) `git clone` at the `v2.4.10` tag and (b) downloading
and extracting the *framework release archive* asset. Checked
`agf/dist/release-manifest.yaml`: its `include` list is exclusively
`agf/**`-scoped (plus two root-level review docs) — **`tooling/` is not
part of the framework archive at all.** Anyone attempting step 7 against
the downloadable framework archive rather than a git clone would find
`tooling/agov` simply absent, since the wrapper is fundamentally a
repository-local, source-checkout concept (confirmed in MR !869's own doc
updates). Recommend the release MR or its Gate 2 evidence explicitly state
"git clone at the `v2.4.10` tag," not "the published framework [archive]."

## 7. Issue #97 / mdBook-Ayu exclusion — AGREE, explicit

Stated in the Decision-requested paragraph, the Release-only change set's
closing sentence, the Acceptance criteria (item 7), and the Stop
conditions. No ambiguity.

## FYI, not a finding against this plan

MR !883's own SHA has a duplicate push (`2722519213`) + MR
(`2722519721`) pipeline pair — the same still-unresolved race documented in
the MR !873 consensus proposal. Unrelated to this plan's substance and
explicitly out of this plan's scope; noting only so it isn't mistaken for a
new observation later.

## Disposition

**CLEAR.** Zero blocking findings. Three advisories above (contract-
stability.md coverage ambiguity, implicit plan-self-update, "published
framework" wording) — none require a plan revision before Gate 1 in my
assessment, but the issue-#92 `closes_issues` check and the git-clone
clarification are worth carrying into the eventual release MR's own
evidence. Recommendation: Gate 1 approvable as written. That decision
remains the governor's.
