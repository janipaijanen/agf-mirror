<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

<!-- AGF: 2.12.0 — update this comment after each framework upgrade to match agf/VERSION -->
# AGF-NG — Agent and Contributor Orientation

**Project codename:** foundry
**What this is:** The next-generation agents-govern framework, developed using
agents-govern 1.1.50 as its own governance framework. The project is both the
product under development and a live proof-of-concept of the governance model
it will supersede.

**Use `agov` first for project orientation and validation.** It saves context
tokens by returning compact deterministic summaries instead of making agents
read the full coordination channel, plans, and schemas by hand. Prefer the
repo-local `./tooling/agov` wrapper in this checkout. For adopter/release
binaries, use the GitLab Releases page:
`https://gitlab.com/janipaijanen/agents-govern-framework/-/releases`.

**If you are developing `agf` or `agov` itself:** build the local Rust binary
(see "Build agov first, then use it heavily" in
`docs/reference/foundry-polling-and-bridge-procedure.md`) as one of the first
things you do in the session, before exploring the repo by hand — then keep
reaching for it for every coordination, plan, or validation check for the
rest of the session. This is the default working habit, not a fallback for
when a command happens to be missing.

---

## Project layout

```
AGF-NG/
├── agf/                      # Canonical AGF v2.0+ adopter-facing framework
│   ├── agents/               # Agent role definitions shipped to adopters
│   ├── rules/                # Governance rule templates
│   └── schemas/              # JSON Schema draft-07 definitions
├── tooling/
│   ├── agov                  # User-facing wrapper for the Rust agov CLI
│   └── rust/                 # AGF tooling implementation
├── docs/
│   ├── coordination/         # Immutable coordination channel (append-only)
│   ├── plans/                # Gate 1 plan files
│   └── proposals/            # Longer-form proposals
├── agents-govern/            # Bootstrap governance reference — see below
└── AGENTS.md                 # This file
```

---

## agents-govern/ is a bootstrap reference, not the product

`agents-govern/` contains the verbatim agents-govern 1.1.50 distribution used
to govern this project's own development (Layout B). It is **not** the product
being built — it is the governance tool currently in use.

Treat it as legacy/bootstrap: avoid making product changes there unless
explicitly directed. The currently permitted exception path is:
- `agents-govern/learnings/foundry.yaml` — learning records, append-only

**Role authority note:** `agf/agents/MANIFEST.yaml` is the current AGF-NG
project role authority for active agents and role status. Agent status changes
require governor-reviewer approval. `agents-govern/agents/MANIFEST.yaml`
remains historical bootstrap reference material only.

The adopter-facing product is in `agf/`. Foundry tooling sources are in
`tooling/`.

---

## Active agents

| Agent | Played by | Role |
|---|---|---|
| planner-architect | claude (AI) | Gate 1: plans, task decomposition, coordination |
| coder-tester | claude (AI) | Implementation, tests, fixtures, validators |
| governor-reviewer | Jani (human) | Gate 1 approval, Gate 2 merge, all binding decisions |
| contribution-auditor | Jani + claude | Reviews plans and proposals; filed as coordination messages |
| infosec-sentinel | CI (agov-validate-infosec) | Mechanically blocks configured checks on credential or cross-record leaks in learnings; governor-reviewer resolves the finding |
| build-engineer | claude (AI) | CI configuration, build automation, artifacts, caching, trigger rules, pipeline efficiency, and release-build stages |
| capability-developer | AI advisory (explicitly assigned per review) | Evidence-triggered agent evolution, maturity, phase, and boundary proposals |
| fact-checker | Jani + AI advisory | Factual/docs claim review when activated by governor-reviewer |
| academic-peer-reviewer | Jani + AI advisory | Academic/manual review for paper-style claims and definitions |
| web-ux-reviewer | Jani + AI advisory (independent of the implementing session) | Visual/UX/accessibility review of the public Pages site |
| documentation-estate-steward | claude (AI) | Estate-wide adopter-facing documentation consistency and currency; runs the release-content-freeze sweep |
| independent-implementation-reviewer | AI, distinct from the implementer (named per Gate 2 candidate) | Targeted exact-head implementation-diff verification when extra review separation is warranted; advisory, never a Gate 2 approver |

Skipped: documentation-writer, deployment-engineer, academic-visual-reviewer.

Activation history for `independent-implementation-reviewer` and
`documentation-estate-steward`: `docs/reference/foundry-agents-md-moved-text.md`.

Active for foundry: improvement-propeller, played by Codex under approved plan
`20260909T015237Z-propeller-stage4-activation` and governor decision `djh1`.
Its weekly opportunity and slow governance-friction lanes are defined by
`agf/rules/always/improvement-propeller-rule.md`; current activation is Level 5
Deployable after the genuine unattended wake confirmed by governor decision
`20260918T162825Z-governor-reviewer-8hs2`. Shipping the
definition still does not activate it for adopters: each adopter requires its
own governor decision naming the player, evidence surfaces, real wake
mechanism, cadence, and cursor state.

Authoritative status: `agf/agents/MANIFEST.yaml`.

Tooling and CI may block progress mechanically when configured to do so, but
they do not approve or reject governance gates. Binding Gate 1 and Gate 2
decisions remain governor-reviewer decisions recorded in coordination.

**Automation rule:** A tool or CI check may only block work when a named rule
in `agf/rules/` explicitly grants that check blocking authority. All other
tool output is advisory evidence. See `agf/rules/always/tool-authority-boundary-rule.md`.

---

## Coordination channel

Channel membership, bounded misplaced-record discovery and unassessed archive inventory are defined by `agf/rules/workflow/coordination-publication-rule.md`. Only direct lowercase `.md` files are channel members; archive exclusion never proves closure. Consult the visibility command reference for its explicit scan scopes and coverage limits.

Recipient-only session annotations for handoffs are defined by `agf/rules/workflow/coordination-publication-rule.md` (Advisory recipient session addressing). Top-level session is author provenance; a mismatch never hides or reprioritizes an obligation.

All inter-agent communication goes through immutable Markdown files in
`docs/coordination/`. Front matter is validated by `agov coord validate`.

AI agents MUST use `agov coord brief --from <role>` to generate coordination
IDs unless tooling is unavailable. Readable IDs are the default; the explicit
`--id-format readable` flag remains valid for clarity. Humans SHOULD use the
tooling. Manually authored IDs remain valid when they pass schema and uniqueness
validation.

| ID format | Filename convention | Use |
|---|---|---|
| Readable ID | `<id>.md` | Preferred for new AI-authored messages. |
| ULID | `<ulid>-<sender>.md` | Valid legacy format; do not use for new AI messages unless recovering. |
| Legacy sequence | `<zero-padded-seq>-<sender>.md` | Historical only. |

New readable IDs use `YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>`. Do not
rename historical coordination files.

**Nine message kinds:** `claim`, `completion`, `review`, `ack`, `handoff`,
`decision-record`, `fast-track-waiver`, `lesson-proposal`, `note`.

Schema: `agf/schemas/coordination-message.schema.json`

Challenge lifecycle and advisory response handling: see `agf/rules/workflow/coordination-challenge-response-rule.md`.

**When to write a coordination entry:** Any action that creates a downstream
obligation for another agent or human requires a coordination entry as part
of completing that action — not as an afterthought. Write one when:

- A plan or design is submitted for Gate 1 approval (kind: `handoff`, to: `governor-reviewer`)
- An implementation is ready for Gate 2 review (kind: `handoff`, to: `governor-reviewer`)
- A decision has been made that others need to know about (kind: `decision-record`)
- A finding, alert, or escalation is routed to another agent (kind: `review`, `note`, or `handoff`)
- A task is completed and the result needs to be recorded (kind: `completion`)

The coordination channel is the audit trail for both decisions and the work
that led to them. Other agents and the human read it asynchronously — write
entries so that a cold-starting agent can reconstruct what happened and why,
without relying on session memory.

**Visibility rule:** A coordination message addressed to another agent or human
is not complete when it merely exists on disk. It must pass validation and be
made visible where the recipient is expected to read it: committed and pushed to
a shared branch, published through an approved shared path, or paired with a
visible bridge note that names the branch/MR containing the record. Local files,
untracked files, and local-only commits are draft state for inter-agent work.

A coordination-only commit (title prefix `coordination:`) publishes promptly and
independently of Gate 2. Never add `[ci skip]`/`[skip ci]` to it. Run
`agov coord publication-check --base-ref origin/main` before pushing. Owner:
`agf/rules/workflow/coordination-publication-rule.md`; the full AGENTS.md text is
in `docs/reference/foundry-agents-md-moved-text.md`.

---

## Learning record IDs

Historical learning records may use `L###` identifiers. New learning records
that use an `id:` field MUST use UTC timestamp IDs:

- `YYYYMMDDThhmmssZ`
- `YYYYMMDDThhmmssZ-<4-char>` when same-second disambiguation is needed

Do not allocate the next `L###` number for new learning records. Do not add a
`learn-` prefix or slug to the ID. Existing historical IDs remain stable and
must not be renamed merely for style.

---

## Gate discipline

A plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports overlaps advisory-only by default. A plan whose `affected_paths` overlaps another live plan's, unacknowledged by `depends_on`, blocks Gate 1 via `agov:governance:plan-collision-gate` -- see `agf/rules/workflow/development-workflow-rule.md`'s "Affected-paths collision blocking" section. Only plan-vs-plan overlaps block; a local branch's or another open MR's changed paths remain advisory-only.

Every non-trivial task requires:

- **Gate 1** (planner-architect → governor-reviewer): plan file in `docs/plans/`
  with YAML front matter conforming to `agf/schemas/gate1-plan.schema.json`.
  The `advisory_consultation` section must declare a status for both
  `infosec-sentinel` and `contribution-auditor`; `pending` is valid pre-Gate-1
  drafting state (a review was requested but has not yet answered) but a plan
  cannot reach `approved`/`active`/`complete` while any declared row --
  required or optional -- is still `pending`. Plan status must be `approved`
  before implementation begins — do not start coding on a `pending-gate1` plan.
  Validation: `agov plan validate <file>`.
- **Gate 2** (coder-tester → governor-reviewer): implementation on a feature
  branch, all tests green, governor-reviewer reviews diff and merges.

**Narrow research-maintenance exception** (edits limited to the existing
`docs/sources.md` and/or `docs/radar.md`): owner `agf/rules/workflow/simplicity-rule.md`;
full AGENTS.md text in `docs/reference/foundry-agents-md-moved-text.md`. Any
uncertainty or disqualifier returns to full Gate 1; Gate 2 stays mandatory.

Gate 1 approval comes from a governor-reviewer coordination message. Gate 2
merge target and remote must be confirmed with the governor-reviewer before
merging — the repository may be local-only or the target branch name may differ.

**A same-role self-ack does not supply governor authority.** A `coder-tester`
(or any non-governor role) acknowledging its own prior work, or citing a merge
event alone, does not satisfy the Gate 1 approval requirement above — only a
`governor-reviewer` coordination message (`ack`, `decision-record`, or
`handoff`) does. This applies equally to post-hoc corrections: if a plan's
`status`/`gate1_approved_by`/`gate1_approved_at` fields are updated after the
fact (e.g. because the original approving commit merged before the fields
were set), the correcting commit must be accompanied by an actual
governor-reviewer coordination record, not merely a plan-file edit. Prefer
updating a plan's own metadata in the same review cycle as its approval, so
this correction is not needed at all — it recurred three times in one session
(2026-07-21) precisely because it was deferred.

**Conversational decisions and bridge records.** When the governor delivers a
gate or scope decision in a live session rather than as a coordination record,
the receiving assistant transcribes it into a role `note` (a *bridge record*).
A bridge record is **not binding authority on its own**: it must state that it
is provisional until a `governor-reviewer` record (an `ack`, a
`decision-record`, or a ratification decision-record) references it. Until
that governor record exists:

- a plan's `status` stays `pending-gate1` even when the bridge record
  accurately transcribes a live "approved" decision — the machine/agent-
  visible `approved` state must not appear before the governor record exists,
  because another session or agent may treat `approved` alone as
  authorization to start implementation. The bridge note may say the governor
  approved it live and that confirmation is pending; it must not flip the
  plan's `status` field;
- **irreversible** actions — a merge to a protected branch, a tag, a forge
  closure — always wait for the governor record, **unless** the governor
  authorizes that specific action in the same session ("merge now"), in which
  case that authorization is captured in the next `governor-reviewer`
  bridge/ratification record **before** the irreversible action executes, not
  after.

A bridge record still unreferenced by any record after the session is a gap;
`agov coord pending` surfaces any unresolved `requires-ack: true` record
generically — its resolution check is "does any later record's
`references-to` list this file," with no check on the referencing record's
sender role. A non-`governor-reviewer` reference also clears a bridge record
from this list. Treat a cleared entry as "something referenced it," not proof
of governor ratification — only an actual `governor-reviewer`-authored `ack`
or `decision-record` supplies binding authority. Worked example:
`docs/coordination/20260910T133145Z-governor-reviewer-q862.md` ratifies a
session's bridge records in one append-only decision-record.

---

## Tooling quick reference

The full tooling, build, command-list, polling-loop, and branch-publication
procedure moved verbatim to `docs/reference/foundry-polling-and-bridge-procedure.md`.
Open it before you build `agov`, set telemetry paths, list supported commands,
start a polling timer or run a poll cycle, publish a brand-new branch, or compare
duplicate pipelines. Its text keeps its force; this section only points to it.

Always apply, even before opening it:

- Use `./tooling/agov` first. If a command is missing from the Rust build, stop
  and report the gap.
- After reading this file and before `agov session resume`, check whether a
  repository-root `ai-cold-start.md` exists and read it as a local overlay.
  Cold start is two steps: `agov session resume --base-branch main`, then rerun
  with `--role <your-role>`.
- Every active AGF development role/session MUST follow
  `agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`.
  Observation is whole-surface; selection and mutation stay role-, authority-,
  gate-, and claim-scoped. Its "Scheduling and notification honesty" section
  governs timer qualification, durability, expiry, rearming, and continuation.
- The action loop never enlarges authority: Gate 1/2 approval, merge, release,
  tag, deploy, scope expansion, destructive action, and new role authority stay
  with the governor.

Codex and other assistants do not load `.claude/rules/`. Before the matching
work, open the named file under `agf/rules/` (`.claude/rules/INDEX.md` has the
same list with fuller scope wording):

- release, `agf/VERSION` bump, git tag, `release:*` jobs: `workflow/release-via-merge-request-rule.md`
- any git tag create, push, move, delete, or retag: `workflow/immutable-tags-rule.md`
- change or open/merge an MR to `main`, `production`, `release/*`: `workflow/merge-request-workflow-rule.md`
- prepare an MR or write its description, `glab mr create/update`: `workflow/merge-request-description-rule.md`
- act under a delegation or write a delegation artifact: `workflow/deputy-governor-rule.md`
- plan or Gate 2 review that adds or changes normative guidance in a Pages page, runbook, or AI loader: `workflow/documentation-rule-projection-rule.md`
- rename, restructure, deprecate, or remove a CLI flag, MCP field, schema field, API parameter, or config key: `workflow/deprecation-policy-rule.md`
- add or change a CI stage over about 5 minutes or its `rules:`/`changes:`: `workflow/ci-trigger-scoping-rule.md`
- series with a `convergence` coordination object: `workflow/review-convergence-rule.md`
- commands or config for a specific host, container, runner, or runtime: `always/environment-interrogation-rule.md`
- docs or publications describing what the project can do (maturity levels): `docs/implementation-honesty-rule.md`
- run anything that could upgrade dependencies, or add a dependency: `workflow/no-auto-package-update.md`
- choose or write a `MAJOR.MINOR.PATCH` version: `workflow/semver-no-carry-rule.md`
- output a human will look at (CAD, PDF, dashboard, chart, report): `workflow/visible-artifact-verification-rule.md`
- introduce, modify, or move an attribute on persisted data: `workflow/concept-modelling-rule.md`
- generate files read by tools you do not control (DXF, SVG, PDF, CSV, JSON-LD): `workflow/file-generator-rule.md`
- create or change a database migration: `workflow/migration-generation-rule.md`
- add or change a handler that creates, updates, or deletes persisted data: `always/audit-log-completeness-rule.md`
- parse structured data from outside the project (APIs, webhooks, tokens): `workflow/boundary-shape-testing-rule.md`
- write `docs/**/*.md`, `README.md`, or other external-facing Markdown: `docs/marketing-tone-rule.md`
- user-facing HTML, templates, CSS, or JavaScript: `ui/accessibility-rule.md`
- add or change a locale or translation catalog: `always/i18n-supported-locales-rule.md`
- non-trivial feature flag with a user-visible workflow: `docs/feature-flagged-docs-rule.md`
- write a test or choose a test tier: `workflow/testing-environments-rule.md`
- notice friction, duplication, confusing workflow, avoidable token spend or a doc gap while doing assigned AGF work, or want to suggest a cleanup or consolidation of duplicated instructions, or route an improvement idea (note, review, lesson-proposal or plan): `always/agent-improvement-proposal-rule.md`
- create or place a learning record, agent definition, ADR, framework rule, working plan, coordination record or `GOVERNANCE.md`: `always/governance-file-layout-rule.md`
- raise, answer or close a coordination `challenge` (`kind: review`, `challenge_id`, `disputed_record_id`): `workflow/coordination-challenge-response-rule.md`
- diagnose a bug, failing test, stack trace or screenshot: `workflow/debugging-rule.md`
- modify a route, service, template conditional, permission check or feature flag, or delete or weaken a test: `workflow/feature-regression-guard-rule.md`
- create any coordination message, or fix an ID or filename (the `agov coord brief` instruction above stays; this adds filename and validator rules): `workflow/coordination-id-rule.md`
- overview of the rule tree: `agf/rules/README.md`

Two obligations moved to `docs/reference/foundry-polling-and-bridge-procedure.md`
keep their force. Read "Time and telemetry memory" there before writing any
durable record with a timestamp or ID (UTC, `date -u`) and before any claim about
tooling or framework capability or the strength of evidence (maturity Levels 1-5,
evidence-status labels).

---

## Current state

For live state, use:

```bash
git log --oneline --decorate -5
ls docs/coordination/ | sort | tail -5
agov coord validate docs/coordination/<file>.md
agov plan validate docs/plans/<file>.md
agov session resume --base-branch main                           # cold start
agov session resume --role planner-architect --base-branch main
agov handoff next planner-architect
agov coord visibility --base-branch main
```

**Snapshot (2026-06-14):** `agf/agents/MANIFEST.yaml` is the AGF-NG role
authority. GitLab upstream is available at
`https://gitlab.com/janipaijanen/agents-govern-framework.git`; current feature
work is tracked on branches and coordinated through `docs/coordination/`.

---

## Branch discipline

- Feature branches: `feat/<task>`, `fix/<what>`, `chore/<what>`, etc.
- Never commit directly to `main`.
- All changes merged after governor-reviewer Gate 2 approval.
- No version bumps or tags on feature branches.

**Worktree boundary:** All linked worktrees must be created under
`.worktrees/<purpose>` within the project root. Never create worktrees in
sibling directories or `/tmp` unless the governor-reviewer explicitly approves
an external path.

See `agents-govern/` rules: `workflow/branch-and-worktree-rule.md`,
`workflow/merge-request-workflow-rule.md`.

Publication-hold evidence: `agov coord hold-status --target-branch origin/main
--candidate-ref <shared-ref>` reports explicit pinned Git declarations only.
`review-required` never permits delay. See
`agf/rules/workflow/coordination-publication-rule.md` for external checks,
rollout compatibility and immediate-publication obligations.
