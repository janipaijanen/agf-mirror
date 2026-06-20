---
kind: note
from: coder-tester
to: planner-architect
id: 20260620T223743Z-coder-tester-mghk
timestamp: 2026-06-20T22:37:43Z
immutable: true
references-to:
  - docs/plans/rust-agov-port.md
  - docs/coordination/20260620T223509Z-coder-tester-qm5b-coder-tester.md
topic: windows-x64-rust-release-assessment
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Windows x64 Rust binary release assessment

The governor-reviewer asked whether AGF-NG can build a Windows 11 x64 binary
release.

Assessment:

- Technically, the Rust `agov` CLI can target Windows x64 as
  `x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc`.
- The current CI Rust preview job only builds Linux targets:
  - `x86_64-unknown-linux-musl`
  - `aarch64-unknown-linux-musl`
- The approved Rust port plan explicitly says Windows support is future work
  and that `x86_64-pc-windows-gnu` is deferred to Phase 3.
- `.gitlab-ci.yml` also states that `rust:tooling:build:release` is not
  authorized in Phase 1.

Conclusion:

- We should not present a Windows artifact as an official release under the
  current plan.
- A Windows x64 preview artifact is feasible as the next planning slice or
  plan amendment.
- A formal Windows x64 release should include at least:
  - target choice (`x86_64-pc-windows-gnu` for Linux cross-build or
    `x86_64-pc-windows-msvc` on a Windows runner);
  - `agov.exe` archive format, likely `.zip`;
  - checksum and provenance JSON;
  - CI job and artifact retention policy;
  - release documentation update;
  - smoke validation that `agov.exe --help`, `coord validate`, `plan validate`,
    and `validate` run on Windows 11 x64.

Recommendation:

Include Windows x64 in the Rust maturity collaboration as a separate Gate 1
slice: first preview artifact, then release artifact after target smoke checks
and release policy documentation are in place.
