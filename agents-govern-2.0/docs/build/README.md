# docs/build — documentation pipeline configuration

This directory holds the build configuration for the v2.0 documentation
outputs. Source content lives in `docs/site/` (HTML site) and — when that
thread is resumed — `docs/paper/` (PDF). Configuration and content are
separated so content authors never touch pipeline files and vice versa.

## Current scope (structural phase)

| Output | Config | Status |
|---|---|---|
| HTML site (GitLab Pages) | `mkdocs.yml` | Active — built by `.gitlab-ci.yml` |
| Figures (Graphviz SVG/PNG/PDF) | `figures.yml` | **Deferred** — returns as its own Gate-1-scoped slice |
| Academic PDF (Pandoc → TeX → latexmk) | `pdf.yml` | **Deferred** — returns as its own Gate-1-scoped slice |

The deferral is a governance decision (coordination seq 107): polish-heavy
figure and PDF work waits until the directory structure and the site
pipeline are stable.

## Building the site locally

```bash
pip install mkdocs==1.6.1          # pinned; match the CI version
mkdocs build -f docs/build/mkdocs.yml --strict
# output: site-html/ at the v2.0 directory root (gitignored)
mkdocs serve -f docs/build/mkdocs.yml   # live preview
```

Strict mode fails the build on any broken internal link. That is the
pipeline's gate; per the gate-self-test rule, its negative case is
demonstrated in the MR that introduces the CI job (a temporary link to a
nonexistent page must fail the build).

## CI wiring

`.gitlab-ci.yml` at the v2.0 directory root defines the jobs. GitLab
executes jobs from the **repository root**, so every path in that file is
prefixed with `agents-govern-2.0/` (the local commands above instead run
from the v2.0 directory). The jobs:

- `validate-governance` — schema validators on agents and learnings.
- `site-html-build` — `mkdocs build --strict`; artifact is the rendered
  site, kept for reviewer download on merge requests.
- `pages` — publishes the built site via GitLab Pages on the default
  branch only.

While v2.0 lives side-by-side with canonical v1.x, this CI file is not
the repository's root pipeline; it is included from the root (or promoted
to root) only when v2.0 is promoted. Nothing in it alters v1.x release
behaviour.
