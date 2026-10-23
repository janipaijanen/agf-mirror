# Tooling conformance

*Status: Draft AGF v2 contract.*

AGF defines contracts, not a sacred implementation language. The distributed
tooling surface is the platform-specific Rust `agov` binary; it is both the
release artifact and the reference implementation.

Future Python, Ruby, C, C++, Julia, or other implementations can be
AGF-compatible if they conform to the same command behavior, schemas, exit
codes, and conformance fixtures.

## Compatibility terms

| Term | Meaning |
|---|---|
| AGF release implementation | The platform-specific Rust `agov` binary distributed for adopters. |
| AGF-compatible implementation | Independently implemented, but passes the AGF conformance suite for the claimed version. |
| Development implementation | Source-checkout tooling used by framework contributors; useful as implementation evidence, but not itself a release artifact. |
| Experimental implementation | Useful for evaluation, but not yet passing the conformance suite. |
| Not verified for AGF v2 | Inherited, partial, or third-party tooling with no current AGF v2 conformance result. |

## Current and Target CLI Surface

The current adopter command is `agov`. A future public interface may become
`agf`, but released v2.0 archives document `agov`.

Command status uses the [maturity vocabulary](maturity-vocabulary.md): Level 4
Validated means implemented and tested in a branch or checkout; Level 5
Deployable means the supported operator path or release artifact includes the
command and the documentation matches that path. Do not describe a command as
deployable merely because a wrapper advertises it.

The table lists the adopter-facing command surface.

| Adopter command | Purpose | Status |
|---|---|---|
| `agov --version` | Print the binary version plus build commit SHA and dirty state. | Release surface |
| `agov version --format json` | Print structured version, commit, dirty-state, and deterministic tooling-source fingerprint provenance. | Release surface |
| `agov coord validate <file>...` | Validate coordination messages against the schema. | Release surface |
| `agov plan validate <file>...` | Validate Gate 1 plan front matter and advisory consultation metadata. | Release surface |
| `agov validate [subcommand]` | Validate framework artifacts such as adoption, agents, learnings, infosec, contributions, or framework config. | Release surface |
| `agov coord brief [args...]` | Generate a coordination message ID and front matter. | Release surface |
| `agov handoff next <role> [args...]` | Show the next unresolved handoff for one role. | Release surface |
| `agov coord status [args...]` | Summarize coordination channel activity, roles, pending handoffs, and claims. | Release surface |
| `agov session resume [args...]` | Print cold-start or role-specific session context. | Release surface |
| `agov coord stale [args...]` | Report expired artifact claims. | Release surface |
| `agov coord visibility [args...]` | Detect coordination obligations hidden on non-base branches. | Release surface |
| `agov coord graph [args...]` | Render role-to-role coordination flows as Graphviz dot or text. | Release surface |
| `agov coord graph-index [args...]` | Emit the coordination graph as deterministic JSON for cold starts and tooling. See the [graph-index contract](coord-graph-index.md). | Release surface |
| `agov coord list [--since <cursor>] [--dir <path>] [--format text\|json] [--ref <commit-ish>]` | List valid coordination messages after a cursor. `--ref` reads the channel from an already-fetched commit (local Git only, bounded: at most 50,000 entries, 1 MiB per record, 128 MiB total; symlinks and submodules are skipped and counted). Exit `0` found / `2` none found or a ref that cannot be read (the reason is on stderr) — not a failure of the tool itself. | Release surface |
| `agov coord pending [--cutoff <RFC3339>] [--format text\|json]` | Report unresolved non-`handoff` `requires-ack: true` coordination records. Exit `0` found / `2` none found — not a failure. | Release surface |
| `agov coord convergence --series <id> [--format text\|json]` | Verify and report the deterministic per-invariant state of an opt-in review-convergence series. Exit `1` for invalid transitions or unresolved CLEAR/BLOCK conflicts. | Release surface |
| `agov token roi report [args...]` | Summarize local deterministic-tool token savings telemetry. | Release surface |
| `agov usage report [args...]` | Count active roles, senders, message kinds, and unknown senders. | Release surface |

Developer/source-checkout users should use the same `agov` command surface as
release-archive adopters.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Success, no findings requiring failure. |
| `1` | Invocation or validation failure. |
| `2` | No work found / clean negative result where that is the command contract. |
| `3` | Reserved for cross-artifact invariant failure. |
| `4` | Reserved for infrastructure or input/output failure. |

For `coord convergence`, exit `0` means the declared event sequence is
structurally coherent. It is not a review recommendation or gate approval.
Exit `1` means the opted-in sequence is invalid or paused by conflicting CLEAR
and BLOCK outcomes; the command never resolves that conflict or accepts risk.
Candidate SHAs and scoped blobs must be locally available so the command can
recompute the declared digest.

The command itself is Level 5 Deployable: shipped since v2.8.0 (2026-08-28)
and verified present in the published v2.10.0 Linux x86_64 archive
(`agov coord convergence --help` exits `0`; archive SHA-256
`b54c55b5b31238973514c130e534b846de701c3c2a0368f81682f295c08383fd`,
matching the published `.sha256` asset). This is separate from the
*review-convergence workflow's* own evidence status — the per-invariant
reset boundary, conflict handling, and blocker-materiality vocabulary the
command implements are Empirically informed (one redesigned candidate and
targeted review to date, per the v2.8.0 CHANGELOG entry), not yet
Empirically validated at scale. Command availability and workflow
track record are two different claims; this page states the former.

## Conformance suite

A future `agf/conformance/` directory should test fixtures, exit codes, JSON
output shape where specified, path handling, deterministic ordering, and
lifecycle behavior such as transitive handoff resolution.

Initial fixture families should cover:

| Fixture family | What it proves |
|---|---|
| Command discovery | `--help`, `--version`, and documented subcommands exist with stable spelling. |
| Exit codes | Success, invocation failure, validation failure, and clean negative-result cases return the documented codes. |
| Schema validation | Valid and invalid coordination, plan, agent, learning, and config files produce deterministic results. |
| JSON output | `version --format json`, `coord graph-index`, usage reports, and other JSON outputs preserve field names, ordering expectations, and omission rules. |
| Path handling | Relative paths, explicit `--dir` options, environment overrides, and release-archive layouts behave the same. |
| Deterministic ordering | Status, graph, graph-index, handoff, and usage outputs sort records predictably across filesystems. |
| Lifecycle behavior | Claims, handoffs, reviews, acks, completions, stale claims, visibility checks, and opted-in convergence transitions resolve the same way across implementations. |

Until those fixtures ship, only the Rust `agov` release binary should be
described as the supported AGF v2 tooling implementation. Other language
implementations may be useful experiments, but they are not AGF-compatible for
a claimed version until they pass the conformance suite for that version.

The contract source lives in `agf/docs/contracts/tooling-conformance.md`.

### Coordination visibility scope

The channel is lowercase `.md` files directly in `docs/coordination`, including README.md; invalid records are diagnosed normally. Visibility also inspects the complete channel subtree, direct docs siblings named `coordination*.md`, and `docs/coordination-archive`. With a custom `--dir`, these scopes are relative to that channel's parent/name. This is bounded discovery, not a whole-repository inbox.

JSON adds `scanned_refs` (ref to exact commit SHA), `scanned_scopes`, `coverage`, `candidate_diagnostics`, `invalid_channel_records`, and `archive_baseline` (count, records, lifecycle). Candidates include exact path/blob, ref SHA and valid record metadata, without a second lifecycle resolver. The matching text output exposes the same evidence; MCP returns CLI stdout and status unchanged. Archive baseline means exact target path/blob equality and always says lifecycle/closure unassessed. New, changed and moved archive records warn; target-present nonarchive nested records also warn. Ordinary prose outside the channel is not automatically a candidate; recognizable invalid front matter is diagnosed.

Exit 0 indicates no advisory warnings within the declared complete scan; exit 1 reports misplaced/invalid candidates, invalid channel records or unbridged obligations; exit 2 retains collision/error semantics and includes incomplete coverage on unavailable targets/schemas/blobs. A nonzero diagnostic does not grant this tool governance authority. See the coordination publication rule for the governing contract.

### Unresolved handoff reply evidence

`handoff next` and both `session resume` views can show
`reply present — acknowledgement unconfirmed` followed by `reply_evidence:`,
a JSON array of objects with `id` and `from` strings. The same text is returned
unchanged in MCP `structuredContent.stdout`; the execution envelope is unchanged.
The line is omitted when no qualifying evidence exists.

Evidence is a strictly later, valid `note` or `review` directly naming the handoff
in structured `references-to`, from its `handoff.next_agent` role or the
governor-reviewer role. Timestamp ties, indirect links and prose references do
not qualify. Role text is not authenticated identity. A progress report, question
or refusal can qualify: this annotation does not mean answered, approved or
complete. It changes no queue membership, ordering, status or exit code.

The existing resolver still clears handoffs when a later ack, completion or
decision-record appears in their transitive descendant graph. Reply evidence
applies only to handoffs that resolver leaves unresolved. `coord pending`
continues to exclude handoffs. This is advisory tooling evidence under the
existing coordination publication contract, not a new gate or lifecycle rule.
