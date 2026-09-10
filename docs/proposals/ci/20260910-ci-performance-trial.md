<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# CI performance trial — independent consultation draft

Status: proposal, pending independent contribution and InfoSec consultation; no implementation approval. Owner: Codex planner-architect/build-engineer under governor allocation eetd and acknowledged Claude standdown mb8k/h5hd. Track through existing #53 and #106. The governor requested trying the proposals; !2673 publishes the evidence and transfer. No new release scope or paid capacity is implied.

This is a pre-Gate-1 proposal because the plan schema cannot represent pending consultation (#130). It must not be labelled consulted until actual evidence exists. After consultation, promote each independently executable slice to its own pending-Gate1 plan with actual consultation references, then obtain a governor decision before implementation. Codex cannot independently clear its authored proposal.

## Measured problem and selection

The 2026-09-05 15:17:45Z through 2026-09-10 15:17:45Z pipeline-created cohort contains 996 pipelines, 3,031 job records and 47.42 recorded runner-hours. All job metadata and 77 traces were inspected. Governance validation used 25.50 hours. Successful median governance runtime was 148 seconds versus 403 seconds queue wait. Representative logs show about 85 seconds before projection results; repeated filesystem glob expansion is a strong candidate, not proven causal attribution. A completed local dist validation took 13.32 seconds; an instrumented run stopped at 45 seconds after 577,806 directory-read calls. The latter is partial diagnostic evidence, not an ordinary runtime.

Some 33-second commercial validator jobs spent 19–23 seconds extracting the broad Rust cache and 3–9 seconds executing scripts. The Rust packaging job installs binutils, jq, musl-tools and zip every run. No installation-specific saving has yet been measured. Source/evidence and qualifications are in g3j5/1has/msq8; this proposal does not claim a speedup.

## A. Native Rust builder image pilot

Proposed files: `tooling/ci/rust-native/Dockerfile`, `tooling/ci/rust-native/packages.lock`, `tooling/ci/rust-native/README.md`, and the native Rust packaging job configuration in `.gitlab-ci.yml`. These are proposed new paths, absent on inspected main 50175057; recheck collisions before implementation. Do not modify shared cross-build templates in a way that removes tools needed by other platforms.

Keep the existing digest-pinned Rust base. Install binutils, jq, musl-tools and zip into the image. The Git-tracked lock records exact package versions plus repository snapshot identity, architecture and base digest, including the resolved dependency inventory. Version numbers alone against a mutable apt repository are insufficient for reproducible rebuilds. Resolve this inventory against the selected base during the approved experiment; do not invent current versions. Package locks and Dockerfile changes receive review. No project source, credentials or precompiled agov binary is baked in.

Proposed destination: this project's existing GitLab registry namespace, subpath `ci/rust-native`; confirm access and retention constraints read-only before asking for publication approval. No new registry service or credentials are requested. Build locally first. Registry publication is a separately recorded decision naming the reviewed input commit and locally verified candidate digest; the image build alone does not grant publication/deploy authority. After authorized publication, independently verify registry bytes/digest and reference that immutable digest in the pilot job. Never consume an unreviewed mutable tag.

Keep native packaging commands, target selection, provenance variables, mandatory tests, output archives and verification unchanged. The pilot removes per-job apt installation only where the image supplies exactly those tools. Preserve linker override and test `musl-gcc` discovery. Do not add cross/Docker/Chromium tools to this native image. A baseline cold pull and candidate cold pull are reported separately from warm image reuse. Do not evict a shared runner cache to manufacture a cold test; use an isolated local storage directory.

Maintenance proposal: build-engineer owns package/base security refresh. Before promotion and when updating any input, inspect package inventory, current scanner findings and changed dependency provenance. Start with manual reviewed refresh; proposing a scheduled rebuild is separate and cannot silently activate a timer. Retain the previous accepted digest during rollout; do not delete other registry artifacts. State the intended rollback-retention period in the Gate 1 plan after checking existing registry policy. Rollback restores the previous job image plus its apt installation together, preserving equivalent tool coverage.

## B. Distribution validation optimization

Proposed files: `tooling/rust/crates/agov/src/dist.rs`, its focused tests, and only if needed an independently reviewed command-call change in `.gitlab-ci.yml`. Keep this implementation and acceptance result separate from A.

Profile a pinned Git commit and fixed representative filesystem tree, documenting restored target/cache contents. Preferred first optimization: expand each unique exclusion/forbidden glob once per invocation and reuse the resulting membership set. Preserve recursive-prefix behaviour, path normalization, containment, symlink handling, optional patterns, invalid-pattern diagnostics and selected-file ordering. No persistent index or alternate authoritative database. Git remains source authority, but validation must still detect relevant untracked filesystem content; do not substitute git ls-files and accidentally remove that coverage.

Inspect duplicate validation: dist preview explicitly calls dist validate, then dist build internally validates again. Reuse already-validated in-process state where possible; removing a check is not assumed safe. Do not weaken distribution, projection lifecycle or exact-head provenance checks. A small source change can be validated independently before altering CI orchestration.

Required tests compare baseline/candidate selected files and errors for valid corpus, forbidden/untracked files, nested paths, symlinks, hidden files, malformed/optional patterns and duplicate/archive membership. Include the real manifest shape and a synthetic large irrelevant target tree to detect repeated traversal regressions. Document any intentional semantic correction separately for approval; it cannot hide inside a performance refactor. Standard Rust focused tests plus required full acceptance pipeline remain mandatory.

## Trial protocol and budget proposed for Gate 1

- One baseline and one candidate local comparison for each slice, same host and fixed inputs. Each comparison has a 30-minute wall-clock ceiling. Profile instrumentation separately; do not compare its wall-clock time with normal execution.
- Record image build, cold pull, warm startup, dependency install, cache restore, compile/validate, artifact work and total elapsed/runner-seconds separately. Queue wait is observed through natural CI, not summed into runner cost. Report package image-build/refresh cost as well as job savings.
- For A, calculate break-even successful jobs as build-and-refresh overhead divided by measured per-job saving when positive; report it as a scenario, not a guarantee. Stop rollout if there is no positive amortized saving or required tool/artifact parity fails.
- For B, require identical validated behaviour and a reproducible reduction in traversal/runtime on the representative large-tree fixture. If effects are noisy or absent, report inconclusive and stop expansion.
- Zero experiment-only paid pipelines, zero added runner capacity, no unchanged retries. Use the next already-required corrected MR acceptance run. If that cannot test the candidate adequately, return a concrete extra-run request with cost ceiling before running it. Local wall-clock limits do not imply zero resource cost.
- No pipeline cancellation policy change in this trial. Continue the standing compare-source/ref/SHA/required-job-coverage procedure, preserve canonical MR acceptance, and prevent duplicate runnable first-push paths. No CI-skip commit markers.

## Subsequent work and boundaries

After evidence: documentation builder with exact Python dependency locks and Git; smaller caches or same-head binary artifacts; earlier publication checks; runner scheduling. Keep these as separately reviewed slices. No self-clearing findings, hidden gate changes, release/tag/deploy, security-risk acceptance, or archive migration. Any archive product remains a separate release proposal.

## Independent consultation requested

Contribution review: challenge measurement fidelity, lifecycle/gate preservation, amortization, per-slice scope and false-green risks. InfoSec review: image input/provenance locks, package supply chain, registry publication boundary, secret-free build context, cache/artifact trust and rollback. Reviewer must not be this authoring session. Existing governed isolated-session fallback is available if no different LLM is available; do not represent same-session self-review as independent evidence. No binding decision is requested until those consultations and remaining registry/retention details are concrete.
