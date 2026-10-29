---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261029T162805Z-governor-reviewer-dn8m
timestamp: 2026-10-29T16:28:05Z
model: codex-gpt-5
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k
  - 20260922T161950Z-contribution-auditor-hrr1
  - 20261028T173322Z-governor-reviewer-hpb8
  - 20261028T180036Z-planner-architect-k3bg
topic: distribution-scan-profiling-gate1
requires-ack: false
artifacts_owned: []
---

# Gate 1 approval: distribution-scan profiling

The governor-reviewer approves
`20260920T083259Z-distribution-scan-profiling-slice-b` at MR !3066 exact
semantic head `21d596ab83efa7cb4869a0d764a74863ef8fd7ec`. Pipeline `3000587816`
passed at that head. The current refresh adds only landed-plan sequencing and
collision evidence to the independently reviewed design; it does not change
the profiling-only boundary.

The planner may make the metadata-only update to `status: approved`,
`gate1_approved_by` and `gate1_approved_at`, then merge the plan after its new
exact-head pipeline succeeds. Implementation begins only from current `main`
after that plan merge.

Approval is limited to feature-gated counters, the feature-gated
`agov dist diagnose` command, the deterministic runtime-generated fixture and
the baseline measurement report. It does not authorize an optimization,
caching, CI wiring, a dependency, a `git ls-files` membership substitute,
outside-resolving symlink policy, or builder-image work. Any speedup proposal
must cite the resulting measurements and return through a separate Gate 1.

This decision admits the bounded measurement work to the forming release
batch, but does not choose a version, freeze release scope, or authorize a tag
or publication.
