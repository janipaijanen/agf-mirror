# AGF Starter Templates

These files make the first governed change copyable.

Default adopter paths:

| Template | Copy to |
|---|---|
| `gate1-plan.md` | `docs/plans/sample-login-error-message.md` |
| `coord-handoff.md` | `docs/coordination/COORD-YYYYMMDD-XXX.md` |
| `coord-decision-record.md` | `docs/coordination/COORD-YYYYMMDD-XXX.md` |
| `coord-gate2-handoff.md` | `docs/coordination/COORD-YYYYMMDD-XXX.md` |
| `coord-completion.md` | `docs/coordination/COORD-YYYYMMDD-XXX.md` |
| `learning-record.yaml` | `agf/learnings/<codename>.yaml` |
| `private-codename-mapping.example` | private records outside git |

**Coordination filename convention:** `COORD-YYYYMMDD-XXX.md` where `YYYYMMDD`
is the ISO date and `XXX` is a 3-character random suffix (hex or Crockford
Base32). Each file also carries a `id:` ULID — replace the example values with
freshly generated ULIDs. Both mechanisms are conflict-free under parallel-branch
and multi-agent workflows; do not use sequential integers.
