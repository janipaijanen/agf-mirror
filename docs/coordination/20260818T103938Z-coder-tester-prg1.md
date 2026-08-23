---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260818T103938Z-coder-tester-prg1
timestamp: 2026-08-18T10:39:38Z
model: codex
immutable: true
references-to: []
topic: CL-4 immutable corpus binding implementation ready for independent exact-head review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Independently review MR !1712 exact head ad54b2eeeb69dc4f1d9b8f0fb27a88ffb554dbd9 against the approved CL-4 plan and binding governor decision.
    - Reproduce the 142 included and 50 individually excluded manifest-v1 accounting, its canonical digest, and historical verification at corpus revision b3a04178f149e287f82988ca90061a9153c13f3b.
    - Verify the blocking-versus-advisory CI boundary, first-15-line candidate predicate, absence of a control_artifacts class exemption, signer-facing evidence fields, and counsel/Gate-2 limitations.
    - Publish CLEAR or concrete findings to coder-tester; route any material authority or legal-sufficiency issue to governor-reviewer.
  must_read:
    - MR !1712 at exact head ad54b2eeeb69dc4f1d9b8f0fb27a88ffb554dbd9 and pipeline 2768789759
    - MR !1750 approved CL-4 plan and MR !1767 final planner handoff
    - MR !1768 binding Gate 1 decision 20260818T100504Z-governor-reviewer-7eph
    - LICENSES/commercial-corpus-manifest-v1.yaml and agf/schemas/commercial-corpus-manifest.schema.json on MR !1712
  do_not_touch:
    - .claude/settings.local.json
---

# CL-4 implementation complete; independent review required

The governor approved the final CL-4 plan and authorized implementation only.
The implementation is now published on MR !1712 at exact head
`ad54b2eeeb69dc4f1d9b8f0fb27a88ffb554dbd9`. Exact-head pipeline
`2768789759` passed all 20 required jobs; the two remaining Windows jobs are
explicitly optional/manual. The new `commercial-corpus:validate` job passed.

## Implemented boundary

The three implementation commits are:

- `b3a04178f149e287f82988ca90061a9153c13f3b`: schema, Rust generator,
  validator and historical verifier, CI job, adopter documentation,
  agreement evidence fields, tests, and rule-projection evidence;
- `64d2642d`: immutable manifest v1, current pointer, and licence-definition
  binding to digest
  `75695c3096e62cfc133590493dafff26efef972184e8ca25f9630dffdacfca52`;
- `ad54b2ee`: telemetry-redaction classification for the new CLI surface.

Manifest v1 records inventory source commit
`d13bb9084fce9a9b1c7c08bb735e0fe6d08d7e98`, corpus content revision
`b3a04178f149e287f82988ca90061a9153c13f3b`, 142 included paths, 50
individually excluded paths, and 192 candidates at that historical revision.
The current bound tree has 191 candidates because the licence file's former
dual-licence header expression was the one post-inventory excluded candidate;
the binding edit removes that expression. `verify-version` reproduces and
validates the 192-entry historical set.

Schema and manifest-shape defects block CI. Live drift and numbered-manifest
path mutation are advisory evidence, as approved; they do not acquire binding
authority by appearing in tooling. No `control_artifacts` field or categorical
future exemption exists. Each later candidate needs an individual disposition.

## Evidence already obtained

- full Rust workspace suite: pass, zero failures and two intentionally ignored
  tests;
- eight focused commercial-corpus tests: pass;
- schema/current-tree accounting and historical manifest-v1 verification:
  pass;
- CL-4 blocking rule projection: pass;
- GitLab expanded CI lint and local release build: pass;
- exact-head GitLab pipeline `2768789759`: success.

This Codex session authored the implementation and must not supply its required
independent contribution-auditor verdict. Prefer Claude or another independent
model/session and disclose the reviewer provenance. A clear review still does
not grant Gate 2, merge authority, licence-offer authority, agreement-execution
authority, release authority, or legal sufficiency. Counsel review remains
mandatory before commercial use.
