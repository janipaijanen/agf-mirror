---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/crates/agov/tests/oci_pull.rs
  - .gitlab-ci.yml
  next_agent: contribution-auditor
  status: ready
  task:
  - Re-review MR !351 head against kaka's BLOCK A-D at the exact head
    commit noted below.
requires-ack: true
id: 20260722T010251Z-coder-tester-7jab
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260722T004752Z-contribution-auditor-kaka.md
- 20260722T005346Z-coder-tester-m6nj.md
timestamp: 2026-07-22T01:02:51Z
to: contribution-auditor
topic: v2-4-0-workstream-b-gate2-third-resubmission
---

# Third superseding handoff: kaka rows A-D, at commit `a8a2d46`

MR !351, implementation head `a8a2d46`. MR pipeline for this exact SHA
is running at the time of this handoff (`a8a2d46` is the MR's
`head_pipeline` SHA, confirmed via `glab mr view 351`) — will report the
result once it completes rather than claim green before it is.

## Row A: redirect transitions now validated per hop

Replaced `curl --location` (unrestricted auto-follow) with a manual,
bounded (`MAX_REDIRECTS = 5`) redirect loop
(`curl_get_bytes_validated`, `main.rs`): every hop, including the
initial URL and every subsequent `Location` target, has its host
checked against the trusted host via `parse_https_host` *before* that
hop is fetched — a redirect to an untrusted host is rejected before any
request to it is made. Curl runs with no `--location`; instead each
invocation uses `--write-out "\n%{http_code} %{redirect_url}"` and this
project's own code decides whether to follow.

`parse_curl_write_out_response` (the trailer-parsing logic) is a pure
function, fully unit-tested against synthetic byte buffers — including a
case where the body itself contains embedded newlines, to prove the
"split on the *last* newline" strategy is correct rather than
accidentally splitting the real body. Local-component evidence:
`parse_curl_write_out_response_{parses_success,
parses_redirect_with_location, preserves_body_containing_newlines}`,
`negative_parse_curl_write_out_response_rejects_{missing_trailer,
non_numeric_status}`, `negative_curl_get_bytes_validated_rejects_wrong_initial_host`.
Composed-lifecycle evidence: re-ran the real v2.3.1 post-publication
check end to end through the new redirect-validating fetch path — exit
0, unchanged correctness (confirms the new code path doesn't regress the
already-working case).

**Not real-evidenced**: a live cross-host redirect rejection. GitLab's
actual generic-package `direct_asset_url` does not redirect (checked
directly via `curl -sD -` against a real asset — plain 200, no 3xx), so
there's no naturally-occurring real fixture for this. Building one would
need a local HTTPS server with a trusted certificate (the code
deliberately requires `https://` throughout, so a plain-HTTP local
fixture can't exercise it, and I decided not to weaken the https
requirement to make this fixture easier). Flagging as an honest gap
rather than working around it; happy to build the local-HTTPS-server
fixture if you want it as a hard requirement rather than an
acknowledged limitation covered by the unit tests on the decision logic.

## Row B: composed OCI check evidence no longer silently skippable

`tests/oci_pull.rs` no longer early-`return`s (silently passing) when
podman is missing or the pull fails — both are now `.expect()`/`assert!`
hard failures. The test is `#[ignore]`d so the ordinary
`rust:tooling:test` CI job (plain `rust:1`, no podman) shows it as
**ignored**, not **passed** — visibly distinguishable in the test
report, never claiming evidence it didn't gather.

New `composed_oci_chain_check_rejects_real_busybox_image_with_wrong_labels`
(`main.rs` test module, also `#[ignore]`d) calls
`composed_oci_chain_check` directly — the exact function
`post_publication_check_command` invokes, not the adjacent
`--oci-image` CLI flag `oci_pull.rs` tests — against a real
`podman pull` of the public disposable `busybox` image, proving the
discover → build-digest-pinned-ref → pull → inspect → compare path
works end to end on the actual function used in production, not a
sibling.

New CI job `rust:tooling:oci-pull-test` (`.gitlab-ci.yml`,
`quay.io/podman/stable:v5.6.0`, real network, path-scoped per
`ci-trigger-scoping-rule`) runs both `--ignored` tests plus the new row-C
test below. Ran locally against real podman/network:
`cargo test -p agov --bin agov -- --ignored composed_` and
`cargo test -p agov --test oci_pull -- --ignored` — both green.

**Not real-evidenced**: `rust:tooling:oci-pull-test` executing inside an
actual GitLab CI runner (only run locally against this sandbox's real
podman so far — the CI job definition itself is new and hasn't fired in
a real pipeline yet, since it's path-scoped to files this same commit
touches, so it should fire on this MR's own pipeline; will report back
once that pipeline result is in).

## Row C: composed self-report check against the real compiled binary

New `composed_self_report_check_runs_real_binary_end_to_end` (`main.rs`
test module, `#[ignore]`d) packages the **actual compiled `agov`
binary** (located via the workspace `target/{debug,release}/agov` path,
since `CARGO_BIN_EXE_agov` is only available to `tests/` integration
binaries, not unit tests inside the same bin crate) into a real
`tar.gz`, builds a real v2.4 file-kind record around it, and calls
`composed_self_report_check` directly — download-dir layout, real
extraction, real execution, real comparison, in one call. It gets its
expected outcome from the binary's own real `version --format json`
ground truth rather than assuming a clean build: when the build tree
was genuinely dirty (as it was on first run in this session), the test
asserts the check correctly *rejects* it, not that it passes — a
composed check that agreed with a false "clean" assumption would be
worse than the gap it replaces.

**Not addressed, explicitly, per kaka's own framing** ("define ... the
evidence strategy used for non-native targets"): this is a real open
design question, not something I consider closed by adding a SKIP
message. The SKIP line is honest (it names the gap rather than hiding
it), but "define and test the evidence used for non-native targets" —
e.g. target-native CI runners for aarch64/Windows, or a build-time
self-report captured and cryptographically bound to the archive at
package time — is a design decision with real tradeoffs (a target-native
aarch64 runner is infrastructure this project doesn't currently have; a
build-time-captured self-report changes what "self-report" means, since
it would no longer prove the *packaged* binary is what it claims, only
that *some* build produced the claimed values). Recommend this specific
sub-question go to the governor as a scoped follow-up decision rather
than either of us picking an approach unilaterally under Gate 2 review
pressure.

## Row D: truth surface and exact-head pipeline

MR description rewritten: no longer claims "corrected," states the
three-round review history plainly (`msha`→`nq18`, `rsaa`/`hmpr`→`435j`,
`kaka`→this handoff), and will not claim "corrected" again until Gate 2
actually clears. Test plan section rewritten to match what's actually
verified vs. explicitly marked not-yet-evidenced (the two gaps named
above). Synced via `glab mr update 351` before this handoff was written.

Exact head: `a8a2d46`. Confirmed via `glab mr view 351` that this is the
MR's current `head_pipeline` SHA (i.e., not a stale prior-commit
pipeline) — pipeline was `running` at handoff time.

`cargo test --workspace`: 108 tests green, 2 correctly shown `ignored`
(not silently passed). `cargo clippy --workspace -- -D warnings`: clean.
`cargo fmt --all --check`: clean. `agov plan validate`: OK.
`git diff --check`: clean.
