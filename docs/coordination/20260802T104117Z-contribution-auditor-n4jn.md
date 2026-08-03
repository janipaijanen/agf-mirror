---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260802T104117Z-contribution-auditor-n4jn
timestamp: 2026-08-02T10:41:17Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260802T094729Z-contribution-auditor-j41s
  - 20260802T100519Z-coder-tester-m3rb
topic: ci-rust-cache-policy-slice-a-implementation-renewed
requires-ack: false
artifacts_owned: []
---

# Renewed independent review of MR !930: CLEAR with one evidence advisory

Scope: MR !930 at exact head
`103b57719c3c5f344eb8ee22f7b325d2d856b241`, pipeline `2725002066`  
Coverage: COMPLETE for the approved pre-Gate-2 MR evidence  
Recommendation: CLEAR  
Blocking findings: 0  
Advisory findings: 1  
Open findings with accepted risk: 0  
Authority: recommendation only; the human governor-reviewer decides Gate 2

The prior blocking finding CI-CACHE-A-001 is resolved. The renewed resolved
sequence runs the cache-policy assertion and no longer changes into
`tooling/rust` twice. The exact-head pipeline passed and GitLab reports the MR
mergeable.

## Independent trace verification

I fetched the raw trace for every automatic shared-cache consumer in pipeline
`2725002066`, rather than relying on the handoff summary:

- `rust:tooling:clippy` (`15664107142`)
- `rust:tooling:test` (`15664107140`)
- `rust:tooling:deny` (`15664107139`)
- `rust:agov:validate` (`15664107138`)
- `agov:governance:validate` (`15664107137`)
- `agf:dist:validate` (`15664107131`)
- `rust:tooling:preview` (`15664107127`)
- `agf:dist:preview` (`15664107126`)

Each trace contains exactly one live
`OK resolved cache policy: pull (source=merge_request_event ...)` assertion
and exactly one `Not uploading cache rust-v2-main-protected due to policy`
message. Across all eight traces there are zero `Uploading cache to` and zero
`Created cache` messages. The separately keyed
`rust:tooling:oci-pull-test` is correctly outside this inventory and also
passed.

The two manual MR consumers
`rust:tooling:preview:windows-package` and
`rust:tooling:build:windows:preview`, plus tag-only
`agf:dist:release` and `rust:tooling:release`, were not manufactured solely
for this review. I independently inspected GitLab CI Lint's resolved merged
YAML: all four resolve `key: rust-v2-main`,
`policy: $RUST_CACHE_POLICY`, and the live policy assertion. The package and
Windows overrides retain the corrected working-directory sequence.

## Job-set and local checks

I submitted both the exact candidate YAML and pre-change main YAML
(`5140de71`) independently to the project CI Lint API with `dry_run=true`,
`include_jobs=true`, and `ref=main`. Both are valid without warnings and
resolve the identical ordered 18-job set. `git diff --check` and the exact
head's cache-policy `selftest` pass.

## Measured exact-head envelope

For the eight shared-cache automatic jobs:

- restore sections: 64–73 seconds each;
- `step_script`: 4–574 seconds each;
- Runner `archive_cache` wrapper sections: 0–1 seconds each;
- shared-cache job total: 2,299.06 runner-seconds.

The full automatic pipeline consumed 2,826.37 aggregate runner-seconds. Its
critical-path/pipeline duration was 1,069 seconds with 2 seconds queued. These
are separate measures; the aggregate is not represented as human waiting
time. Post-merge sampling remains required by the approved plan before Slice
B.

## Advisory AG-CACHE-EVIDENCE-001

The approved plan's Gate 2 item 7 literally says automatic traces must contain
“no `archive_cache` section.” GitLab Runner does open such a wrapper section
for every pull-only job, but it lasts at most about one second and contains
only `Not uploading cache ... due to policy`; it performs no archive creation
or upload. Thus the intended control is empirically satisfied, but the literal
trace-presentation predicate is not.

This is not a code blocker: making GitLab Runner omit its bookkeeping section
is outside the implementation's control, while zero archive/upload work is
directly verified. The Gate 2 decision should explicitly interpret the plan's
phrase as “no cache archive creation or upload,” and the future Slice B plan
should use that observable wording. Do not claim that the section itself is
absent.
