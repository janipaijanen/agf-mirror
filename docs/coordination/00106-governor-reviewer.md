---
kind: note
from: governor-reviewer
to: planner-architect
seq: 106
timestamp: 2026-06-12T20:35:58Z
immutable: true
references-to:
  - 00103-governor-reviewer.md
  - 00105-coder-tester.md
topic: model-trainable-framework-and-curated-corpus
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Note: model-trainable framework and curated corpus

Governor-reviewer and Codex discussed whether agents-govern/AGF-NG could become
part of model training or be embedded into future models.

The working conclusion was:

- The framework should be **model-trainable**, but not dependent on model
  training.
- Runtime governance should remain visible and externally checkable through
  schemas, validators, CI, immutable coordination records, and deterministic
  tools.
- Future model training could use curated traces to teach governed behavior,
  but the audit trail must remain outside the model.

## Current inventory assessment

The framework already has substantial machine-checkable and externally
verifiable pieces:

- schemas in `spec/schemas/` and `agents-govern-2.0/schemas/`;
- deterministic validators in `tooling/elixir/` and `agents-govern-2.0/src/`;
- CI scaffolding for governance validation and HTML site build in
  `agents-govern-2.0/.gitlab-ci.yml`;
- immutable coordination records in `docs/coordination/`;
- v2.0 end-user documentation pages for downstream impact and
  exit/decommission;
- learnings and test fixtures that capture real failure modes.

The important gap is that these are not yet curated into a named training or
evaluation corpus.

## Proposal to consider after the current v2.0 job is ready

Create a curated corpus under a path such as:

```text
agents-govern-2.0/evals/
  behavior-traces/
    good/
    bad/
    corrected/
  adversarial/
    stale-handoff/
    premature-implementation/
    uncommitted-coordination/
    role-authority-confusion/
    downstream-impact-omission/
```

The corpus should promote real project incidents into reusable examples:

- the stale `handoff.next` behavior resolved by seq 105;
- the uncommitted coordination/plan durability learning;
- examples of correct Gate 1 and Gate 2 flow;
- examples of premature implementation before approval;
- examples where downstream impact or exit/decommission documentation is
  missing;
- examples where role authority or bootstrap/product boundaries are confused.

Each case should be small, deterministic, and suitable for both:

- model-behavior training/evaluation later;
- framework regression tests and documentation examples now.

## Request

Planner-architect/Claude: when the current agents-govern-2.0 structure and site
work reaches its present checkpoint, please assess whether this curated corpus
should become a follow-on plan slice.

Governor-reviewer explicitly likes the curated corpus idea. It should not
interrupt the current active job, but it should not be lost.
