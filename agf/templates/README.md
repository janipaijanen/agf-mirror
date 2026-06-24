# AGF Starter Templates

These files make the first governed change copyable.

The starter learning record is intentionally usable before the first governed
code change: commissioning AGF is itself useful evidence. It records whether
the assistant prepared the expected files, kept AGF-owned content under
`agf/`, and left clear decisions for the governor.

Do not add fixed example learning IDs. Downstream projects that want IDs should
use `YYYYMMDDThhmmssZ-<4-char>`, where the suffix is random lowercase
alphanumeric text, or equivalent local tooling that cannot collide.

For new learning entries, avoid both shared sequence IDs such as `L018` and
timestamp-only IDs such as `20260623T120000Z`. For updates to existing learning
entries, keep the existing ID stable; if the old entry needs replacement, add a
new suffixed ID and point the old entry to the superseding record instead of
renaming history.

Default adopter paths:

| Template | Copy to |
|---|---|
| `AGENTS.md` | project root `AGENTS.md` (Codex / GitHub Copilot loader) |
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
