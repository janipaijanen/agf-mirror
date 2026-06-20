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

**Coordination filename convention:** Prefer generating files with
`agov coord brief --id-format readable --from <role>`. Readable IDs use
`YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>` in both the filename and
frontmatter `id:`. The `COORD-YYYYMMDD-XXX.md` template names and existing
ULID records remain valid compatibility forms; do not use sequential integers.
