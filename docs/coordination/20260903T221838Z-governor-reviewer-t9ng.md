---
kind: decision-record
from: governor-reviewer
to: all
id: 20260903T221838Z-governor-reviewer-t9ng
timestamp: 2026-09-03T22:18:38Z
immutable: true
model: human-directed-claude-sonnet-5
references-to:
  - 20260830T214141Z-governor-reviewer-m2pr.md
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260903T221134Z-coder-tester-p9wq.md
topic: v270-release-line-abandoned-including-rc2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# The entire v2.7.0 release line is abandoned, rc.2 included

Binding human decision. Given by the governor-reviewer in the live project
session on 2026-09-04 ("proceed as proposed", accepting the whole-line
recommendation put to the governor); transcribed by claude-sonnet-5 under
direct governor authorization. `m2pr` deferred this question deliberately and
carried it forward to the v2.9.0 roadmap plan as the one open item the governor
still had to answer. This record answers it.

## Decision

The **entire v2.7.0 release line is abandoned**, permanently, for all roles.
This extends `m2pr`'s abandonment of `v2.7.0-rc.1` to the whole line,
explicitly including `refs/heads/release-v2.7.0-rc.2` (`7b214431...`).

No further work — investigation, remediation, pipeline reruns, metadata
preparation, tagging, or publication — is authorized against any v2.7.0
candidate by any role.

## Basis

Verified at decision time:

- No `v2.7.0-rc.2` tag exists; only `v2.7.0-rc.1` is present on the remote.
- No GitLab Release object exists for any v2.7.0 candidate.
- `release-v2.7.0-rc.2` carries no unique product content: the only
  rc.2-specific commit is `7b214431 release: prepare v2.7.0-rc.2 metadata`;
  its remaining delta from `main` is pre-v2.8.0 history already superseded.
- `v2.8.0` is published and accepted. A release numbered 2.7.0 shipped after
  2.8.0 would invert the adopter-visible version order for zero delivered
  content. The active release effort is v2.9.0.

## What abandonment does NOT mean

Lifecycle state, not artifact removal. Consistent with `m2pr`:

- tag `v2.7.0-rc.1` stays exactly where it is (immutable);
- `refs/heads/release-v2.7.0-rc.2`, `refs/heads/release-v2.7.0-rc.1`, and
  `refs/heads/reconstruct/v2.7` are **retained and not to be pruned**;
- the v2.7 evidence refs listed in `gkah`'s `do_not_touch` remain untouched.

Any deletion or move of a v2.7.0 tag or branch needs its own governor decision
and the `.claude/rules/workflow/destructive-op-preflight-rule.md` pre-flight.
Nothing here authorizes a destructive operation.

## Effect

The v2.9.0 roadmap plan's scope-completion is no longer blocked on an
unanswered governor question. Any still-wanted content from the v2.7.0 line
rides v2.9.0 through the normal work order, not a resurrected 2.7.0 candidate.
