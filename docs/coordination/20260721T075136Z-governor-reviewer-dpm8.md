---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T075136Z-governor-reviewer-dpm8
timestamp: 2026-07-21T07:51:36Z
model: human-governor-via-codex-session
immutable: true
references-to:
  - 20260626T055951Z-coder-tester-76jm
  - 20260720T122830Z-planner-architect-rpgr
  - 20260720T222023Z-coder-tester-ksgr
  - 20260721T073926Z-planner-architect-rgg6
topic: v2-2-6-unpublished-disposition-and-release-state-model
requires-ack: false
artifacts_owned:
  - docs/plans/v230-release-preparation.md
---

# v2.2.6 is prepared and merged but unpublished

The human governor accepts the following public-truth disposition and release
process model.

## Binding v2.2.6 disposition

Version 2.2.6 was stable release content prepared and merged on 2026-06-26 at
merge commit `320405f38457f4890ba16fc25f952b4a8c6d1307`, but it was never tagged
or published. It is not a SemVer prerelease and must not be described as a
completed release.

Do not retroactively create `v2.2.6`. The v2.3.0 release preparation must state:

> **2.2.6 was prepared and merged on 2026-06-26 but was not published as a
> tagged release. Its changes are incorporated into 2.3.0.**

The 2.2.6 changelog heading must be annotated as prepared/unpublished, and the
2.3.0 changelog entry must say it incorporates that unpublished content.

## Binding release states

Release work must distinguish these observable states:

```text
prepared → reviewed → merged → tagged → tag pipeline passed
→ assets published → public artifacts verified → completed
```

A merged release MR is authorization to publish, not evidence that publication
occurred. Only the final state is a completed release.

## Post-v2.3 follow-up boundary

After v2.3.0, propose enforcement for a completion record containing tag,
pipeline, GitLab Release, and artifact links; a release-truth check comparing
version, tags, and public releases; Pages wording that does not equate an
untagged content version with publication; and status output distinguishing
prepared, merged-unpublished, and published states. These are not v2.3.0 scope
and must not delay it.
