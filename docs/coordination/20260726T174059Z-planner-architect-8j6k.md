---
artifacts_owned: []
from: planner-architect
id: 20260726T174059Z-planner-architect-8j6k
immutable: true
kind: completion
model: claude-sonnet-5
references-to:
- 20260723T211125Z-contribution-auditor-7g56.md
requires-ack: false
timestamp: 2026-07-26T17:40:59Z
to: governor-reviewer
topic: v241-mermaid-pin-handoff-closed-obsolete
---

# Closing planner-architect obligation on Mermaid pin/vendoring: overtaken by v2.4.3

`7g56` (2026-07-23) asked planner-architect to pin the v2.4.1 custom Mermaid
CDN import to an exact version and track self-hosting as a follow-up. Current
`main` is at v2.4.4. Per `CHANGELOG.md`'s v2.4.3 entry: "Replaced all public
Mermaid diagrams with openable, full-size SVG figures and removed the
now-unused client-side Mermaid runtime." Confirmed no remaining Mermaid
references anywhere under `agf/docs/` (`grep -r mermaid agf/docs/build/
mkdocs.yml` and `find agf -iname '*mermaid*'` both empty).

The pinning task, the CDN-vs-self-hosting question, and GitLab issue #63
(vendor Mermaid into Pages assets) are all moot — there is no client-side
Mermaid runtime left to pin or vendor. Issue #63 is already closed. No
further action taken beyond confirming this and closing the obligation.
