---
date: 2026-06-23T12:48:00Z
author: coder-tester
status: draft-for-joint-assessment
subject: abhix2112/Cachet
upstream_repo: https://github.com/abhix2112/Cachet
upstream_commit: 9a3e1459889e236c75ff94448fea38731dcb8f0a
assessment_type: independent-tool-assessment
---

# Cachet Applicability Assessment

## Summary

Cachet is a small Rust LLM proxy that performs local in-memory response caching
for OpenAI- or Anthropic-compatible HTTP APIs. It forwards requests to the
configured upstream on misses and returns cached responses on exact or lexical
"semantic" hits. Upstream commit assessed:
`9a3e1459889e236c75ff94448fea38731dcb8f0a`.

My assessment: Cachet is promising as an optional downstream optimization for
deterministic-ish repeated workloads, but it should not become a default AGF
development component or transparent reviewer accelerator without additional
guardrails. The governance conflict is real: semantic response caching can
collapse independent review diversity, serve stale answers after repository
state changes, and hide that a model did not actually re-evaluate the current
artifact.

Recommended posture: **observe / optional experiment**, not adopt-as-core.

## Evidence Reviewed

- Upstream repository page and README for product claims, quickstart,
  configuration, and stated limits.
- `SECURITY.md` for trust model, cross-caller sharing, network exposure,
  credential handling, and false-positive caveats.
- Rust source files: `src/proxy.rs`, `src/cache.rs`, `src/embedder.rs`,
  `src/config.rs`, `src/metrics.rs`, `src/dashboard.rs`, and `src/main.rs`.
- Local upstream verification: `cargo test` passed, 15 tests.
- Repository maturity signal at time of review: 3 commits on `main`, tag
  `v0.3.0`, no `.github` workflow files in the cloned repository, GitHub page
  showed 0 stars and 0 forks.

## What Cachet Does Well

Cachet's design has several good properties for a local developer tool:

- It is easy to insert when the calling client supports a configurable
  `base_url`.
- It uses an in-process lexical embedder rather than an external embedding API
  or vector database.
- It handles both non-streaming and streaming chat responses, with separate
  cache entries for the two formats.
- It documents its own fit limits unusually clearly: lexical matching is not
  neural semantic matching, savings are estimates, cache is in memory, and it
  is not a multi-tenant gateway.
- It has a useful security posture for a small tool: loopback default outside
  Docker, rustls upstream verification, no default prompt logging, and no
  request-header storage in cache.

For AGF, the strongest potential benefit is not day-to-day agent development.
It is repeatable, scripted evaluation: examples, docs Q&A probes, regression
prompts, external-review prompt harnesses, or adopter support bots where many
similar questions are expected and freshness is controlled.

## Main Conflicts With AGF Development

### 1. Cache key omits behavior-changing request parameters

The implementation's request analysis canonicalizes only:

- `model`;
- `messages` as `role: content`;
- response format derived from `stream`.

The cache key then uses format, model, and normalized prompt. I did not find
temperature, top-p, max tokens, seed, tools, tool choice, JSON schema response
format, reasoning controls, metadata, or provider-specific parameters included
in the canonical cache identity.

That is acceptable for simple repeated Q&A, but it is unsafe as a transparent
default for governed development. In AGF work, two requests can have the same
messages and model but different tool availability, output schema, randomness,
or reasoning budget. Returning the earlier response would be an incorrect
cache hit.

Risk: **high for AGF reviewer/agent workflows**.

### 2. Semantic cache hits can reduce independent review diversity

AGF deliberately asks multiple agents or external models to challenge plans,
papers, releases, and MRs independently. A semantic cache in front of those
requests can turn "independent review" into "reuse the first sufficiently
similar review." That conflicts with the governance reason for multi-agent
challenge: different models and prompts should create genuinely new evidence.

Cachet exposes `X-Cachet: hit`, but unless the workflow records it, the audit
trail can falsely imply fresh review.

Risk: **high for challenge/review workflows**.

### 3. Repository-sensitive prompts become stale quickly

AGF prompts often embed current branch state, diffs, issue context, coordination
history, and reviewer instructions. A semantically similar prompt after a rebase
or new coordination message may require a different answer. Cachet's TTL helps,
but TTL does not understand git state.

Risk: **medium to high for active development**.

### 4. Trust boundary and dashboard constraints matter

Cachet's own security doc states the cache is shared across all callers of an
instance, not keyed by API key or caller identity. It also states hits are served
without re-authorizing upstream and the dashboard/event feed has no auth.

That is fine for one developer running one local instance for one trust domain.
It is not fine as shared team infrastructure without an authenticated front
door, per-key/per-tenant isolation, and an explicit cache-probing threat model.

Risk: **medium for single-user local use, high for shared deployment**.

## Applicability Matrix

| Context | Fit | Rationale |
|---|---:|---|
| AGF core development through Codex/Claude interactive surfaces | Low | Current agent surfaces generally do not expose a simple OpenAI-compatible `base_url`; transparent caching would also obscure fresh reasoning. |
| AGF scripted prompt regression harnesses | Medium | Useful if prompts are deterministic, cache status is logged, TTL is short, and request parameters are part of the cache key or held constant. |
| AGF external review/challenge workflow | Low | Reusing similar answers undermines the purpose of independent challenge. |
| AGF docs Q&A or public-site evaluation bot | Medium | Repeated questions over stable docs could benefit, but cache invalidation must track docs commit or build hash. |
| Downstream internal support bot | Medium to high | Strong fit when one trust boundary, repeated questions, low temperature, and answer freshness are controlled. |
| Downstream multi-tenant SaaS gateway | Low | Cachet explicitly is not a multi-tenant gateway; shared cache and unauthenticated dashboard are blockers. |
| Downstream evaluation loops and retries | Medium | Repeated deterministic prompts can save cost, but cache hits must be separated from fresh model calls in metrics. |
| Regulated or high-stakes decision support | Low | Semantic false positives and stale cached answers are hard to justify unless cache is exact-only and fully audited. |

## Potential Benefits If Used Carefully

- Lower cost for repeated prompt regression tests and docs/support Q&A.
- Faster local development loops when exercising the same LLM-backed feature.
- A concrete example of "capability provider" governance: a local tool that
  alters model-call behavior and therefore needs explicit authority, scope, and
  audit signals.
- Possible downstream guidance opportunity: AGF could document when semantic
  caches are appropriate, what evidence must be logged, and when they are
  forbidden.

## Conditions For Any AGF Experiment

Before using Cachet in AGF work, I would require:

1. Exact-only mode or threshold `1.0` for any governance review, merge, release,
   or paper-assessment workflow.
2. Cache identity includes all behavior-changing request parameters, or the
   harness proves they are fixed.
3. Cache status (`hit`, `miss`, `bypass`, layer, score) is captured in the
   assessment artifact or coordination message.
4. Cache namespace includes repository commit, branch, or docs build hash for
   repository-sensitive prompts.
5. One Cachet instance per trust boundary, never exposed without external auth.
6. No use for independent challenge unless explicitly marked as "cached
   reference answer" rather than fresh review.

## Recommendation

Do not integrate Cachet into AGF core tooling now.

Do consider a small future Gate 1 plan for a **cache-aware LLM-call policy**
or **capability-provider assessment rule**, using Cachet as one concrete case.
The useful framework-level lesson is broader than this repository: semantic
caches are capability amplifiers that can save cost but can also falsify
freshness, independence, and authorization assumptions.

For downstream users, AGF guidance should say:

- semantic caches are suitable for repeated, low-risk, deterministic-ish
  workloads inside one trust boundary;
- they are unsuitable for multi-tenant use, high-stakes decisions, and
  independent review unless cache hits are disabled or explicitly recorded;
- cache keys must include enough request context to preserve semantics;
- stale-answer risk must be managed with TTL plus domain-specific invalidation.

## Open Questions For Joint Assessment

1. Should AGF add an explicit rule that any LLM response cache must disclose
   hit/miss state in governed artifacts?
2. Should independent-review prompts be cache-bypass by default?
3. Is a generic "capability provider registry" the right place to model tools
   like Cachet, MCP servers, sub-agents, and external reviewers?
4. Should downstream docs include a short "semantic cache use policy" section,
   or is this too niche until adopter evidence appears?
5. Does the omitted-parameter cache key issue make Cachet unsuitable even for
   AGF scripted experiments until upstream changes?

## Source References

- README product claim and base-url drop-in model:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/README.md#L27-L32
- README cache design and limits:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/README.md#L111-L166
- README security summary:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/README.md#L175-L183
- Security trust model, cross-caller sharing, and no upstream reauthorization
  on hits:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/SECURITY.md#L7-L32
- Security network exposure and dashboard caveats:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/SECURITY.md#L34-L44
- Security semantic false-positive and resource-bound caveats:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/SECURITY.md#L65-L75
- Request analysis fields:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/src/proxy.rs#L347-L385
- Cache lookup and semantic scan:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/src/cache.rs#L142-L188
- Cache key construction:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/src/cache.rs#L249-L260
- Configuration defaults:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/src/config.rs#L15-L29
- License and dependency metadata:
  https://github.com/abhix2112/Cachet/blob/9a3e1459889e236c75ff94448fea38731dcb8f0a/Cargo.toml#L1-L44
