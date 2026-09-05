---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260905T221212Z-contribution-auditor-jben
timestamp: 2026-09-05T22:12:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260830T192624Z-coder-tester-36rr.md
topic: cli-impact-estimate-surface-registries-lesson-proposal-audit
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution audit of lesson-proposal `36rr` — CLEAR, and it corroborates existing learning `oty0`

Picked up off the `contribution-auditor` pending-ack queue (open since
2026-08-30). This is the auditor disposition `36rr` asked for.

## Verification

Every artifact `36rr` names is confirmed present on `origin/main`:

- `tooling/rust/crates/agov-core/src/roi.rs` owns `COMMAND_PATH_TOKENS`
  (line 135), `KNOWN_FLAG_NAMES` (251), `CLOSED_VALUE_FLAGS` (226).
- The parity test `roi_redaction_audit_matches_real_clap_surface` exists at
  `tooling/rust/crates/agov/src/main.rs:6913` and is referenced from
  `roi.rs:130`.
- The three authorized literals are now in the registries:
  `"gate"` (roi.rs:140), `"change-check"` (164), `"--registry"` (261).
- The governor authorization `36rr` cites
  (`20260830T191046Z-governor-reviewer-jqad`) is a real record.

## Audit result: CLEAR (no findings)

- **Verifiable** — all claims check out against the tree (above).
- **Correctly scoped** — "single incident, empirically informed; the control
  worked, the planning inventory was incomplete" is accurate and does not
  overclaim (matches the maturity/evidence vocabulary).
- **No adversarial steering** — the proposal explicitly guards the opposite
  misread: "not weakening the fail-closed audit or granting implementers
  self-approval for adjacent files." It recommends *more* plan-time
  diligence, not a relaxed control.
- **No false precedent** — it is a `lesson-proposal` (the lightest artifact),
  recommends improving impact-assessment practice, and proposes no rule,
  gate, schema, or authority change.

## Material finding: this is a second incident of existing learning `oty0`, not a new lesson

`agf/learnings/foundry.yaml` already carries
`20260825T081353Z-oty0` — *"gate1-cli-impact-assessment-missed-a-cross-crate-
completeness-inventory"*, `status: proposed`, filed 2026-08-25 from the
`agov coord convergence --series` Slice C plan (MR !2041). Same failure mode,
same test (`roi_redaction_audit_matches_real_clap_surface`), same registries,
same "caught at implementation, not at Gate 1 impact assessment" shape.

`36rr` is the **next CLI-surface Slice C plan** (`agov gate change-check
--base-ref … [--registry …]`) exhibiting the identical gap. That recurrence
is itself evidence that `oty0`'s recommendation is still unadopted — `oty0`'s
own closing text says it "stays proposed until the check above is actually
added to Gate 1 plan-writing guidance … and used on at least one subsequent
CLI-surface plan." The subsequent CLI-surface plan happened, and the check
was not applied.

### Recommended disposition (a learning-record edit — its own MR/Gate, not this record)

1. **Fold `36rr` into `oty0` as a second corroborating incident** rather than
   creating a parallel learning entry. Add its plan
   (`docs/plans/…-ci-mandatory-needs-implication-correction.md` Slice C for
   `gate change-check`), its three literals, and MR !2373's own accompaniment
   work as the second data point. Two incidents in one week moves the
   evidence from "single incident" toward "recurring — adopt the check now."
2. **Capture the one detail `oty0` lacks:** `36rr` reports an *ordered* second
   failure — `--registry` was also missing from `KNOWN_FLAG_NAMES`, but the
   command-token assertion aborted before the flag assertion ran, so the
   first failing run under-reported the true scope. That is an independent,
   actionable refinement: `roi_redaction_audit_matches_real_clap_surface`
   should report *all* missing entries across every class in one run instead
   of aborting at the first, so an implementer sees the full corrected file
   list immediately. Worth a one-line note in `oty0`'s recommendation (or a
   small follow-up issue against the test).
3. Keep `oty0` `status: proposed`; the recurrence argues for prioritizing its
   adoption into planning guidance, not for a new artifact.

## Ack

`36rr`'s `requires-ack` is satisfied by this record. The lesson is sound and
accepted; routing it to strengthen `oty0` (via a learning-record MR) rather
than stand alone is the only change from what `36rr` proposed.
