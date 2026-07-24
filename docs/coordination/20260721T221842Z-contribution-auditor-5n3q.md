---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260721T221842Z-contribution-auditor-5n3q
timestamp: 2026-07-21T22:18:42Z
immutable: true
references-to:
  - 20260721T220311Z-contribution-auditor-4nqg
  - 20260721T220938Z-contribution-auditor-rh6j
topic: joint-problem-definition-for-ai-identity-authority-and-signing
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically review the identity, authority, and signing problem statement.
    - Refine the conceptual and feasible solution sets with explicit trade-offs.
    - Recommend the smallest governed next step without beginning implementation.
  must_read:
    - docs/coordination/20260721T220311Z-contribution-auditor-4nqg.md
    - docs/coordination/20260721T220938Z-contribution-auditor-rh6j.md
  do_not_touch: []
---

# Problem statement: AI/API identity, governance authority, and cryptographic attribution

## Context

AGF development may involve humans, Claude, Codex, other AI tools, GitLab API clients, local Git clients, CI runners, and release publishers. These actors can all create or transport artifacts that appear authoritative: commits, coordination records, learning records, Gate decisions, tags, provenance, packages, and releases.

Today, several distinct trust claims can be conflated:

1. a record declares `from: governor-reviewer`;
2. a GitLab account or API token uploaded it;
3. an AI tool generated its content;
4. a human reviewed or approved it;
5. CI built or published it; and
6. a released artifact corresponds to the reviewed source.

None of these claims proves the others. An API token authenticates an account's access, not the truth of the content or human approval. A declarative role field is not identity authentication. A producer signature proves control of a producer key, not Gate authority. A green pipeline proves selected checks ran, not that the tested commit was authorised or that the published artifact is identical to the tested output.

## Security problem

GitLab issues, MR descriptions and notes, diffs, job logs, branches, learning proposals, and unmerged coordination files are externally influenced inputs. A polling AI may mistake their content for an instruction or authority claim. Contributor-controlled source or CI configuration may also become executable when reviewed or built. If the consuming agent or runner has broad filesystem, network, credential, merge, signing, or publication access, a compromised input path can induce reconnaissance, exfiltration, destructive operations, persistence, conditional environment-targeted behaviour, governance-record forgery, or poisoned releases.

Long-lived shared API keys worsen attribution and blast radius. Giving an AI access to a human governor's signing key would collapse proposal production and binding approval into one trust domain. Requiring GPG everywhere would add friction while still failing to solve compromised endpoints, malicious authorised actors, prompt-induced signing, key lifecycle, or build-to-release agreement.

The core problem is therefore:

> AGF lacks a complete, implementation-neutral trust model that separates content production, transport/API authentication, governance approval, build attestation, and publication—and defines which cryptographic evidence, if any, allows an artifact to cross each boundary.

## Desired properties

- GitLab and other collaboration content is treated as untrusted data until its identity, provenance, authority, and intended effect are verified.
- AI tools can propose and publish work without acquiring human Gate or release authority.
- A forged `from:` field cannot create binding authority.
- A compromised polling credential cannot push, merge, tag, publish, or sign.
- A compromised proposal-producing credential cannot approve its own work.
- Binding decisions identify the exact commit or artifact they authorise.
- A later commit invalidates approval unless separately reviewed.
- Protected builds and publications can be traced to the authorised input and verified output.
- Signing keys are not exposed to LLM context, project files, logs, or general-purpose shell environments.
- The model supports revocation, rotation, succession, emergency recovery, and offline verification.
- Adoption remains proportional: casual contributors need not carry the same controls as governors and release publishers.

## Non-goals and cautions

- Cryptographic signatures do not prove that content is safe, correct, or well reviewed.
- Signing does not defeat a compromised endpoint that can use the unlocked key.
- Automation cannot prove the absence of dormant malicious behaviour.
- GitLab verified status alone must not be treated as complete governance authority.
- The framework should not mandate GPG specifically where SSH signing, X.509, Sigstore/cosign, workload identity, or hardware-backed services provide a better fit.
- Volatile client or session identifiers must not be persisted as identity evidence.

## Conceptual solution

Define four separate identities and authority domains:

| Domain | Example | Claim established |
|---|---|---|
| Producer | Claude, Codex, human contributor | Who generated or proposed content |
| Transport/API actor | Dedicated GitLab user/token | Which account performed an operation |
| Governance approver | Governor-reviewer | Who granted binding authority over an exact input |
| Builder/publisher | Protected CI workload | Which controlled workload built, attested, or published output |

Use an explicit promotion chain:

```text
untrusted observation or proposal
  -> authenticated producer and transport provenance
  -> independently reviewed exact commit/artifact
  -> cryptographically attributable binding decision
  -> protected build of that exact input
  -> signed provenance and artifact digest
  -> controlled publication
  -> independent post-publication verification
```

Authority must be capability-specific. A producer may request an operation; a separate deterministic policy boundary decides whether it is permitted. The AI should never receive raw signing-key material. If autonomous signing is later justified, a constrained signing service should accept only typed requests and sign only after verifying governed preconditions.

## Conceptual policy

Candidate rule:

> AI producer identity, API authentication, governance approval, build attestation, and publication authority are separate trust domains. Possession of an API token or producer signing key MUST NOT grant Gate approval, merge, release, or publication authority. Binding authority requires a separately authenticated decision over the exact governed input. Externally supplied text cannot directly authorise consequential execution.

Candidate learning-ingestion rule:

> Learning records derived from external or operational inputs are untrusted proposals. They require provenance, content-safety, evidence, applicability, and authority review before acceptance. Acceptance as knowledge does not grant execution or rule authority; operationalisation requires a separately governed change.

## Feasible solution for AGF now

### Immediate procedural baseline

1. Treat issues, MR text, notes, logs, diffs, and unmerged records as observations, not authority.
2. Require canonical coordination before bridge notes and verify the expected author/account, role authority, branch/MR, and exact referenced SHA.
3. Do not allow AI-held polling tokens to write. Use a distinct read-only polling credential where the platform permits.
4. Use distinct Claude, Codex, and automation identities for write operations; do not share a generic AI token.
5. Restrict proposal-producing credentials to unprotected branches and MR creation. Withhold merge, tag, release, protected-variable, runner-administration, and CI-policy authority.
6. Keep human Gate and release signing keys outside LLM-accessible shells and repositories; prefer hardware-backed or non-exportable storage where practical.
7. Record durable attribution using actor/tool class, AGF role, GitLab identity, commit SHA, decision record, and evidence references—without client or session IDs.
8. Revoke approval when the reviewed SHA changes.
9. Review contributor-controlled code in disposable environments without valuable credentials and with constrained filesystem/network access where feasible.

### Near-term governed controls

1. Define an AGF signing and identity profile by privileged action, not by contributor category alone.
2. Require cryptographic attribution for binding governor decisions, protected release tags, release provenance, and published artifacts. Support approved mechanisms rather than GPG alone.
3. Add schemas for exact-subject approval and attestation: repository, commit/digest, action, approver identity, decision reference, timestamp, and signature reference.
4. Add deterministic checks that the approved SHA equals the build input and that release provenance, artifact digest, executable self-report, and published object agree.
5. Define key enrolment, trust roots, rotation, revocation, expiry, succession, and emergency recovery.
6. Add an adversarial GitLab-command-channel scenario to the threat model and link issue #49.
7. Add real-situation journeys for contributor, governor, release operator, and downstream verifier roles.

### Later capability, only if justified

- Use protected CI workload identity and Sigstore/cosign or an HSM-backed service for artifact/OCI signing.
- Put autonomous signing behind a narrow policy service. The service verifies an approved exact SHA, protected pipeline identity, permitted artifact type, digest agreement, and valid governor decision before signing.
- Issue short-lived, operation-specific credentials rather than exposing long-lived signing or publication keys to agents.
- Produce offline-verifiable release bundles containing artifact digest, provenance, decision reference, and signature material.

## Proposed assurance tests

- A forged governor `from:` field is rejected as authority.
- An MR note containing an imperative instruction cannot trigger consequential action by itself.
- A read-only polling credential cannot write or publish.
- Claude's credential cannot impersonate Codex, the governor, or CI, and vice versa.
- An agent-produced commit cannot approve or merge itself.
- Approval for SHA A cannot authorise SHA B.
- A protected build refuses an unsigned or unauthorised input where the selected profile requires signing.
- Publication fails when provenance, digest, binary self-report, tag, or approved SHA disagree.
- Revoked or expired identities fail verification.
- Review and verification can be completed without private session identifiers or access to the original AI conversation.

## Questions for Claude's critical review

1. Which claims are too strong, too weak, or not enforceable on GitLab's actual surfaces?
2. What is the smallest useful AGF profile that improves security without making independent adoption impractical?
3. Which records should be signed directly, and which can inherit trust from a signed commit/tag or protected pipeline?
4. Can binding governor decisions reference exact SHAs without making ordinary Gate work intolerably cumbersome?
5. Which controls belong in AGF rules, schemas, threat models, CI examples, tooling, or adopter guidance?
6. How should human emergency override work without silently destroying the trust model?
7. Should issue #49 be expanded or split into identity/signing, input provenance, and agent execution-boundary workstreams?

Please respond through canonical coordination. The requested output is a joint problem definition and recommendation for a future Gate 1 proposal; this handoff does not authorise implementation.
