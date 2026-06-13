---
kind: handoff
from: coder-tester
to: planner-architect
seq: 166
timestamp: 2026-06-13T12:32:43Z
immutable: true
references-to:
  - 00165-coder-tester.md
topic: binary-bundle-supply-chain-todos
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Add supply-chain requirements to the binary/tooling bundle design assessment.
    - Decide which items need their own Gate 1 implementation plan.
    - Treat license-risk blocking as CI policy, not only documentation.
  must_read:
    - docs/coordination/00165-coder-tester.md
    - agents-govern-2.0/pyproject.toml
    - agents-govern-2.0/LICENSE
    - agents-govern-2.0/LICENSE-MIT
    - agents-govern-2.0/docs/site/reference/release-info.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Addendum: provenance, SBOM, third-party licenses, and CI license blocking

The binary/tooling bundle design question in seq 165 must include explicit
supply-chain work. This is not polish; it affects whether adopters can trust,
redistribute, and decommission the tooling cleanly.

## TODOs for the Release Packaging Design

1. **Provenance**
   - Release artifacts should carry machine-readable provenance: version,
     source commit, build timestamp, CI pipeline/job identity when available,
     target OS/architecture/libc family, build toolchain versions, and checksum.
   - The public documentation release-info page should link to or describe the
     provenance format once it exists.

2. **Software Bill of Materials (SBOM)**
   - Each tooling binary/bundle should ship with an SBOM, preferably in a
     standard format such as SPDX or CycloneDX.
   - The SBOM should cover runtime dependencies and bundled artifacts, not only
     direct source dependencies.
   - CI should produce the SBOM as a release artifact and keep it beside the
     binary bundle.

3. **Third-party acknowledgements / kudos file**
   - Add a third-party notices file for distributed tooling bundles.
   - It should list dependency names, versions, upstream URLs, licenses, and
     short acknowledgements where appropriate.
   - Candidate filenames to assess: `THIRD_PARTY_NOTICES.md`,
     `THIRD_PARTY_LICENSES.md`, or `KUDOS.md`. My preference is
     `THIRD_PARTY_NOTICES.md` for legal clarity, with human-friendly thanks
     inside it rather than a separate informal file.

4. **Dangerous license detection**
   - CI should parse dependency licenses and block known-incompatible or
     policy-dangerous licenses before release artifacts are produced.
   - Minimum deny list to assess:
     - GPL-2.0-only / GPL-2.0-or-later;
     - GPL-3.0-only / GPL-3.0-or-later;
     - AGPL-3.0-only / AGPL-3.0-or-later;
     - legacy or ambiguous GPL/AGPL spellings that dependency tools emit.
   - The planner should assess whether LGPL, SSPL, BUSL, Commons Clause,
     PolyForm, source-available licenses, unknown licenses, and missing license
     metadata should be blocked, warned, or manually reviewed.

5. **CI policy**
   - License scanning should fail CI for blocked licenses.
   - Unknown or missing license metadata should at least warn; for release
     jobs, defaulting unknowns to block may be safer.
   - The policy should be documented as a release gate, with a deliberate
     negative test showing the gate fires.

## Design Constraint

Do not hide these requirements inside installer scripts. Installers may verify
checksums and print license/provenance links, but provenance, SBOM generation,
third-party notices, and license policy belong in CI and release artifacts.

## Requested Planner Output

Please fold these into the seq 165 response. If the answer becomes an
implementation proposal, split it into at least:

- binary/bundle artifact strategy;
- provenance + SBOM generation;
- third-party notices generation;
- license policy scanner and CI blocking gate.
