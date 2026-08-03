---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T074541Z-contribution-auditor-rs7b
timestamp: 2026-08-02T07:45:41Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260801T181444Z-coder-tester-ggs2
topic: ci-cache-policy-independent-review
requires-ack: false
artifacts_owned: []
---

# Independent review: CI cache-policy diagnosis

Scope: MR !910 at exact head `e61374d426451f26eb47f2f835690a82a308779f`,
checked against current `main` at `0ba1d9e7ebf8a04f6847acc8fad405241914e8c2`
Coverage: COMPLETE
Recommendation: CLEAR for the diagnosis; REFINE the implementation design
Blocking findings: 0
Advisory findings: 3
Open findings with accepted risk: 0
Authority: recommendation only; no implementation or gate decision is authorized

## Disposition

I independently reproduce the core diagnosis. The shared Rust cache is the
dominant cost of the affected jobs, most branch cache namespaces are written
only once, and parallel jobs do upload different archives to the same cache
object. Changing non-default work to pull-only is a strong first slice.

I do **not** yet endorse "split the cache by job or build profile" as the
settled second slice. That removes the overwrite collision, but can retain the
large upload cost and multiply cache storage. The second slice should instead
be selected after an A/B measurement and should prefer one controlled writer
with pull-only consumers unless measurements show job-specific target caches
have better total cost.

## Independently reproduced evidence

1. Current `.gitlab-ci.yml` lines 46-57 still define one `.rust-base` cache:
   key `rust-$CI_COMMIT_REF_SLUG`, fallback `rust-main`, and paths
   `.cargo/registry`, `.cargo/bin`, and `tooling/rust/target`. No policy is
   specified, so GitLab's default `pull-push` applies.
2. From the newest 2,000 project jobs queried on 2026-08-02:
   - `agov:governance:validate`: 438 successful jobs, mean 261.08 s;
     205 of 256 refs (80.08%) appear exactly once.
   - `rust:agov:validate`: 134 successful jobs, mean 231.73 s;
     76 of 92 refs (82.61%) appear exactly once.
   These independently reproduce the original 80-84% branch-lifetime result.
3. Job `15663334506` on current `main` spent approximately 72 s restoring the
   cache, 18 s in `step_script`, and 147 s archiving it. Archive creation saw
   16,671 registry entries plus 27,750 target entries; about 119 s elapsed
   before upload began, followed by about 27 s of upload time.
4. Job `15663090789` on the AgentCouch plan branch missed its branch cache,
   restored `rust-main-protected`, did about 20 s of script work, then spent
   about 125 s creating and uploading a new branch cache. This is the exact
   waste pattern described in the request.
5. Pipeline `2724116160` launched all five `.rust-base` jobs within 242 ms.
   Jobs `15659882290` through `15659882294` all uploaded to the identical
   cache-object suffix
   `21d01076c84bda41fe101982f5bf2ed9889f24f6ab717ebe35aaa444a907c695`.
   GitLab documents that jobs using the same key overwrite the same cache
   archive; the last completed upload wins.
6. GitLab's current official examples explicitly support a CI variable for
   `cache:policy`, using `pull-push` on the default branch and `pull` elsewhere.
   GitLab also recommends pull-only consumers when many parallel jobs use one
   cache.

## Refined implementation recommendation

### Slice A: implement and measure the low-risk win

- Make non-default branches, merge-request pipelines, and tags pull-only.
- Keep a deliberate default-branch writer so the fallback cache remains warm.
- Cover every pipeline source explicitly. A variable that is defined only by
  a branch comparison can be unset on tag pipelines because
  `CI_COMMIT_BRANCH` is absent there.
- Add a CI configuration test that proves the resolved policy for default
  branch, non-default branch, merge request, and tag contexts.
- Compare at least one before/after sample for archive duration, total job
  duration, aggregate runner-seconds, and human-visible pipeline critical path.

### Slice B: design the writer topology from evidence

- Prefer one controlled cache writer and pull-only parallel consumers. This
  directly removes concurrent uploads rather than merely giving every upload a
  different destination.
- Consider separating dependency caches from compiled `target` output and
  binding keys to relevant inputs such as `Cargo.lock`, Rust toolchain/image,
  and build profile. Do not assume a per-job target cache is cheaper: it can
  duplicate the same registry and target trees several times.
- Run an A/B comparison of at least: current shared cache, pull-only consumers
  with one writer, and (only if still warranted) job/profile-specific target
  caches.

## Advisories

1. Existing branch-specific cache objects can shadow the `rust-main` fallback
   after a branch becomes pull-only. The implementation plan should decide
   whether to change the read key, version the cache namespace, or accept that
   bounded transition explicitly.
2. The current `rust:1` image is floating. Any cache-key redesign should bind
   the execution image/toolchain before relying on compiled-target reuse as
   evidence across pipelines.
3. The claim that restore cost is worth keeping is plausible but not proven by
   the current traces. Restore takes roughly 50-72 s; test it rather than
   treating it as settled. This does not block Slice A, which changes only
   writes.

## Conclusion

The diagnosis is sound and now independently reproduced by a third pass. The
next governed artifact should be a Gate 1 plan for Slice A plus its measurement
contract. Keep the more invasive cache topology as a separately justified
Slice B, not as an automatic appendage to the pull-only fix.

Official semantics checked:

- <https://docs.gitlab.com/ci/caching/>
- <https://docs.gitlab.com/ci/caching/examples/#use-a-variable-to-control-a-jobs-cache-policy>
