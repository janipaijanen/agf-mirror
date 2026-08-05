---
id: drop-aarch64-linux-required-target
status: pending-gate1
topic: drop-aarch64-linux-required-target
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Governor-reviewer gave this instruction directly and explicitly
      ("I think until we get a sponsor we drop aarch64-musl as required
      target") in response to MR !1167's CI pipeline blocking on
      persistent runner disk exhaustion during the aarch64-musl
      cross-compile step. Filing this plan per uncertainty-and-scope-rule
      (>2 modules touched) before implementing; requesting review before
      merge per the standard MR workflow, not as a Gate 1 plan gate this
      instruction already cleared verbally.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      CI target-matrix and release-artifact-inventory change only; no
      credential, auth, network-trust-boundary, or secret handling.
---

# Plan: drop `aarch64-unknown-linux-musl` as a required/mandatory build target

## Intent

MR !1167's pipeline (exact head `0ad8d69e`, and its retry) failed twice
in `rust:tooling:preview` on two different GitLab shared
`saas-linux-small-amd64` runner instances (`5-green` and `2-green`),
both with an identical `no space left on device` error while pulling the
`aarch64-unknown-linux-musl` cross-compilation Docker image
(`ghcr.io/cross-rs/aarch64-unknown-linux-musl:main`). The same job
definition succeeded on 2026-08-04. Because `rust:tooling:preview` runs
in the `build` stage, which GitLab's default stage ordering runs before
`test`, its failure blocks the entire `test` stage (clippy, unit tests,
`cargo deny`, `agov validate`) from running at all — even though those
checks are unrelated to the aarch64 cross-build.

The governor-reviewer gave a direct instruction in response: drop
`aarch64-unknown-linux-musl` as a required target until CI runner
capacity is sponsored. This plan implements that instruction narrowly —
it removes the target from the loop that both `rust:tooling:preview`
(MR/main preview, currently blocking) and `rust:tooling:release`
(tag-time mandatory release archives) extend via the shared
`.rust-tooling-package` template, so the fix removes the blocking
failure at its actual cause (the shared build/loop, not either job
individually) with a single, minimal change.

## Downstream-impact classification

Per `always/downstream-impact-rule.md`: **BREAKING**, not EDITORIAL —
`.gitlab-ci.yml` is a trigger-path artifact, and this removes a
currently-mandatory published release artifact
(`agov-X.Y.Z-linux-aarch64.tar.gz` and its checksum/provenance sidecars)
that has shipped on every tag release since v2.4.7
(`CHANGELOG.md` [2.4.7]: "Tag pipelines now publish framework archives,
Linux x86_64/aarch64 Rust tooling...").

### Three-circle impact statement

```
Impact:
  Foundry:          Yes. Foundry's own release pipeline stops producing
                     an aarch64-linux tooling archive from the next
                     tagged release forward. No other Foundry-internal
                     workflow depends on the aarch64 archive existing.
  Direct adopters:  BREAKING. An adopter deploying agov on Linux arm64
                     hosts (e.g. AWS Graviton, Raspberry Pi class
                     hardware, Apple Silicon under Linux VMs) who
                     expects a prebuilt aarch64 tooling archive on
                     future GitLab Release pages will not find one until
                     this is reinstated. Already-published releases
                     (v2.4.7 through the current v2.5.0-rc.2 line) keep
                     their existing aarch64 archives; nothing already
                     published is retracted.
  Downstream:       No schema, CLI flag, or MCP behavior changes. Only
                     the release-artifact inventory going forward
                     changes; `agf/dist/release-manifest.yaml` and
                     `agf/schemas/release-provenance.schema.json` are
                     untouched (neither references target platforms by
                     name).
  Migration:        Adopters needing arm64 Linux tooling before this is
                     reinstated must build `agov`/`agov-mcp` from source
                     for `aarch64-unknown-linux-musl` (or
                     `aarch64-unknown-linux-gnu`) themselves; this is
                     already how any unlisted target must be obtained
                     today (Windows preview is the existing precedent
                     for "not part of the mandatory tag contract").
  Evidence:         MR !1167 exact-head pipeline failures
                     (job 15717814688, runner
                     5-green.saas-linux-small-amd64; job 15718329798,
                     runner 2-green.saas-linux-small-amd64 — both
                     "docker: failed to register layer: write
                     /usr/local/bin/qemu-aarch64: no space left on
                     device"), and the governor-reviewer's direct verbal
                     instruction in this session on 2026-08-05.
```

## Deprecation-policy applicability

Per `workflow/deprecation-policy-rule.md`: this project is in **Phase
0** (`agf/learnings/foundry.yaml`:
`customer_state.paying_customers: false`), so this BREAKING change may
ship without a deprecation cycle. Requirements even in Phase 0, applied
here:

- Disclosed plainly in `CHANGELOG.md` and `agf/CHANGELOG.md` under a
  `Changed` (breaking) heading, not folded into `Added`.
- One-sentence reason for no deprecation cycle: zero known external
  dependents on the aarch64-linux tooling archive today (pre-customer
  project; see `customer_state` above), and the removal is
  resource-driven (CI runner disk capacity), not a design change users
  need advance warning to adapt to.
- No softening language ("improved", "simplified") — stated plainly as
  a temporary removal pending sponsored CI capacity.

## Rule-projection screen

Per `documentation-rule-projection-rule.md`, this screen applies to
changes to **normative** guidance in a Pages page or runbook.
`agf/docs/site/reference/release-artifacts.md` is a factual artifact
inventory (what files a release publishes), not MUST/SHOULD-style
normative guidance — classified `not normative` for this rule's
purposes. The six-row matrix is not required; the page is still
corrected for factual accuracy (see "Affected files" below), since
`always/verifiable-claims-rule.md` requires the page's claims to match
what CI actually produces.

## Affected files

1. **`.gitlab-ci.yml`** — in the `.rust-tooling-package` template's
   script, change the loop `for target in x86_64-unknown-linux-musl
   aarch64-unknown-linux-musl; do` to build only
   `x86_64-unknown-linux-musl`. This is the single source both
   `rust:tooling:preview` (MR/main preview builds) and
   `rust:tooling:release` (tag-time mandatory archives) extend, so one
   change removes the target from both the currently-blocking preview
   path and the release contract. Leave the `case "$target" in ...
   aarch64-unknown-linux-musl) slug="linux-aarch64" ;;` mapping line in
   place (harmless, documents the historical slug, trivial to reinstate
   by adding the target back to the loop when sponsored).

2. **`agf/docs/site/reference/release-artifacts.md`** — remove the three
   `linux-aarch64` rows from the "Artifacts produced by a release build"
   table (currently described as "Mandatory"); update the "Tooling
   archive filenames use a short public platform slug" paragraph to drop
   `linux-aarch64` / `aarch64-unknown-linux-musl` from the two
   parenthetical lists; add one sentence noting aarch64-linux tooling
   archives are not currently published, pending sponsored CI capacity
   for the cross-compilation build. Tone: practical/operational per
   `marketing-tone-rule.md`'s Business-IT/admin row — state the fact and
   the reason, no apology, no vendor-negative framing (the cause is
   Foundry's own current CI resource tier, not any named vendor's
   limitation).

3. **`CHANGELOG.md`** and **`agf/CHANGELOG.md`** — add an `## [Unreleased]`
   `### Changed` (breaking) entry stating aarch64-linux tooling archives
   are temporarily dropped from the mandatory release-asset contract,
   pending sponsored CI runner capacity, with the one-sentence
   Phase-0 no-deprecation-cycle reason per the section above.

## Explicitly out of scope

- Any change to `agf/dist/release-manifest.yaml` or
  `agf/schemas/release-provenance.schema.json` — neither references
  target platforms by name; no change needed.
- Adding a manual/optional aarch64-linux preview job (mirroring the
  existing Windows-preview pattern). Not requested; would reintroduce
  the same disk-pressure risk on the same small-amd64 runner class
  whenever manually triggered. If wanted later, that is a separate,
  reviewable follow-up once CI capacity is actually sponsored.
- Any change to MR !1167's own commits (#84/#85/#89/#76) — this plan's
  branch is independent and only removes the CI blocker those commits
  were hitting.
- Reinstating the target — tracked as future work, triggered by
  sponsored CI capacity becoming available; not scheduled here.

## Test strategy and verification

CI-configuration and documentation change; no unit/integration tests
apply. Verification:

1. `.gitlab-ci.yml` YAML remains valid (CI lints it on push; no local
   YAML linter is wired into this repo beyond that).
2. Push this branch's own MR and confirm `rust:tooling:preview` builds
   only `linux-x86_64` and passes without the aarch64 Docker pull step
   — this is itself the negative-to-positive proof that the CI blocker
   is resolved.
3. `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` from
   the repository root, confirming the edited release-artifacts.md page
   still builds.

## Risks

- Silent scope creep into a full CI restructure (e.g. reordering
  `build`/`test` stages so an unrelated build-stage failure can never
  again block the test stage) — that stage-ordering coupling is a real,
  separate CI-design concern this plan does not fix; flagging it as a
  candidate follow-up, not resolving it here, since this plan's fix
  (removing the failing step from the mandatory path entirely) already
  resolves MR !1167's immediate block without touching stage topology.

## Rollback

Single-line loop change plus a documentation/changelog correction.
Reinstating aarch64-linux is a one-line revert of the loop change (add
the target back) plus restoring the three table rows and two
parenthetical-list entries in `release-artifacts.md` — no schema,
code, or irreversible state change either direction.
