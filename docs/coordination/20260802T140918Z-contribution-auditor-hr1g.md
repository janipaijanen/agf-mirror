---
artifacts_owned: []
from: contribution-auditor
id: 20260802T140918Z-contribution-auditor-hr1g
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260802T140009Z-planner-architect-5mgm.md
requires-ack: false
timestamp: 2026-08-02T14:09:18Z
to: governor-reviewer
topic: ci-rust-cache-policy-slice-b-plan-review
---

# Independent review of MR !936: CLEAR with one advisory finding

Scope: `docs/plans/ci-rust-cache-policy-slice-b.md` at exact plan commit
`5b24854ffa2e84d01d0fae87534221597ca08a05` (MR !936 head `3448d0d2`,
unchanged plan content — confirmed via `git log -- docs/plans/
ci-rust-cache-policy-slice-b.md` showing that commit as the only one
touching the file; the head commit only adds the review-request
coordination record).
Coverage: COMPLETE for a Gate 1 plan review.
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 1
Authority: recommendation only; the human governor-reviewer decides Gate 1.

I tried to reject the single-writer topology, the image/key boundary, and
the fail-safe role contract, per the handoff's explicit instruction. Two
of the plan's most load-bearing factual claims were independently
re-verified against live external sources rather than trusted from the
plan's own text, and one design mechanism was empirically tested rather
than assumed from GitLab documentation recollection.

## Independent verification

**Docker Hub image-digest claim.** The plan pins `.rust-base.image` to
`rust:1.97.1@sha256:1bcff4befb740599103a2c7cb51058e14479b2e35e3a34a3f0dc4ede09927488`
and claims `rust:1` and `rust:1.97.1` currently share this index digest.
I queried the Docker Hub registry API live (not the plan's claim):

- `rust:1.97.1` → `Docker-Content-Digest: sha256:1bcff4befb...` — matches.
- `rust:1` → `Docker-Content-Digest: sha256:1bcff4befb...` — matches, same
  digest.
- The linux/amd64 platform manifest's config blob declares
  `RUST_VERSION=1.97.1` — matches.
- The resolved image config digest (`sha256:e9f26332...`) matches the
  `rust:1` image GitLab's own runner actually used in MR !930's exact-head
  trace, which I independently observed during that Gate 2 review before
  this plan existed (`Using docker image sha256:e9f26332...451222c1831d0e45
  for rust:1 with digest rust@sha256:1bcff4befb...`). Three independent
  sources (plan text, live registry query, prior session trace evidence)
  agree.

**`extends` hash-merge for `cache:`.** Slice A was bitten twice by
`extends` *not* merging array keys (`before_script`). Slice B's writer
override relies on the opposite claim — that `extends` *does* deep-merge
hash keys, so overriding only `cache.policy` on `rust:tooling:test`
retains the inherited `key`/`fallback_keys`/`paths`. Rather than trust
this from memory, I submitted a minimal reproduction (the real
`.rust-base` cache block plus a child job overriding only `policy`) to
the project's live CI Lint API (`dry_run=true`, `include_jobs=true`) and
inspected `merged_yaml`. Confirmed: the child job's merged cache block
retains `key: rust-v3-main`, both fallback keys, and all three paths,
overriding only `policy`. The plan's central mechanism works as claimed.

**`rust:tooling:oci-pull-test` isolation.** Confirmed by reading its
actual definition: no `.rust-base` extension, its own image
(`quay.io/podman/stable`), its own unmanaged cache key
(`rust-oci-pull-test`, no `policy:` key at all — always GitLab's default
`pull-push`). Correctly outside this plan's scope, as both the plan and
MR !935 already stated; I re-confirmed rather than re-assumed it.

**Dynamic topology-check design.** The plan requires the topology
assertion to dynamically inventory direct *and indirect* `.rust-base`
inheritors rather than hard-code a job list — directly addressing the
exact failure mode Slice A hit (`.rust-tooling-package` and
`rust:tooling:build:windows:preview` silently skipped the policy
assertion because a naive check wouldn't have traced two-level
`extends`). The plan's proposed fallback — GitLab CI Lint's `merged_yaml`
as the topology input — is sound: `merged_yaml` is already fully
flattened regardless of `extends` depth, which is exactly the property
needed. This is the right lesson taken from the right prior incident.

**`rust:tooling:test`'s current definition** (`stage`, `interruptible`,
`script`, `rules`) is unaffected by the plan's proposed additive
`variables`/`cache` override block — confirmed by reading the job as it
exists today, not assumed from the plan's snippet in isolation.

## Advisory finding AG-CACHE-SLICEB-001 (non-blocking)

The "why `rust:tooling:test`" section explicitly rejects three
alternatives (`agov:governance:validate`, `rust:tooling:deny`, packaging
jobs) but does not address the other four of the eight current automatic
shared-cache consumers (`rust:agov:validate`, `agf:dist:validate`,
`agf:dist:preview`, `rust:tooling:clippy`). Having read each job's
`script:`, `rust:tooling:test` (`cargo test --workspace`) does appear to
be the strongest candidate — it is the only job that compiles every
workspace crate including `#[cfg(test)]` code paths in debug profile,
making its cache content the broadest overlap with what the other
debug-profile consumers need. But the plan does not state this
reasoning, so a reviewer has to reconstruct rather than verify it.

More substantively: `agov:governance:validate` is the *only* current
shared-cache consumer that builds `--release` (`cargo build --release -p
agov`). Under the old eight-writer, last-writer-wins topology, it
sometimes populated the shared cache with release-profile artifacts when
it happened to finish last — an inconsistent but nonzero benefit. Under
Slice B, since the sole writer (`rust:tooling:test`) only ever builds
debug/test profile, `agov:governance:validate` will *never* get
release-profile object reuse from the shared cache going forward — a
guaranteed-absent benefit replacing a previously inconsistent one. This
is acceleration-only (the plan is explicit and repeated that cache
content is never acceptance evidence, so no correctness risk), and the
plan's own Gate 2 item 10 (per-job duration reporting) and item 14
(before/after comparison against the MR !935 eight-writer baseline)
would surface it empirically if it matters in practice. Recommend the
Slice B implementation handoff or the Gate 2 evidence report name this
tradeoff explicitly rather than leave it for a reader to notice
independently in the post-merge numbers.

## Disposition

CLEAR. No blocking findings. The plan's Gate 2 acceptance matrix (14
items) is unusually rigorous, directly answers both concrete bugs Slice A
actually produced (array-merge assumption, hard-coded consumer list), and
correctly keeps issue #101 open through the full measurement window. Ready
for governor-reviewer's binding Gate 1 decision.
