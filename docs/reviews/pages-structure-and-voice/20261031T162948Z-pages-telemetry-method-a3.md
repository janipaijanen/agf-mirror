<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# A3 dated assessment and acceptance evidence draft

Assessment date: 31 October 2026 UTC. Evidence status: Empirically informed, operational self-application observations rather than an independently validated savings study. Planning evidence only: no source mutation, strict build, new CI, render, screenshots, A3 consultation clearance, A3 Gate 1/2 or deployment is claimed by this draft. A2 is separately approved by jaa4 (published carrier 18371d0a, approved plan branch 7a2eb2a9); that is not approval of A3.

## Provenance and limits

The originating user asked for Codex/Claude ROI reports, then approved a bounded “What our telemetry measures” method/limitations example. Local Codex snapshot SHA-256: 13fd3bd1f01ac464669a982a6c7c460d93c4021dc731f3dc7aa2480d455b6257. Its source interval is 2026-09-04T04:21:11Z through 2026-10-31T15:19:19Z; it is one assistant-named stream, not a complete authenticated history of this conversation. Claude's recent aggregate is self-reported for 2026-10-30T21:05Z through 2026-10-31T15:27Z; raw logs were not independently inspected. Historical named-log routing differed from recent per-worktree default logs, which may have multiple writers. Transport attribution is self-reported. These intervals cannot support an assistant comparison.

Implementation inspected for this revised draft: `tooling/rust/crates/agov-core/src/roi.rs` at head `2c4ff4319cbb072a756a76f405932977bf483d65`, blob `5d2d804f86769dbe07fe2f1307b67f68f0fd4af5`. The inspected functions are `record_tool_output`, `workflow_baseline`, `saved_tokens`, `summarize` and `sum_field_values`. Built-in workflow estimates are zero-clamped per record by saturating_sub; supplied estimate fields are merely summed as unsigned values. This inspection pins current code, not the producer of every historical event. Per-record producer versions were not captured; the long collection interval may contain mixed or unknown executable versions. No single released-version provenance is asserted for all records. CLI-produced output differs from envelopes, truncated/client-visible text and actual model consumption. Wrapper version probes and large truncated reports were observed. Approximate tokens use a character heuristic, not billing-grade accounting. No actual manual counterfactual, controlled quality comparison, authenticated identity or full API cost/usage record was collected. No numerical benefit, rank or general efficiency claim follows.

Local runtime artifacts remain gitignored under tmp/token-roi-assessment-20261031T152747Z in the Codex checkout: snapshot, report, aggregate summaries and assessment. They are not published fixtures or downloadable proof. This durable summary preserves provenance/hash/limits without raw logs, personal data or a numeric dataset. Code semantics are independently inspectable in the named ROI source. A public raw fixture or measured-savings study would need separately reviewed provenance/disclosure and method.

## Planned acceptance evidence (not yet run)

- Exact before/after clauses for the existing Why-meter prose and old timing/size paragraph; preserve existing headings, anchors, commands, log controls and unrelated sections.
- Review built-in versus supplied-record behavior and client/transport truncation wording; remove causal role/gate cost assertions and invented quantitative examples.
- Validate exact plan/projection and fresh scope/collision/owner coverage, resolving any actual overlap before source edits.
- Record strict build/link/projection/distribution checks, canonical source/target/pipeline/job/artifact identities and independent exact-head source/factual review.
- Independently inspect the added table and nearby command at desktop/mobile light/dark with existing bounded QA; retain the actual evidence and limits.
- Keep original A1 extraction, ranges, limits 982/1080/1174 and answer-key/endpoints; preserve all parent residuals and deployment sequencing. No QA, Gate 2 or parent closure from this planning draft.
