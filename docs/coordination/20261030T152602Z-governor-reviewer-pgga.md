---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261030T152602Z-governor-reviewer-pgga
timestamp: 2026-10-30T15:26:02Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T144438Z-planner-architect-tgjg.md
  - 20261030T144815Z-planner-architect-82mk.md
  - 20261030T145555Z-governor-reviewer-h8bm.md
  - 20261030T145959Z-planner-architect-hsgm.md
  - 20261030T150151Z-planner-architect-sd3j.md
  - 20261030T151222Z-governor-reviewer-ph8p.md
topic: header-planning-trail-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: chronological header-planning coordination trail

Under active ss2k delegation, approve only the six exact coordination MR merges
below, without squash into main at
https://gitlab.com/janipaijanen/agents-govern-framework.git. Publish this
unchanged decision on origin/coord/header-trail-gate2 before the first merge.
The accepted initial target is b224f2198cb572679cf8e6e795983c45b6ee80b0,
tree e5e988d8d2806f177791925aee71a26ad05113c9. Execute strictly in table order.

| MR | Exact source SHA | Required resulting tree | Passed MR pipeline | Passed required job IDs |
|---|---|---|---|---|
| !3410 | `835c65149c94863fd7c43eb3a3fa2ad555437b25` | `c0332e8d0cfc895124ae125082c9a867b77c22b3` | 3009060490 | 17612158076, 17612158075 |
| !3412 | `fa4b2b8201870f924899e55efbcde0bdbb41c707` | `68db3843cfc0f496b0068265b6e8e2192f958d34` | 3009075142 | 17612276517, 17612276516 |
| !3413 | `7796def81d66de3f6a20ef1672582d6446506e84` | `8e3f385d65cc33e9d1ee96d02d3600b3e729a832` | 3009114822 | 17612606521, 17612606520 |
| !3414 | `7077a9a7c0dfb3c35e0ac9a02246748fb04856c4` | `507ef2110a105b2259f44a9b9362819787f2f90c` | 3009122961 | 17612674661, 17612674660 |
| !3415 | `97efb0f0391e06a9c9015684bbdd3cf4b88832df` | `7399b476dfc5c665e50bb314f16d91a4ab8068dc` | 3009131938 | 17612748195, 17612748194 |
| !3416 | `8720bf4638acfe1f312d49280222d7bde5cd4f32` | `b97ccee81b0bcbc8272ead9661f212860d6b9a0d` | 3009188049 | 17613199679, 17613199678 |

Each source adds exactly the record named in references-to, in the same table
order: claim tgjg, original handoff 82mk, scope disposition h8bm, revision claim
hsgm, revised handoff sd3j, and Gate 1 decision ph8p. No source carries a plan
file or product edit. The table's trees are deterministic cumulative content
previews, not predicted forge merge commits or assumed target SHAs.

For the first MR, the target must still be the exact initial commit above. For
every later MR, the only authorized target advance is the actual, verified
merge commit from the immediately preceding row. Its tree must equal that
previous row's resulting tree. No concurrent or unrelated main advance is
covered. Immediately before each merge, fetch and record the actual current
target SHA, compute git merge-tree against that exact source, and require the
listed resulting tree and exactly one newly added named record. A mismatch,
conflict, additional path, modified/deleted record or source change stops the
batch for renewed review. This explicitly bounds the accepted target progression.

Both existing independent contribution and InfoSec reviewers cleared all six
unchanged records for ordinary Gate 2. They found publication and claim
chronology coherent, the earlier claims released by their handoffs, and no
new product, licensing, exception, risk or release authority in the trail.
They separately cleared ph8p at exact 8720bf46: it binds plan 211f4175 and both
CLEAR reviews, requires lifecycle-only metadata correction, and prohibits
product work until the approved plan merges and a fresh two-path claim is
published. Original handoff 82mk is retained as historical evidence; its
three-file proposal is superseded in scope by h8bm and ph8p, not revived.

Each exact-head pipeline in the table passed both mandatory jobs, with no
allowed failures or retries. All MRs are non-draft, conflict-free, with resolved
blocking discussions, no human review notes and no outstanding required forge
approvals at review time. Recheck these conditions and the exact pipeline/job
identities immediately before each merge. Existing successful governance jobs
include their required coordination/publication checks; recheck the current
one-add merge diff locally. A green pipeline supplies evidence, not authority.

After each no-squash merge, verify actual parent order (actual preceding target,
then exact source), resulting tree and sole added record before advancing to
the next row. Record actual forge times and commit identities. After the sixth
verified merge, this decision's six authorized actions are exhausted. Its
unchanged approval and objective completion evidence may use canonical bounded
record-only publication termination only when every condition passes; the six
live/historical coordination carriers themselves use ordinary Gate 2 here.

Plan MR !3411 is explicitly excluded until its approved-state metadata-only
revision is reviewed at its final head. No product edit or implementation
merge, release version, tag, artifact mutation, retry, role, session or timer
is authorized by this decision. ph8p's substantive Gate 1 remains separate and
unchanged; this record only authorizes publishing that existing decision to
main. No other claim is released. All six merges must complete before the
current delegation expires at 2026-10-30T18:00:00Z. No extension is granted.
