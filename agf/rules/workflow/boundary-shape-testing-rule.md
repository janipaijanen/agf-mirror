---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Boundary Shape Testing: Test External Data Parsing With Real Data Shapes
#
# Framework principle: Verification over Assumption, simplest-correct-thing
# See: framework.md — Pattern 4 (Tester Discipline)

> **Provisional status.** Single-adopter evidence (silver-birch,
> 2026-04-18, framework v0.33.0 — issues #25 / #37). The pattern
> is universal across languages and frameworks (any boundary where
> structured external data is parsed), but only one adopter has
> reported it through the framework's evidence channel. Adopters
> hitting this rule: please file a learning record so we can promote
> from provisional to recommended.

**Scope:** Any code that parses structured data crossing a boundary
the project does not control: external API responses, library
callbacks, OAuth/OIDC token exchanges, webhook payloads, RPC
responses, JSON-decoded message-queue messages.

## The rule

Whenever code reaches into a structured value coming from outside
the project (an HTTP response body, a library callback argument, a
deserialised JSON payload), the test that exercises the parsing
MUST use the **real shape** the boundary actually emits — including
key types (atom vs. string), nested structure, and the exact
field names of the upstream contract.

Tests that use simplified or invented shapes pass while production
code fails on the real shape. The cost of inventing a fixture
("`%{user: %{groups: ["editor"]}}`" vs the real "`%{
  "user" => %{"groups" => ["editor"]}, "id_token_claims" => ...
}`") is exactly the bug the test was supposed to catch.

## What the test must include

1. **A positive case using the real shape.** The shape must be
   verified against documentation *and* against a recorded real
   response (recorded once, committed to fixtures, refreshed when
   the upstream contract changes).

2. **A negative case using the wrong shape.** A common refactor
   regression replaces atom keys with string keys (or vice-versa)
   based on vague memory of a different provider. The negative test
   catches this:

       # Right: atom key — returns groups
       assert Groups.extract(Map.get(response, :user, %{})) == ["editor"]

       # Wrong: string "userinfo" — returns empty
       assert Groups.extract(Map.get(response, "userinfo", %{})) == []

   <!-- CUSTOMIZE: replace with a real shape from your stack
        (e.g., axios response, requests.Response.json(),
         Pydantic model, jackson-deserialized struct) -->

   The negative test is **critical**: a positive test alone passes
   whether the shape is correct or wrong. Only the negative test
   catches the silent bug.

3. **One canonical parser per boundary.** Duplicating boundary
   parsing across multiple call sites guarantees drift — one copy
   uses string keys, another uses atom keys, both compile, only one
   actually works. Extract one canonical parser module (e.g.,
   `Auth.Groups`, `Webhook.Stripe`, `Provider.GitLab`) and test it
   in isolation.

## Applies to

- OAuth / OIDC token exchange responses.
- HTTP `/userinfo` and similar identity-provider endpoints.
- Webhook payloads (Stripe, GitHub, GitLab, Slack, etc.).
- RPC responses (gRPC, JSON-RPC, AMQP messages).
- Library callbacks with structured return values.
- Any new integration with an external service or library.

## When NOT needed

- Internal function-to-function calls within the same module.
- Schema-validated data on ingress (the schema is the contract;
  the parser uses the schema's shape by construction).
- Framework-set context values (HTTP framework's request object,
  background-job context — set by your own code path).

## Why this exists

Type checkers and simplified fixtures both fail silently for this
class of bug:

- A statically-typed wrong-shape access often *compiles* (e.g., a
  language with optional / dynamic field access; or
  `Map.get(map, :wrong_key)` returning `nil` without warning).
- Simplified fixtures don't expose the real shape — they reflect
  what the developer *thought* the shape was when the test was
  written.
- Code review usually accepts the wrong shape as plausible — both
  shapes are syntactically valid and the reviewer rarely runs the
  real boundary call.

The only reliable guard is a unit test using the actual external
data shape, with a negative case that exercises the failure mode
the future regression will introduce.

## CUSTOMIZE

Adopters typically need not customize this rule — the pattern
applies uniformly across languages and frameworks. Adopters MAY:

- Add stack-specific examples (FastAPI Pydantic model parsing,
  axios response objects, Go struct tags, Rust serde deserialisation).
- Document the project's recorded-fixture refresh cadence (e.g.,
  "boundary fixtures are refreshed quarterly, on dependency major-
  version upgrade, or after any production parse failure").

## Related rules

- `workflow/tdd-rule.md` — TDD discipline applies here too: the
  negative test is written first; the parser is implemented to
  satisfy both positive and negative.
- `workflow/systematic-code-review-rule.md` — Gate 2 checklist
  should ask "if this change parses external data, does the test
  use the real shape?".
