# Rule index: rules excluded from always-loaded context

Foundry-local pointer file. It adds no rules and grants no authority. The files
below stay in `agf/rules/` (and as symlinks under `.claude/rules/`) but are
excluded from startup loading by `claudeMdExcludes` in `.claude/settings.json`.
Read the named file with the Read tool BEFORE doing the matching work. Plan
`20261022T205113Z-instruction-context-reduction` and issue #138 explain why.

## Situational rules

| When you are about to... | Read first |
|---|---|
| prepare a release, bump `agf/VERSION`, tag, publish, or edit `release:*` / `agov:oci:release` jobs | `agf/rules/workflow/release-via-merge-request-rule.md` |
| act under a delegation, review a Gate 2 "deputy" decision, or write a delegation artifact | `agf/rules/workflow/deputy-governor-rule.md` |
| write a plan, or edit a Pages page, runbook, `AGENTS.md`, or other AI loader, that adds, changes, or removes MUST/SHOULD/blocking/approval wording (this needs a six-row projection manifest in `docs/reviews/rule-projection/`) | `agf/rules/workflow/documentation-rule-projection-rule.md` |
| rename, deprecate, or remove a CLI flag, schema field, MCP field, or config key | `agf/rules/workflow/deprecation-policy-rule.md` |
| add or change a long-running CI stage or its `rules:`/`changes:` paths | `agf/rules/workflow/ci-trigger-scoping-rule.md` |
| work in a series that declares a `convergence` coordination object | `agf/rules/workflow/review-convergence-rule.md` |
| recommend commands or config for a specific host, runner, or container | `agf/rules/always/environment-interrogation-rule.md` |
| describe what the project can do (maturity claims, capability tables) | `agf/rules/docs/implementation-honesty-rule.md` |
| write a merge request description or run `glab mr create/update` | `agf/rules/workflow/merge-request-description-rule.md` |
| open, update, or merge any merge request to a protected branch | `agf/rules/workflow/merge-request-workflow-rule.md` (core: never commit to `main`) |
| create, move, delete, or retag a git tag | `agf/rules/workflow/immutable-tags-rule.md` |
| upgrade or add a dependency or run a package manager | `agf/rules/workflow/no-auto-package-update.md` |
| choose or bump a version number | `agf/rules/workflow/semver-no-carry-rule.md` |

## Specialized surfaces (load when the task matches)

| Topic | Read first |
|---|---|
| generated visible output (drawings, PDFs, charts) | `agf/rules/workflow/visible-artifact-verification-rule.md` |
| new or moved persisted attributes | `agf/rules/workflow/concept-modelling-rule.md` |
| generating files for third-party viewers | `agf/rules/workflow/file-generator-rule.md` |
| database migrations | `agf/rules/workflow/migration-generation-rule.md` |
| create/update/delete audit trails | `agf/rules/always/audit-log-completeness-rule.md` |
| parsing structured data from outside the project: external APIs, webhooks, library callbacks, OAuth/OIDC, RPC responses, queue messages | `agf/rules/workflow/boundary-shape-testing-rule.md` |
| write or edit any `docs/**/*.md`, `README.md`, or other Markdown meant for external readers (tone, claims, vendor mentions) | `agf/rules/docs/marketing-tone-rule.md` |
| user-facing HTML or UI | `agf/rules/ui/accessibility-rule.md` |
| adding a locale | `agf/rules/always/i18n-supported-locales-rule.md` |
| feature-flagged user workflows | `agf/rules/docs/feature-flagged-docs-rule.md` |
| choosing a test tier (unit, integration, e2e) | `agf/rules/workflow/testing-environments-rule.md` |
| overview of the rule tree | `agf/rules/README.md` |
