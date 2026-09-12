---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260905T205759Z-planner-architect-gd7t
timestamp: 2026-09-05T20:57:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T205424Z-planner-architect-tr6r.md
  - 20260905T195732Z-coder-tester-83nk.md
topic: dind-runner-live-config-not-tag-isolated
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# The `dind` runner is registered but not tag-isolated — one operator change needed before Gate 1

Answering `tr6r`'s "confirm runner `55975189` config is stable" task with a
read-only GitLab runner API check (2026-09-05T20:54Z):

| Runner | id | `tag_list` | `run_untagged` | online |
|---|---|---|---|---|
| `priv runner` | `55908035` | `['podman', 'linux', 'amd64']` | — | yes |
| `dind` | `55975189` | **`['podman', 'linux', 'amd64', 'dind']`** | **`true`** | yes |

The `dind` runner carries the *ordinary* tag tuple as well as `dind`, and
picks up untagged jobs. So MR !2362's global
`default: tags: [podman, linux, amd64]` matches **both** runners: adding
`tags: [dind]` to the six nested-container jobs forces those onto `55975189`
correctly, but ordinary jobs are still free to race onto it — the exact
shared-vs-dedicated failure MR !2362 exists to remove, reintroduced between
`priv runner` and `dind`.

## One operator change

Reconfigure runner `55975189`: `tag_list: ['dind']` only, `run_untagged:
false`. `priv runner` (`55908035`) keeps `['podman', 'linux', 'amd64']`.
Then `[dind]` is the only way to select the dind runner and ordinary /
untagged jobs cannot land on it. This is a runner-host setting, not a
`.gitlab-ci.yml` change.

## Plan updated

The revised MR !2373 plan (`729925ca`) now carries this as a **Gate 1 /
implementation precondition**, plus a new acceptance criterion that
re-reads both runners' tag lists from the API at implementation time and
fails closed if `55975189` still carries an ordinary tag or `run_untagged:
true`. The re-reviewer requested in `tr6r` should check this precondition is
satisfied (or explicitly waived by you) before recommending Gate 1.

Nothing else in the chain changes. This is the concrete answer to the
runner-config question `tr6r` raised.
