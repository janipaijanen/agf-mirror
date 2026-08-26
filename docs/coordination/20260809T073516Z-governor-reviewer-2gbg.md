---
artifacts_owned: []
from: governor-reviewer
id: 20260809T073516Z-governor-reviewer-2gbg
immutable: true
kind: decision-record
model: human
references-to:
  - 20260809T061139Z-contribution-auditor-qjt5.md
  - 20260809T063108Z-independent-implementation-reviewer-12nr.md
  - 20260809T064214Z-contribution-auditor-7pth.md
  - 20260809T065546Z-coder-tester-smtj.md
  - 20260809T071139Z-contribution-auditor-v4nq.md
requires-ack: false
timestamp: 2026-08-09T07:35:16Z
to: all
topic: approved-corrections-v26-v27-roadmap-sequence
---

# Governor decision: Beagle Gate 2, corrective work, and roadmap sequence

Direct human instruction, transcribed from chat on 2026-08-09. This record
supplies the binding decisions below; it does not pre-approve later fresh heads
whose review is explicitly required.

## 1. Beagle MR !1252 — Gate 2 approved

Gate 2 is **APPROVED** for MR !1252 at exact source head
`fdd4cd1fcb5937c1584eb770d6330120e5eb03df`.

Before recording this decision, the live MR was rechecked as open, non-draft,
targeting `main`, conflict-free, with discussions resolved. Exact-head pipeline
`2744213491` remained successful. Target advances since the approved plan did
not touch `docs/sources.md`, and a synthetic merge against current `main`
succeeded. Contribution-auditor review `qjt5` is COMPLETE/CLEAR and fact-check
`3err` is COMPLETE/CLEAR. MR !1252 may merge while those facts remain true.

## 2. Corrective-forward work — authorized

Corrective-forward work is authorized for the already-merged MR !1265 role
implementation, MR !1267 Pages discovery/IA plan, and MR !1272 learning.

- Accept findings B1-B6 in `12nr`. The
  `independent-implementation-reviewer` role is **provisionally inactive,
  effective immediately**: no work may rely on or commission it until the
  canonical role/status projections are corrected, an independent exact-head
  review returns COMPLETE/CLEAR, and a fresh Governor Gate 2 decision
  reactivates it. The corrective work must project this temporary status into
  `agf/agents/MANIFEST.yaml` and every status-bearing adopter/foundry roster
  surface rather than leaving coordination and the manifest contradictory.
- Accept findings P1-P5 in `7pth`. Planner-architect may correct the merged
  Pages plan in a fresh MR. The corrected exact head requires a fresh
  contribution-auditor review and then a fresh binding Governor Gate 1
  decision. No discovery, design execution, or Pages implementation begins
  before that approval; implementation remains a separate later Gate 1 plan.
- Accept findings L1-L2 in `v4nq`. Coder-tester may correct the learning in a
  fresh MR and repair the authoritative GitLab description. The exact
  corrective head must receive the requested focused review before merge.

## 3. v2.6.0 coordination reliability — already executed; do not duplicate

The human instruction was to execute the two approved, coupled plans for #76
and #84/#85/#89. A fresh current-main audit before dispatch established that
this work is already complete:

- MR !1167 implemented both plans on one coordinated branch at final head
  `efaef6a27e6234181b85bd2af19acc11c709ec52`;
- it received COMPLETE/CLEAR independent review, binding Governor Gate 2, and
  successful exact-head pipeline `2733820720`; and
- it merged as `bcb0fadd9704b782ba02e9672943eae386e8e7e1`.

No relevant Rust, read-path test, or operator-reference implementation surface
has changed since that merge. Therefore **do not reimplement these plans**.
Treat #76/#84/#85/#89 as administrative/release tracking: the capability is
Level 4 Validated on `main`, not Level 5 Deployable until a supported v2.6.0
release artifact and documentation path ships it. Any new behavioral change
requires fresh scope and Gate 1 rather than reusing these exhausted plans.

## 4. v2.7.0 child plans — review, do not implement

Contribution-auditor review is commissioned for:

- MR !806, `agov-mcp` transport completeness/protocol correctness (#86/#88);
- MR !807, assistant discovery/archive setup (#87/#90).

Neither plan is Gate 1-approved by this decision. Address review findings and
return each corrected exact head for a separate binding Governor Gate 1
decision before implementation.

## 5. Release choice — deferred until corrections settle

After the corrective work above settles, reassess whether the next publication
should be final `v2.5.0`, `v2.5.0-rc.3`, or a bounded corrective patch. No
version change, release branch, tag, package, deployment, or final-release plan
is authorized yet.

## 6. Roadmap and empty milestone cleanup — authorized

Refresh `agf/docs/site/roadmap.md` from live evidence: `v2.5.0-rc.2` was
published on 2026-08-04 and is also the version on current `main`. Add the live
corrective, Pages, v2.6.0, v2.7.0, and release-decision lanes without presenting
candidates as approved work.

The empty legacy GitLab milestones `v2.4.1` and `v2.4.2` may be closed as
historical completed-release containers. Do not delete them, reassign issues,
or alter their released tags/history. Roadmap changes require normal exact-head
review and Gate 2; closing the verified-empty milestones is bounded tracker
hygiene under this decision and must be recorded with exact API evidence.
