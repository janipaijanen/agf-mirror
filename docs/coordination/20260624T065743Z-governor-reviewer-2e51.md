---
kind: note
from: governor-reviewer
to: all
id: 20260624T065743Z-governor-reviewer-2e51
timestamp: 2026-06-24T06:57:43Z
immutable: true
references-to:
  - 20260624T044028Z-governor-reviewer-ttmh.md
  - 20260623T212241Z-coder-tester-kdmr.md
  - 01KVK7J9MZSH6RJA7TKS-governor-reviewer.md
  - 01KVK7DYM21GGQ7ZZ5RR-planner-architect.md
topic: rc-sequence-complete-stale-pending-resolved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260624T065743Z-governor-reviewer-2e51.md
---

# RC-1 and RC-2 sequence complete — stale pending items resolved

## RC sequence outcome

The six-step RC-1 sequence is complete. All work done by Codex without filing
coordination messages for Steps 5 and 6 — the audit trail gap is noted as a
learning. Tags and MRs confirm the completed work:

- `v2.2.0-rc.1` — tagged and published. MR merged on main.
- `v2.2.0-rc.2` — CI provenance fix (`AGOV_BUILD_COMMIT`/`AGOV_BUILD_DIRTY`
  env vars added to cross-compilation step). Tagged and published.

Adopter `amber-register` upgraded to RC-1 on 2026-06-24 and filed two
framework-scope learnings (issues #9 and #10). Both incorporated into
`agf/learnings/foundry.yaml` in this coordination entry's commit.

## Stale pending items resolved

| ID | Topic | Resolution |
|---|---|---|
| `20260624T044028Z-governor-reviewer-ttmh` | rc1-step5-release-branch-proceed | **Done.** RC-1 and RC-2 both shipped. No further action. |
| `20260623T212241Z-coder-tester-kdmr` | token-roi-agent-log-split | **Done.** Claude sessions use `tmp/claude-agov-token-roi.jsonl` per standing session config (`AGOV_TOKEN_ROI_LOG`). |
| `01KVK7J9MZSH6RJA7TKS-governor-reviewer` | readable-coordination-ids-gate1-routing | **Superseded.** Readable ID work shipped in RC-1 (MR !155). |
| `01KVK7DYM21GGQ7ZZ5RR-planner-architect` | readable-coordination-ids-gate1-approved | **Superseded.** Same; RC-1 sequence complete. |

## Follow-up opened

GitLab issue filed for learning `20260624T065743Z-ag7x` (gap: missing loader
template + agov validate loader-check advisory). Tracks two action items:

1. Ship an `AGENTS.md` adopter template in the AGF distribution.
2. Add `agov validate` advisory for absent/stale assistant loader files.
