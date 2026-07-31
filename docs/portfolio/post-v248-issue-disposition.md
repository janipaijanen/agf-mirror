# Post-v2.4.8 issue disposition ledger

Observed: 2026-07-31. Baseline: v2.4.8. Authority:
`docs/plans/post-v248-portfolio-and-roadmap-reconciliation.md`.

“Candidate” means routed for governed planning; it is not a shipped-capability
claim. Items remain open until the closure condition is evidenced.

| Issue | Destination | Verified consideration and closure condition |
|---|---|---|
| #17 | T0 evidence intake | Preserve the external fit assessment in the evidence archive; close after a canonical repository artifact and issue note link each other. |
| #18 | T0 learning intake | Reconcile the OCI learning with the shipped v2.4.8 OCI path; close after canonical learning acceptance or explicit supersession. |
| #19 | T0 learning intake | Reconcile validator self-sufficiency learning with current Rust tooling; close after canonical learning acceptance or a named remaining gap. |
| #31 | Post-v2.5 backlog | Issue-template ergonomics; revisit after issue #75 has operated for one full release cycle. |
| #32 | Post-v2.5 backlog | Write-path commands remain parked; revisit after #80 defines obligation semantics. |
| #33 | v2.5.0-rc.1 dependency | Only the minimum concurrency contract required by #80; residual concurrency work returns to backlog. |
| #34 | Post-v2.5 trust-chain | Revisit when a non-circular approval-binding design exists. |
| #35 | Post-v2.5 trust-chain | Revisit when an authentication plan exists; until then identity remains asserted, not authenticated. |
| #36 | Post-v2.5 backlog | Revisit after #76 and #80 establish correct incremental semantics. |
| #37 | v2.7.0 surface reconciliation | `agov-mcp` exists, but the issue's full no-shell handoff-cycle/parity acceptance is not assumed; close only after exact released-surface comparison or split residual gaps. |
| #38 | Post-v2.5 research | Run only under an approved controlled experiment protocol; do not market as causal evidence. |
| #39 | Post-v2.5 backlog | Revisit at the governed removal-drill milestone. |
| #40 | Post-v2.5 backlog | Revisit on confirmed adopter demand for a named compliance export. |
| #41 | Post-v2.5 backlog | Revisit on confirmed public-buyer demand and a bounded profile. |
| #42 | Post-v2.5 trust-chain | Revisit after a primary-source-reviewed threat-model plan. |
| #43 | Post-v2.5 backlog | Revisit after enough amendment/sunset cases exist to design from evidence. |
| #44 | Legal/entity track | Not an engineering release item; requires legal/entity-formation review. |
| #45 | Post-v2.5 trust-chain | Revisit after accountability terminology, authority, and evidence gaps are resolved. |
| #46 | Post-v2.5 backlog | Revisit after a real incident supplies a bounded workflow case. |
| #47 | Post-v2.5 trust-chain | Revisit after independence axes and evidence claims are source-defined. |
| #48 | Post-v2.5 backlog | Revisit with a concrete safety case and enforceable budget semantics. |
| #49 | Post-v2.5 trust-chain | Revisit after input identity and runtime-attestation boundaries are defined. |
| #53 | T0 measurement | Preserve the completed measurements; keep open only for a named uncompleted causal-measurement slice, otherwise close with the merged assessment. |
| #57 | v2.5.0 portfolio parent | Close when every child is linked to release scope, triggered backlog, radar, or rejection and the SoW is on `main`. |
| #59 | Radar | No release commitment; retain with its existing review trigger. |
| #60 | Radar | No release commitment; retain with its existing review trigger. |
| #61 | Radar | No release commitment; retain with its existing review trigger. |
| #62 | Radar | No release commitment; retain with its existing review trigger. |
| #67 | v2.5.0-rc.1 verified remainder | Reproduce each of the five findings on v2.4.8; close fixed slices and route only verified contract/advisory-role remainder. |
| #75 | T0 workflow | Gate 1 plan !745 remains pending; implement only after approval, then close after one coherent board/label workflow is live. |
| #76 | v2.6.0 reliability slice | Include only correctness-preserving work required by the coordination reliability release; residual performance work parks later. |
| #77 | Post-v2.5 intake | Revisit after v2.5.0 or a real cross-repository intake event; use manual recorded intake meanwhile. |
| #78 | v2.5.0-rc.1 verified remainder | Reproduce every v2.4.5 screening finding on v2.4.8; route only surviving contract/documentation gaps. |
| #79 | v2.5.0-rc.1 core | Release-blocking minor contract: upgrades must preserve adopter role authority. |
| #80 | v2.5.0-rc.1 core | Release-blocking minor contract: `requires-ack` must create consistent queue/graph/session/CLI/MCP obligations. |
| #81 | Close as fixed in v2.4.8 | v2.4.8 ships the three fixtures and clean-archive tests passed for tar/zip; close with release pipeline and completion-record evidence. |
| #82 | T0 learning intake | Preserve as canonical learning/evaluation guidance; close after accepted artifact is linked, without claiming independent corroboration. |
| #83 | Radar | Foundation-model information watch; no release commitment. |
| #84 | v2.6.0 candidate | Child plan !805; fix only after Gate 1 and exact compact-timestamp tests. |
| #85 | v2.6.0 candidate | Child plan !805; align visibility collision handling with session-resume warnings. |
| #86 | v2.7.0 candidate | Child plan !806; MCP parity scope must remain CLI-subordinate. |
| #87 | v2.7.0 candidate | Child plan !807; verify loader discovery from clean archive/checkouts. |
| #88 | v2.7.0 candidate | Child plan !806; enforce JSON-RPC version/protocol correctness. |
| #89 | v2.6.0 candidate | Child plan !805; include fetched remote-tracking branches without correctness loss. |
| #90 | v2.7.0 candidate | Child plan !807; package and test a ready-to-copy Claude MCP template. |
| #91 | Later minor candidate | Bound to observable source-omission controls; do not claim omission elimination without evidence. |
| #92 | v2.7.0 candidate | Reproduced wrapper/Rust registry drift; require one authoritative command inventory and parity tests. |
| #93 | v2.4.9 patch | **Still reproduces on post-v2.4.8 `main`:** `bin/agf` exists and MR templates invoke Mix. Plan a bounded patch-compatible live-surface cleanup. |
| #94 | Later minor candidate | Post-recovery release-truth/compatibility work; child-plan and compatibility classification required. |

## Completeness check

The table contains 49 distinct issue IIDs, matching the live open-issue set at
the snapshot. A final API query is required before Gate 2 because the tracker
may change during execution.
