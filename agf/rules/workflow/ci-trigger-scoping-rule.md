---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# CI Trigger Scoping: Long-Running Stages Must Be Path-Scoped
#
# Framework principle: "right tests for right changes; never waste CI
# resources on irrelevant test suites."
# See: framework.md — Section 2 (Build Engineer); Pattern 1 (Pipeline)

> **Provisional status.** Single-adopter evidence (silver-birch,
> 2026-04-18, framework v0.33.0 — issue #38). The pattern is
> well-known in CI/DevOps practice but has not previously been
> codified in the framework's `rules/`. Adopters with
> different CI systems (GitLab vs GitHub Actions vs Jenkins vs
> Buildkite vs CircleCI etc.) hitting this rule: please file a
> learning record so we can promote from provisional to recommended.

**Scope:** Every CI pipeline stage that takes longer than the
project's quick-feedback threshold (typical default: 5 minutes).

## The rule

Long-running CI stages MUST have **path-scoped trigger rules** so
they only execute when the changes actually affect the layer they
verify. Governance-only changes (agent definitions, learning records,
governance documentation) MUST NOT trigger application-layer test
suites.

## Why this rule exists

A blanket "run all tests on every MR" policy:

- Wastes CI compute (runner minutes, registry pulls, cache writes).
- Slows feedback loops — a 75-minute browser test suite blocks a
  one-line README fix.
- Burns out adopters who watch governance-only MRs sit in CI for
  over an hour.
- Trains adopters to expect long CI cycles, normalizing the waste.

The cost compounds with team size. silver-birch reported a 75-minute
Playwright e2e suite firing on every MR including governance-only
changes; for a 10-person team with 5 daily governance MRs that each
trigger a 75-min suite, that is 375 wasted runner-minutes per day —
plus opportunity cost from application MRs blocked behind the queue.

After path-scoping, governance-only MRs went from ~80 minutes to
1–2 minutes; application MRs unchanged; no regression escaped due
to scoping in the months following.

## What counts as "long-running"

Stages that typically warrant path-scoped triggers:

- **Browser / E2E tests** (Playwright, Selenium, Cypress, Puppeteer)
  — minutes to hours.
- **Load tests** (k6, JMeter, Locust, Gatling) — minutes to hours.
- **Container security scans** (Trivy, Grype, Snyk) — minutes.
- **Database migration verification against full datasets** —
  minutes.
- **Documentation site builds with screenshot regeneration** —
  minutes.
- **Mobile build pipelines** (Xcode / Gradle full build + signing).

Stages that should always run (no scoping):

- Compile / type check.
- Unit tests (sub-minute).
- Linter / formatter.
- Quick security scans (secret detection, basic SAST).
- Schema-parity / contract checks (the gap they catch is invisible).

## How to scope

Every major CI system supports path-based filters.

<!-- CUSTOMIZE: keep only your CI system's example -->

**GitLab CI:**

    browser-tests:
      rules:
        - if: $CI_MERGE_REQUEST_TARGET_BRANCH_NAME == "main"
          changes:
            - "src/**/*"
            - "templates/**/*"
            - "static/**/*"
            - "package.json"
            - "package-lock.json"

**GitHub Actions:**

    on:
      pull_request:
        paths:
          - 'src/**'
          - 'templates/**'
          - 'static/**'
          - 'package.json'
          - 'package-lock.json'

**Jenkins (Pipeline):**

    when {
      changeset pattern: "src/**/*", caseSensitive: true
    }

## What NOT to scope out

Per-stage trigger scoping has a failure mode: forgetting to include
a path that DOES affect the stage. Common mistakes:

- **Scoping out translation directories** when browser tests verify
  translations (translation changes do affect rendered text).
- **Scoping out lockfiles** (`package-lock.json`, `mix.lock`,
  `Gemfile.lock`, `Cargo.lock`) when dependency updates affect
  runtime behaviour.
- **Scoping out migration directories** when schema changes affect
  integration tests.
- **Scoping out CI configuration itself** — a change to the CI
  config should re-run the full suite at least once to verify the
  config still works.

When in doubt, **include the path**. The cost of an extra CI run is
much lower than the cost of a regression escaping to production
because a path was wrongly excluded.

## Trigger contexts: MRs vs. tags vs. manual

Long-running stages typically have three trigger contexts:

1. **MRs to main** — path-scoped, blocking on failure. This is where
   scoping matters most.
2. **Tag pipelines** (releases) — always run, advisory (the tag is
   immutable; failure cannot be retroactively gated).
3. **Manual trigger** — always available, advisory.

The MR context is where scoping discipline pays off. Tag and manual
contexts are explicit signals that the user wants the full suite —
do not scope them.

## Pairs with: build-engineer agent

This rule operationalises the build-engineer agent's existing design
principle ("right tests for right changes; never waste CI resources
on irrelevant test suites" — added in framework v0.30.0). The agent
*names* the concern; this rule *codifies the implementation pattern*.

## CUSTOMIZE

Adopters MUST customize:

- **Quick-feedback threshold** — what counts as "long-running" in your
  project (default: 5 minutes).
- **Path patterns per long-running stage** — depends on your
  directory layout and which paths drive each stage.
- **Always-run path list** — lockfiles, migration directories,
  translation directories, CI config itself.

Adopters MAY customize:

- Per-stage manual-trigger fallback (some projects always want a
  manual override available even when paths don't match).
- Cross-pipeline coordination (e.g., browser tests skip if unit
  tests in the same pipeline failed).
- Time-budget alerts (CI job that flags any pipeline exceeding a
  ceiling, separate from the path-scoping rule).

## Related rules

- `workflow/tdd-rule.md` — TDD discipline and CI scoping are
  complementary: TDD ensures tests exist; scoping ensures the right
  ones run.
- `workflow/systematic-code-review-rule.md` — Gate 2 reviewers
  should ask "did this MR really need to run the full e2e suite?"
- (Project-specific) `domain/ci-pipeline-rule.md` — adopters may
  keep stack-specific CI details in a project-domain rule.
