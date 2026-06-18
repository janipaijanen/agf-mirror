## Summary

- Amend `release-via-merge-request-rule.md` in both `agf/rules/` (distributed to adopters) and `.claude/rules/` (Claude-facing) to make explicit that steps 7–8 of the release workflow (tag + push) require no additional confirmation — the merged MR is the authorization.

## Why

The current rule described the workflow correctly but was silent on whether tagging and pushing require a separate confirmation gate. This left room for agents to stop and ask before pushing the tag, adding unnecessary friction and diverging from the intended workflow.

## Change

After step 8, adds:

> **Steps 7–8 require no additional confirmation.** The merged MR is the authorization. An agent or contributor MUST tag and push in a single action immediately after merge — not stop to ask permission.

This applies to any human contributor or AI agent following the rule.
