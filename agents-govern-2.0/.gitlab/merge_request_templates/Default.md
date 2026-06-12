<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
Closes # <!-- Replace with the issue number, or delete this line -->

**Framework version:** <!-- Which version are you targeting? Check VERSION file or docs/governance/manifest.md -->

## Summary

<!-- What does this MR do and why? -->

## Evidence

<!-- Which learning(s) or issue(s) motivate this change? -->
<!-- Format: codename, category, severity -->

## Changes

<!-- List the key changes, one per line -->

-

## Checklist

- [ ] No real project, product, or organisation names (anonymization policy)
- [ ] Commit message cites the learning(s) that triggered the change
- [ ] `<!-- CUSTOMIZE -->` markers added where adopters need to adapt
- [ ] `uv run agov-validate-learnings` passes (if learnings/ changed)
- [ ] `uv run agov-validate-infosec` passes or all findings have `sentinel_ack` (if learnings/, docs/governance/retrospective.md, or reviews/ changed)
- [ ] `uv run agov-dist` builds successfully (if dist contents changed)
- [ ] Changes do not relax governance rules, agent boundaries, or escalation triggers without explicit evidence and human approval
