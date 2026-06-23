---
id: cachet-semantic-caching-assessment
date: 2026-06-23
status: accepted
deciders: [governor-reviewer, coder-tester]
assessed-by: governor-reviewer
peer-reviewed-by: coder-tester
assessment-date: 2026-06-23T12:45:53Z
joint-synthesis-date: 2026-06-23T12:52:48Z
requires-peer-review: false
references:
  - https://github.com/abhix2112/Cachet
  - docs/coordination/20260623T124553Z-governor-reviewer-cach.md
  - docs/proposals/cachet-applicability-assessment-2026-06-23.md
  - docs/coordination/20260623T125248Z-governor-reviewer-synt.md
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

## Technical Finding: Cache Key Omits Behavior-Changing Parameters

*Added from coder-tester independent assessment (source: `src/proxy.rs` and
`src/cache.rs` at commit `9a3e145`):*

The cache key canonicalizes only: model, messages (role + content), and
stream/non-stream format. The following request parameters are **not** included
in the cache identity:

- temperature
- top-p / top-k
- max tokens
- seed
- tools / tool_choice
- JSON schema response format
- reasoning controls / thinking budget
- provider-specific parameters

This means two requests with identical messages but different temperature,
tool availability, or output schema receive the same cache key. A low-temperature
deterministic request and a high-temperature exploratory request are
indistinguishable. A review prompt with tools available and one without tools
are treated as identical.

**Implication:** even for non-governance use cases (evals, scripted harnesses),
Cachet is only safe when all behavior-changing parameters are fixed and held
constant across the harness run. This narrows the safe applicability further
than the README implies.

## Conflict with AGF Principles

**Principle 2 (Artifact Ownership):** Coordination artifacts must represent
actual agent reasoning at the time of creation. A semantically cached response
violates this by inserting a prior response as if it were current reasoning.
Cachet exposes an `X-Cachet: hit` header, but unless the calling workflow
records this header in the governance artifact, the audit trail cannot
distinguish a cache hit from a fresh inference.

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

**Independent review diversity:** AGF deliberately uses multi-agent challenge
(multiple roles reviewing the same artifact) to create genuinely new evidence.
A semantic cache can collapse independent review into "reuse the first
sufficiently similar review" — defeating the purpose of the challenge workflow.

## Benefit to AGF Downstream Users

AGF adopters who run non-governance agent workflows (documentation agents,
Q&A, support bots) alongside their governance infrastructure could benefit from
Cachet for the non-governance portion of their agent workload.

Downstream adopters should be explicitly warned not to route governance agents
through Cachet. The distinction between governance and non-governance agent
calls is an integration concern that Cachet itself cannot enforce — the
adopter must configure routing deliberately.

## Joint Recommendation (governor-reviewer + coder-tester)

**Do not integrate Cachet into AGF's core governance workflow.** The semantic
cache introduces a trust-integrity gap that is silent, unchecked, and
incompatible with the audit-trail requirements of governance.

**For scripted evaluation harnesses and non-governance workloads**, Cachet is
applicable only under all of the following conditions (from coder-tester
assessment):

1. Exact-only mode (`CACHET_THRESHOLD=1.0`) for any workflow touching
   governance review, merge, release, or paper assessment.
2. All behavior-changing request parameters (temperature, tools, output schema,
   seed) are fixed and constant across the harness run.
3. Cache status (`hit`, `miss`, layer, similarity score) is captured in the
   assessment artifact or coordination message.
4. Cache namespace includes repository commit or docs build hash for
   repository-sensitive prompts.
5. One Cachet instance per trust boundary — never shared across agent roles
   without per-role isolation.
6. Independent challenge prompts are cache-bypass by design.

**Consider a future Gate 1 plan** for a cache-aware LLM-call policy or
capability-provider assessment rule, using Cachet as one concrete case.
The framework-level lesson is broader: semantic caches are capability amplifiers
that can reduce cost but can also falsify freshness, independence, and
authorization assumptions. This class of tool deserves an explicit AGF
governance category.

**Wait for maturity.** Cachet's current Level 3 maturity (no persistence, no
auth, no rate limiting, 3 commits) means it is not production-ready even for
the use cases where it is applicable. A re-assessment at Level 4 or 5 would
be appropriate if adoption grows.

## Open Questions (from joint assessment)

1. Should AGF add an explicit rule that any LLM response cache must disclose
   hit/miss state in governed artifacts?
2. Should independent-review prompts be cache-bypass by default, as a
   governance rule?
3. Is a generic "capability provider registry" the right model for tools like
   Cachet, MCP servers, sub-agents, and external reviewers?
4. Should downstream docs include a short "semantic cache use policy" section?
5. Does the omitted-parameter cache key issue make Cachet unsuitable even for
   AGF scripted experiments until upstream adds full parameter coverage to the
   cache key?

These questions are deferred — no Gate 1 plan is initiated here. They become
relevant when an adopter requests a caching policy or when Cachet reaches
Level 4 maturity.

## Claims basis

Governor-reviewer claims are derived from:

1. The repository's README.md as fetched on 2026-06-23.
2. The repository's observable state (commit count, stars, forks) as
   observed on 2026-06-23.
3. AGF framework principles as documented in `agf/framework.md` and
   `.claude/rules/always/`.

Coder-tester claims (Technical Finding section and conditions above) are
derived from:

1. Source code analysis of `src/proxy.rs` and `src/cache.rs` at commit
   `9a3e1459889e236c75ff94448fea38731dcb8f0a`.
2. `SECURITY.md` trust model and false-positive caveats.
3. `cargo test` — 15 tests passed.

See `docs/proposals/cachet-applicability-assessment-2026-06-23.md` for
coder-tester's full independent assessment.
