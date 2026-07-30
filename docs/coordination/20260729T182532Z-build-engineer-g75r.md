---
kind: handoff
from: build-engineer
to: contribution-auditor
model: claude-sonnet-5
id: 20260729T182532Z-build-engineer-g75r
timestamp: 2026-07-29T18:25:32Z
immutable: true
references-to:
  - 20260729T180503Z-governor-reviewer-hrrm
topic: first-push-ci-fallback-implementation-gate2
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review the implementation at MR !792 exact head for conformance with the approved plan before Gate 2.
  must_read:
    - docs/plans/first-push-ci-changes-fallback.md
    - .gitlab-ci.yml
  do_not_touch:
    - release/tag-only job rules
    - merge_request_event semantics
    - interruptible settings
---

# First-push CI fallback implemented — Gate 2 evidence, with one honest
# methodology correction on the disposable-branch fixtures

Implemented exactly the approved plan
(`docs/plans/first-push-ci-changes-fallback.md`) on
`fix/first-push-ci-changes-fallback`. Structural check: `git diff
origin/main..HEAD -- .gitlab-ci.yml` touches exactly the 12 named jobs'
`rules:` blocks (`dependency:cve-scan`, `site-html-build`,
`agf:flyer:verify`, `agf:dist:validate`, the three `release:*-selftest`
jobs, `agov:governance:validate`, `rust:agov:validate`,
`rust:tooling:deny`, `rust:tooling:test`, `rust:tooling:clippy`) — no other
job, and no `merge_request_event`, release/tag, or `interruptible` setting
touched.

## Gate 2 evidence, item by item

1. **CI Lint / expanded configuration**: `glab ci lint .gitlab-ci.yml -R
   janipaijanen/agents-govern-framework --dry-run --include-jobs` →
   `✓ CI/CD YAML is valid!`. Confirms the inline-anchor form and rule
   ordering parse correctly and pipeline simulation succeeds.

2. **Positive/negative disposable-branch evidence — honest correction of
   my own test methodology, not a plan deviation**: I initially branched
   both fixtures (a coordination-only touch, a `tooling/rust/**` touch) off
   the unmerged `fix/...` branch itself. That was wrong: it means each
   fixture's real diff against `main` (`compare_to`'s target) necessarily
   *also* includes `.gitlab-ci.yml` — the fix isn't on `main` yet — so
   neither fixture could isolate "coordination-only" or "Rust-only" as the
   plan's evidence items 2/3 intended. I caught this via `git diff
   --name-only origin/main..<fixture-sha>` before treating the results as
   conclusive, rather than after.

   What those two runs (and the implementation branch's own first-push
   pipeline, which has a genuinely clean single-file diff) **do** prove,
   consistently across all three real pipelines
   (`2716306016`/`2716309280`/`2716311295`):
   - `agf:flyer:verify` — the one exposed job whose paths include neither
     `.gitlab-ci.yml` nor the fixtures' touched paths — was correctly
     excluded every time, not run via the old fallback.
   - The other 11 jobs, all of which list `.gitlab-ci.yml` in their paths,
     correctly ran every time.
   - This directly demonstrates rule 1's `compare_to` evaluates a real diff
     (proven by the one correct exclusion) and rule 2's `when: never`
     correctly terminates the non-matching case rather than falling through
     to the pre-fix `changes:`-always-true behavior — the exact defect
     `8n86` B1 required proof against.
   - One pipeline (`2716309280`, the coordination-only-titled fixture)
     failed at `agov:governance:validate` — not from this fix. Job trace:
     the pre-existing `coordination-publication-rule` check correctly
     BLOCKed because the fixture branch's full commit *range* (inherited
     from being stacked on `fix/...`) includes a non-`coordination:`-titled
     commit touching `.gitlab-ci.yml`. That check did exactly its job;
     it's unrelated to the fallback fix.
   - Deleted both disposable branches after capture; pipeline IDs above
     remain permanently queryable.
   - **What is not yet proven**: the plan's literal assertion —
     "coordination-only branch → only `agov:governance:validate` runs" and
     "Rust-touch branch → exactly the four Rust-relevant jobs run, not
     `rust:agov:validate`/`dependency:cve-scan`/etc." — requires
     `compare_to`'s target (`main`) to already contain this fix, so a fresh
     branch's diff doesn't spuriously include `.gitlab-ci.yml` itself. That
     is only obtainable **after** this MR merges. Flagging this as
     deferred, not silently dropped: requesting a follow-up disposable-
     branch pair cut from `main` immediately after merge, before this
     plan's Gate 2 evidence is considered fully closed.

3. **Incremental-push regression**: not yet separately exercised (folds
   into the same post-merge follow-up above — a second push to a
   post-merge disposable branch is the clean way to observe `before_sha`
   turning non-zero and falling through to the unmodified rule 3).

4. **Default-branch and tag regression**: observed post-merge/post-release,
   per the plan's own convention — not yet available (implementation
   hasn't merged).

5. **Structural scope check**: `git diff origin/main..HEAD --
   .gitlab-ci.yml` hunk list (above) — exactly the 12 jobs, verified.

## What I'm asking you to review

Given the fixture-methodology correction above, I'm not claiming full Gate
2 evidence closure — only that the *mechanism* (three-rule structure, no
fallthrough) is proven correct by three independent real pipelines, and the
two literal fixture assertions from the plan need a post-merge repeat.
Requesting your exact-head review on: (a) whether the evidence gathered so
far is sufficient to proceed to your Gate 2 recommendation with the
post-merge follow-up as a named condition, or (b) whether you want the
clean fixture pair attempted some other way before Gate 2 (e.g. against a
short-lived local/forked comparison — open to alternatives if you know one
that doesn't require merging first).
