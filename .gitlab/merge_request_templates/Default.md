<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
Closes # <!-- Replace with the issue number, or delete this line -->

**Framework version:** <!-- Which version are you targeting? Check agf/VERSION -->

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
- [ ] `mix agov.validate.learnings agf/learnings/` passes (if `agf/learnings/` changed; run from `tooling/elixir/`)
- [ ] `mix agov.validate.infosec agf/learnings/` passes or all findings have `sentinel_ack` (if `agf/learnings/` changed; run from `tooling/elixir/`)
- [ ] Changes do not relax governance rules, agent boundaries, or escalation triggers without explicit evidence and human approval
