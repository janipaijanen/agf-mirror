---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 163
timestamp: 2026-06-13T12:30:53Z
immutable: true
references-to:
  - 00161-coder-tester.md
topic: v2-public-docs-gate2-provenance-addendum
requires-ack: true
artifacts_owned:
  - .gitignore
  - agents-govern-2.0/docs/build/README.md
  - agents-govern-2.0/docs/build/mkdocs.yml
  - agents-govern-2.0/docs/build/hooks/release_info.py
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Include commit 48a5071 in the Gate 2 review for feat/v2-pedagogical-docs-restructure.
    - Confirm the public site now exposes version, source commit, copyright, and license information.
  must_read:
    - docs/coordination/00161-coder-tester.md
    - agents-govern-2.0/docs/build/mkdocs.yml
    - agents-govern-2.0/docs/build/hooks/release_info.py
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Gate 2 addendum: public site release provenance

After seq 161 was filed, the governor noted that public documentation needs
version, commit, copyright, and license information.

The branch head is now `48a5071` on
`feat/v2-pedagogical-docs-restructure`.

## Added

- MkDocs footer now states copyright and the license split:
  framework/documentation content under `CC-BY-SA-4.0`, software tooling under
  MIT.
- `Reference > Release information` is generated at build time by
  `docs/build/hooks/release_info.py`.
- The generated page records:
  - framework content version from `VERSION`;
  - full source commit;
  - short source commit;
  - source branch;
  - build timestamp;
  - copyright;
  - license table.
- The generated source file is gitignored because CI must regenerate it for
  the exact commit being published.

## Verification

```bash
# /home/devi/src/AGF-NG-v2-pedagogy
git diff --check

# /home/devi/src/AGF-NG-v2-pedagogy/agents-govern-2.0
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f docs/build/mkdocs.yml --strict
rg -n "Framework content version|Source commit|Copyright|CC-BY-SA|MIT License|Software tooling code" \
  site-html/reference/release-info/index.html site-html/index.html
```

All commands passed.
