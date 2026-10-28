# Rule index: rules excluded from always-loaded context

Foundry-local pointer file. It adds no rules and grants no authority. The files
below stay in `agf/rules/` (and as symlinks under `.claude/rules/`) but are
excluded from startup loading by `claudeMdExcludes` in `.claude/settings.json`.
Each trigger below restates the canonical rule's own Scope. Read the named file
with the Read tool BEFORE doing the matching work. Plan
`20261022T205113Z-instruction-context-reduction` and issue #138 explain why.

## Situational rules

| When you are about to... | Read first |
|---|---|
| prepare or publish a release, bump `agf/VERSION`, create or push a git tag, or edit `release:*` / `agov:oci:release` jobs | `agf/rules/workflow/release-via-merge-request-rule.md` |
| act under a delegation, review a deputized Gate 2 decision, or write a delegation artifact | `agf/rules/workflow/deputy-governor-rule.md` |
| write a plan or Gate 2 review, or edit a Pages page, runbook, `AGENTS.md`, or other AI loader, that introduces, changes, or removes normative operating guidance (MUST/SHOULD/blocking/approve/prohibited wording); this needs a six-row projection manifest in `docs/reviews/rule-projection/` | `agf/rules/workflow/documentation-rule-projection-rule.md` |
| rename, restructure, deprecate, or remove a public contract: CLI flag, MCP tool field, schema field, HTTP/API parameter, or config key | `agf/rules/workflow/deprecation-policy-rule.md` |
| add or change any CI pipeline stage that runs longer than about 5 minutes, or its `rules:`/`changes:` paths | `agf/rules/workflow/ci-trigger-scoping-rule.md` |
| work in a series that declares a `convergence` coordination object | `agf/rules/workflow/review-convergence-rule.md` |
| recommend commands, config files, or deployment instructions that will run against a specific host, container, runner, or runtime | `agf/rules/always/environment-interrogation-rule.md` |
| write documentation, presentations, publications, or other external-facing material that describes what the project can do (maturity levels, capability tables) | `agf/rules/docs/implementation-honesty-rule.md` |
| prepare a branch for merge request review, write an MR description, or run `glab mr create/update` | `agf/rules/workflow/merge-request-description-rule.md` |
| change a protected branch (`main`, `production`, `release/*`) in any way, or open, update, or merge a merge request to one | `agf/rules/workflow/merge-request-workflow-rule.md` (core: never commit to `main`) |
| perform any git tag operation: create, push, move, delete, or retag | `agf/rules/workflow/immutable-tags-rule.md` |
| run any command that could upgrade dependencies, or add or change a dependency | `agf/rules/workflow/no-auto-package-update.md` |
| choose, bump, or write a `MAJOR.MINOR.PATCH` version number in a manifest, changelog, or tag | `agf/rules/workflow/semver-no-carry-rule.md` |
| notice friction, duplication, confusing workflow, avoidable token spend or a documentation gap while doing assigned AGF work, want to suggest a cleanup or consolidation of duplicated instructions, or propose an improvement (route it as a note, review, lesson-proposal or plan) | `agf/rules/always/agent-improvement-proposal-rule.md` |
| create or place a learning record, agent definition, ADR, framework rule, working plan, coordination record or `GOVERNANCE.md`, or decide where a governance artifact lives | `agf/rules/always/governance-file-layout-rule.md` |
| raise, answer or close a coordination `challenge` (`kind: review`, `challenge_id`, `disputed_record_id`) | `agf/rules/workflow/coordination-challenge-response-rule.md` |
| diagnose a bug, failing test, stack trace or screenshot | `agf/rules/workflow/debugging-rule.md` |
| modify a route, service, template conditional, permission check or feature flag, or delete or weaken a test | `agf/rules/workflow/feature-regression-guard-rule.md` |
| create any coordination message, or fix an ID or filename (the always-loaded `AGENTS.md` instruction to use `agov coord brief` stays; this adds the filename and validator rules) | `agf/rules/workflow/coordination-id-rule.md` |

## Specialized surfaces (load when the task matches)

| When you are about to... | Read first |
|---|---|
| produce output a human will look at: CAD geometry (DXF, STEP, GLB), generated PDFs, dashboards, charts, diagrams, exported reports, or UI screenshots in tests | `agf/rules/workflow/visible-artifact-verification-rule.md` |
| introduce, modify, or move an attribute on persisted data: a column, model field, analytics dimension, classification label, or index key | `agf/rules/workflow/concept-modelling-rule.md` |
| generate files that tools you do not control will read: drawings (DXF, SVG, PDF), 3D models (STEP, GLB, OBJ), spreadsheets (CSV, XLSX, ODS), structured data (JSON-LD, RDF), OOXML/ODF, captions (SRT, VTT), or any format with several independent parsers | `agf/rules/workflow/file-generator-rule.md` |
| create, modify, or remove a database migration file | `agf/rules/workflow/migration-generation-rule.md` |
| add or change a controller, handler, RPC endpoint, GraphQL mutation, or other entry point that creates, updates, or deletes persisted data | `agf/rules/always/audit-log-completeness-rule.md` |
| parse structured data from outside the project: external API responses, library callbacks, OAuth/OIDC token exchanges, webhook payloads, RPC responses, queue messages | `agf/rules/workflow/boundary-shape-testing-rule.md` |
| write or edit any `docs/**/*.md`, `README.md`, or other Markdown meant for external readers (tone, claims, vendor mentions) | `agf/rules/docs/marketing-tone-rule.md` |
| write or change user-facing HTML, templates, CSS, or JavaScript interactions | `agf/rules/ui/accessibility-rule.md` |
| add, scaffold, or change a locale or translation catalog | `agf/rules/always/i18n-supported-locales-rule.md` |
| add or change a non-trivial feature flag that adds a user-visible workflow | `agf/rules/docs/feature-flagged-docs-rule.md` |
| write any test, or choose a unit, integration, or e2e tier | `agf/rules/workflow/testing-environments-rule.md` |
| need an overview of the rule tree | `agf/rules/README.md` |
