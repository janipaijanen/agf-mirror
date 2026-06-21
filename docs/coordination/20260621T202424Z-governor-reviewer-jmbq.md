---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T202424Z-governor-reviewer-jmbq
timestamp: 2026-06-21T20:24:24Z
immutable: true
references-to:
  - 20260621T201947Z-coder-tester-qrjr.md
topic: slice-b-and-disclaimer-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T202424Z-governor-reviewer-jmbq.md
decision:
  outcome: approved
  branch: docs/slice-b-and-disclaimer
  mr: "!86"
---

# Gate 2 approval — docs/slice-b-and-disclaimer

Reviewing against PA handoff `20260621T201527Z-planner-architect-7rg6.md`.
Pipeline !86: success. `mkdocs build --strict`: passed (Codex-verified).

## B1 — Nav reorder: Start → Concepts → Adopt → Operate → Advanced

Verified. `agf/docs/build/mkdocs.yml` nav section order is now Start, Concepts,
Adopt, Operate, Advanced. The `docs/index.md` ASCII path and numbered sections
match: 1. Start, 2. Concepts, 3. Adopt, 4. Operate, 5. Advanced, 6. Evolution.

Codex's IA argument was sound: the previous order asked readers to commission
roles and write coordination records before those concepts were explained.
The new order is honest about prerequisites. Correct.

## B2 — No physical move for first-governed-change.md

Verified. The file remains at `docs/start/first-governed-change.md`. The nav
entry under Adopt points to `docs/start/first-governed-change.md` (not a
copy at `docs/adopt/`). No URL churn. Correct.

## B3 — Exit/decommission at end of Adopt

Verified. `exit-and-decommission.md` is the last item under Adopt in both
`mkdocs.yml` and `docs/index.md` (section 3). The "read before adopting"
emphasis is retained on `downstream-impact.md` (still first in Adopt) and
removed from exit/decommission — appropriate, since by the end of the Adopt
path the reader has already committed to evaluating adoption. The contextual
signal is preserved where it matters most. Correct.

## B4 — coordination-protocol.md added to docs/index.md Concepts section

Verified. The Concepts section in `docs/index.md` now lists all three pages
in nav order: roles-and-gates, coordination-protocol, evidence-and-learnings.
One line added, consistent with the surrounding style. Correct.

## Disclaimer — agf/docs/site/reference/disclaimer.md

Five sections in the specified order:

1. **No professional advice** — present. Covers legal, security, compliance,
   privacy, operational-risk. Correct.
2. **Use on authorized systems** — present. Covers AI assistants, validators,
   CI, history-changing procedures, and the requirement for human approval on
   consequential operations. Correct.
3. **Your responsibility** — present. Covers evaluation, AI output review,
   backup, compliance obligations, and the "starting points, not proof" framing.
   Correct.
4. **AI and tooling limits** — present. Covers validators-as-aids, human
   review as the risk-acceptance authority, and the independence/endorsement
   statement (Codex correctly folded the sixth section into this one). Correct.
5. **No warranty** — present. One sentence referencing CC BY-SA 4.0 with a
   link to the legal code; separate note for tooling code. Correct.

Written from scratch — not adapted from `agents-govern/DISCLAIMER.md`. Tone is
practical and confident, not defensive. Marketing-tone-rule compliant.
Documentation-audience-rule: targeted at developer/admin evaluating adoption.

### Nav and links

- `Reference → Disclaimer` added at end of Reference nav in `mkdocs.yml`. Correct.
- README Safety and independence section now links to the published disclaimer
  URL. The absolute URL is appropriate here since README is read from the
  repository root by external users who can reach the published site. Correct.
- `agf/ADOPTING.md` broken link (`docs/governance/disclaimer.md`) fixed to
  `docs/site/reference/disclaimer.md`. Relative path resolves correctly from
  `agf/ADOPTING.md`. Correct.

## Decision

**Approved.** All four Slice B changes implemented as specified. Disclaimer page
matches the PA spec on all five sections. No URL churn, no physical file moves
outside the do-not-touch list, no license or copyright language touched.
Acks Gate 2 handoff `20260621T201947Z-coder-tester-qrjr.md`.
