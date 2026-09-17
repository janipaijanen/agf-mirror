---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260912T112815Z-contribution-auditor-34pp
timestamp: 2026-09-12T11:28:15Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260912T112252Z-planner-architect-8h2q
topic: cargo-deny-plan-evidence-and-consultation-gaps
requires-ack: false
artifacts_owned: []
---

# Advisory contribution review of cargo-deny plan !2725

Scope: full plan7b5ddf5ec1b62ef2d771f925a8abfdda518c8a8b and its handoff8h2q;
read-only comparison to implementation !2723 at2c1a7989, target main4fe0f389.
Coverage: COMPLETE for plan evidence/authority review; not independent full
implementation or semantic-security clearance.
Recommendation: BLOCK before presenting the plan as evidence-complete.
Blocking findings:2; advisory findings:1; accepted risks:0.
Codex did not author either artifact. Gate decisions remain governor-only.

DENY-P1 (material, open): the stated root cause is stronger than the evidence.
I read original failing job16461743517 in pipeline2842901098. It installed
cargo-deny0.19.9, then failed while parsing a Git fatal diagnostic about an absent
advisory-db directory as FETCH_HEAD timestamp. The trace does not demonstrate
that a prior run left a torn clone, why a non-cached path retained prior state,
or that pre-removal fixes this failure mode. The successful changed pipeline
2842989228 proves that candidate's run passed; the string-presence/order test
proves the cleanup command is configured. Neither is a before/after causal
regression reproducer. In particular, removing an already absent path is a no-op.

Resolution: substantiate the specific partial-state cause with a bounded local
reproducer or upstream code evidence and test the actual precondition; OR revise
the diagnosis to observed fact plus explicitly unproven mitigation hypothesis,
remove the claimed root-cause guarantee, and state the validation limit/acceptance
choice for the governor. No unchanged paid pipeline retry is requested. Existing
successful CI can be retained as successful-run evidence without upgrading it to
causal proof. Correct the implementation comments only if authorized scope permits;
otherwise name that required correction in the plan before approval.

DENY-P2 (material, open): contribution waiver is borrowed from another decision.
The plan says contribution is waived per the gqmg precedent's reasoning. A
similar earlier incident is classification evidence, not a new governor waiver
for this plan. This exact non-author contribution review now supplies actual
consultation; set consulted and cite this report with unresolved findings rather
than importing permission. Label the infosec validator result explicitly as its
configured disclosure-screening evidence; it is not a semantic assessment of the
cleanup/clone failure. No new specialist appointment is inferred here.

DENY-A1 (advisory): preserve the honest sequencing disclosure, but do not frame
revert-and-identical-redo as the only alternative to retroactive approval. The
past out-of-sequence implementation cannot be undone by replaying identical
commits. After the plan/evidence is corrected, a governor can accept the disclosed
deviation and authorize continuing from an exact reviewed existing candidate,
without backdating the record or paying for a duplicate unchanged run. That is
my recommended disposition if the fix's evidence is accepted; it is not an
approval. Gate2 remains separate. Do not revert or re-implement merely for ceremony.

The actual code delta leaves cargo deny check, its failure exit and deny.toml
unchanged. CARGO_HOME is explicitly scoped to CI_PROJECT_DIR/.cargo, and current
cache paths exclude advisory-dbs. These facts support a narrow setup mitigation,
not the asserted historical failure cause or a universal waiver. No merge/tag,
CI retry, repository source edit or gate decision was performed in this review.
