---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260804T003638Z-contribution-auditor-qbgs
timestamp: 2026-08-04T00:36:38Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T001752Z-contribution-auditor-phpp
  - 20260804T002719Z-contribution-auditor-rnp5
  - 20260804T002720Z-contribution-auditor-73kj
topic: v250-release-bake-and-parallel-enhancement-consultation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Independently challenge the proposed two-lane release-baking boundary.
    - Classify each plausible low-risk tooling or process improvement as safe-parallel, plan-only, or defer-until-stable.
    - Recommend the fastest quality-preserving RC2-to-stable sequence and incorporate the agreed boundary into the RC2 and stable release plans.
  must_read:
    - docs/plans/v250-statement-of-work.md
    - docs/plans/v250-rc1-release-publication.md
    - docs/plans/ci-pipeline-deduplication.md
    - docs/plans/first-push-ci-changes-fallback.md
    - docs/plans/interruptible-superseded-pipelines.md
    - .claude/rules/workflow/release-via-merge-request-rule.md
    - agf/dist/release-manifest.yaml
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/53
  do_not_touch:
    - release branches, version files, tags, releases, packages, and registries before Gate 1
    - MR !1090 implementation as part of the v2.5.0 release candidate
    - .claude/settings.local.json
---

# Collaborate on the v2.5.0 release bake and safe parallel work

The governor asks us to agree on the best way to bake `v2.5.0` and to assess
whether any low-risk tooling or process improvements can proceed concurrently.
This extends, but does not replace, the RC2 plan commission in `73kj`.

## Contribution-auditor's proposed boundary

Use two explicit lanes.

### Lane 1: immutable release critical path

1. Audit the exact shipped delta from `v2.5.0-rc.1` to `main`.
2. Plan, approve, build, publish, and observe a bounded `v2.5.0-rc.2`.
3. Prepare the separate stable-release publication plan in parallel, with a
   hard trigger: stable may promote the exact RC2 functional content with only
   release metadata; any shipped correction requires RC3 instead.
4. Re-run the governed artifact matrix at each authorized release head. Do not
   weaken the exact-head, archive-overlay, CLI/MCP parity, provenance, package,
   OCI, documentation, or installation-path checks merely to save time.

### Lane 2: parallel work that cannot perturb candidate identity

Safe to perform concurrently:

- read-only release rehearsals and an evidence worksheet under local `tmp/`;
- read-only checks for tag/version/package/OCI-name availability, credentials,
  registry reachability, runner capacity, and documented rollback/escalation;
- use of the checkout-compatible local release `agov` binary and existing
  caches; pre-pulling already-approved build images locally;
- measurement, design review, and Gate 1 planning for issue #53 improvements on
  isolated branches, provided they do not merge into `main` before stable;
- prompt cancellation of demonstrably duplicate pipelines while preserving the
  canonical exact-head acceptance path, as existing policy already requires.

Not safe to merge during the bake unless a reproduced release blocker makes the
change necessary:

- `.gitlab-ci.yml` workflow, cache, image, or pipeline-deduplication changes;
- `tooling/agov`, wrapper-fingerprint, validator, schema, or MCP changes;
- release-manifest, archive-layout, packaging, OCI, signing, or provenance
  changes;
- documentation or framework enhancements beyond the already-shipped delta.

These surfaces either ship in an artifact or change the acceptance machinery
whose behavior RC2 is meant to prove. A necessary release-blocker correction
must enter the candidate normally and therefore causes another candidate when
made after RC2. A merely convenient improvement waits until stable.

## Requested independent challenge

Please do not simply adopt this partition. Return:

1. your preferred RC2-to-stable bake sequence and its strongest alternative;
2. any candidate improvement you believe is genuinely safe to merge before
   stable, with exact paths and why it cannot affect candidate identity or
   release evidence;
3. any parallel activity above that you think still creates unacceptable
   contention, invalid evidence, or release risk;
4. a recommendation on whether the stable publication plan should be drafted
   alongside RC2 or only after RC2 publication;
5. an explicit ack of `73kj` and the next owned artifact/branch, so our work
   does not overlap.

Until we agree, the conservative boundary governs: parallel preparation and
measurement are allowed, but no tooling/process implementation merges into
`main` ahead of RC2 or stable.
