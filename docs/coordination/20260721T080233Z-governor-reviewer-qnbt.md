---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T080233Z-governor-reviewer-qnbt
timestamp: 2026-07-21T08:02:33Z
model: human-governor-via-codex-session
immutable: true
references-to:
  - 20260721T073926Z-planner-architect-rgg6
  - 20260721T074951Z-contribution-auditor-rgbr
  - 20260721T075136Z-governor-reviewer-dpm8
  - 20260721T075344Z-planner-architect-tphp
topic: v2-3-0-release-preparation-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/v230-release-preparation.md
---

# Gate 1 approved: v2.3.0 release preparation

The human governor stated **"!296 approved"** in the active session after
reviewing the revised plan and the disposition of Claude / Contribution
Auditor's findings.

Gate 1 is approved for `docs/plans/v230-release-preparation.md` at head
`ea9d6e5`, including:

- the frozen release scope and explicit exclusions;
- the binding prepared/merged/unpublished disposition for v2.2.6;
- the observable release-state chain through public artifact verification;
- pre-tag packaged-binary, provenance, OCI, vulnerability, and negative-gate
  evidence;
- immutable-tag and push-timeout recovery rules; and
- post-v2.3 enforcement follow-ups that must not delay v2.3.0.

Implementation may begin only after this approved plan is merged. Gate 2 for
the release implementation remains a separate governor decision on the future
release MR. The immutable tag may be created only under the approved
release-via-MR sequence.
