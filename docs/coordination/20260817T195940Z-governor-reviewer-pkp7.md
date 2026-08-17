---
kind: decision-record
from: governor-reviewer
to: all
id: 20260817T195940Z-governor-reviewer-pkp7
timestamp: 2026-08-17T19:59:40Z
model: human
immutable: true
references-to: []
topic: tooling-license-mit-decision
requires-ack: false
artifacts_owned: []
---

# Governor decision: Rust tooling license is MIT

## Transcription and authority

This is the binding record of the human governor's direct instruction in
chat, 2026-08-17: "see local tmp/dd-20260817-license for question answers
regarding the license question. Tooling is MIT." Given after the governor
read `tmp/dd-20260817-license/` (a 4-model round-3 DD -- ChatGPT, Claude,
Grok, Lumo -- specifically on the licensing contradiction between root
`LICENCE` and `tooling/rust/Cargo.toml`/`Containerfile.agov`/`Cross.toml`).
Claude/planner-architect files this record mechanically; the decision
originates from the human governor-reviewer, who reviewed and approved
this exact draft text before publication.

## Decision

Root `LICENCE`'s existing statement that tooling is MIT is correct and
authoritative. `tooling/rust/Cargo.toml`, `Containerfile.agov`, and
`Cross.toml`'s `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`
declaration was the defect; those files are corrected to MIT, not the
reverse.

## Scope and boundary

This decision authorizes exactly: the metadata correction in
`tooling/rust/Cargo.toml`, `Containerfile.agov`, and `Cross.toml` from
`CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` to `MIT`, per
`docs/plans/20260817T175929Z-tooling-license-mit-alignment.md`.

It does NOT itself approve MR !1712 (the commercial-licensing program --
`LICENSES/LicenseRef-AgentsGovern-Commercial.txt`, the agreement
template, and the public commercial-licensing page). That content was
separately confirmed by the governor in the same chat session
(2026-08-17): Finland governing law, and the 2026-founding-period-free-
through-2030 / 2027-onward-tiered-pricing model. MR !1712 remains
subject to its own review and merge process; this record does not
substitute for that.
