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
| What role should I resume as, and what is pending? | Read recent coordination files, manifest, branch status, and claims. | `mix agov.session.resume` |
| Who owns or has claimed an artifact? | Search coordination records and infer claim state. | `mix agov.coord.status` and `mix agov.coord.stale` |
| What is the next handoff for a role? | Search handoff files and mentally subtract resolved items. | `mix agov.handoff.next` |
| Is a feature branch hiding coordination obligations? | Compare branch-local messages against base branch manually. | `mix agov.coord.visibility` |
| Is a coordination message or plan structurally valid? | Ask an AI or reviewer to inspect front matter. | `mix agov.coord.validate` and `mix agov.plan.validate` |
| Which roles have been active recently? | Count senders across the channel manually. | `mix agov.usage.report` |
| How much deterministic output is replacing manual context reads? | Manually count command output and compare to broad context ranges. | `mix agov.token.roi` |

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
tool-output sizes over time:

```bash
cd tooling/elixir

# Print accumulated records.
mix agov.token.roi report

# Explicitly record a measurement from any command or manual count.
mix agov.token.roi record \
  --command agov.coord.visibility \
  --workflow branch-visibility \
  --chars 8235 \
  --manual-chars 48000
```

Default log path:

```text
../../tmp/agov-token-roi.jsonl
```

The path is local and ignored by git through the repository's existing `tmp/`
ignore rule. Set `AGOV_TOKEN_ROI_LOG` to write somewhere else. Set
`AGOV_TOKEN_ROI=0` to disable automatic writes.

Automatic POC collection currently records successful output sizes for:

- `mix agov.session.resume`
- `mix agov.handoff.next`

Each JSONL record stores the command name, arguments, workflow label, character
count, approximate token count, and timestamp. The log intentionally does not
capture prompts, model responses, secrets, billing data, or external API
metadata.

This is enough to answer local questions such as:

- how often the deterministic resume and handoff tools are used;
- how many characters/tokens their summaries produce;
- whether manual baseline estimates recorded with `--manual-chars` exceed the
  deterministic tool-output size.

It is not billing telemetry and does not claim exact model-token accounting.

## Live Baseline

Measured on the foundry coordination channel after seq 251:

| Artifact or command | Character count | Approx tokens |
|---|---:|---:|
| Numbered coordination files, full channel | 489,229 | 122,308 |
| `mix agov.session.resume --base-branch master` | 1,802 | 451 |
| `mix agov.session.resume --role governor-reviewer --base-branch master` | 1,098 | 275 |
| `mix agov.handoff.next governor-reviewer` | 618 | 155 |
| `mix agov.coord.status` | 1,118 | 280 |
| `mix agov.usage.report --last 50` | 577 | 145 |
| `mix agov.coord.visibility --base-branch master` | 8,235 | 2,059 |

These numbers do not mean every session previously read the whole channel.
They show the upper bound of context that tooling protects agents from loading
when the question is narrow.

A more realistic cold-start comparison is:

| Workflow | Manual context estimate | Tool output estimate | Estimated saving |
|---|---:|---:|---:|
| Cold start: identify role, pending handoffs, decisions, dirty state | 3,000-5,000 tokens | 300-500 tokens | 2,500-4,500 tokens |
| Find next governor-reviewer handoff | 1,000-3,000 tokens | 100-300 tokens | 700-2,900 tokens |
| Role activity review, last 50 messages | 2,000-6,000 tokens | 100-300 tokens | 1,700-5,900 tokens |
| Branch visibility review | 4,000-12,000 tokens | 1,000-3,000 tokens | 1,000-11,000 tokens |

The branch visibility scanner can still produce long output because it reports
real unresolved historical state. Even then, it is cheaper and safer than
asking an AI to compare every local branch by hand.

## How To Reproduce

Run from `tooling/elixir/`:

```bash
# Full channel size, from repository root:
find docs/coordination -maxdepth 1 -type f \
  | grep -E '/[0-9]{5}-[^/]+\.md$' \
  | sort \
  | xargs wc -c \
  | tail -1

# Tool output sizes:
mix agov.session.resume --base-branch master | wc -c
mix agov.session.resume --role governor-reviewer --base-branch master | wc -c
mix agov.handoff.next governor-reviewer 2>&1 | wc -c
mix agov.coord.status 2>&1 | wc -c
mix agov.usage.report --last 50 2>&1 | wc -c
mix agov.coord.visibility --base-branch master 2>&1 | wc -c
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

The POC reports measured output size, but more automation is still useful. A
future workflow-aware command could compute:

```bash
mix agov.token.roi --workflow cold-start --role governor-reviewer
```

Useful follow-ons:

- Add `--format json` to existing reporting commands so token ROI can be
  measured without scraping text.
- Extend automatic collection to more deterministic commands once their output
  formatting is centralized.
- Track command output character counts in coordination handoffs when useful.
- Add a lightweight `mix agov.token.estimate <file-or-command-output>` helper.
- Extend `mix agov.usage.report` to include receiver counts and model/session
  provenance once coordination records consistently include those fields.

## Conclusion

AGF v2 has achieved the first stage of the token-reduction goal for foundry:
the highest-repeat coordination tasks now have deterministic summaries and
validators. The POC starts the second stage: local, append-only measurement of
deterministic output so ROI reports can be generated on request.
