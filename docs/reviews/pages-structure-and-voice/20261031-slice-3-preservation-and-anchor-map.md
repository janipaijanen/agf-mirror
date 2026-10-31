<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 3 ("How do I start?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 3.
- **Reproduce:** `python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main agf/docs/site/docs/start/commission.md agf/docs/site/docs/start/first-governed-change.md agf/docs/site/docs/adopt/install-and-layouts.md`

## Extractor output

```
Base ref: origin/main (6bd6cfb02b594ecc5a2ce94eb96f56374cb330cd)

## commission.md

- Candidate sentences in base: 2; present verbatim in final: 1; not found: 1
  - NOT FOUND: "An existing project may already have a `docs/` directory used for something else, CI configuration, or a prior partial AGF install — the commissioning prompt in step 3 branches on this and must never silently overwrite what is already there."
- Headings before: 11; after: 11
- Heading ids before: commission-agf-and-agov, overview, prerequisites, step-1-choose-new-or-existing-project, step-2-get-your-pinned-agf-and-agov-artifacts, step-3-paste-one-assistant-commissioning-prompt, step-4-review-the-assistants-report, step-5-confirm-validation, step-6-continue-to-your-first-governed-change, entry-points-summary, see-also
- Heading ids after:  set-up-agf, overview, prerequisites, step-1-choose-new-or-existing-project, step-2-get-your-pinned-agf-and-agov-artifacts, step-3-paste-one-assistant-commissioning-prompt, step-4-review-the-assistants-report, step-5-confirm-validation, step-6-continue-to-your-first-governed-change, entry-points-summary, see-also
- Removed ids: ['commission-agf-and-agov']; added ids: ['set-up-agf']

## first-governed-change.md

- Candidate sentences in base: 6; present verbatim in final: 4; not found: 2
  - NOT FOUND: "Validate it: Nothing is implemented before Gate 1 approval lands."
  - NOT FOUND: "**Who does this: Human decision — the assistant may prepare the handoff evidence, but the governor reviews and approves.** After implementation, update the prepared Gate 2 handoff with the real changed files and validation commands: The governor-reviewer reviews the implementation diff, the tests, and the handoff evidence."
- Headings before: 11; after: 11
- Heading ids before: your-first-governed-change, 1-know-where-the-files-will-go, 2-ask-your-assistant-to-prepare-the-starter-files, 3-validate-the-plan-and-first-handoff, 4-gate-1-decide-before-implementation, 5-branch-and-implement-test-first, 6-gate-2-review-before-merge, 7-merge-and-close-the-loop, 8-record-what-you-learned, fast-track-after-the-first-run, next
- Heading ids after:  your-first-governed-change, 1-know-where-the-files-will-go, 2-ask-your-assistant-to-prepare-the-starter-files, 3-validate-the-plan-and-first-handoff, 4-gate-1-decide-before-implementation, 5-branch-and-implement-test-first, 6-gate-2-review-before-merge, 7-merge-and-close-the-loop, 8-record-what-you-learned, fast-track-after-the-first-run, next
- Removed ids: none; added ids: none

## install-and-layouts.md

- Candidate sentences in base: 12; present verbatim in final: 10; not found: 2
  - NOT FOUND: "If you choose to use the optional `agov` CLI from the full checkout or tooling archive, run the shipped core validators: A framework-only release archive has schemas and templates, but not the current tooling executable."
  - NOT FOUND: "Without that bridge to branch protection, the gates still bind the process, but malformed governance files may not mechanically stop a merge. - Confirm `agf/learnings/<your-codename>.yaml` exists and uses only the codename."
- Headings before: 12; after: 12
- Heading ids before: install-the-agf-directory, prerequisites, first-decide-the-repository-layout, step-1-get-the-distribution, step-2-commission-the-install-with-your-assistant, step-3-review-the-prepared-layout, step-4-review-rule-loading-and-customization, step-5-adopt-the-agent-definitions, step-6-wire-the-validators, step-7-initialise-your-evidence, verify, next
- Heading ids after:  install-the-agf-directory, prerequisites, first-decide-the-repository-layout, step-1-get-the-distribution, step-2-commission-the-install-with-your-assistant, step-3-review-the-prepared-layout, step-4-review-rule-loading-and-customization, step-5-adopt-the-agent-definitions, step-6-wire-the-validators, step-7-initialise-your-evidence, verify, next
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- commission.md -> ../adopt/assistant-loader-configuration.md#verify-the-loader: ok
- commission.md -> ../adopt/install-and-layouts.md#step-6-wire-the-validators: ok
- first-governed-change.md -> ../index.md#how-to-read-this-site: ok
- install-and-layouts.md -> ../start/commission.md#step-2-get-your-pinned-agf-and-agov-artifacts: ok
- install-and-layouts.md -> ../start/should-you-adopt.md#what-ships-and-what-you-activate: ok
- install-and-layouts.md -> ../advanced/contracts-and-validators.md#minimum-structural-guardrail: ok
```

## Changes and dispositions

| Page | Change | Disposition |
|---|---|---|
| `commission.md` | Title "Commission AGF and AGOV" becomes "Set up AGF" (the navigation label and the Decide page already say Set up AGF). The prerequisite names the accountable person first, then the role name. Step 1 "Why" is shortened to offset the added words. | The h1 id changes from `commission-agf-and-agov` to `set-up-agf`. No page in the repository links to the old id, but external links may exist (review mpq4), so a separate anchor `<a id="commission-agf-and-agov"></a>` sits immediately before the h1 and both fragments resolve. The Step 1 sentence reported as not found is reworded with the same meaning: new project has nothing to conflict with; an existing project may use `docs/`, hold CI configuration or contain a partial install; the step 3 prompt branches on this and never silently overwrites. |
| `first-governed-change.md` | A short paragraph says what Gate 1 and Gate 2 are before the figure, naming the human governor as the approver ("in a solo project, that is you"; the assistant cannot approve its own work). The audience line is deliberately unchanged: the CI check `check_commissioning_links.py` pins its exact text (EXPECTED_AUDIENCE_LINE, verified in pqja); a first attempt that relinked it failed that check, and it was reverted. Changing the gate's expected text would be gate mechanics and is out of scope. | Additive. The extractor "not found" blocks are false negatives from block merging: `git diff` shows only these hunks changed. |
| `install-and-layouts.md` | Governor prerequisite names the person first; three link labels say "Set up AGF". | Wording only. The two "not found" blocks are unchanged text (false negatives); see the diff. All step headings and ids are kept so existing anchors still work. |

## A1 Adopt route

Source-level: the five counted sections keep their length (the added prerequisite words are offset by the shorter Step 1 "Why"; crude total 1,217 before and 1,216 after). The Adopt cap is 1,080 with payload 476. The rendered counter runs on the strict-build artifact of the published head. No baseline is reset.

## Not done

No change to any rule, role, figure, navigation or other page. `assistant-loader-configuration.md` is unchanged: its first section already says why loading matters. The old label "Commission AGF" remains in `docs/index.md` (Reference index) and the FAQ; those belong to slices 2 and 6.
