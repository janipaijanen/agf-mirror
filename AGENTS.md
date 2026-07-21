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
(see "Build agov first, then use it heavily" below) as one of the first
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
│   ├── rust/                 # Preferred AGF tooling implementation
│   └── elixir/               # Reference/development implementation and parity oracle
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
| fact-checker | Jani + AI advisory | Factual/docs claim review when activated by governor-reviewer |
| academic-peer-reviewer | Jani + AI advisory | Academic/manual review for paper-style claims and definitions |

Suspended: build-engineer (no CI pipeline yet), capability-developer.
Skipped: documentation-writer, deployment-engineer, academic-visual-reviewer.

Authoritative status: `agf/agents/MANIFEST.yaml`.

Tooling and CI may block progress mechanically when configured to do so, but
they do not approve or reject governance gates. Binding Gate 1 and Gate 2
decisions remain governor-reviewer decisions recorded in coordination.

**Automation rule:** A tool or CI check may only block work when a named rule
in `agf/rules/` explicitly grants that check blocking authority. All other
tool output is advisory evidence. See `agf/rules/always/tool-authority-boundary-rule.md`.

---

## Coordination channel

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

Every non-trivial task requires:

- **Gate 1** (planner-architect → governor-reviewer): plan file in `docs/plans/`
  with YAML front matter conforming to `agf/schemas/gate1-plan.schema.json`.
  The `advisory_consultation` section must declare a status for both
  `infosec-sentinel` and `contribution-auditor`. Plan status must be `approved`
  before implementation begins — do not start coding on a `pending-gate1` plan.
  Validation: `agov plan validate <file>`.
- **Gate 2** (coder-tester → governor-reviewer): implementation on a feature
  branch, all tests green, governor-reviewer reviews diff and merges.

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

---

## Tooling quick reference

### Time and telemetry memory

Use UTC for durable project records unless an external artifact explicitly
requires a local timezone. This includes coordination timestamps, learning
record IDs and dates, telemetry continuation notes, scheduler references, and
handoff evidence. Prefer `date -u` when deriving timestamps by hand.

Use AGF maturity labels for capability-sensitive claims. For tooling and
framework features, distinguish Level 1 Planned, Level 2 Surface-defined,
Level 3 Instantiated, Level 4 Validated, and Level 5 Deployable. A command
implemented and tested in a branch is Level 4; it is not Level 5 until the
supported operator path or release artifact includes it and documentation
matches that path.

Use evidence-status labels for framework claims, principles, and recommendations:
Empirically validated, Empirically informed, or Theoretical. Capability maturity
answers "what exists?"; evidence status answers "how strong is the proof behind
the claim?" Use both when a claim could otherwise sound stronger than the
evidence supports. See `agf/docs/site/reference/maturity-vocabulary.md`.

### Build agov first, then use it heavily

The distribution archive (`agov-X.Y.Z-*/agov`) is a release snapshot. It does
not include commands added after that release, so anyone developing `agf` or
`agov` itself needs the local build regardless of whether a specific command
is currently missing. Build the Rust workspace once at session start and
shadow the release binary with the local build, then default to `agov` over
manual repo exploration for the rest of the session — every coordination
read, plan check, or validation is a token cost avoided. For routine
agent/operator work, prefer the local **release** build; use the debug build
only when actively debugging Rust internals.

```bash
# Preferred: local release build for normal agent/operator work
cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov

# Optional: debug build for active Rust development/debugging
cargo build --manifest-path tooling/rust/Cargo.toml -p agov

# Prepend tooling/ to PATH for this shell session.
# tooling/agov is a wrapper that auto-selects the local release build first.
export PATH="$PWD/tooling:$PATH"

# Verify the local build is in use
agov coord graph-index --help   # should print usage for graph-index if built from main
```

Add the `export PATH=…` line to your shell profile (`.bashrc`, `.zshrc`) for
the repo, or use a per-directory tool like `direnv` to set it automatically.

Live token ROI telemetry belongs under `tmp/` using a stable per-assistant
filename:

```text
tmp/<ai-abbrev>-agov-token-roi.jsonl
```

These files are local runtime telemetry and are not committed. When telemetry
is needed as test evidence or showcase data, copy a dated snapshot into an
explicit fixture path and document its source, date, and confidence limits. Do
not let multiple AI assistants write to the same default file when comparing
tool savings. Codex sessions should use:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
```

Claude sessions should use:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

The historical shared Codex log was renamed from
`tmp/agov-token-roi.jsonl` to `tmp/codex-agov-token-roi.jsonl`. Use the
AI-specific filenames as the durable continuation points on future machines;
do not rename them back to the shared default.

The `tooling/agov` wrapper prefers (in order):
1. `tooling/rust/target/release/agov` — if a release build exists
2. `tooling/rust/target/debug/agov` — if a debug build exists
3. `cargo run -p agov` — if Cargo is available but no binary is built yet

If a command is not available in the local Rust build, stop and report the gap;
do not fall back to Elixir/Mix.

### Supported commands

Use the user-local `agov` command for AGF tooling. It points at this
repository's `tooling/agov` executable. The Rust implementation is the
supported operator path. If a needed command is not available in Rust, stop and
report the Rust tooling gap instead of falling back to legacy tooling.

Rust is the default operator path for supported commands:

- `agov --version`
- `agov version --format json`
- `agov coord validate`
- `agov coord brief`
- `agov coord status`
- `agov coord stale`
- `agov coord visibility`
- `agov coord graph`
- `agov coord graph-index`
- `agov handoff next`
- `agov session resume`
- `agov validate ...`
  - `agov.validate.assistant_loader` — checks that AGENTS.md/.claude/CLAUDE.md
    exist and that their AGF version comment matches `agf/VERSION`
- `agov token roi report`
- `agov usage report`

Cold-start rule for humans and AI agents: use one command in two steps. First,
run `agov session resume` without `--role` to get the project map. Then
rerun the same command with the role you are actually playing.

After reading this file and before running `agov session resume`, agents
MUST check whether a repository-root `ai-cold-start.md` file exists. If it
exists, read it as a local overlay for machine-specific paths, assistant-surface
quirks, current operator preferences, or other session-local reminders. The
overlay is subordinate to this `AGENTS.md`, AGF rules, gate discipline,
coordination records, and role authority. It cannot override governed
instructions, and it cannot replace published coordination for anything another
agent or human must see. The file is gitignored by default; adopters may track
a template or pointer separately if their project needs one.

### Polling timer notation

When the governor asks for a polling timer such as `15mins/180min`, interpret
it as an internal attention schedule, not an external shell timer:

- scan every 15 minutes for new coordination messages and merge-request
  activity;
- keep scanning until 180 minutes have passed with no new incoming activity;
- reset the 180-minute idle window whenever a new relevant coordination message
  or merge-request event appears;
- do not start background shell, cron, sleep, or detached process timers unless
  the governor explicitly asks for an external process.

If the current assistant surface cannot schedule internal wake-ups, say so
plainly and perform the scan when active or resumed. Do not claim that an
external process will be noticed by the assistant.

Legacy Elixir sources remain in `tooling/elixir/` as reference material. Do
not use them for routine agent work, cold start, validation, release-page
downloads, or missing-command fallback. Shipped `agov` tooling artifacts are
Rust, with platform-specific archive names.

```bash
# Validate coordination messages
agov coord validate docs/coordination/<file>.md

# Validate plan files (Gate 1 front matter + required advisory roles)
agov plan validate docs/plans/<file>.md

# Dotted compatibility aliases also work
agov agov.coord.validate docs/coordination/<file>.md
agov agov.plan.validate docs/plans/<file>.md

# Cold start: discover active roles and pending queues
agov session resume --base-branch main

# Resume a session before acting or writing durable files
agov session resume --role <agent-role> --base-branch main

# Find the next handoff for an agent role
agov handoff next planner-architect

# Scan local branches for coordination obligations hidden from main
agov coord visibility --base-branch main

# Report local deterministic-tool token ROI telemetry
agov token roi report

# Report recent coordination usage
agov usage report

# Rust tooling tests
cargo test --manifest-path tooling/rust/Cargo.toml
```

Schema path resolution uses `AGOV_SCHEMA_DIR` (the `tooling/agov` launcher
defaults it to `agf/schemas` in the repository root). AGF-controlled
environment variables are documented together in
`agf/docs/site/reference/environment-variables.md`.

Session resume guidance for AI agents and human operators is in
`docs/reference/session-resume.md`. Local token ROI measurement guidance is in
`docs/reference/tooling-token-roi.md`.

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
