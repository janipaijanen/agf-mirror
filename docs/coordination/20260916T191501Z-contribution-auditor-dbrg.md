---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260916T191501Z-contribution-auditor-dbrg
timestamp: 2026-09-16T19:15:01Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260904T104400Z-governor-reviewer-q7mv
  - 20260909T035322Z-planner-architect-6mpr
topic: otgr-semantic-relay-fresh-planning-branch
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Challenge and refine the revised OTGR direction below.
    - Claim and create one fresh planning branch from current main if the
      direction remains sound, then draft a new Gate 1 plan without starting
      implementation.
    - Give the existing OTGR MRs an explicit supersession or retained-input
      disposition so the redesign does not grow a parallel open-MR queue.
  must_read:
    - docs/coordination/20260904T104400Z-governor-reviewer-q7mv.md
    - docs/plans/20260909T035056Z-otgr-relay-service.md
    - docs/proposals/otgr-relay-api-and-comms-spec.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2375
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2587
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2592
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2743
  do_not_touch:
    - .claude/settings.local.json
---

# Handoff: challenge and draft the revised semantic OTGR plan on a fresh branch

The governor's current off-channel direction is: start a separate OTGR
planning branch, collaborate with Claude, and do not implement yet. This
record makes the planning obligation visible; it is not Gate 1 approval and
does not authorize service, client, repository, VM, or deployment creation.

## Why a fresh plan

MR !2587 and companion MR !2592 are useful provenance, but their principal
architecture is now stale: Rust-first static relay, embedded storage, a
broader adopter/enterprise posture, and no semantic-data design. Editing them
in place would hide a material design change. A fresh branch from current
`main` should name them as inputs and state which parts are retained,
superseded, or deferred.

Proposed branch name: `plan/otgr-semantic-relay`. Use a different collision-
free name if another session has already claimed it. Publish one plan MR; do
not create a second proposal MR unless the plan demonstrates that a separate
artifact is necessary. When the replacement plan is visible and covers their
scope, propose closing !2587 and !2592 as superseded rather than leaving both
design generations open. Assess !2375 and !2743 in the same disposition table.

## Revised candidate architecture to challenge

1. **Boundary and purpose.** A standalone open-source repository and service,
   outside AGF and rediflow-elixir. The first supported deployment is the AGF
   development experiment under charter `q7mv`. Git and
   `docs/coordination/` remain the only authoritative governance record; relay
   content is untrusted, non-authoritative conversation. No enterprise support,
   multi-tenancy, enterprise IdP, compliance package, or commercial SLA in the
   initial plan.

2. **Server.** Elixir/OTP, with Bandit + Plug + WebSock/WebSockAdapter for the
   machine protocol, Ecto + PostgreSQL for durable storage, and PubSub plus
   supervised per-discussion processes where evidence shows they help. Package
   as a containerized Mix release. State the operational/runtime cost compared
   with the earlier Rust static-binary design.

3. **Client.** A small Rust client and MCP adapter so assistant hosts do not
   need BEAM installed. Keep the protocol language-neutral. Initial tools stay
   bounded around send, receive, reply, acknowledgment/cursor state, and health;
   no governance authority is inferred from a message or role label.

4. **Semantic data by design.** Candidate canonical interchange is a strict
   operational JSON envelope carrying a compact JSON-LD semantic document.
   Reuse Schema.org for generic people/organization/software/document concepts,
   ActivityStreams 2.0 for communication activities, PROV-O for attribution and
   derivation, and small versioned OTGR/AGF vocabularies for relay and governance
   links. Pin and bundle context documents; never dereference arbitrary remote
   contexts while accepting a message. Treat YAML-LD as an optional human-
   authored fixture/config representation, not the initial wire format.

5. **Validation and storage.** JSON Schema validates the delivery envelope;
   semantic validation/indexing is asynchronous and may use SHACL if the plan
   identifies concrete constraints. PostgreSQL remains the operational source;
   selected identifiers and relationships receive relational indexes, with the
   compact semantic document retained as `jsonb`. Do not make message delivery
   depend on an RDF store, SPARQL service, or semantic reasoner.

6. **Identity and provenance.** Give messages, sessions, participants, rooms,
   Git artifacts, and promoted coordination records stable identifiers. Keep
   `authoredBy`, `transmittedBy`, `actsAs`, `observedBy`, `classifiedBy`, and
   `promotedAs` distinct. A claimed AGF role is display/provenance data and
   never grants authority.

7. **Diagnostics, not an operator product.** Begin with Rust/Elixir CLI health,
   sessions, rooms, cursor lag, delivery/ack state, rate/loop suppression,
   storage status, and controlled transcript inspection/export. A small
   read-mostly LiveView diagnostics page is a later slice after the protocol
   and state model stabilize. Do not build a governance dashboard, gate UI, or
   general-purpose chat application.

8. **Attention model.** Initial delivery should use durable inbox cursors and
   bounded receive/polling. Automatic assistant wake is a later, separately
   evaluated capability because it can create token-consuming feedback loops
   and prompt/control-injection paths. The plan should still state what is
   required to make communication materially faster than Git.

9. **Development and deployment.** Develop in the standalone repository on a
   separate development VM/workspace. A later runtime deployment uses a
   separate minimal VM/container and database. Do not create either environment
   under this planning authorization.

10. **Confidentiality boundary.** Keep the first release private to AGF
    development and exclude enterprise confidential support. Preserve a clean
    path to isolated future deployments, configurable retention, and approved
    model-provider policy, but do not pull OIDC, KMS, DLP, legal hold, or
    multi-tenant administration into the initial delivery.

## AGF implementation boundary

AGF is the first governed consumer, not the service repository. Keep the AGF
side deliberately thin and separately gated after the external protocol is
stable:

- adopter-facing Rust MCP-client configuration and a minimal deployment/use
  example;
- stable OTGR/AGF semantic identifiers for roles, plans, MRs, coordination
  records, learnings, and promotion provenance;
- an explicit helper flow that can prepare a **draft** Git coordination record
  from a selected conversation conclusion, while normal `agov` validation,
  publication, review, and gate authority still apply;
- rule text and negative tests proving relay messages, acknowledgements, and
  claimed roles cannot approve gates or grant authority; and
- optional `agov` discovery/validation support only where a later AGF Gate 1
  plan establishes that it belongs in the framework rather than the external
  client.

Do not place the Elixir server, PostgreSQL persistence, Rust transport client,
or diagnostics implementation under AGF `tooling/`, and do not make ordinary
AGF operation depend on a reachable relay. AGF must remain fully usable and
its governed state reconstructible when OTGR is absent.

## Questions for the planner challenge

- Is compact JSON-LD inside a strict envelope the smallest semantic design that
  preserves normal JSON clients, or should semantics initially be export-only?
- Does PostgreSQL's operational value justify the extra AGF development
  dependency from day one?
- Which existing OTGR protocol clauses remain valid, and which depend on the
  old static-Rust/embedded-store assumptions?
- Can the initial diagnostics surface remain CLI-only without impairing the
  research experiment?
- What exact event causes a conversation conclusion to be deliberately
  promoted into Git without making raw chat a shadow governance channel?

Please answer the challenge in coordination, claim the planning branch, and
draft the Gate 1 plan if the architecture survives. Route contribution audit,
infosec, build/deployment, privacy/research-data, and semantic-data review as
the plan's scope requires. No implementation begins from this handoff.
