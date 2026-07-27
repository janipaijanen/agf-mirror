# Tooling Token ROI

One AGF v2 goal is to reduce AI token consumption by moving repeatable
coordination and validation work out of prompts and into deterministic tooling.
This is partly accomplished for foundry's coordination workflow.

This page defines what counts as offloaded work, what has been shipped, how to
estimate the token return on that tooling, and how the local POC starts
collecting data that can be reported on request.

## Current Assessment

AGF has reduced token use where agents previously had to reread broad
governance context to answer narrow operational questions:

| Question | Manual AI work before tooling | Current tooling |
|---|---|---|
| What role should I resume as, and what is pending? | Read recent coordination files, manifest, branch status, and claims. | `agov session resume` |
| Who owns or has claimed an artifact? | Search coordination records and infer claim state. | `agov coord status` and `agov coord stale` |
| What is the next handoff for a role? | Search handoff files and mentally subtract resolved items. | `agov handoff next` |
| Is a feature branch hiding coordination obligations? | Compare branch-local messages against base branch manually. | `agov coord visibility` |
| Is a coordination message or plan structurally valid? | Ask an AI or reviewer to inspect front matter. | `agov coord validate` and `agov plan validate` |
| Which roles have been active recently? | Count senders across the channel manually. | `agov usage report` |
| How much deterministic output is replacing manual context reads? | Manually count command output and compare to broad context ranges. | `agov token roi` |

The goal is not to eliminate AI reasoning. The goal is to reserve AI context for
judgment, design, review, and synthesis, while tooling handles scanning,
counting, validation, sorting, and state reconstruction.

## What To Measure

Token savings are estimated per repeated workflow:

```text
estimated_tokens_saved =
  tokens(manual_context_that_would_have_been_read)
  - tokens(tool_output_that_was_read)
  - tokens(extra_prompt_needed_to_invoke_or_explain_the_tool)
```

Use this as an estimate, not a billing-grade measurement. Different models
tokenize text differently. For local tracking, use one consistent
approximation:

```text
estimated_tokens = ceil(character_count / 4)
```

When precision matters, replace the approximation with the tokenizer for the
model actually used.

## Local Telemetry POC

Foundry now has a lightweight local telemetry proof of concept for collecting
tool-output sizes and potential token-savings estimates over time:

```bash

# Print accumulated records.
agov token roi report

# Explicitly record a measurement from any command or manual count.
agov token roi record \
  --command agov.coord.visibility \
  --workflow branch-visibility \
  --chars 8235 \
  --manual-chars 48000
```

Default log path, relative to the detected AGF project root:

```text
tmp/agov-token-roi.jsonl
```

The path is local and ignored by git through the repository's existing `tmp/`
ignore rule. See the AGF environment variable reference for the canonical
documentation of `AGOV_TOKEN_ROI_LOG` and `AGOV_TOKEN_ROI`.

The writer and `agov token roi report` use the same resolution order: report
`--log`, then `AGOV_TOKEN_ROI_LOG`, then the detected project's default. Do not
configure `export AGOV_TOKEN_ROI_LOG="$PWD/..."` globally: the expanded value is
tied to the directory where that shell started. Outside an AGF project there is
no implicit current-directory fallback; supply an explicit path when that is
intentional.

Automatic POC collection currently records successful output sizes and
workflow-baseline savings estimates for:

- `agov session resume`
- `agov handoff next`

Each JSONL record stores the command name, arguments, workflow label, character
count, approximate token count, prompt-overhead estimate, baseline source, and
approximate saved-token range when the workflow has a documented baseline. The
log intentionally does not capture prompts, model responses, secrets, billing
data, or external API metadata.

### Transport attribution (`transport`, `adapter`, `tool`)

v2.4.0 Workstream C adds three optional fields distinguishing an
MCP-mediated invocation from a direct CLI invocation:

- `transport`: `"cli"` (explicit, so a new direct-CLI record is
  distinguishable from a historical record predating these fields) or
  `"mcp"`.
- `adapter`: `"agov-mcp"`, present only when `transport` is `"mcp"`.
- `tool`: the MCP tool name (e.g. `agov_coord_validate`), present only
  when `transport` is `"mcp"`, drawn from `agov-mcp`'s own compiled tool
  list (the same list backing `tools/list`).

**These fields are self-reported workflow measurement, not verified or
authenticated identity.** `agov-mcp` sets fixed environment markers
(`AGOV_MCP_TRANSPORT`, `AGOV_MCP_ADAPTER`, `AGOV_MCP_TOOL`) on the `agov`
child process it spawns; `agov` parses those markers and emits fixed
constants, never the raw environment strings. A local process could set
the same environment variables directly and produce a record indistinguishable
from a real MCP-mediated call — this is not prevented, and no
pseudo-authentication (signing, PID-tree verification) is attempted, per
`simplicity-rule`. Treat `transport`/`adapter`/`tool` as a trusted-adapter
self-report for usage measurement, never as security or audit evidence.

If the environment markers are absent, all three fields resolve to the
`"cli"` case. If the markers are present but partial, malformed, or name
an unrecognized tool, **all three fields are omitted** from the record —
a third, distinct shape from both the clean-CLI and clean-MCP cases — and
the raw marker values are never persisted.

Old JSONL records written before this change simply lack these three
keys; readers (`agov token roi report`, `summarize()`) already ignore
unknown/absent keys, so no migration or schema-version bump was needed.

### Argument redaction (default-deny)

The `args` field is **redacted by default**, uniformly for both
transports — this is not an MCP-specific policy. Only closed-vocabulary
tokens survive:

- subcommand/sub-subcommand path tokens (e.g. `coord`, `validate`,
  `dist`);
- flag *names* that are part of the CLI's own compiled surface;
- flag *values* for a small allowlist of genuinely closed-vocabulary
  flags (currently `--kind`, `--id-format`, `--format`,
  `--implementation`).

Every other token — free-form flag values (`--topic`, `--task`, `--dir`,
`--ref`, …), positional values (file paths, coordination IDs), and any
unrecognized flag (name **and** value, since an unrecognized flag name is
caller-controlled and could smuggle identifying content) — is replaced
with the literal string `[redacted]`, preserving record shape and
argument-count diagnostics without preserving content. This closes a
gap that predated Workstream C: the ROI log previously wrote the
complete raw argument vector for every invocation, direct CLI included.

This is enough to answer local questions such as:

- how often the deterministic resume and handoff tools are used;
- how many characters/tokens their summaries produce;
- how much context was probably avoided for known workflows;
- whether manual baseline estimates recorded with `--manual-chars` exceed the
  deterministic tool-output size.

It is not billing telemetry and does not claim exact model-token accounting.

## Live Baseline

Measured on the foundry coordination channel after seq 251:

| Artifact or command | Character count | Approx tokens |
|---|---:|---:|
| Numbered coordination files, full channel | 489,229 | 122,308 |
| `agov session resume --base-branch master` | 1,802 | 451 |
| `agov session resume --role governor-reviewer --base-branch master` | 1,098 | 275 |
| `agov handoff next governor-reviewer` | 618 | 155 |
| `agov coord status` | 1,118 | 280 |
| `agov usage report --last 50` | 577 | 145 |
| `agov coord visibility --base-branch master` | 8,235 | 2,059 |

These numbers do not mean every session previously read the whole channel.
They show the upper bound of context that tooling protects agents from loading
when the question is narrow.

A more realistic cold-start comparison is:

These ranges are also used by automatic telemetry for known workflows:

| Workflow | Manual context estimate | Tool output estimate | Estimated saving | Automatic workflow labels |
|---|---:|---:|---:|---|
| Cold start: identify role, pending handoffs, decisions, dirty state | 3,000-5,000 tokens | 300-500 tokens | 2,500-4,500 tokens | `cold-start`, `session-resume:<role>` |
| Find next governor-reviewer handoff | 1,000-3,000 tokens | 100-300 tokens | 700-2,900 tokens | `next-handoff:<role>` |
| Role activity review, last 50 messages | 2,000-6,000 tokens | 100-300 tokens | 1,700-5,900 tokens | not automatic yet |
| Branch visibility review | 4,000-12,000 tokens | 1,000-3,000 tokens | 1,000-11,000 tokens | not automatic yet |

### Validation Baseline

Rust `agov` Phase 1 validation commands also record local `tool_output`
telemetry when they complete successfully. These ranges estimate the manual
schema/rule/context reads avoided by deterministic validation output:

| Workflow | Manual context estimate | Tool output estimate | Estimated saving | Automatic workflow labels |
|---|---:|---:|---:|---|
| Coordination message validation | 500-1,500 tokens | 25-300 tokens | 200-1,400 tokens | `coord-validate` |
| Gate 1 plan validation | 800-2,000 tokens | 25-400 tokens | 400-1,900 tokens | `plan-validate` |
| Full validation suite | 1,500-4,000 tokens | 300-1,500 tokens | 0-3,700 tokens | `validate:umbrella` |
| Single validation subcommand | 500-2,000 tokens | 25-1,000 tokens | 0-1,900 tokens | `validate:<subcommand>` |

The branch visibility scanner can still produce long output because it reports
real unresolved historical state. Even then, it is cheaper and safer than
asking an AI to compare every local branch by hand.

## How To Reproduce

Run from the repository root:

```bash
# Full channel size, from repository root:
find docs/coordination -maxdepth 1 -type f \
  | grep -E '/[0-9]{5}-[^/]+\.md$' \
  | sort \
  | xargs wc -c \
  | tail -1

# Core validation output sizes:
agov coord validate docs/coordination/00302-governor-reviewer.md 2>&1 | wc -c
agov plan validate docs/plans/tool-authority-boundary-rule.md 2>&1 | wc -c

# Tool output sizes:
agov session resume --base-branch master | wc -c
agov session resume --role governor-reviewer --base-branch master | wc -c
agov handoff next governor-reviewer 2>&1 | wc -c
agov coord status 2>&1 | wc -c
agov usage report --last 50 2>&1 | wc -c
agov coord visibility --base-branch master 2>&1 | wc -c
```

Then divide each character count by 4 for the rough token estimate.

## Evidence Standard

When claiming a tooling change reduces token consumption, record:

- the workflow being replaced;
- the manual files or context an agent previously had to read;
- the command output an agent reads after the change;
- character counts for both sides, or a stated estimate range;
- any quality benefit that is not captured by token count, such as fewer stale
  handoff mistakes or fewer invalid coordination records.

Avoid claiming exact savings unless measured with the model's tokenizer and a
fixed prompt. Most AGF evidence should use ranges.

## Remaining Gaps

The POC reports measured output size and potential savings for known live
workflows. More automation is still useful. A future workflow-aware command
could compute:

```bash
agov token roi --workflow cold-start --role governor-reviewer
```

Useful follow-ons:

- Add `--format json` to existing reporting commands so token ROI can be
  measured without scraping text.
- Extend automatic collection to more deterministic commands once their output
  formatting is centralized.
- Track command output character counts in coordination handoffs when useful.
- Add a lightweight `agov token estimate <file-or-command-output>` helper.
- Extend `agov usage report` to include receiver counts and model/session
  provenance once coordination records consistently include those fields.

## Conclusion

AGF v2 has achieved the first stage of the token-reduction goal for foundry:
the highest-repeat coordination tasks now have deterministic summaries and
validators. The POC starts the second stage: local, append-only measurement of
deterministic output so ROI reports can be generated on request.
