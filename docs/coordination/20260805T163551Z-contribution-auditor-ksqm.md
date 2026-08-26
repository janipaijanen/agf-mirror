---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260805T163551Z-contribution-auditor-ksqm
timestamp: 2026-08-05T16:35:51Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T153006Z-coder-tester-nq7h.md
  - 20260805T150801Z-contribution-auditor-h7rq.md
topic: mr1193-readme-slice-gate2-rereview
requires-ack: true
artifacts_owned: []
---

# MR !1193 renewed contribution audit at exact head `cf066993`

Scope: complete re-review of `B-MR1193-CA-1/2/3` at exact head
`cf0669930ac5af3aed3dfb4b0f993c16fc40c393`.

Coverage: **COMPLETE**

Recommendation: **BLOCK**

Blocking findings: **1**

Advisory findings: **0**

Open findings with accepted risk: **0**

Authority: recommendation only; the governor-reviewer makes the binding Gate 2
decision.

## Finding disposition

- **`B-MR1193-CA-1`: resolved.** The opening now restores Decision 4's
  selected Variant A verbatim: “Plans reviewed before code. Code reviewed
  before merge.” The accompanying sentence is bounded to repository-native
  files and explicitly says AGF is not a hosted service, runtime, or coding
  agent. The earlier “every AI-implemented change” absolute is gone.
- **`B-MR1193-CA-2`: resolved.** “Not this” now immediately follows the
  three-line title/one-liner. The approved triage sequence runs through Deep
  dive before the byte-identical public-feature-summary, flyer, disclosure,
  and gate flow. The summary invariant still passes and no Pages file changed.
- **`B-MR1193-CA-3`: remains open.** The unsupported numeric heading is
  correctly renamed “Quick install,” but the new `curl` command is not a valid
  GitLab release-asset URL and therefore does not satisfy the requested actual
  pinned command/path.

## B-MR1193-CA-3 residual — install command returns HTTP 404

Check: executable pinned install path

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `B-MR1193-CA-3`

Impact: material

Enforcement: blocking

Basis: `verifiable-claims-rule.md`, Decision 5a, and the prior finding's
resolution condition

Finding state: open

Risk disposition: none

The README command uses a GitHub-style route and adds `v` to the archive
filename:

```text
https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/download/vX.Y.Z/agf-vX.Y.Z.tar.gz
```

Substituting the published stable release `v2.4.11` was tested directly and
returned HTTP 404:

```text
https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/download/v2.4.11/agf-v2.4.11.tar.gz
```

The project Releases API confirms the actual asset is named
`agf-2.4.11.tar.gz` and currently points to the generic package URL. This
tested URL returned HTTP 200:

```text
https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.4.11/agf-2.4.11.tar.gz
```

Resolution condition: replace the command with the verified project asset
pattern (tag/package path `vX.Y.Z`, archive filename `agf-X.Y.Z.tar.gz`) or
remove the unverified direct command and route the reader through a genuinely
pinned asset link from the Releases page. Test the substituted current stable
version for HTTP 200 before requesting another review.

## Verification evidence

- Reviewed the full one-commit correction diff `45052ebc..cf066993`; only
  `README.md` changed.
- `agov plan validate docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`:
  pass.
- `agov validate adopter-docs`: pass.
- `python3 -m unittest tooling.tests.test_public_feature_summary`: 2/2 pass.
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3 pass.
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass.
- GitLab pipeline `2734479827`: success and sole exact-head pipeline for
  `cf066993` at review time.
- Direct HTTP probe of the proposed stable-version URL: 404.
- Direct HTTP probe of the Releases-API-reported stable asset URL: 200.

Request: correct only the residual install URL/path, verify it with a published
version, run exact-head acceptance, and request a focused re-review of
`B-MR1193-CA-3`. The ordered Pages theme/hero slice remains held.
