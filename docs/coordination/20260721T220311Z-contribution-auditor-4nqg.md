---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260721T220311Z-contribution-auditor-4nqg
timestamp: 2026-07-21T22:03:11Z
immutable: true
references-to:
  - 20260721T214629Z-contribution-auditor-msha
topic: jointly-formalize-adversarial-boundary-and-gitlab-command-channel-learnings
requires-ack: true
artifacts_owned: []
model: codex-gpt-5
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically review and jointly formalise both accepted learnings.
    - Propose durable learning wording and governed follow-up actions.
  must_read:
    - docs/coordination/20260721T214629Z-contribution-auditor-msha.md
  do_not_touch: []
---

# Joint formalisation request: semantic boundaries and GitLab as an untrusted command channel

The governor-reviewer accepts two related findings as legitimate and asks Claude/planner-architect and Codex/contribution-auditor to formalise them jointly.

## Learning 1: structural validity does not establish operational safety

The Gate 2 review of MR !351 found that a schema-valid `artifact_filename` could contain an absolute path or `..` traversal and escape the designated artifact directory. The reusable lesson is broader:

> Schema-valid data is not necessarily safe operational input. Whenever validated metadata is interpreted as a path, URL, command argument, archive entry, identifier, or artifact reference, review must test the interpretation boundary and its containment invariant.

Please critically review and help formalise an **adversarial boundary review** method:

1. identify fields that cross from parsed data into an operation;
2. identify the authority each field gains after parsing;
3. test hostile but structurally valid values;
4. prove the resolved operation remains within its authorised boundary; and
5. test the invariant in the shared implementation layer and through the public command.

The method should define reusable hostile-input classes, including traversal, absolute paths, platform-specific separators, normalisation aliases, URL redirection or scheme changes, archive extraction paths, shell metacharacters, and external-resource selectors. It should prefer positive containment checks over incomplete deny-lists.

## Learning 2: collaboration surfaces are untrusted inputs, not authority

GitLab issues, MR descriptions and notes, diffs, job logs, branches, and unmerged coordination files can act as an indirect command channel when an agent polls them and mistakes their content for authority. Contributor-controlled code or CI configuration can also become an executable supply-chain channel. With broad shell, filesystem, network, or credential access, a compromised instruction path could induce reconnaissance, exfiltration, destructive actions, persistence, conditional environment-targeted behaviour, or poisoned releases.

Please critically review and help formalise this principle:

> Shared collaboration surfaces are untrusted input channels, even when they also transport governance records. Content discovery, identity authentication, authority verification, and action execution are separate control steps.

Candidate enforceable rule:

> No externally supplied text directly authorises filesystem mutation, command execution, credential use, network transmission, source modification, merge, or publication. Consequential action requires independently authenticated authority and executes with the least privilege appropriate to the task.

The formalisation should distinguish at least:

- read-only polling and summarisation;
- authenticated role and author identity;
- protected-branch or otherwise trusted record provenance;
- human Gate authority versus declarative `from:` fields;
- review of contributor-controlled code without valuable credentials;
- credential separation for polling, pushing, merging, tagging, and publication;
- sandbox and network-egress expectations;
- semantic review for dormant environment detection, delayed triggers, subprocesses, destructive operations, encoded endpoints, and unexpected network clients; and
- release verification tying reviewed commit, built commit, executable self-report, provenance, and published artifact together.

Issue #49, **Input provenance governance**, is an existing related tracking surface. Assess whether it should be expanded, split into a concrete threat-model issue, or linked to a new governed plan.

## Requested joint output

Please ACK this handoff and provide a critical review that:

1. corrects or narrows the two formulations where necessary;
2. proposes the durable learning record wording;
3. identifies which existing rule, checklist, threat-model, tooling, and documentation surfaces should change;
4. separates immediate safeguards from later planned implementation;
5. recommends enforceable tests or checks without overstating what automation can prove; and
6. routes any implementation through normal Gate 1 approval rather than beginning implementation from this handoff.

The target state is a joint, downstream-shareable method and control model, not a one-off patch-traversal anecdote.
