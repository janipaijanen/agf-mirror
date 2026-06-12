---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 88
timestamp: 2026-06-12T21:18:00Z
immutable: true
references-to:
  - 00087-coder-tester.md
topic: dist-foundry-replace-on-upgrade-gate2
requires-ack: true
artifacts_owned:
  - agents-govern/src/agents_govern/dist.py
  - agents-govern/MANIFEST.md
model: claude-sonnet-4-6
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 2 review of feat/dist-foundry-replace-on-upgrade"
    - "Approve or reject for merge to master"
  must_read:
    - agents-govern/src/agents_govern/dist.py
    - agents-govern/MANIFEST.md
  do_not_touch:
    - agents-govern/src/agents_govern/dist.py
    - agents-govern/MANIFEST.md
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 2 Handoff: dist — foundry.yaml replace-on-upgrade

Branch: `feat/dist-foundry-replace-on-upgrade`
Commit: `1f4b17a`

## What changed

**`agents-govern/src/agents_govern/dist.py`** (2 additions):
1. `build_dist()` — copies `learnings/foundry.yaml` into the dist package
   alongside the existing `_template.yaml` and `.adopter-mapping.example`.
   A comment marks it as upstream-owned / replaced on upgrade.
2. Manifest table in the generated MANIFEST.md — new row for `foundry.yaml`
   with "Read-only reference; replaced on upgrade" action.
3. Quickstart block — two-line note explaining foundry.yaml is upstream and
   will be replaced on upgrade.

**`agents-govern/MANIFEST.md`** (same two additions to the static MANIFEST).

## Why

Downstream adopters had no way to distinguish framework-owned learning records
from their own. The convention is now explicit and manifest-declared:
- `learnings/foundry.yaml` — upstream, ships in dist, replaced on upgrade
- `learnings/<codename>.yaml` — adopter-owned, never in dist, never touched
  by upgrade tooling

## Fast-track record

Human governor authorized with "proceed." Criteria met: 2 files, additive
only, no interface change, no security boundary, no data model change.

## Gate 2 checklist

- [x] Diff confined to dist.py and MANIFEST.md
- [x] No other files modified
- [x] foundry.yaml existence guard (`if foundry.exists()`) matches pattern
  of other optional dist files
- [x] MANIFEST.md row and quickstart note consistent with dist.py changes
- [x] Downstream impact: ADDITIVE — new artifact in dist package; no existing
  adopter file touched; adopters who already have foundry.yaml in their copy
  will have it refreshed on next upgrade (intended)
