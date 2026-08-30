---
kind: decision-record
from: governor-reviewer
to: all
id: 20260830T214141Z-governor-reviewer-m2pr
timestamp: 2026-08-30T21:41:41Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260828T061002Z-coder-tester-eqq3.md
  - 20260828T070637Z-coder-tester-k4mk.md
  - 20260828T080250Z-coder-tester-g586.md
  - 20260828T143959Z-coder-tester-gkah.md
topic: v270-rc1-abandoned-permanently
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.7.0-rc.1 is abandoned permanently

Binding decision, given by the governor-reviewer in session ("skip that
2.7.0-rc.1, never return") and transcribed by Claude. The decision is the
governor's; the transcription and the boundary notes below are mine.

## Decision

The v2.7.0-rc.1 release candidate is **abandoned**. It will not be resumed,
re-attempted, or revisited. The OCI CVE publication block recorded in `eqq3`
and the security-correction governance question in `k4mk` are closed as moot:
the artifact they gate is not shipping.

No further work — investigation, remediation, pipeline reruns, or publication
attempts — is authorized against v2.7.0-rc.1 by any role.

## What abandonment does NOT mean

**The `v2.7.0-rc.1` tag stays exactly where it is.** It is pushed and
therefore immutable under `agf/rules/workflow/immutable-tags-rule.md`:
annotated tag `eb9a59372755104097ca59c9b9cf456a398e83f0`, peeling to commit
`25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`. That rule has no exceptions and is
not weakened by abandonment. Abandonment is a **lifecycle state**, not artifact
removal.

Anyone reading "never return" as authorization to delete or move the tag, or
to prune the branches below, is reading it wrong. Nothing here authorizes a
destructive operation, and any such operation would need its own governor
decision and the pre-flight in
`.claude/rules/workflow/destructive-op-preflight-rule.md`.

Retained and not to be touched:

- tag `v2.7.0-rc.1` (immutable, stays)
- `refs/heads/reconstruct/v2.7` (`25e6d752…`)
- `refs/heads/release-v2.7.0-rc.1` (`c39f9bcc…`)
- the v2.7 evidence refs `gkah` already lists under `do_not_touch`

Verified at decision time: **no GitLab Release object exists for
`v2.7.0-rc.1`.** Publication was stopped before that step, so there is no
published Release, and none will now be created. The tag exists without a
Release, which is the correct terminal state for an abandoned candidate — the
history stays honest about what was attempted.

## Deliberately not decided here

`refs/heads/release-v2.7.0-rc.2` (`7b214431…`) exists, and `g586`
acknowledged rc.2 construction. The governor's instruction named rc.1
explicitly and did not mention rc.2.

I am **not** inferring rc.2's disposition from it. Two readings are available —
abandon rc.1 alone and let rc.2 proceed, or treat the whole v2.7.0 line as
dead — and they lead to materially different work, so guessing is not
appropriate for a release decision.

Context a decision-maker should have: **v2.8.0 is already published and
accepted** (`prn4`, `4rh7`), which makes a subsequent v2.7.0 release of
questionable value, and the active release effort is the v2.9.0 work order in
`gkah`. That points toward the whole-line reading, but it is not my call.

This question is carried forward as an explicit input to the v2.9.0 roadmap
plan, where it needs a governor answer before that plan's scope can be called
complete. Until then, no work proceeds on rc.2 either — not because it is
abandoned, but because its status is undecided and starting would presume the
answer.

## Effect on the roadmap

v2.7.0 is removed from the shipping path. The next release under preparation
remains v2.9.0 per `gkah` and the clustering baseline approved in `nsne`.
