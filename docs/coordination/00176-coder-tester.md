---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 176
timestamp: 2026-06-13T12:39:31Z
immutable: true
references-to:
  - 00175-coder-tester.md
topic: v2-public-docs-landing-version-role-expansion-addendum
requires-ack: true
artifacts_owned:
  - agents-govern-2.0/docs/site/index.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Include commit b619e81 in the Gate 2 review for feat/v2-pedagogical-docs-restructure.
    - Confirm the public landing page itself now surfaces version/provenance and role expansion.
  must_read:
    - docs/coordination/00174-coder-tester.md
    - docs/coordination/00175-coder-tester.md
    - agents-govern-2.0/docs/site/index.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Gate 2 addendum: landing page version and role expansion

The governor noted that `public/index.html` itself did not show version
information and did not mention the evidence-led role expansion frame.

Commit `b619e81` updates the landing page source:

- status callout now says this site documents `v2.0.0`;
- the status callout links to `Reference > Release information` for exact
  source commit/provenance;
- the Defined roles bullet now says role expansion is supported, but
  evidence-led: add a role when repeated work, incidents, or ownership
  conflicts show an existing boundary is too broad;
- the landing page closes with a link to version, source commit, copyright,
  and license details.

## Verification

```bash
# /home/devi/src/AGF-NG-v2-pedagogy
git diff --check

# default local output
cd agents-govern-2.0
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f docs/build/mkdocs.yml --strict

# CI/GitLab Pages output
cd /home/devi/src/AGF-NG-v2-pedagogy
mise exec -- uvx --from 'mkdocs>=1.6,<2' mkdocs build -f agents-govern-2.0/docs/build/mkdocs.yml --strict --site-dir ../../../public

rg -n "v2\\.0\\.0|Release information|Role expansion is supported|evidence-led|source commit|software tooling code|licensed MIT" \
  public/index.html agents-govern-2.0/site-html/index.html
```

All commands passed. The regenerated `public/index.html` contains the new
version/provenance link and role-expansion wording.
