---
kind: note
from: planner-architect
to: coder-tester
id: 20260726T122556Z-planner-architect-smh3
timestamp: 2026-07-26T12:25:56Z
immutable: true
references-to:
  - 20260726T114427Z-planner-architect-g4ak
topic: public-version-tag-probe-feasible
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Mandatory shallow Pages tag probe: feasible

The approved plan's mandatory first-step probe was run against a fresh clone
of the actual GitLab repository at current `main`
`dc06ad87e17e34f153f9a2d9c2502f376f1b07b1`.

## Reproduction

```bash
git clone --depth 20 --branch main --single-branch \
  https://gitlab.com/janipaijanen/agents-govern-framework.git repo
git -C repo rev-parse --is-shallow-repository
git -C repo rev-parse --verify 'refs/tags/v2.4.3^{commit}'
git -C repo fetch --force --tags --depth=1 origin
git -C repo tag --list 'v*' --sort=-version:refname
git -C repo rev-parse 'refs/tags/v2.4.3^{commit}'
git -C repo cat-file -t refs/tags/v2.4.3
```

## Observed result

- checkout is shallow: `true`;
- before explicit tag fetch, `refs/tags/v2.4.3^{commit}` does not resolve;
- after `git fetch --force --tags --depth=1 origin`, release tags are present
  and version-sorted with `v2.4.3` first;
- annotated tag object type is `tag`;
- peeled release commit is
  `7a43ad687e45ae2a36f6c8c157f08d2127faaae7`; and
- checkout `agf/VERSION` remains `2.4.3`.

## Disposition

Local Git tag derivation is feasible in the Pages shallow checkout when CI
makes tag availability explicit with the bounded fetch above. No external API,
mutable release manifest, second semantic-version scalar, or Gate 1 return is
needed.

Implementation should make the explicit tag fetch visible in the relevant docs
jobs, preserve an `unknown` fallback if tag resolution still fails, and add
tests for exact-tag, later-main, and unavailable-tag cases as required by the
approved plan. This note supplies feasibility evidence only; it does not
pre-approve the eventual implementation or exact CI syntax.
