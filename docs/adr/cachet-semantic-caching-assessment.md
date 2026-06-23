---
id: cachet-semantic-caching-assessment
date: 2026-06-23
status: proposed
deciders: [governor-reviewer]
assessed-by: governor-reviewer
assessment-date: 2026-06-23T12:45:53Z
requires-peer-review: true
references:
  - https://github.com/abhix2112/Cachet
  - docs/coordination/20260623T124553Z-governor-reviewer-cach.md
---

# Assessment: Cachet Semantic LLM Caching Proxy

## Subject

Repository: `abhix2112/Cachet`
Assessed at: 2026-06-23
Commit activity: 3 commits (early-stage; no releases)
Stars / forks: 0 / 0

This is an independent project named Cachet — it is not derived from the
original Cachet status page system.

## What Cachet Is

A transparent, locally-operated LLM caching proxy written in Rust. It
intercepts API calls destined for OpenAI or Anthropic-compatible endpoints and
serves cached responses for repeated or semantically similar queries. The
application runs as a single binary; callers redirect their `base_url` to the
local proxy endpoint (`localhost:8080` by default).

**Two-layer caching strategy:**

1. **Exact match**: SHA hash of the prompt. Instant, zero-cost hit for
   identical requests.
2. **Semantic match**: Lexical word-feature embedder (not neural) using
   cosine similarity with a configurable threshold (default 0.82). Catches
   "rephrasings that share vocabulary" but explicitly cannot detect
   "paraphrases with zero vocabulary overlap" (stated limitation in README).

**Notable constraints (stated by the authors):**

- In-memory cache only — no persistent storage; cache is lost on restart.
- No authentication or rate limiting.
- Single trust boundary — cache is shared across all callers of the same
  instance.
- Cost estimates are character-count approximations, not actual token counts.
- Streaming responses (SSE) are cached only on successful completion.

**Technology:** Rust (79.5% of codebase). Pure-Rust TLS, no system dependencies
beyond libc. Supports Docker deployment (~47 MB distroless image).

## Maturity Level

Using AGF's five-level domain maturity model (adapted for software tools):

| Level | Name | Assessment |
|-------|------|-----------|
| 5 | Deployable | No — incomplete auth, no persistence, no production hardening |
| **3** | **Instantiated** | **Core functionality exists and runs; limited validation, no persistence, no auth** |
| 2 | Schema-defined | Surpassed |
| 1 | Planned | Surpassed |

**Cachet is Level 3 (Instantiated)** as of the assessment date.

## Applicability to AGF

### Use cases where Cachet provides genuine value

| Use case | Benefit | Conditions |
|----------|---------|-----------|
| Development cost reduction | HIGH | Repetitive queries during framework development (code generation, scaffolding) |
| Evaluation harnesses | HIGH | AGF adopters running repeated similar eval prompts in testing |
| Documentation / support bots | HIGH | Non-governance agents answering similar questions from a corpus |
| CI test speed-up | MEDIUM | If agent calls are part of automated tests with repeating patterns |

These are the use cases the Cachet authors explicitly recommend: "Shared Q&A,
docs/support bots, evals, and retries benefit a lot."

### Use cases where Cachet is inappropriate for AGF

| Use case | Risk | Reason |
|----------|------|--------|
| Governance agent responses (Gate 1/2 reviews, plans, coordination) | HIGH | Cached response appears as fresh agent reasoning; audit trail is false |
| Per-session decision-making | HIGH | Agents must reason about current context; semantically similar ≠ same context |
| Multi-agent coordination message responses | HIGH | Two coordination messages sharing vocabulary may have entirely different implications |
| Security review (infosec-sentinel) | HIGH | A cached security finding from a prior context is not a valid finding on the current artifact |

## Conflict with AGF Principles

**Principle 2 (Artifact Ownership):** Coordination artifacts must represent
actual agent reasoning at the time of creation. A semantically cached response
violates this by inserting a prior response as if it were current reasoning.
There is no marker in the artifact distinguishing a cache hit from a fresh
inference.

**Principle 5 (Escalation over Improvisation):** AGF agents are expected to
reason about their specific context and escalate when uncertain. Semantic
caching may suppress an escalation that would have occurred on the current
context, substituting a cached confident response from a similar (but not
identical) prior context.

**Gate quality gates:** A Gate 2 review served from cache is reviewing a prior
artifact, not the current implementation. This would allow a changed
implementation to receive approval based on a cached review of an earlier
version.

**Shared trust boundary:** Cachet's single cache is shared across all callers.
In a multi-agent setup, coder-tester and governor-reviewer share the cache.
A response cached from a coder-tester session could be served to a
governor-reviewer session with a different agent identity and different
governance context.

## Benefit to AGF Downstream Users

AGF adopters who run non-governance agent workflows (documentation agents,
Q&A, support bots) alongside their governance infrastructure could benefit from
Cachet for the non-governance portion of their agent workload.

Downstream adopters should be explicitly warned not to route governance agents
through Cachet. The distinction between governance and non-governance agent
calls is an integration concern that Cachet itself cannot enforce — the
adopter must configure routing deliberately.

## Recommendation (Governor-Reviewer, independent)

**Do not integrate Cachet into AGF's core governance workflow.** The semantic
cache introduces a trust-integrity gap that is silent, unchecked, and
incompatible with the audit-trail requirements of governance.

**Consider documenting Cachet as an example** of a tool with clear
applicability and clear exclusion zones for AGF adopters — specifically:

- Applicable to: non-governance agent workloads (docs bots, eval harnesses,
  development cost reduction).
- Excluded from: governance agent calls (Gate 1/2, coordination responses,
  security reviews, plan generation).

**Wait for maturity.** Cachet's current Level 3 maturity (no persistence, no
auth, no rate limiting, 3 commits) means it is not production-ready even for
the use cases where it is applicable. A re-assessment at Level 4 or 5 would
be appropriate if adoption grows.

## Claims basis

All claims in this assessment are derived from:

1. The repository's README.md as fetched from
   `github.com/abhix2112/Cachet` on 2026-06-23.
2. The repository's observable state (commit count, stars, forks) as
   observed on 2026-06-23.
3. AGF framework principles as documented in `agf/framework.md` and
   `.claude/rules/always/`.

No claims about Cachet's internal behavior are made beyond what the
repository's own documentation states.

## Pending peer review

This assessment is marked `status: proposed`. It requires independent
assessment from planner-architect and coder-tester before a final
`status: accepted` decision can be recorded. See coordination file
`docs/coordination/20260623T124553Z-governor-reviewer-cach.md`.
