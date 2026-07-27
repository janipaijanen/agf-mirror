# Pages/runbook normative-language baseline inventory

Baseline audit required by `agf/rules/workflow/documentation-rule-projection-rule.md`
§4 ("Baseline-audit existing Pages and runbooks"), produced as part of issue
#73's implementation. This is **prospective evidence** — it classifies the
existing corpus, it does not retroactively invalidate or rewrite historical
plans, coordination records, or already-published releases.

## Method

Reproducible command (corrected per fact-checker finding F2 — the first
version of this document used a non-reproducible ad-hoc script and reported
inaccurate counts):

```bash
git grep -c -E -o 'MUST|REQUIRED|blocking|approve(s|d)|prohibited' HEAD -- 'agf/docs/site/docs/**/*.md'
```

Per the rule, **keyword detection is advisory candidate discovery only** —
it is not a semantic judgment about whether the surrounding prose is
actually normative, duplicated, or correctly deferring to a named rule.
Every occurrence below was read in context (not counted blind) to classify
its owner.

## Per-file summary

| File | Lines | Owner classification |
|---|---|---|
| `advanced/contracts-and-validators.md` | 12 | `tool-authority-boundary-rule.md` — page's subject *is* the advisory/blocking distinction; consistently defers to "a named rule" rather than asserting one |
| `concepts/coordination-protocol.md` | 7 | `coordination-publication-rule.md` (rewritten section, this implementation) + `roles-and-gates.md` (existing gate-chain description) |
| `concepts/roles-and-gates.md` | 5 | Self-owning — this is the canonical page defining the role/gate authority split; not a projection target for another rule |
| `operate/review-and-release.md` | 5 | `tool-authority-boundary-rule.md` (blocking-findings language) + `roles-and-gates.md` (approval language) |
| `start/should-you-adopt.md` | 3 | `roles-and-gates.md` — describes the human-governor role generically for prospective adopters |
| `operate/run-agf-in-real-situations.md` | 3 | `roles-and-gates.md` (Gate 1/Gate 2 language) — situational examples, not new rules |
| `adopt/assistant-loader-configuration.md` | 3 | `roles-and-gates.md` |
| `adopt/install-and-layouts.md` | 2 | `tool-authority-boundary-rule.md` — explicit "the rule that grants it blocking authority" |
| `adopt/tooling-and-tokens.md` | 2 | `tool-authority-boundary-rule.md` — CI wiring guide pointer |
| `operate/runbook.md` | 2 | `roles-and-gates.md` (approved-scope language) + `always/coordination-message-rule.md` (visible-bridge language) |
| `start/adopter-faq.md` | 2 | `roles-and-gates.md` — describes governor-reviewer approval authority, does not redefine it |
| `concepts/philosophical-lenses.md` | 2 | `tool-authority-boundary-rule.md` (blocking authority) + `roles-and-gates.md` (accountability) |
| `operate/day-to-day.md` | 1 | `roles-and-gates.md` — anti-pattern example ("gates approved in seconds") |
| `operate/poll-and-coordinate.md` | 1 | `coordination-publication-rule.md` (this implementation's added pointer) |
| `operate/session-continuity.md` | 1 | `always/coordination-message-rule.md` — visible-bridge language |
| `start/first-governed-change.md` | 1 | `roles-and-gates.md` — "governor reviews and approves" |
| `concepts/evidence-and-learnings.md` | 1 | `always/uncertainty-and-scope-rule.md`-adjacent — "required coordination entry" in a learning-category table |

17 files, 53 matching lines (git-grep-verified, replacing the earlier
inconsistent 20-file/~79-occurrence estimate).

## Full per-occurrence reconciliation (issue #74, folded into this release)

Every one of the 53 matching lines, individually classified. `Pointer in
file?` records whether the page already names its owning rule by filename
(mechanically checked: `grep -c '<rule-filename>' <page>`), not whether the
surrounding prose is otherwise correct.

| File:Line | Excerpt | Owner rule | Pointer in file? |
|---|---|---|---|
| `assistant-loader-configuration.md:53` | "authority; schemas, approved plans, and coordination records define governed" | `roles-and-gates.md` | No |
| `assistant-loader-configuration.md:212` | "says the governor-reviewer approves gates;" | `roles-and-gates.md` | No |
| `assistant-loader-configuration.md:230` | "really approved a decision or that the implementation is correct. That final" | `roles-and-gates.md` | No |
| `install-and-layouts.md:276` | "governor decision makes the check blocking. When a check is blocking, document" | `tool-authority-boundary-rule.md` | Yes |
| `install-and-layouts.md:277` | "the rule that grants it blocking authority in the CI job or release procedure." | `tool-authority-boundary-rule.md` | Yes |
| `tooling-and-tokens.md:133` | "full CI wiring guide, including how to declare checks as blocking vs advisory." | `tool-authority-boundary-rule.md` | No |
| `tooling-and-tokens.md:269` | "`agov` checks into CI as blocking or advisory gates." | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:6` | "contracts, guarantees, and blocking-policy choices.*" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:17` | "governor. A validator failure becomes a blocking gate only when a named AGF" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:19` | "blocking. Otherwise the result is advisory evidence for the responsible role." | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:55` | "implementation diff matches the approved Gate 1 plan, or that every Gate 2" | `roles-and-gates.md` | No |
| `contracts-and-validators.md:74` | "## Advisory and blocking checks" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:81` | "Is failure advisory or blocking? \| `advisory` unless a named rule or decision grants blocking authority" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:85` | "`advisory:agf-coordination-review` for a non-blocking report." | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:86` | "`blocking:agf-coordination-schema` when the coordination schema is a required gate." | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:90` | "stalling work. Reserve blocking checks for violations that the project has" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:147` | "checks the governor, CI policy, or named rule has granted blocking force." | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:225` | "4. Make schema failures blocking under a named rule, CI policy, or recorded" | `tool-authority-boundary-rule.md` | No |
| `contracts-and-validators.md:233` | "actually approved the work, that the approval was wise, or that the diff" | `roles-and-gates.md` | No |
| `coordination-protocol.md:112` | "next agent" — it does not mean the action is approved or complete." | `coordination-publication-rule.md` | Yes |
| `coordination-protocol.md:124` | "A false lifecycle claim is a blocking finding wherever it is caught," | `coordination-publication-rule.md` | Yes |
| `coordination-protocol.md:167` | "**Step 1 — Gate 1 plan approved**" | `roles-and-gates.md` | No |
| `coordination-protocol.md:184` | "**Step 2 — Governor approves**" | `roles-and-gates.md` | No |
| `coordination-protocol.md:193` | "status: approved" (worked example) | `roles-and-gates.md` | No |
| `coordination-protocol.md:198` | "Gate 1 approved. Proceed with implementation." | `roles-and-gates.md` | No |
| `coordination-protocol.md:351` | "handoffs passing approved plans to implementation." | `roles-and-gates.md` | No |
| `evidence-and-learnings.md:122` | "...or skipped a required coordination entry." | `always/coordination-message-rule.md` | No |
| `philosophical-lenses.md:10` | "...create no obligations and do not grant blocking" | `tool-authority-boundary-rule.md` | No |
| `philosophical-lenses.md:56` | "...\"the process approved it\" becomes the" | `roles-and-gates.md` | No |
| `roles-and-gates.md:14` | "No agent both implements and approves its own work." | self | — |
| `roles-and-gates.md:17` | "An implementing agent that \"approves\" its own merge has" | self | — |
| `roles-and-gates.md:89` | "the approved plan, visual inspection where the output is visible." | self | — |
| `roles-and-gates.md:240` | "Skipped Gate 1 \| Implementation started without an approved plan..." | self | — |
| `roles-and-gates.md:241` | "Silent scope drift \| The diff no longer matches the approved plan..." | self | — |
| `day-to-day.md:186` | "Gates approved in seconds, always \| The reviewer is rubber-stamping" | `roles-and-gates.md` | No |
| `poll-and-coordinate.md:52` | "Before delegating, identify the approved plan, role, owned files..." | `roles-and-gates.md` | Yes (pointer covers a different occurrence on the same page) |
| `review-and-release.md:30` | "Findings are filed as a review entry; blocking findings return the work;" | `tool-authority-boundary-rule.md` | No |
| `review-and-release.md:38` | "of blocking and gate authority." | `tool-authority-boundary-rule.md` | No |
| `review-and-release.md:41` | "the implemented branch against the approved plan before the merge, not after" | `roles-and-gates.md` | No |
| `review-and-release.md:53` | "Findings recorded (blocking findings return the work; non-blocking noted)" | `tool-authority-boundary-rule.md` | No |
| `review-and-release.md:96` | "Before a release MR is approved, run `agov validate adopter-docs`." | `roles-and-gates.md` | No |
| `run-agf-in-real-situations.md:40` | "Gate 1 was approved \| Record the decision..." | `roles-and-gates.md` | No |
| `run-agf-in-real-situations.md:41` | "Implementation is ready \| Submit evidence for Gate 2..." | `roles-and-gates.md` | No |
| `run-agf-in-real-situations.md:42` | "...unless the governor explicitly approves." | `roles-and-gates.md` | No |
| `runbook.md:18` | "...Claim, implement, validate, and hand off within the approved scope" | `roles-and-gates.md` | Yes (pointer covers a different occurrence on the same page) |
| `runbook.md:49` | "an approved shared path, or paired with a visible bridge naming where it lives." | `always/coordination-message-rule.md` | Yes (see above) |
| `session-continuity.md:37` | "or publish an approved visible bridge naming its branch or merge request." | `always/coordination-message-rule.md` | No |
| `adopter-faq.md:99` | "the governor-reviewer approves the gate, not the author of the ADR alone;" | `roles-and-gates.md` | Yes (pointer covers a different occurrence on the same page) |
| `adopter-faq.md:131` | "give the Gate 2 reviewer the exact approved plan, Gate 1 decision, conditions," | `roles-and-gates.md` | Yes (see above) |
| `first-governed-change.md:158` | "evidence, but the governor reviews and approves.**" | `roles-and-gates.md` | Yes |
| `should-you-adopt.md:10` | "a review process that a human approves before anything merges," | `roles-and-gates.md` | No |
| `should-you-adopt.md:14` | "the person who approves gates and owns project risk..." | `roles-and-gates.md` | No |
| `should-you-adopt.md:107` | "(or are willing to be) a human governor who approves plans and merges." | `roles-and-gates.md` | No |

## Disposition

Per `documentation-rule-projection-rule.md`'s bounded baseline-audit scope,
this implementation:

- **Fixes:** issue #72/#73's own projections (see
  `docs/reviews/rule-projection/coordination-independent-publication.yaml`)
  and confirms no broken pointer was introduced (validated by the Gate 2
  strict Pages build and internal-link check).
- **Classifies:** all 53 matching lines across the 17 candidate files,
  individually, to a plausible owning rule, based on a contextual read of
  each occurrence rather than keyword count alone (see the table above). No
  file was found asserting a new or competing rule; every occurrence
  describes or points at an existing authority (the role/gate split in
  `roles-and-gates.md`, the advisory/blocking framework in
  `tool-authority-boundary-rule.md`, or the coordination-lifecycle/bridge
  rules).
- **Closes issue #74 via this release**, per governor-reviewer decision
  (`docs/coordination/20260727T152325Z-coder-tester-rrtp.md`, 2026-07-27):
  the per-occurrence reconciliation this issue tracked is complete above,
  not deferred.
- **Explicitly out of scope for this reconciliation:** inserting new
  cross-reference pointers into the 11 files whose classified lines have no
  existing filename pointer (`Pointer in file? No` above — 39 of the 53
  lines). Reconciliation establishes *which* rule owns each occurrence; it
  does not itself edit each of those 11 Pages files to add a visible
  pointer. That is a larger, separate Pages-editing task with its own
  review surface (11 files, wording choices per page) and was not part of
  the governor's "pull the reconciliation into this MR's scope" decision.
  Flagging here rather than silently treating pointer-insertion as also
  done: if the governor wants it folded in too, it needs its own scoping
  pass, not a rider on this correction.
