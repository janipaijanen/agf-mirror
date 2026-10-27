<!-- Radar carrier. Retained verbatim from MR !1718 (source branch docs/learning-dd-operational-debt-backlog, head a5e690e2), file docs/reviews/operational-debt-phase0/20260817T224900Z-snapshot-1/README.md.
     Non-authoritative: parked on the radar, grants no Gate 1, rule, template, validator or implementation authority. -->
# Operational-debt Phase 0 — Snapshot 1

- Captured at: `2026-08-17T22:49:00Z`
- Coordination/source revision: `fcee3b3709bdd82592465cd5b9b123f117f10faf` (`origin/main` as fetched before capture)
- GitLab scope: all 553 repository branches and MR history returned by the project API
- Prior snapshot: none; arrivals, dispositions, and transitions are **not applicable**
- Snapshot 2 earliest capture: `2026-08-18T22:49:00Z`

This is a measurement artifact, not an authorization to acknowledge records, delete branches, close MRs, or repair drift.

## Counts

| Inventory | Count |
|---|---:|
| Pending coordination obligations | 8 |
| Repository branches | 553 |
| Assistant drift findings | 51 |

### Obligation states

| State | Count |
|---|---:|
| `completed-but-unacknowledged` | 7 |
| `human-gated` | 1 |

### Branch states

| State | Count |
|---|---:|
| `active-MR` | 69 |
| `closed/merged-MR` | 404 |
| `orphaned` | 75 |
| `protected` | 5 |

### Drift states

| State | Count |
|---|---:|
| `actionable difference` | 51 |

Validator finding types: {'differ': 44, 'missing': 5, 'extra': 2}.

Oldest obligation: `20260803T073120Z-infosec-sentinel-nqh5` (14.64 days). Oldest branch head: `plan/T3-spec-alignment` (67.14 days since its last commit). Oldest drift item: `.claude/rules/README.md|differ` (68.01 days since the copied path last changed).

## Method

Pending non-handoff obligations came from `agov coord pending --format json` at the named revision, then each was checked against current GitLab state. Branches came from the paginated GitLab branches API; all-MR history supplied the precedence-ordered state assignment. An exact branch-name match in a coordination record containing an explicit retention phrase was required for `explicitly-retained`. Drift identities were reconstructed with the existing validator's SHA-256 tree comparison over `agf/rules` vs `.claude/rules` and `agf/skills` vs `.claude/skills`.

## Unresolved exceptions

- One live human-gated coordination item remains: 20260803T073120Z-infosec-sentinel-nqh5 for MR !967.
- Seven tool-pending records appear complete from GitLab state but lack direct resolving references.
- All 51 assistant-drift findings are actionable differences for measurement purposes; this plan authorizes no remediation.
- Retention classification is based only on exact branch-name matches in coordination records containing an explicit retention phrase; unrecorded intent cannot be inferred.

## Item inventories

- `obligations.csv` — stable key is the immutable coordination `id`.
- `branches.csv` — stable key is the exact Git ref name.
- `assistant-drift.csv` — stable key is copied-tree path plus finding type.
- `summary.json` — machine-readable counts and Snapshot-1 N/A denominators.

Snapshot 2 must join these stable keys to its own item rows. Counts alone must not be subtracted to infer arrivals, dispositions, or transitions.

<!-- Fidelity exception: the two CSV files under docs/proposals/radar/data/ with the 1718-snapshot-1 prefix were normalized from CRLF to LF line endings so the repository whitespace check passes. Nothing else differs from the source blobs; the 554-line branches.csv is not carried. -->
