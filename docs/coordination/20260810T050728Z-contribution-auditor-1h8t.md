---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260810T050728Z-contribution-auditor-1h8t
timestamp: 2026-08-10T05:07:28Z
model: Codex GPT-5
immutable: true
references-to:
  - "https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/102"
  - "20260810T045847Z-contribution-auditor-t1jj"
topic: v251-issue102-elixir-exclusion-removal-planning
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - "Author a new Gate 1 child plan for issue #102 targeting v2.5.1."
    - "Define a separate v2.5.1 release umbrella that may collect only independently approved child implementations and verified due-diligence remediations."
  must_read:
    - "https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/102"
    - "docs/plans/v2.4.5-elixir-downstream-exclusion.md"
    - ".gitlab-ci.yml"
    - "agf/dist/release-manifest.yaml"
    - "tooling/rust/scripts/check-no-elixir-artifacts.sh"
    - "docs/coordination/20260810T045847Z-contribution-auditor-t1jj.md"
  do_not_touch:
    - ".claude/settings.local.json"
    - "release/2.5"
    - "release-2.5.0"
---

# Governor direction and release target

The human Governor has directed planning for removal of the legacy
Elixir/Mix release-payload exclusion build step under issue #102, targeting
v2.5.1. The same release should incorporate corrections arising from the
requested fresh-context due-diligence assessment, but only after those
findings exist, are fact-checked, and each remediation is independently
authorized.

This handoff authorizes planning, not implementation, Gate 1 approval, Gate 2
approval, version mutation, release branching, or publication.

# Required plan decomposition

Use a child-plan/umbrella structure:

1. A dedicated issue-#102 Gate 1 child plan owns the CI and packaging-policy
   change. It must be reviewable and reversible independently of unrelated
   due-diligence findings.
2. The v2.5.1 umbrella may collect only child implementations that have their
   own valid Gate 1 and Gate 2 trails. It must not pre-authorize an unknown
   bundle by citing the premature same-session memorandum in draft MR !1366.
3. The genuine due-diligence run remains separately gated. Its verified
   findings must be converted into bounded child plans or explicit no-change
   dispositions before they enter v2.5.1 scope.

# Issue #102 minimum plan contract

The issue-#102 plan must, at minimum:

- Inventory every current enforcement point, not only
  `release:elixir-exclusion-selftest`: the preview AGF archive, tagged AGF
  archive, assembled release, and `agov` archive calls to
  `check-no-elixir-artifacts.sh check`; the script and its self-test; trigger
  rules; documentation; and release acceptance evidence.
- Freeze an explicit disposition for the standalone CI job, all archive-check
  calls, and the script. Removing only the standalone self-test while retaining
  the deny-list checks does not establish issue-#102 completion.
- Prove, from clean distribution and release staging areas, that
  `agf/dist/release-manifest.yaml` and the applicable `agov` archive assembly
  are closed allowlists that independently exclude every currently forbidden
  class: `tooling/elixir/**`, `mix.exs`, `mix.lock`, `*.ex`, and `*.exs`.
- Add reverse negative fixtures for every forbidden class and for nested,
  renamed, path-normalized, tar, and zip cases. The fixtures must first prove
  the allowlist rejects or omits the injected artifact without invoking the
  legacy deny-list script, then prove removing the script/job does not reduce
  the resulting protection.
- Include a mutation-style control: deliberately broaden or bypass the
  allowlist in a test fixture and demonstrate that the test suite detects the
  coverage loss. A test that passes only because no Elixir file happens to be
  present is insufficient.
- Compare pre-change and post-change release contents byte-for-byte or by a
  deterministic manifest, apart from evidence artifacts explicitly authorized
  by the plan. This is a CI-policy simplification, not permission to change the
  shipped payload.
- Preserve historical records and language-neutral tooling-conformance claims.
  No live Elixir implementation is reintroduced, and no broader language-policy
  decision is implied.
- Classify downstream impact across foundry, direct adopters, and downstream
  consumers; provide migration evidence and the exact Gate 2 `Adopter docs
  parity` outcome required by the active rule.
- Assign implementation of `.gitlab-ci.yml`, packaging jobs, scripts, and
  release evidence to build-engineer. Assign exact-head plan and implementation
  review to contribution-auditor. Binding decisions remain with
  governor-reviewer.
- Require a sole green exact-head pipeline, clean-archive tests, fresh tar/zip
  inspection, release-path validation, and the complete release acceptance
  matrix before Gate 2.
- State that v2.5.1 version/tag/release work requires a separate approved
  release-publication plan after all selected child work is merged and
  validated on main.

# Due-diligence boundary

Record `20260810T045847Z-contribution-auditor-t1jj` blocks the original
same-session MR !1366 method at its reviewed head. MR !1366 has since advanced
and remains draft, but editing that artifact cannot create the requested
context discontinuity. Do not treat its present findings as an approved v2.5.1
backlog.

The replacement assessment must use a genuinely fresh context, public Pages,
public source, public issue/MR state, and published release artifacts. It must
state that it is a simulated consultancy-style assessment rather than an
independent KPMG, EY, or McKinsey attestation. Fact-checker must verify factual
and comparative claims before remediation planning.

# Requested next visible state

Publish a planner-owned Gate 1 plan branch/MR and a planner-authored handoff to
contribution-auditor and governor-reviewer. The MR description must identify
its source branch, exact plan artifact, issue #102, v2.5.1 target, consultations,
validation commands, residual risks, and the due-diligence dependency boundary.

