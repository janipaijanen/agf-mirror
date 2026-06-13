---
kind: note
from: planner-architect
to: governor-reviewer
seq: 275
timestamp: 2026-06-14T13:00:00+03:00
immutable: true
references-to:
  - 00165-coder-tester.md
  - 00166-coder-tester.md
topic: binary-bundle-supply-chain-design-positions
requires-ack: false
artifacts_owned: []
---

PA positions on seq 165 (binary/bundle strategy) and seq 166 (supply-chain
requirements). Combined because seq 166 is an addendum to seq 165.

---

## On seq 165 — binary tooling release bundles

### Platform targets: recommend a narrow initial scope

The Codex assessment lists seven platform targets. I recommend starting with
three and deferring the rest until adoption evidence exists:

| Target | Recommendation | Reason |
|---|---|---|
| Linux x86_64 glibc | **Include** | Primary CI runner environment across GitHub Actions and GitLab CI |
| macOS aarch64 | **Include** | Developer laptop default since Apple Silicon; needed for local validation |
| macOS x86_64 | **Defer** | Rosetta 2 covers the arm64 binary on x86 Macs acceptably for now |
| Linux x86_64 musl / static | **Defer** | Meaningful only if adopters hit glibc version mismatches; no evidence yet |
| Linux aarch64 | **Defer** | GitHub Actions arm64 runners exist but add cost; wait for demand |
| Windows x86_64 | **Defer** | High maintenance cost, untested CI toolchain; no adopter demand yet |
| BSD | **Exclude until testable** | No CI runner available; cannot verify what we cannot test |

Claim: shipping untested platform binaries is a supply-chain risk, not a
convenience. A binary we cannot run in CI is a binary we cannot trust.

### Bundle strategy: separate content and tooling, no vendoring

Agree with Codex's split position, with one addition:

- **Framework bundle** (`agf-X.Y.Z.tar.gz` / `.zip`): pure content, as today.
  Rules, schemas, docs, templates, examples. Human-readable. Vendorable by
  copy into adopter repo. No binary blobs.
- **Tooling bundles** (per-platform, separate release artifacts): binary
  tooling for adopters who want CI gates without an Elixir dev environment.
  Named `agf-tools-X.Y.Z-linux-x86_64.tar.gz` etc. Separate download,
  separate checksum file.
- **No vendoring of tooling binaries into adopter repos.** The exit-and-
  decommission story depends on adoption being reversible by deleting
  framework files. A binary in the adopter repo complicates that.

The Python `agov` CLI is already distribution-ready via pip/uv and needs no
binary packaging — pure Python is cross-platform without compilation.

### Installer scripts: convenience wrapper only, not the trust path

The trust path is:

1. Download platform artifact.
2. Verify checksum against the release's `SHA256SUMS` file.
3. Optionally verify the `SHA256SUMS` file against a signing key (if we add
   release signing in a future slice).
4. Place on `PATH`.
5. Remove independently at decommission time.

Convenience installer scripts (`install.sh`, optionally `install.ps1`) are
acceptable if they:
- Download from a versioned URL (no "latest" shortcuts).
- Verify the checksum before installing.
- Are themselves versioned and checksum-verifiable.
- Do NOT execute the binary or configure adopter systems beyond PATH placement.

Gate the convenience installers behind a future slice; they must not block
the primary distribution path.

### v2.0 honest scope

The current `bin/agf` wrapper delegates to `mix agov.*`. That is the correct
v2.0 reality. Binary packaging of the Elixir tooling is a v2.1+ feature;
claiming it exists in v2.0 documentation would violate the implementation-
honesty rule. The public docs install page is correct to say Elixir and Mix
are currently required.

---

## On seq 166 — supply-chain requirements

### Provenance

For each tooling binary artifact, include a `BUILD_INFO.json` file beside
the binary:

```json
{
  "version": "2.0.0",
  "source_commit": "abc1234...",
  "build_timestamp": "2026-06-14T12:00:00Z",
  "ci_pipeline": "<pipeline-id-or-unknown>",
  "target_os": "linux",
  "target_arch": "x86_64",
  "libc": "glibc",
  "build_tool": "mix/burrito@x.y.z"
}
```

For the Python `agov` package: PyPI metadata + `agov --version` is sufficient.
No additional artifact needed.

### SBOM

Use SPDX format (not CycloneDX). Reasons:
- SPDX is an ISO standard (ISO/IEC 5962:2021); CycloneDX is OWASP-hosted.
- SPDX is closer to what legal reviewers and supply-chain tools expect.
- The current Python deps (pyyaml, graphviz) are MIT; the SBOM would be
  short but the format needs to be right for future additions.

Generate at release time from the locked dependency tree. Place `agf-tools-X.Y.Z-<target>.spdx.json` beside the binary bundle.

For v2.0: since there are no binary bundles yet, the SBOM requirement applies
when binary packaging ships. For the Python package: integrate with
`pyproject.toml` metadata and add to the release workflow.

### Third-party notices

File name: `THIRD_PARTY_NOTICES.md`. Agree with Codex's reasoning over
`KUDOS.md` — legal clarity matters more than warmth when distributing
tooling.

The current Python tooling deps are clean and minimal:
- **pyyaml** — MIT. Safe to redistribute, no copyleft propagation.
- **graphviz** (Python binding) — MIT. The Graphviz executable itself is
  EPL-1.0, which is copyleft for modifications but not for use in tooling
  that merely invokes it. Needs explicit mention.

Third-party notices should list: name, version, upstream URL, SPDX identifier,
and brief acknowledgement. Keep format machine-parseable (YAML or TOML front-
matter) for downstream automated review.

This is fast-track eligible for the Python package today. For Elixir/Mix
tooling: Elixir dependencies (hex packages) should be audited before any
binary bundle ships.

### License policy scanner

Proposed blocked licenses for CI:

```
GPL-2.0-only, GPL-2.0-or-later, GPL-2.0+
GPL-3.0-only, GPL-3.0-or-later, GPL-3.0+
AGPL-1.0, AGPL-3.0-only, AGPL-3.0-or-later, AGPL-3.0+
```

Requires human review (warn, don't block):
- **LGPL-2.0, LGPL-2.1, LGPL-3.0**: Linking to LGPL deps in a binary
  release may require offering object files; assess per target.
- **SSPL, BUSL, Commons Clause, PolyForm**: Source-available, not open
  source; commercial use restrictions may apply depending on adopter context.
- **Unknown / missing license metadata**: Block by default for release jobs;
  warn-only for dev builds.

Rationale for blocking GPL/AGPL: the tooling is MIT-licensed; distributing
a binary that statically links GPL code creates a copyleft obligation on the
whole binary. That conflicts with the MIT license on the tooling code.

LGPL is more nuanced: LGPL allows linking without copyleft propagation if
the library is dynamically linked and the user can substitute it. For Elixir/
BEAM OTP releases (which typically bundle deps), this needs case-by-case
review. Block LGPL for the static/bundled case; warn for the dynamic case.

CI policy: the license scanner gate must pass before any release artifact is
signed or published. Per gate-self-test-rule, the CI job must include a
deliberate negative test showing the gate fires on a known-blocked license
in the dep tree.

---

## Recommended implementation splits

These are Gate 1 plan candidates, in priority order:

1. **Third-party notices for Python tooling** — Fast-track. Add
   `THIRD_PARTY_NOTICES.md` to the Python package; include in `agf-dist`
   manifest. Low risk, high auditability gain. No binary prerequisite.

2. **License scanner CI gate** — Gate 1 plan. Define the block/warn/manual
   lists; implement as a CI step that runs before `agov-dist` produces
   archives. Include negative test (introduce a GPL-licensed dep to a test
   fixture; confirm gate fires and blocks release).

3. **Provenance BUILD_INFO** — Gate 1 plan. Design the JSON schema; implement
   as a CI step producing the file alongside the dist archive. Starts simple
   (version + commit + timestamp), extends to full tooling inventory when
   binary bundles ship.

4. **SBOM generation** — Gate 1 plan. Depends on finalized dep tree. Introduce
   `spdx-tools` or equivalent; integrate with release workflow. Gate-test:
   SBOM missing = release job fails.

5. **Binary tooling artifacts** — Gate 1 plan (deferred to v2.1+). Design
   the platform target matrix, the naming convention, the checksum/signing
   scheme, and the Elixir/Burrito build configuration. Do not ship until we
   have at least two adopters to validate the real-world platform matrix.

Items 1–2 can proceed in parallel immediately. Items 3–4 follow. Item 5 is
post-adoption-evidence.

---

## What needs a governor decision

No blocking decisions before items 1–2 can begin. The governor should confirm:

A. Is the binary target scope (Linux x86_64 + macOS aarch64 for v2.1+) acceptable,
   or should Windows or musl be included sooner?

B. Should LGPL block binary releases by default (safer, may exclude useful deps)
   or warn by default (more permissive, requires manual review cadence)?

C. Is `THIRD_PARTY_NOTICES.md` the agreed filename, or does the governor
   prefer a different convention?

These are confirmations, not blockers for items 1–2 (which use only MIT deps
and have no LGPL ambiguity).
