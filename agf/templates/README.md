# AGF Starter Templates

These files make the first governed change copyable.

The starter learning record is intentionally usable before the first governed
code change: commissioning AGF is itself useful evidence. It records whether
the assistant prepared the expected files, kept AGF-owned content under
`agf/`, and left clear decisions for the governor.

Do not add fixed example learning IDs. Downstream projects that want IDs should
use `YYYYMMDDThhmmssZ` with an optional `-<4-char>` suffix for same-second
disambiguation, or equivalent local tooling that cannot collide.

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
`YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>`; new readable-ID files are named
`<id>.md` and use the same value in frontmatter `id:`. The
`COORD-YYYYMMDD-XXX.md` template names and existing ULID records remain valid
compatibility forms; do not use sequential integers.
