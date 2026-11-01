---
id: forge-publication-evidence-rule
scope: workflow
applies_to: [llm-agent, human-agent]
evidence: 20261030T155116Z-governor-reviewer-ghgs, 20261101T120448Z-governor-reviewer-saps
strength: MUST
status: provisional
see_also: [merge-request-description-rule, external-anonymization-rule, security-hardening-rule]
provenance: foundry-native
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Forge Publications Cite Durable Evidence

> **Provisional status.** Evidence status: Theoretical. One project and one governor
> direction support this rule (2026-10-30). Adopters hitting this rule: please file a
> learning record so we can promote it from provisional to recommended, or narrow it.

**Scope:** The title, description and notes of any merge request or issue on the
project's forge.

## Rule

A merge request or issue title, description or note MUST NOT rely on a live-chat
message as evidence of approval, scope, status, verification or authority. That
includes a message number, link, quote or paraphrase of the conversation. Cite the
durable artifact instead: a coordination record ID or path, a plan file, a commit SHA,
a pipeline or job ID, or another Git or forge item. Chat provenance belongs in the
coordination audit trail as a minimal reference, not a transcript. The reference
SHOULD carry no credential, which `security-hardening-rule.md` already keeps out of
committed text, and no signed or tokenized URL.

**Allowed.** Discuss a chat, messaging or relay system when it is the subject of the
work, for example an issue about an integration defect or a merge request that changes
a relay adapter. Include the minimum sanitized reproducer needed to review that work.

## Privacy

**Existing rules, unchanged.** This rule adds no new privacy prohibition and extends the
scope of no existing one. Secrets and credentials stay out of committed text
(`security-hardening-rule.md`; `systematic-code-review-rule.md`, section 3). Across the
codebase boundary only the project codename may appear, and text an AI assistant sends
to a provider is an external destination (`external-anonymization-rule.md`). Those rules
keep their own scope and strength. Moving sensitive text into an immutable record does
not make it safe.

**Minimization (advisory).** Authors SHOULD record only the minimum decision or evidence
summary needed for audit and SHOULD avoid copying raw conversation text that the audit
does not need.

**Destination.** Public or external forge text and access-controlled internal
coordination are different surfaces. When evidence crosses the codebase boundary,
`external-anonymization-rule.md` applies; a sanitized summary is the usual way to
comply. Text that an AI assistant sends to a provider is an external destination;
"access-controlled" does not cover it. A private internal coordination record MAY keep a
necessary, access-appropriate reference when its governing privacy and access rules
allow it.

## Stricter project direction

A project MAY forbid any mention of its chat surface in forge text. This rule does not
weaken such a direction.

## Limits

This rule adds no automated check, no rewrite of history and no deletion of existing
text. Existing coordination records stay immutable. Whether text relies on chat as
evidence is a human judgment.

## Examples (review evidence, not a validator)

| Case | Result |
|---|---|
| MR description says "approved in chat, message 412" | Forbidden: chat as approval evidence. Cite the decision record ID. |
| Issue says "scope agreed in the team thread" | Forbidden: chat as scope evidence. Cite the plan and its Gate 1 record. |
| MR note quotes a credential or token copied from a chat | Forbidden: chat as evidence. A credential in forge text is also unsafe; this rule adds no separate ban. `security-hardening-rule.md` keeps credentials out of committed text, and `merge-request-description-rule.md` says a merge request description MUST NOT contain secrets, tokens or private credentials. |
| Internal MR note cites a private thread link, with no secret in the URL, as evidence of authority | Forbidden: chat as evidence. The privacy text adds no separate ban on the link. Cite a decision record ID instead. |
| Issue reports that a webhook relay drops events, with a sanitized payload that reproduces it | Allowed: the relay is the subject; the payload is minimal and sanitized. |
| MR changes a chat-integration adapter and describes its behavior | Allowed: chat is the artifact under review. |
| Public issue pastes the project's real name and a private thread link as evidence | Forbidden: chat as evidence, and a non-codename project handle crossing the boundary (`external-anonymization-rule.md`). Use the codename and a sanitized summary. |
| Internal coordination record cites a private design thread by an access-appropriate reference, with no sensitive content copied | Allowed where the project's privacy and access rules allow it. A decision record ID is still preferred as evidence. |
| MR cites a decision record ID and a commit SHA | Allowed and preferred. |

## Why this exists

A live conversation is not a durable, reviewable record: it can be edited, expire,
or sit outside the repository's access and retention controls. Governance evidence must
be reconstructible from the repository and forge alone. Citing a chat message as the
basis of an approval turns a conversation into an unreviewable source of authority.

## Related rules

- `workflow/merge-request-description-rule.md` — what a merge request description must
  contain and where its staging file lives; this rule governs what it may rely on.
- `always/external-anonymization-rule.md` — the codename-only boundary this rule's
  destination paragraph points to.
- `always/security-hardening-rule.md` — where credentials stay out of committed text.
