---
kind: decision-record
from: governor-reviewer
to: planner-architect
seq: 325
timestamp: 2026-06-14T19:14:19Z
immutable: true
references-to:
  - 00165-coder-tester.md
  - 00166-coder-tester.md
  - 00166-planner-architect.md
  - 00275-planner-architect.md
topic: v2-tooling-distribution-python-premise-correction
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
---

Scribed governor-reviewer decision from direct instruction.

The premise in seq 165/166 and the later seq 275 response is wrong for AGF-NG
v2.0: there is no Python v2.0 tooling distribution path to plan around.

Confirmations:

1. **No Python tooling for v2.0.** AGF-NG v2.0 tooling is Elixir/Mix-based.
   Do not write a v2.0 Gate 1 plan that builds Python binaries, PyInstaller
   artifacts, Python installer scripts, Python SBOMs, or Python package
   distribution for adopter tooling.
2. **No v2.0 binary tooling commitment yet.** The next correct planning target
   is Elixir tooling distribution. The plan may evaluate Mix archives,
   escripts, Burrito-style releases, container images, or documented
   source/Mix invocation, but it must not assume a Python `uvx agents-govern`
   path for v2.0.
3. **v1.x Python maintenance is maintenance-only.** The Python
   `agents-govern` 1.x line may receive critical/security fixes and narrow
   compatibility repairs while adopters migrate, but it is not the feature
   track for AGF-NG v2.0. Do not add new packaging formats, platform binary
   matrices, installer scripts, SBOM infrastructure, or release-distribution
   roadmap work to v1.x unless governor-reviewer explicitly opens a separate
   maintenance plan.

Seq 165/166 should be closed as superseded for AGF-NG v2.0 planning. A fresh
Gate 1 plan may now be written for v2.0 Elixir tooling distribution, with
supply-chain requirements scoped to the Elixir artifact shape actually chosen.

The v2.0 content/framework source archive path remains separate from tooling
distribution. Framework content bundles must not vendor tooling binaries.
