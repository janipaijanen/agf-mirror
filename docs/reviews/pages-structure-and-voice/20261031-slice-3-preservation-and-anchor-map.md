<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Slice 3 ("How do I start?"): sentence-preservation report and anchor map

- **Status:** advisory review evidence. Not a complete semantic inventory and not a blocking check.
- **Plan:** `docs/plans/20261030T163253Z-pages-structure-and-voice.md`, work allocation item 3. Claim: `20261031T134611Z-planner-architect-tktd`.
- **Immutable base:** `5faa9d4e527e82294304ccd01cd2715df6e9b67f` (main after slice 2 merged). An earlier version of this report used the pre-slice-2 base `6bd6cfb0`; review bqqq asked for the exact base. The three setup pages did not change in slice 2, so this is a provenance correction.
- **Reproduce:** from the repository root with the candidate checked out:

```
python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py 5faa9d4e527e82294304ccd01cd2715df6e9b67f \
  agf/docs/site/docs/start/commission.md agf/docs/site/docs/start/first-governed-change.md \
  agf/docs/site/docs/adopt/install-and-layouts.md
```

## Extractor output

```
Base ref: 5faa9d4e527e82294304ccd01cd2715df6e9b67f (5faa9d4e527e82294304ccd01cd2715df6e9b67f)

## commission.md

- Candidate sentences in base: 2; present verbatim in final: 1; not found: 1
  - NOT FOUND: "An existing project may already have a `docs/` directory used for something else, CI configuration, or a prior partial AGF install — the commissioning prompt in step 3 branches on this and must never silently overwrite what is already there."
- Headings before: 11; after: 11
- Heading ids before: commission-agf-and-agov, overview, prerequisites, step-1-choose-new-or-existing-project, step-2-get-your-pinned-agf-and-agov-artifacts, step-3-paste-one-assistant-commissioning-prompt, step-4-review-the-assistants-report, step-5-confirm-validation, step-6-continue-to-your-first-governed-change, entry-points-summary, see-also
- Heading ids after:  set-up-agf, overview, prerequisites, step-1-choose-new-or-existing-project, step-2-get-your-pinned-agf-and-agov-artifacts, step-3-paste-one-assistant-commissioning-prompt, step-4-review-the-assistants-report, step-5-confirm-validation, step-6-continue-to-your-first-governed-change, entry-points-summary, see-also
- Removed ids: ['commission-agf-and-agov']; added ids: ['set-up-agf']

## first-governed-change.md

- Candidate sentences in base: 6; present verbatim in final: 6; not found: 0
- Headings before: 11; after: 11
- Heading ids before: your-first-governed-change, 1-know-where-the-files-will-go, 2-ask-your-assistant-to-prepare-the-starter-files, 3-validate-the-plan-and-first-handoff, 4-gate-1-decide-before-implementation, 5-branch-and-implement-test-first, 6-gate-2-review-before-merge, 7-merge-and-close-the-loop, 8-record-what-you-learned, fast-track-after-the-first-run, next
- Heading ids after:  your-first-governed-change, 1-know-where-the-files-will-go, 2-ask-your-assistant-to-prepare-the-starter-files, 3-validate-the-plan-and-first-handoff, 4-gate-1-decide-before-implementation, 5-branch-and-implement-test-first, 6-gate-2-review-before-merge, 7-merge-and-close-the-loop, 8-record-what-you-learned, fast-track-after-the-first-run, next
- Removed ids: none; added ids: none

## install-and-layouts.md

- Candidate sentences in base: 12; present verbatim in final: 12; not found: 0
- Headings before: 12; after: 12
- Heading ids before: install-the-agf-directory, prerequisites, first-decide-the-repository-layout, step-1-get-the-distribution, step-2-commission-the-install-with-your-assistant, step-3-review-the-prepared-layout, step-4-review-rule-loading-and-customization, step-5-adopt-the-agent-definitions, step-6-wire-the-validators, step-7-initialise-your-evidence, verify, next
- Heading ids after:  install-the-agf-directory, prerequisites, first-decide-the-repository-layout, step-1-get-the-distribution, step-2-commission-the-install-with-your-assistant, step-3-review-the-prepared-layout, step-4-review-rule-loading-and-customization, step-5-adopt-the-agent-definitions, step-6-wire-the-validators, step-7-initialise-your-evidence, verify, next
- Removed ids: none; added ids: none

## Cross-file anchor links in the final files

- commission.md -> ../adopt/assistant-loader-configuration.md#verify-the-loader: ok
- commission.md -> ../adopt/install-and-layouts.md#step-6-wire-the-validators: ok
- first-governed-change.md -> ../concepts/roles-and-gates.md#approval-authority-and-optional-deputies: ok
- first-governed-change.md -> ../index.md#how-to-read-this-site: ok
- install-and-layouts.md -> ../start/commission.md#step-2-get-your-pinned-agf-and-agov-artifacts: ok
- install-and-layouts.md -> ../start/should-you-adopt.md#what-ships-and-what-you-activate: ok
- install-and-layouts.md -> ../advanced/contracts-and-validators.md#minimum-structural-guardrail: ok
```

## Changes and dispositions

| Page | Change | Disposition |
|---|---|---|
| `commission.md` | Title "Commission AGF and AGOV" becomes "Set up AGF". A separate anchor `<a id="commission-agf-and-agov"></a>` sits immediately before the h1. The prerequisite names the accountable person first, then the role name. Step 1 "Why" is shortened to offset the added words. | The heading id `commission-agf-and-agov` is replaced by `set-up-agf`; the extractor lists it as removed because the old fragment is now an explicit anchor, not a heading id. Both fragments resolve in the built page (job 17632088839). The Step 1 sentence reported as not found is reworded with the same meaning: a new project has nothing to conflict with; an existing project may use `docs/`, hold CI configuration or contain a partial install; the step 3 prompt branches on this and never silently overwrites. |
| `first-governed-change.md` | One paragraph before the figure: this walkthrough uses the default human-approval path; Gate 1 and Gate 2 are the human governor's approvals; in a solo project that is you; the assistant cannot approve its own work; a link points to approval authority and optional deputies for the separately governed, provisional, default-off deputy route. The audience line is deliberately unchanged. | Additive. `check_commissioning_links.py` pins the audience line (EXPECTED_AUDIENCE_LINE, verified in pqja); a first attempt that relinked it failed that check (job 17631541170) and was reverted. Changing the hook would be gate mechanics and is out of scope. |
| `install-and-layouts.md` | The governor prerequisite names the person first; three link labels say "Set up AGF". | Wording only. All step headings and ids are kept. |

## A1 Adopt route

Source-level: the five counted sections keep their length; the added prerequisite words are offset by the shorter Step 1 "Why". The Adopt cap is 1,080 with payload 476. The rendered counter result belongs to the artifact of the published corrected head and is recorded in its handoff. No baseline is reset.

## Not done

No change to any rule, role, figure, navigation or other page. `assistant-loader-configuration.md` is unchanged: its first section already says why loading matters. The old label "Commission AGF" remains in `docs/index.md` (Reference index) and the FAQ; those belong to slices 6 and 2's final batch.
