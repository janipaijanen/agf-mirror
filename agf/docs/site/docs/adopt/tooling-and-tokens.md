<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# Tooling and token savings

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: AI assistants adding
deterministic tooling to the inner agent loop, with developers and
administrators supervising setup and reviewing the evidence.*

## The premise

AI agents use context tokens every time they read governance files to
reconstruct coordination state: which handoffs are pending, which artifacts are
claimed, whether a coordination message is schema-valid. When those questions
are answered by deterministic tooling instead, the agent reads a compact summary
rather than scanning the full file set. Those tokens are then available for
judgment, design, and review — the work that actually benefits from AI
reasoning.

This page documents the AGF tooling, what it replaces, how to set it up, and
what the evidence base for the savings looks like.

## What the tooling does

The optional `agov` command is the adopter-facing CLI for AGF validators and
coordination helpers. It is a static binary (~5–12 MB) with no runtime
dependency. See [How to set it up](#how-to-set-it-up) below for the
recommended published-artifact install path.

### Source-checkout wrapper for contributors

Contributors working in an AGF source checkout should invoke that checkout's
`tooling/agov` wrapper, usually by prepending `$PWD/tooling` to `PATH`. The
wrapper delegates command parsing to the Rust binary, verifies a deterministic
fingerprint of behavior-affecting tooling inputs, and performs one local
release rebuild when no matching binary exists and Cargo is available. It
fails closed if rebuilding is unavailable or does not produce matching
provenance.

Linked worktrees must use their own `tooling/agov` path. Invoking a wrapper
owned by a different worktree exits with a diagnostic naming the correct path;
this prevents a binary, schema directory, and agent manifest from being bound
silently across checkouts. These repository-local checks do not alter the
standalone published binary described below.

## Current command maturity

Maturity levels follow the
[maturity vocabulary](../../reference/maturity-vocabulary.md): Level 5 is
deployable and in active use; Level 1 is planned.

### Level 5 — Deployable (in production use)

This is the canonical command inventory. Each command below is implemented,
tested, documented, and present in the supported released operator path. A
future command that exists only in a branch or design must be listed separately
at its actual maturity level rather than added to this table by analogy.

| Command | What it replaces |
|---|---|
| `agov --version` / `agov version --format json` | Inferring build provenance from file paths, timestamps, or `git log`. |
| `agov coord validate <file>...` | Agent reads a coordination file and judges schema compliance. Validation is deterministic; agents should not improvise it. |
| `agov plan validate <file>...` | Agent reads a Gate 1 plan file and checks front-matter. |
| `agov validate` | Agent reads adoption, agent definitions, learning records, adopter-docs parity, and copied-assistant drift warnings to spot structural problems across the framework tree. It does not validate every plan or coordination file. |
| `agov validate adopter-docs` | Agent or release reviewer checks whether adopter-relevant framework/tooling changes since the latest reachable `v*` tag include downstream-facing docs/templates or a visible changelog waiver. |
| `agov coord brief [--id-format readable\|ulid] [--from <role>] [--kind <kind>]` | Agent guesses a coordination ID, makes up a filename, or asks a human for the next ID. The command generates a valid, non-colliding ID with the correct filename format. |
| `agov handoff next <role>` | Agent reads handoff messages, mentally subtracts resolved items, and selects the pending one for its role. The command returns the relevant handoff in one invocation. |
| `agov coord status [--recent N] [--roles]` | Agent counts pending handoffs, reads claim messages, and scans the channel for channel health. The `--roles` flag adds a per-role activity summary for session orientation. |
| `agov session resume [--role <role>]` | Agent reads recent coordination files, the manifest, branch status, and claims to reconstruct its starting state for the session. This is potentially the highest-token-cost activity in the agent loop because it happens at every session boundary. |
| `agov coord stale` | Agent identifies expired artifact claims by reading all claim messages and comparing timestamps. |
| `agov coord visibility` | Agent compares branches for coordination obligations that exist off the base branch. |
| `agov coord graph` | Agent reads coordination files to reconstruct which roles communicated with which, then manually renders a flow summary. The command produces a Graphviz dot map in one invocation. |
| `agov coord graph-index` | Agent scans all coordination files to produce a structured JSON index of handoffs, references, and pending obligations — replaces ad-hoc mental reconstruction. |
| `agov coord pending` | Agent scans the coordination channel by hand for unresolved non-`handoff` `requires-ack: true` records (claim, review, note, decision-record, fast-track-waiver, lesson-proposal). Reports; does not resolve — a later record's direct `references-to` resolves an entry. Exit code `2` means none found, not a failure. |
| `agov token roi report` | Agent summarizes local deterministic-tool telemetry instead of estimating savings by hand. |
| `agov usage report [--last <n>]` | Agent counts message kinds, active senders, inactive active-status agents, and unknown senders. |

For adoption work, `agov usage report --last 50` is the quickest answer to
"which roles have actually been active recently?"

## How to set it up

Choose a published artifact unless you are contributing to AGF or no artifact
exists for your platform. Ordinary downstream adopters should not need an AGF
source checkout to run `agov`.

### Option A — Platform tooling release archive (recommended for local use)

Download the `agov` tooling archive — not the `agf-<version>.tar.gz`/`.zip`
framework source archive, which is a different asset on the same release page
— matching your OS, architecture, and adopted AGF version from the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).
The archive assets attached to each release are the authoritative platform
inventory; select the OS and architecture suffix that matches your system.

Verify the adjacent `.sha256` checksum and `-provenance.json` file (both
linked next to the archive on the Releases page) before trusting the binary.
A checksum fetched from the same publication channel confirms the archive
matches that published value — it does not independently prove that neither
the archive nor the checksum was substituted, and it is not a signature:

```bash
# Linux (.tar.gz)
sha256sum -c agov-<version>-linux-x86_64.tar.gz.sha256
```

Then open the adjacent `-provenance.json` file and confirm it names the
version, tag, source commit, artifact filename, and checksum you expect —
this establishes consistency with the published record, not an independent
attestation or cryptographic signature.

Unpack the archive, place the `agov` (and, if you use MCP, `agov-mcp`)
executable on PATH, and confirm the reported version matches what you
downloaded:

```bash
agov --version
```

### Option B — Official OCI image (recommended for CI/container use)

Pin an exact release tag or digest — never `:latest` — and mount your
repository into the container:

```yaml
validate-governance:
  image:
    # Replace the placeholder with an exact version from the Releases page.
    name: "registry.gitlab.com/janipaijanen/agents-govern-framework/agov:<version-from-release-page>"
    entrypoint: [""]
  variables:
    # A three-dot range needs a merge base. GitLab's default GIT_DEPTH is
    # shallow, so a branch that has moved past it has no merge base in the
    # checkout and the comparison cannot be made at all.
    GIT_DEPTH: "0"
  script:
    - |
      set -eu

      # Resolve the base explicitly. Do NOT use
      # CI_MERGE_REQUEST_TARGET_BRANCH_SHA: it is empty in common pipeline
      # shapes (first pipeline on a branch, force-push, shallow clone), and
      # an empty base degenerates "$base"...HEAD into HEAD...HEAD, which is
      # legitimately empty -- so the job exits 0 having validated nothing.
      target="${CI_MERGE_REQUEST_TARGET_BRANCH_NAME:-$CI_DEFAULT_BRANCH}"
      git fetch --no-tags origin "$target"
      BASE="$(git rev-parse --verify "refs/remotes/origin/$target")"
      git merge-base "$BASE" HEAD >/dev/null

      LIST="$(mktemp)"

      # Capture the diff to a file and let its exit status stand. Never pipe
      # `git diff` straight into `xargs`: `set -e` observes only a
      # pipeline's LAST exit status, so a failed diff is masked by a
      # successful `xargs -0r`, which on empty input simply declines to run.
      changed() { filter="$1"; shift; : > "$LIST"; git diff --name-only -z "--diff-filter=$filter" "$BASE...HEAD" -- "$@" > "$LIST"; }
      # --no-renames is load-bearing. Git reports a rename as R, not D, so a
      # plain --diff-filter=D is blind to rename-and-rewrite: the original
      # record is gone, a validly-shaped destination passes the validator,
      # and the append-only guard sees nothing. --no-renames decomposes the
      # rename into D(old) + A(new) so the vanished path is visible.
      deleted() { : > "$LIST"; git diff --no-renames --name-only -z --diff-filter=D "$BASE...HEAD" -- "$@" > "$LIST"; }
      # `if`, not `[ -s ... ] && ...` -- under `set -e` a false `&&` list
      # would exit the script.
      run_on() { if [ -s "$LIST" ]; then xargs -0r "$@" < "$LIST"; fi; }

      # Coordination records are append-only. Reject a removal explicitly
      # rather than letting the validator report "file not found", which
      # explains nothing. A rename counts: it removes the published path
      # other records cite by name.
      deleted 'docs/coordination/*.md'
      if [ -s "$LIST" ]; then
        echo "ERR: this range removes coordination records from their published paths; records are append-only, and a rename counts." >&2
        tr '\0' '\n' < "$LIST" >&2
        exit 1
      fi

      changed d 'docs/coordination/*.md'
      run_on agov coord validate

      # Deleting a plan file is legitimate (revert, supersession, cleanup),
      # so deletions are excluded here -- passing a deleted path to
      # `agov plan validate` fails at blocking severity on a file that is
      # supposed to be gone.
      changed d 'docs/plans/*.md'
      run_on agov plan validate
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

This is a container execution path for CI or scripted validation, not a host
installation recipe — it does not put `agov` on a developer's local `PATH`,
and it is not an installation path for the local stdio `agov-mcp` server
(see [agov MCP server](mcp-server.md)). The release archive (Option A) remains
a supported alternative when your CI environment does not allow project
registry images.

See [Contracts and validators](../advanced/contracts-and-validators.md) for the
full CI wiring guide, including how to declare checks as blocking vs advisory.

### Option C — Build from source (not usually recommended downstream)

Keep this for contributing to AGF, debugging Rust internals, an unsupported
platform, or the absence of a suitable published artifact for your OS/
architecture. Check out an exact release ref (a tag, not a moving branch) and
build the locked release profile — do not treat an ordinary full checkout as
the recommended adopter path:

```bash
git checkout <adopted-version>   # pin the exact release tag you adopted
cargo build --release --locked --manifest-path tooling/rust/Cargo.toml -p agov
```

Add `-p agov-mcp` to the same command if you also need the MCP server binary.
`--locked` requires `Cargo.lock` to be present and unmodified — this is the
case on any tagged release ref — and fails rather than silently resolving
different dependency versions than the ones that shipped in that release.

Then add it to your PATH:

```bash
ln -sfn "$PWD/tooling/rust/target/release/agov" ~/.local/bin/agov
```

`agov` is optional tooling. If you do not install it, keep the same governance
obligations: manually review changed plan and coordination front matter,
confirm assistant loaders point at `agf/`, and record the manual checks in the
MR. When `agov validate` is available, treat its output as framework coverage;
for complete governance coverage also run `agov plan validate` and
`agov coord validate` on the relevant files.

A human or administrator selects, verifies, installs, and pins the artifact
(Option A, B, or C); the AI assistant uses the CLI or MCP transport already
configured for it and must not silently choose an unpinned download or build
from source on its own.

## Why meter at the governance gate

"You cannot optimise what you do not meter, and you cannot meter what flows
through no chokepoint."

Every agent invocation passes through one or more AGF gates: coordination
validation, handoff routing, session resume. Capturing token consumption at
those chokepoints produces attribution data that API-level billing cannot: cost
per role, per coordination message kind, per gate pass. API billing shows total
spend; gate-level metering shows which governance activity caused it.

This matters for efficiency decisions. "AI is expensive" is not actionable.
"Session-resume accounts for 40% of coordination overhead and runs 12 times
per day" is. The `agov token roi report` command reads the local JSONL telemetry
and surfaces savings per command, giving teams the visibility to reduce
coordination overhead without reducing coordination discipline.

Governance also enables greater agent autonomy safely: scoped permissions,
recorded gate decisions, and reviewable coordination history make the
consequences of missteps bounded and visible.

## Token savings: the evidence model

AGF does not claim exact token counts — token consumption varies by model,
context window state, and whether the agent needs to read the full file or a
summary. The honest model is:

```
estimated_tokens_saved =
  tokens(manual_context_that_would_have_been_read)
  - tokens(tool_output_that_was_read)
  - tokens(extra_prompt_needed_to_invoke_the_tool)
```

Where estimates are needed for tooling planning, use:

```
approx_tokens = ceil(character_count / 4)
```

**What the foundry project has measured:** The AGF foundry project (which
develops the framework itself) runs the `agov` tooling in every agent polling
loop. The `agov coord validate` and `agov coord brief` commands each run in
under 100 milliseconds and produce output that is one to two orders of magnitude
shorter than reading the underlying coordination files with an AI agent (foundry
estimate; your channel size will vary).

The telemetry system collects saved-token estimates in a local JSONL file. Live
token ROI telemetry belongs under `tmp/` using a stable per-assistant filename:

```text
tmp/<ai-abbrev>-agov-token-roi.jsonl
```

These files are local runtime telemetry and are not committed. When telemetry
is needed as test evidence or showcase data, copy a dated snapshot into an
explicit fixture path and document its source, date, and confidence limits.
Teams running multiple assistants in the same checkout should split the logs by
agent so later analysis does not mix streams accidentally:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
# or, for a Claude-run shell:
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

Keep these exports project-local (for example in a per-directory environment),
not in a shell-global profile. `$PWD` is expanded when exported and does not
follow later directory changes. With no override, `agov` discovers the current
AGF root and writes `tmp/agov-token-roi.jsonl` there. An override outside that
root remains usable but produces an isolation warning.

To print accumulated records:

```bash
agov token roi report --log tmp/codex-agov-token-roi.jsonl
```

To disable telemetry:

```bash
export AGOV_TOKEN_ROI=0
```

See the [environment variable reference](../../reference/environment-variables.md)
for the full list of telemetry controls.

## What is not in scope

- Language/ecosystem registry publishing, such as crates.io, or code signing
  (a separate future decision) — the GitLab generic-package archives and the
  GitLab container registry OCI image described above are already published.
- macOS and Windows tooling are unsupported. The current tooling release path
  is Linux x86_64; consult the selected release page for its actual assets.

## See also

- [Contracts and validators](../advanced/contracts-and-validators.md) — wiring
  `agov` checks into CI as blocking or advisory gates.
- [Tooling conformance](../../reference/tooling-conformance.md) — the
  command-surface contract that any AGF-compatible implementation must pass.
- [Environment variables](../../reference/environment-variables.md) — telemetry
  controls and path overrides.
- [Your first governed change](../start/first-governed-change.md) — where to
  start before worrying about token savings.
