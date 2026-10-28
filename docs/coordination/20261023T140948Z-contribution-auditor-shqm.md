---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20261023T140948Z-contribution-auditor-shqm
timestamp: 2026-10-23T14:09:48Z
immutable: true
references-to:
  - 20261023T140946Z-planner-architect-gmpg.md
topic: contribution-audit-recall-corpus-plan
requires-ack: false
artifacts_owned: []
---

# Contribution audit and infosec screen: rule-trigger recall corpus plan (issue #138)

Performed by the AI side of the contribution-auditor role (claude) for the
planner-architect's own plan `20261023T140050Z-rule-trigger-recall-corpus`
(MR !3167, head `bb6e66ce6f4cb24e9e49fec0a8380d5303ef3918`). Author and auditor are the
same assistant; the governor may pull or redo this. Advisory evidence only; it grants no approval.

## Result: no blocking finding

- Adversarial steering and unsound guidance: none. All 13 rules the first slice names
  resolve to files under `agf/rules/` on `main`; 13 rules x 4 scenarios = 52, and the
  hold-out quarter is 13. The target directory `docs/reference/rule-trigger-recall/`
  does not exist yet, so the plan adds new files only.
- False precedent: none. No rule, loader, schema or tool changes. The plan states the
  honest limits (3 samples cannot claim 95%) and defers a runner.
- Finding 1 (WARN, plan gap): the claim gate costs at least 20 trials x 13 rules x 2 models =
  520 paid model calls per run, before paraphrases. The plan gives no cost bound. Add
  an estimate and a rule that the claim gate runs only when the governor asks.
- Finding 2 (INFO): classification reads "NORMAL-to-STRICT". The development workflow
  wants exactly one mode. Treat it as STRICT for review purposes, since later MRs will
  cite the corpus as acceptance evidence.
- Finding 3 (INFO): the hold-out is authored by Codex only; record that author in
  `corpus.yaml` so the separation is auditable.

## Infosec screen: clean

`agov validate infosec` passes. The plan adds prompts and paths only; no credentials,
cross-project linkage or fictional names.

Highest severity: WARN (finding 1, to be resolved by a plan edit before approval).
