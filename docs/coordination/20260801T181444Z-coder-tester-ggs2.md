---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .gitlab-ci.yml (read-only investigation; no implementation authorized)
  - main
  must_read:
  - .gitlab-ci.yml (lines roughly 21-65, 651-729 for .rust-base and
    agov:governance:validate)
  next_agent: contribution-auditor
  status: ready
  task:
  - "Critically assess the CI cache-policy diagnosis below. Independently
    reproduce the evidence via glab api (do not trust the numbers as
    given) and either confirm, refute, or refine. This is an ad hoc
    technical investigation, not tied to any Gate 1/Gate 2 plan; no
    implementation is authorized yet."
id: 20260801T181444Z-coder-tester-ggs2
immutable: true
kind: handoff
model: claude-sonnet-5
references-to: []
requires-ack: false
timestamp: 2026-08-01T18:14:44Z
to: contribution-auditor
topic: agov-governance-validate-cache-policy-critical-review
---

# Critical review requested: agov:governance:validate CI cache-policy diagnosis

This is an ad hoc technical investigation prompted by the human governor,
outside any current Gate 1/Gate 2 plan. No implementation is authorized.
The governor explicitly asked for a critical, consensus-seeking second
opinion — please push back on anything that doesn't hold up under your own
independent reproduction, not just agree.

## Chain so far

1. I (coder-tester, Claude) diagnosed why `agov:governance:validate` is
   slow, using `glab api projects/janipaijanen%2Fagents-govern-framework/jobs`
   (paginated, ~400 jobs) and `glab api .../jobs/<id>/trace` for several
   job IDs, parsing GitLab's `section_start:<epoch>:<name>` /
   `section_end:<epoch>:<name>` trace markers.
2. A Claude subagent then independently re-derived the same evidence from
   scratch (different sample window, ~2000 jobs, 33 traces) specifically
   to stress-test the diagnosis before treating it as settled. It confirmed
   the core diagnosis, added a new finding I had missed, and refined the
   proposed fix. Full subagent output is reproduced below so you can
   critique it directly rather than a paraphrase.

## Diagnosis (as it stands after both passes)

- `agov:governance:validate` job duration: avg ~260-278s across samples.
- The actual work (`cargo build --release -p agov` + `agov coord
  validate`/`plan validate`/`plan projection-check`/`coord
  publication-check` calls) — the `step_script` trace section — is only
  3-53s. A same-template debug-profile job (`rust:agov:validate`) shows
  near-identical overhead with `step_script` down to 3-9s, ruling out
  build profile (release vs debug) as the driver.
- `restore_cache` (~50-66s) + `archive_cache` (~85-190s, avg ~129s) account
  for 70%+ of total job wall-clock time. `archive_cache` alone is ~50%.
- Inside `archive_cache`, the dominant cost is local zip creation over
  ~42,800 files (`.cargo/registry` + `.cargo/bin` + `tooling/rust/target`
  combined file counts, confirmed from raw trace log lines), not network
  upload (~17s of the ~104-129s).
- `.rust-base`'s cache key is `rust-$CI_COMMIT_REF_SLUG` (per-branch), no
  `policy:` set (GitLab default `pull-push` — every job both restores AND
  re-uploads unconditionally). The subagent quantified that 80-84% of
  distinct refs that trigger these jobs run them exactly once — meaning
  most branches pay the full ~120-130s archive cost to write a
  never-read-again cache namespace, only benefiting from `main`'s cache
  via `fallback_keys: [rust-main]` on read.
- The subagent additionally found a same-pipeline concurrent-write race:
  on any commit touching `.gitlab-ci.yml` or `tooling/rust/**/*` (including
  on `main`), 5 `.rust-base` jobs (governance:validate, rust:agov:validate,
  tooling:deny, tooling:test, tooling:clippy) start within ~250ms of each
  other, all reading and writing the *same* cache key — last-writer-wins
  overwrites the other four jobs' archive work, wasting it even on the
  protected branch.

## Proposed fix (not yet implemented, not yet Gate-reviewed)

Make `cache:policy` conditional: `pull-push` only when `$CI_COMMIT_BRANCH
== $CI_DEFAULT_BRANCH`, `policy: pull` (skips `archive_cache` entirely,
confirmed via GitLab's own docs on `policy: pull` semantics) everywhere
else. Refine by also splitting the shared `tooling/rust/target` cache
path/key by job or build profile, to fix the concurrent-write race that
the policy change alone doesn't address.

## Full subagent output (verbatim, for your own critique)

---

Independently verified — matches the claim:

1. `.rust-base` cache config confirmed: key `rust-$CI_COMMIT_REF_SLUG`,
   fallback `rust-main`, paths `.cargo/registry`, `.cargo/bin`,
   `tooling/rust/target`; no `policy:` anywhere in `.gitlab-ci.yml`.
2. Duration numbers recomputed independently (n=983, last ~6 days):
   governance:validate n=471 avg=260.5s median=258.8s; rust:agov:validate
   n=156 avg=228.0s median=227.5s. Release-vs-debug delta ~32.5s, far
   smaller than a real compile-time difference would predict.
3. Section-duration breakdown across 33 traces: restore_cache avg 57.0s
   median 53.0s; archive_cache avg 129.4s median 127.0s range 85-187s,
   the single largest section in every trace with no exceptions.
   archive_cache alone = 50.4% of total wall-clock; restore_cache +
   archive_cache = 72.6%.
4. Reproduced the exact 104s-local-zip / 17s-upload split from job
   15659937103's raw trace, including the literal
   "tooling/rust/target: found 26125 matching artifact files and
   directories" log line and the 104.2s silent gap before "Using
   presigned URL for cache upload".
5. Quantified branch-cache waste directly from job ref fields:
   governance:validate 213/265 distinct refs (80%) ran exactly once;
   rust:agov:validate 90/107 (84%). Also caught a live fallback-miss-then-
   hit sequence in a trace (job 15545452450): branch-specific cache lookup
   fails ("WARNING: file does not exist" / "Failed to extract cache"),
   falls through to `rust-main-protected`, succeeds.
6. Confirmed `cache:policy: pull` semantics via GitLab's own docs: skips
   the upload step entirely, and accepts CI/CD variables (so a
   rules-conditional variable driving `policy:` is valid syntax).

Pushback / refinements:

1. Same-pipeline concurrent cache-write race (described above) — a
   distinct, compounding inefficiency the branch-lifetime framing doesn't
   cover, and one the proposed fix alone doesn't fix since it leaves
   `main` on `pull-push`.
2. Branch cache isn't 100% waste: ~20% of refs get 2-5 runs and do benefit
   from their own branch-scoped cache on push 2+. Under the fix, every
   push on a non-default branch always falls back to `main`'s cache
   instead. Not a correctness risk, but a real (small) trade-off worth
   stating rather than assuming away.
3. Wall-clock savings framing: since the 5 jobs run in parallel, removing
   archive_cache from all of them shrinks the *stage's* wall-clock by
   roughly one archive_cache duration (gated by whichever job's
   step_script is slowest, e.g. rust:tooling:deny), not five archive_cache
   durations stacked. The "~120s+ eliminated" claim is more accurate
   for aggregate runner-minutes/compute cost than for human-perceived
   pipeline wall-clock; the two should be stated separately.
4. Checked every `needs:`/artifact dependency in `.gitlab-ci.yml`: none of
   them consume cache contents from a sibling `.rust-base` job (they all
   depend on `artifacts:` from `.rust-tooling-package`, a separate
   mechanism unaffected by `cache:policy`). No staleness risk found from
   switching non-default branches to `policy: pull`.
5. Checked for a deliberate per-branch toolchain-isolation rationale for
   the cache key shape: none found — all `.rust-base` jobs pin the same
   `image: rust:1`; GitLab's own `-protected`/`-non_protected` key suffix
   already handles the actual trust boundary. The proposed fix only
   changes write behavior, not key shape, so this isn't an objection to it.
6. restore_cache (~50-65s) judged not worth chasing separately — real
   content value (avoids a cold build against 16,671 registry files), not
   misattributed as the bottleneck in the original diagnosis.

Subagent's own verdict: agree with the core diagnosis; confirm the
proposed fix direction as semantically correct and low-risk; refine by
also splitting the shared cache key/path by job or build profile to
address the same-pipeline concurrent-write race, which the policy change
alone leaves unfixed even on `main`.

---

## What's being asked of you

Reproduce this yourself — `glab api` is available; don't trust either pass
above at face value. Then state plainly: agree, disagree, or refine, with
your own evidence (job/trace IDs, exact log excerpts, or `.gitlab-ci.yml`
line citations), same standard the subagent held itself to. If you reach a
different root cause or a materially different fix, say so directly —
the goal is a converged, correct recommendation, not three passes that
each politely agree with the last.
