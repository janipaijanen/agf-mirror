<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Binding Contract

How agents-govern ties an artefact (a file, a commit, a sidecar) to
the execution that produced it. This is the framework's **Layer 3**
of the [five-layer verifiability model](integration-contract.md#five-layer-verifiability-model)
— the missing middle between artefact compliance (Layer 1) and
runtime behaviour (Layer 4, out-of-framework by design).

The work to close Layer 3 is split across three issues:

- **B1** — Universally Unique Identifier (UUID) v7 correlation
  identifier convention (this page).
- **B2** — `Agent-ID:` commit trailer schema (this page).
- **B3** — Sidecar metadata `<artefact>.agov.json` with
  filename-suffix fallback for environments stripping non-image
  files (this page).

**Implementation status as of v1.1.45.** All three legs ship in
[`src/agents_govern/correlation_id.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/correlation_id.py)
and
[`src/agents_govern/validate_binding.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/validate_binding.py).
The
[capability report](integration-contract.md#capability-reporting)
flips `binding: true` at v1.1.45 — the umbrella capability is
now claimable by any adopter targeting the Full
[compliance tier](adopt/compliance-profile.md).

## Correlation identifier (B1)

A correlation identifier (also: *correlation ID*) names a single
unit of governed work — typically a task assignment from
Governor-Reviewer to a production agent. The identifier is
attached to every artefact the work produces (commits via the
trailer in the next section; sidecars via B3) so a future
investigator can answer *"what produced this artefact, and what
else came out of the same work?"* without consulting an external
tracing system.

### Format: UUID v7

UUID v7 per Request For Comments (RFC) 9562 §5.7. The 128-bit
value carries a 48-bit Unix-epoch millisecond timestamp prefix
followed by 74 bits of random material and the version (7) and
variant (10) bits required by the standard. The canonical string
form is the standard 36-character lowercase hyphenated shape:

```text
01931f2c-7e8a-7b6c-9def-0123456789ab
└──ts──┘ └──┘└┬┘└──┘└──────rand_b────┘
              version 7
```

**Why v7, not v4.** The time-ordered prefix makes UUID v7 values
*sortable lexicographically by creation time*. A directory listing
of correlation-ID-named sidecars sorts oldest-first without a
separate timestamp field; a database index on the column gives
range queries by time for free; debugging "what happened in the
last hour?" is a single string-prefix scan. UUID v4 (random) loses
all of those affordances. The trade-off — UUID v7 leaks the
creation timestamp at millisecond granularity — is a feature for
governance work, not a problem.

### Lifecycle

| Phase | Owner | Action |
|---|---|---|
| Creation | Governor-Reviewer | Generates one ID at task assignment using `correlation_id.generate()`. |
| Propagation | Production agent | Attaches the ID to every artefact via commit trailers (B2) and sidecars (B3). |
| Closure | Governor-Reviewer | Records the ID in the merge approval (or escalation resolution) artefact. |

The lifecycle responsibility is recorded in
[`agents/governor-reviewer.yaml`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/agents/governor-reviewer.yaml)
under `accountability:` (added v1.1.44 with this page).

### Concurrency

**Single identifier per task.** Parallel subtasks inherit the
parent task's identifier; the framework does not introduce
parent-child or Directed Acyclic Graph (DAG) relationships at this
layer.

This is a deliberate scope cut. Hierarchical correlation-ID
relationships invite every hard distributed-tracing problem (clock
skew, fan-out reconciliation, cross-pipeline joins) the framework
explicitly does not need. Tasks that genuinely require nested
correlation can be modelled as multiple top-level tasks coordinated
through the existing escalation channel.

### Coverage requirement

Every artefact in **binding scope** must carry the identifier.
*Binding scope* is defined per compliance tier in
[Compliance profile](adopt/compliance-profile.md):

- **Artefact-only tier:** binding not required; `agov-validate-binding`
  passes regardless.
- **Process-aware tier:** Layer 2 enforced; binding still optional.
- **Full tier:** every commit on an agent-active branch carries one
  or more `Agent-ID:` trailers; every artefact stored outside the
  commit history (build outputs, container images) carries a sidecar
  per B3 (or the filename-suffix fallback in environments stripping
  sidecars).

Until [P1](https://gitlab.com/janipaijanen/agent-governance/-/issues/?label_name%5B%5D=engagement-action-plan-2026-05-04)
ships `framework.yaml` declaring an adopter's tier, the validator
runs in *schema-only* mode — see [Validator surface](#validator-surface)
below.

## Commit trailer (B2)

### Format

```text
Agent-ID: <agent-name>:<agent-version>:<correlation-id>
```

Concrete example:

```text
Agent-ID: coder-tester:0.3.2:01931f2c-7e8a-7b6c-9def-0123456789ab
```

Field shapes:

| Field | Shape | Source |
|---|---|---|
| `<agent-name>` | Kebab-case identifier matching an agent definition file (`agents/<name>.yaml`) | The agent definition's `agent.name` field |
| `<agent-version>` | Semantic version of the agent definition that produced the artefact | The agent definition's `agent.version` field |
| `<correlation-id>` | UUID v7 in canonical lowercase string form | `correlation_id.generate()` at task assignment |

The trailer lives in the commit's *trailer block* — the last
paragraph of the commit message, where every line matches the
`key: value` shape. An `Agent-ID:`-prefixed line in the body proper
is not a trailer and is not extracted by the validator.

### Multi-trailer policy

Multiple `Agent-ID:` trailers are permitted on a single commit and
are **ordered chronologically by contribution**. The shape arises
when a commit is the joint output of more than one agent — for
example, planner-architect drafted the implementation plan, then
coder-tester produced the code on top of that plan, both
contributing to the same merge candidate.

```text
feat: add per-trip currency conversion

Implements ADR-0042. Closes #131.

Agent-ID: planner-architect:0.4.0:01931f2c-7e8a-7b6c-9def-0123456789ab
Agent-ID: coder-tester:0.3.2:01931f2c-9012-7345-8678-9abcdef01234
```

The two correlation IDs differ here because each agent's task was
assigned separately by Governor-Reviewer; they would share an ID if
the second agent's work was a parallel subtask of the first under
one umbrella assignment.

### Branch classification

Not every branch is governed by the same enforcement policy. The
framework recognises three classes:

| Class | Trailer required? | Notes |
|---|---|---|
| **Agent-active** | Yes (in enforce mode) | Branches where production agents commit. Default pattern: `agent/*`. |
| **Human-only** | No | Branches reserved for direct human work (hot-fixes, emergency overrides). Trailers are optional even in enforce mode. |
| **Protected** | No | Branches that should not receive direct commits at all (e.g., `main`, `develop`). Treated as exempt from the trailer requirement; enforcement comes from branch-protection rules instead. |

Configuration lives in `framework.yaml` under `binding.agent_active_branches`
(introduced by P1). Until P1 ships, the validator falls back to the
defaults: `agent/*` is agent-active; everything else is treated as
non-active and exempt from missing-trailer findings.

### Default `agent/*` fallback

Why `agent/*` specifically: it is the convention the framework's
own contributor guide uses for branches where AI agents do
production work, and it is unlikely to collide with adopter
naming conventions for human-only or feature branches. Adopters
who already use `agent/*` for something else can override the
pattern in `framework.yaml` (after P1 lands) without modifying the
validator.

## Sidecar metadata (B3)

For artefacts that do not live in commit history — build outputs,
container images, generated documentation bundles — the binding
mechanism is a **sidecar file** rather than a trailer.

### Sidecar shape

```text
<artefact-filename>.agov.json
```

The conventional sidecar location appends `.agov.json` to the
artefact's full filename — so `release.tar.gz` is paired with
`release.tar.gz.agov.json` (existing extension preserved). The
helper `validate_binding.sidecar_path(artefact)` resolves the
location.

Required fields:

```json
{
  "correlation_id": "01931f2c-7e8a-7b6c-9def-0123456789ab",
  "agent_id": "coder-tester:0.3.2",
  "produced_at": "2026-05-04T18:42:00Z",
  "artefact_sha256": "abc123…"
}
```

| Field | Type | Validation |
|---|---|---|
| `correlation_id` | string | UUID v7 in canonical lowercase form (per [B1](#correlation-identifier-b1)) |
| `agent_id` | string | `name:version` form, exactly one colon (mirrors the trailer's first two fields per [B2](#commit-trailer-b2)) |
| `produced_at` | string | Timestamp the artefact was produced. Recommended: Internet Society (ISO) 8601 in Coordinated Universal Time (UTC) (`2026-05-04T18:42:00Z`); the parser does not enforce a strict timestamp format |
| `artefact_sha256` | string | Lowercase hex digest of the artefact file from the Secure Hash Algorithm (SHA), 256-bit variant (SHA-256), 64 characters |

Extra fields are accepted silently for forward compatibility.

### Hash verification

The validator recomputes the artefact's SHA-256 with
`compute_artefact_sha256(path)` and compares against the sidecar's
declared `artefact_sha256`. Mismatch yields a finding of the form
*"artefact sha256 mismatch for &lt;name&gt;: sidecar declared X,
computed Y"*.

This is the binding's tamper-evidence layer at the artefact level:
if the sidecar travels with the artefact through a build pipeline
and the artefact is later modified, the next `verify_sidecar` run
fails.

### Filename-suffix fallback

Some artefact pipelines strip non-image files. Container registries
that only accept Open Container Initiative (OCI) layers, certain
Continuous Delivery (CD) systems that publish only the primary
artefact, and signing pipelines that re-package outputs all have
the property: the sidecar does not arrive with the artefact at the
consuming end.

For these environments, encode the correlation identifier into the
artefact's filename:

```text
<base>.<corr-id>.<ext>
```

Example: `release.01931f2c-7e8a-7b6c-9def-0123456789ab.tar.gz`.
The validator extracts the identifier with
`parse_filename_fallback(path_or_filename)`. Only UUID v7 tokens
are recognised (per `is_valid()` in [B1](#correlation-identifier-b1));
a UUID v4 token in a filename is not a correlation identifier.

The fallback gives up the hash check (filename can carry the ID,
not the digest) and gives up `agent_id` and `produced_at` (those
live in the sidecar JavaScript Object Notation (JSON) only).
Adopters whose pipeline strips sidecars should consider whether
the loss of tamper-evidence is acceptable; if not, the sidecar
should be re-attached at a downstream stage that does preserve
auxiliary files.

### Pipeline propagation

**Build pipelines must include sidecars in artefact-publishing
steps.** This is a per-adopter responsibility: the framework
provides the sidecar shape and the validator; the adopter ensures
the sidecar travels with its artefact through the publishing
pipeline. Documenting the propagation requirement in the adopter's
`GOVERNANCE.md` or build documentation is the recommended pattern.

For artefacts whose pipeline cannot carry sidecars at all, choose
the filename-suffix fallback at the artefact's *creation* step —
later steps cannot reconstruct an identifier they were never given.

## Validator surface

The CI-callable entry point is `agov-validate-binding`, exposed as
a Python script in `pyproject.toml [project.scripts]`. The
underlying Application Programming Interface (API) is also
importable for adopter glue:

```python
from agents_govern.validate_binding import (
    # Commit trailer (B2)
    parse_trailer,           # parse one trailer line
    extract_trailers,        # extract from a commit message
    is_agent_active_branch,  # classify a branch per config
    validate_commit,         # per-commit finding list
    # Sidecar metadata + filename fallback (B3)
    Sidecar,                 # dataclass for parsed sidecar
    parse_sidecar,           # JSON-text parser
    sidecar_path,            # resolve <artefact>.agov.json
    compute_artefact_sha256, # SHA-256 hex digest
    read_sidecar,            # read sidecar from filesystem
    verify_sidecar,          # recompute hash, report mismatch
    parse_filename_fallback, # extract corr-id from filename
)
from agents_govern.correlation_id import generate, is_valid
```

### Schema-only mode (today)

When no `framework.yaml` is present (or its `binding:` section is
absent), the validator runs in *schema-only* mode:

- **Confirms the contract is loadable** — UUID v7 generator, trailer
  parser, branch classifier, sidecar parser, hash helper, and
  filename-fallback parser all import without error.
- **Flags malformed `Agent-ID:` lines** wherever they appear in a
  commit message. A contributor who wrote `Agent-ID:` clearly
  intended a trailer; a malformed value is a contract violation
  regardless of where it appears.
- **Does not flag missing trailers or sidecars.** A commit without
  an `Agent-ID:` trailer or an artefact without a sidecar is fine
  in schema-only mode — enforcement applies prospectively from the
  adopter's tier-upgrade date, never retroactively.

This is the mode the framework's own repository runs in at v1.1.45
(no `framework.yaml` exists yet; P1 introduces it in a later batch).

### Enforce mode (after P1 ships `framework.yaml`)

When `framework.yaml` declares a `binding:` section, the validator
walks recent commits, classifies each branch via
`is_agent_active_branch`, and applies `validate_commit` per commit.
Commits on agent-active branches must carry one or more well-formed
trailers; commits on human-only or protected branches are exempt.

### Exit codes

Per the [strict-contract exit-code taxonomy](integration-contract.md#exit-codes):

| Code | Condition |
|---|---|
| `0` | Pass (schema-only mode loadable; or enforce mode with no findings). |
| `1` | Validation failure — malformed trailer, or missing trailer in enforce mode on an agent-active branch. |

Higher codes (2 / 3 / 4) are reserved for configuration errors,
invariant violations, and infrastructure failures per the taxonomy;
the binding validator does not currently emit them.

## Migration discipline

**Prospective only.** Existing commits without `Agent-ID:` trailers
are not retroactively validated. Enforcement applies from the
adopter's declared tier-upgrade date — typically the merge commit
that introduces the `framework.yaml` `binding:` section into the
adopter's repository. Earlier history is grandfathered.

This mirrors the discipline the framework's own
[acronym lint](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/.claude/rules/docs/abbreviation-rule.md)
and `validate-learnings` apply: per-MR enforcement on touched
artefacts, never sweeping retroactive validation that would block
unrelated work.

## See also

- [Integration contract](integration-contract.md) — canonical home
  for the strict-contract surface; the binding section there
  cross-references this page.
- [Compliance profile](adopt/compliance-profile.md) — three-tier
  vocabulary (Artefact-only / Process-aware / Full); binding scope
  per tier.
- [Position overview's boundary refinement](position/overview.md#boundary-refinement-2026-05-04)
  — *"no runtime behaviour enforcement; runtime artefact production
  requirements stand"*. Binding sits squarely on the
  artefact-production side of that refinement.
- [Action plan — Phase 2 binding layer](plans/2026-05-04-chatgpt-engagement-action-plan.md#phase-2-binding-layer-closes-layer-3-meta-finding)
  — the engagement-derived plan that scoped B1, B2, B3.
- [`src/agents_govern/correlation_id.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/correlation_id.py)
  and
  [`src/agents_govern/validate_binding.py`](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/src/agents_govern/validate_binding.py)
  — implementations.
