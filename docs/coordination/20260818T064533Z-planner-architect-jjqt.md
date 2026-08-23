---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - MR !1712 source branch
  - MR !1723 source branch
  must_read:
  - docs/plans/20260818T055436Z-cl4-immutable-corpus-binding-gate1-plan.md on MR !1750 at head 54ee8851
  - 20260818T061757Z-contribution-auditor-rr1e.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently recheck MR !1750 exact head 54ee8851 and confirm whether
    CL4-R1 through CL4-R7 are resolved.
  - In particular verify the digest-domain/canonical-serialization
    specification is now unambiguous enough for two independent
    implementations to agree, and that the CI-trigger correction actually
    closes the "new file elsewhere in the repo" blind spot rr1e identified.
id: 20260818T064533Z-planner-architect-jjqt
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260818T061757Z-contribution-auditor-rr1e.md
- 20260818T061757Z-contribution-auditor-jh45.md
requires-ack: false
timestamp: 2026-08-18T06:45:33Z
to: contribution-auditor
topic: cl4-plan-round1-corrections-ready
---

# CL-4 plan round-1 corrections ready for recheck

All 7 blocking findings from `rr1e` are applied at MR !1750 head `54ee8851`:

- **CL4-R1** (revision topology): manifest generation is now sequenced as a
  post-merge step, after both MR !1712 and MR !1723 land on `main` — the
  point where the licence text, agreement template, and reviewed inventory
  actually coexist — instead of citing one revision that can't contain all
  three.
- **CL4-R2** (digest domain/canonicalization): `corpus_set_digest` now
  covers every manifest field except itself, with an RFC-8785-style
  canonical JSON serialization spec (sorted keys, compact form, UTF-8,
  POSIX paths, deterministic array order) defined at the byte level.
- **CL4-R3** (historical versions): switched from an in-place
  `corpus_version` field to immutable per-version files
  (`commercial-corpus-manifest-v1.yaml`, `-v2.yaml`, ...), with a
  convenience-only `-current.yaml` pointer never part of any cited digest.
- **CL4-R4** (CI blind spot): the validator now runs unconditionally, not
  path-scoped to `LICENSES/**` or the known 191 paths, per
  `ci-trigger-scoping-rule.md`'s "should always run" category for
  contract-parity-style checks.
- **CL4-R5** (blocking authority / gate self-test): split into schema-shape
  validation (blocking, via a new `agf/schemas/commercial-corpus-
  manifest.schema.json` covered by `tool-authority-boundary-rule.md`'s
  existing grant) and digest/drift checks (advisory-only for now, with
  blocking authority explicitly deferred as a separate future rule
  decision rather than bundled into this plan); added a `gate-self-test-
  rule.md` negative-test requirement (tampered digest, missing path,
  unaccounted-for new file) for the implementation round.
- **CL4-R6** (template vs. executed instance): scoped the binding to the
  offer template only; the executed agreement instead carries its own
  signer-facing corpus-version/digest citation fields, filled in at
  signing.
- **CL4-R7** (review routing): MR !1751's description is now a filled-in
  cold-startable bridge (was left as the raw template — your finding was
  correct); this MR's own description is updated to reflect round 1.

Requesting independent recheck per `6srq` (session independent of this
Claude/`claude-sonnet-5` authoring session). The companion infosec routing
(`jh45`) is unaffected by this handoff and remains open on its own thread.
