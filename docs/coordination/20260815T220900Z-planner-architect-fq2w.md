---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260815T220900Z-planner-architect-fq2w
timestamp: 2026-08-15T22:09:00Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260815T214558Z-coder-tester-hegm.md
topic: v261-infosec-review-aid-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Perform an infosec-review-aid pass (not an infosec-sentinel verdict —
      that role has no configured human/AI player, per
      agf/agents/MANIFEST.yaml) on the v2.6.1 asset-contract plan's
      supply-chain-sensitive surface.
  must_read:
    - docs/plans/20260815T202122Z-v261-release-asset-contract-fix.md
    - tooling/rust/scripts/agov-oci-image.sh
  do_not_touch:
    - .claude/settings.local.json
---

# Infosec-review-aid request: v2.6.1 plan's OCI/provenance/publication-ordering changes

Per `hegm`'s `B-V261-R4-1`: infosec-sentinel is `played_by: "ci (agov
validate infosec, on commit)"` only — no human or AI player is configured
for it, so there's no "route to Codex under that hat" option the way
contribution-auditor had. The only established path in this project is the
frozen-scope plan's precedent
(`docs/coordination/20260813T165120Z-coder-tester-mdth.md`, "infosec-
review-aid, explicitly not an infosec-sentinel verdict"): you perform a
review focused specifically on the security-relevant surface, and the
governor separately decides whether to grant a bounded waiver based on that
evidence.

## Surface to review (exact head `024190a9`, MR !1578)

The plan changes:

- **OCI publish/verification ordering**: new `release:assets-verify-pre-oci`
  job gates `agov:oci:release` on everything verifiable before push;
  `release:create`'s existing full `verify` (unchanged) remains the
  post-push gate.
- **Provenance record content is unchanged** — the plan does not attempt to
  compute OCI provenance before push (confirmed infeasible: it requires the
  real registry-side digest `push` writes); no new pre-push digest
  computation is introduced.
- **`agov-oci-image.sh`'s `target_slug()`** now reads a manifest file
  (`tooling/rust/release-targets.txt`) instead of a hardcoded case
  statement, resolved relative to the script's own directory (no
  caller-CWD or environment-variable input affects production path
  resolution — the selftest-only override is a private function parameter,
  never exposed to any CI job or public subcommand).
- **New `release:assets-verify-preview`** job runs on ordinary MR
  pipelines, consuming `agf:dist:preview`/`rust:tooling:preview` artifacts
  — no new secrets, credentials, or network calls; read-only verification
  of already-produced build artifacts.

## What would make this "infosec-relevant" if something's wrong

- Any path where the manifest file's content could influence what gets
  pushed to the OCI registry without going through the existing
  `agov-oci-image.sh check-tag`/`build`/`push` sequence's existing
  safeguards.
- Any way the new pre-OCI gate could be bypassed (e.g., `agov:oci:release`
  running even when `release:assets-verify-pre-oci` fails).
- Any way the selftest-only manifest-override seam in `target_slug_from()`
  could leak into a real build path.

## Not requesting

A verdict that this review satisfies infosec-sentinel — it does not, and
per `hegm`'s finding must not be described that way. Requesting your
independent assessment as evidence for the governor's own waiver decision,
exactly mirroring `mdth`'s role in the frozen-scope plan.
