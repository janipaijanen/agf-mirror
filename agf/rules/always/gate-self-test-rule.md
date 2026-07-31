---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Gate Self-Test on Introduction
#
# Framework principle: Process discipline + environment enforcement = governance.
# Process discipline alone is at-will and breaks under speed.
# See: docs/position/index.md (Process vs enforcement)

When introducing a new gate — git hook, validator, CI check, pre-commit script, MR-blocking rule, anything that's supposed to *prevent* a known failure mode — the same MR MUST include a deliberate negative test demonstrating that the gate fires on a known-bad input.

## Why this rule exists

A gate that ships broken is worse than no gate at all. Adopters trust that the gate gates; if it silently doesn't, they relax the discipline the gate replaces. Then the failure mode the gate was supposed to prevent recurs while everyone assumes it cannot.

Three documented incidents in this framework's own history shipped a gate broken:

- **v0.25.0 release** (2026-04-12, [`learnings/iron-gate.yaml`](../../learnings/iron-gate.yaml)): release skill defined version-tag alignment as process discipline. Skill ran by humans; humans forgot. Tag pushed before VERSION matched. Recovery: three patch releases.
- **v0.38.0 release** (2026-04-27): `agov-pre-tag-check` existed (added in v0.25.5 specifically because of the v0.25.0 incident). It worked correctly when invoked. It was never invoked at the moment that mattered because raw `git tag` bypassed it. The gate existed for 21 days without being verified-to-fire on the failure mode it was designed for. Recovery: one immutable broken tag.
- **v0.41.0 release** (2026-04-27): pre-push hook designed to gate `agov-pre-tag-check` shipped non-executable (`mode 100644`). Git silently skipped it on every push. The gate was visibly committed; nobody verified it could actually run. Recovery: three patch releases.

All three are the same shape: gate ships, nobody negative-tests it, the gate doesn't gate, the failure mode recurs.

## The rule

When this MR introduces a new gate, the MR MUST also include:

1. **A negative test** — an explicit demonstration that the gate fires on a known-bad input. The test produces visible failure (error message, exit code, blocking output) and is reproducible.
2. **Documentation of the negative test** in the MR description. State what bad input was used, what the gate did with it, what the expected output was, what the actual output was.
3. **Verification the test ran** — pipeline log, command transcript, or screenshot. "I tested locally" is insufficient; the artifact must be visible to a reviewer.

The rule fires at MR-introduction time, not at adoption time. The gate either gates on day one with proof, or it doesn't ship.

## Concrete examples

### Hook script

If the MR adds `.git-hooks/pre-push`:

```bash
# Negative test: feed the hook a wrong-named tag and confirm exit 1
$ echo "refs/tags/v9.99.99 abc refs/tags/v9.99.99 def" | .git-hooks/pre-push
[gate output: VERSION 0.41.5 != tag v9.99.99]
[gate output: Push aborted]
$ echo $?
1
```

### Validator (`agov validate ...`)

If the MR adds a new validator:

```bash
# Negative test: construct a deliberately-bad input file
$ cat > /tmp/bad-input.yaml <<EOF
[malformed content per the validator's contract]
EOF
$ agov validate newthing /tmp/bad-input.yaml
[validator output: errors listed]
$ echo $?
1
```

### CI check / job rule

If the MR adds a new CI gate:

```yaml
# In the MR's pipeline log, the new job appears AND fails on the negative input
new-gate:
  status: failed
  log: "[expected gate output here]"
```

### Pre-commit hook

```bash
# Negative test: stage a known-bad change, attempt commit
$ git add path-with-known-bad-content
$ git commit -m "test: negative case for new pre-commit gate"
[hook output: blocked]
$ echo $?  # commit was refused
1
```

## What the rule explicitly forbids

- **"I tested it locally" without artifact** — the failure must be reproducible from the MR description. A reviewer who clones the branch should be able to re-run the negative test.
- **A passing-only test** — "I tested it on a good input and it passed" is not a negative test. The gate's *job* is to block bad inputs; that's what must be demonstrated.
- **Deferring the negative test to a follow-up MR** — gates ship with proof or don't ship. The framework's release pattern produced three "ship-broken-then-patch" incidents (v0.25.x, v0.38.0, v0.41.x); this rule exists to break that pattern.

## CUSTOMIZE

Adopters MAY:

- Adapt the example commands to their tooling (different shell, different test runner, different CI vendor).
- Add their own categories of gate (deployment gates, schema-migration gates, secrets-scan gates, etc.) to the example list.
- Require additional artifacts (e.g., a pull-request comment from a CI bot, a CI badge) in the MR description.

Adopters MUST NOT:

- Skip the rule for "small" gates. Small gates fail the same way (v0.41.0's hook script was 12 lines).

## Enforcement

Per-MR: the contributor includes the negative test in the MR description; the reviewer verifies the test transcript before approving.

Per-milestone: framework operators audit recent gate-introduction MRs; any that landed without a negative test are flagged and the gate is re-verified post-hoc.

Tooling (recommended, future): a CI check that scans MR descriptions for a `## Negative test` heading or similar marker, blocking merge if absent on MRs that touch known gate-introduction paths (`.git-hooks/`, Rust validator/CLI sources under `tooling/rust/crates/`, `.gitlab-ci.yml` rules blocks). Out of scope for the rule's introduction; track as a follow-up issue if adopted.

## Why MUST on three-source evidence

The three incidents (v0.25.0, v0.38.0, v0.41.0) all shipped a gate that didn't gate, and all three required release-tag patches to recover. Three-source corroboration on the parent thread (gates-ship-broken-without-self-test) clears the framework's symmetric evidence threshold for MUST status. The rule formalizes a discipline already documented in maintainer-private memory (`feedback_version_before_tag.md`, the five disciplines), translating it into rule-template form so adopters get it too.

## Related rules

- [`workflow/branch-and-worktree-rule.md`](../workflow/branch-and-worktree-rule.md) — branch and worktree discipline; the AI-assisted-branch-state subsection (v0.44.0) is itself a gate-discipline that should be self-tested by anyone implementing it.
- [`always/external-anonymization-rule.md`](external-anonymization-rule.md) — the codename-only-at-the-codebase-boundary rule's tooling follow-up (historical predecessor-tracker issue 46) is exactly the kind of gate this rule applies to: when the forbidden-term scanner ships, it ships with a negative test demonstrating it blocks a leak attempt.
