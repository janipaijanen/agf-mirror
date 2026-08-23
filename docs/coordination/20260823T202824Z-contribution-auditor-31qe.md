---
kind: note
from: contribution-auditor
to: all
id: 20260823T202824Z-contribution-auditor-31qe
timestamp: 2026-08-23T20:28:24Z
model: codex-gpt-5
immutable: true
references-to: []
topic: agov-roi-canonical-log-and-mcp-evaluation-queue
requires-ack: true
artifacts_owned: []
---

# Canonical local ROI logs and queued CLI-versus-MCP evaluation

## Human-authorized operating direction

The governor requested consolidation of worktree-local `agov` token-ROI
telemetry, direction for future worktrees to write to the primary checkout's
logs, and queuing of the remaining MCP adoption and cost-evaluation work.

This note does not authorize Gate 1, implementation, merge, release, tag, or
deployment. Each recipient must acknowledge it directly after applying the
runtime configuration appropriate to that assistant.

## Canonical live-log convention

Each assistant must set `AGOV_TOKEN_ROI_LOG` to an absolute path under the
primary checkout, not its disposable or feature worktree:

```text
<primary-checkout>/tmp/<assistant-abbreviation>-agov-token-roi.jsonl
```

Foundry's current canonical names remain:

```text
<primary-checkout>/tmp/codex-agov-token-roi.jsonl
<primary-checkout>/tmp/claude-agov-token-roi.jsonl
```

Pass the same variable into `agov-mcp`'s server environment. Do not configure
a `$PWD`-expanded value globally: a long-lived shell can pin it to the wrong
project. Do not use one live cross-assistant file yet; current records do not
contain assistant/session provenance, so doing so would destroy attribution.
Cross-assistant reporting must derive a snapshot from the per-assistant logs.

This is local ignored telemetry. It must not be committed to `main` or a
feature branch. The phrase "primary checkout log" means the ignored `tmp/`
path in the primary checkout, not a Git-tracked artifact.

## Consolidation evidence

On 2026-08-23 the local inventory found 76 Codex-named runtime logs and two
unattributed default-name logs. It found no correctly named Claude or other-
assistant runtime log; attribution was not inferred from worktree names.

- Codex-named valid records were JSON-normalized and exact-record deduplicated
  into `/home/devi/src/AGF-NG/tmp/codex-agov-token-roi.jsonl`.
- Fourteen valid default-name records were kept separately at
  `/home/devi/src/AGF-NG/tmp/unattributed-agov-token-roi.jsonl`.
- Two malformed historical lines in `/home/devi/tmp/codex-agov-token-roi.jsonl`
  were excluded. They contained interleaved partial records from older
  concurrent writes.
- Original source logs were not deleted.
- The immutable local analysis snapshot is
  `/home/devi/src/AGF-NG/tmp/roi-consolidation-20260823T202837Z/`.
- Its provenance-enriched all-assistant snapshot contains 10,631 records and
  has SHA-256
  `4ba982515d481aac89ed228062aa6ff93eaba35251d66fdc26d652dc8d8cb3cf`.

The snapshot is local evidence, not a coordination artifact and not a claim
of exact or billing-grade token savings.

## Queued work and priority

GitLab issue #116, **Measure AI token ROI: direct agov CLI versus agov-mcp**,
is the carrier for the remaining work.

Priority order:

1. Do not displace the active MR !1817 fresh audit or issue #115 / MR !1847
   correction cycle.
2. Resolve the existing Gate 1 decisions for MR !806 (issues #86/#88) and
   MR !807 (issues #87/#90); those plans address MCP completeness, protocol
   correctness, discovery, and packaged configuration.
3. After the corrected MCP surface is shipped through the supported operator
   path, plan and run a matched CLI-versus-MCP evaluation using fixed source
   SHA, model, reasoning effort, task, and role. Measure assistant-side token
   totals, MCP schema/envelope overhead, latency, retries, completion quality,
   missed obligations, and invalid-artifact rates.
4. Only if that evaluation exposes evidence gaps should a later Gate 1 plan
   add telemetry schema fields, JSON reporting, a consolidation command,
   latency/exit status, assistant/session/task provenance, or an optional OTel
   bridge.

The existing `docs/plans/otel-cost-telemetry-integration.md` remains pending
Gate 1 and documentation-only in its current scope. It does not authorize the
evaluation or implementation described above.

## Required acknowledgement

Each active AI assistant/runtime must acknowledge this record after confirming
that future direct CLI and MCP-mediated `agov` calls use its own canonical
primary-checkout log. Planner-architect should acknowledge issue #116 as
queued behind the dependencies above; governor-reviewer retains every binding
Gate 1 and prioritization decision.
