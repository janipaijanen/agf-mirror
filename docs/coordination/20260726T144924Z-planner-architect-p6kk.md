---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260726T144924Z-planner-architect-p6kk
timestamp: 2026-07-26T14:49:24Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T140427Z-governor-reviewer-bdq7
  - 20260726T122520Z-contribution-auditor-nr4g
topic: v2-4-5-published-artifact-first-install-guidance
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement the published-artifact-first installation ordering in the separately reviewed v2.4.5 public-human-documentation slice of the approved AI-tool-guidance-coherence plan.
    - Fact-check archive names, supported platforms, checksums, provenance files, OCI behavior, and version examples against the release inventory before Gate 2.
  must_read:
    - docs/plans/ai-tool-guidance-coherence.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/adopt/mcp-server.md
  do_not_touch:
    - agf/VERSION
    - CHANGELOG.md
    - agf/CHANGELOG.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Make published artifacts the normal downstream installation path

The governor directed that v2.4.5 correct the setup guidance at
`/docs/adopt/tooling-and-tokens/`: ordinary downstream adopters must not be
expected to fetch and compile the AGF source tree merely to run `agov`.

This requirement fits the already approved public-human-documentation slice of
`ai-tool-guidance-coherence`. That plan explicitly names this page and requires
it to distinguish human setup from AI operation. No separate Gate 1 plan is
needed if implementation stays within the bounded documentation projection and
receives its own exact-head Gate 2 review.

Use this order:

1. **Option A — Platform tooling release archive (recommended for local use).**
   Identify it as the `agov` tooling archive, not the framework source archive.
   Select the archive matching the adopter's OS, architecture, and adopted AGF
   version; verify the adjacent checksum and provenance metadata to the extent
   actually published; unpack it; place the executables on `PATH`; and verify
   the reported version. Do not imply that a checksum is a signature.
2. **Option B — Official OCI image (recommended for CI/container use).** Pin an
   exact release tag or digest, mount the adopter repository, and run `agov`
   against it. Explain that this is a container execution path, not a host
   installation recipe and not necessarily an installation path for the local
   stdio `agov-mcp` server.
3. **Option C — Build from source (not usually recommended downstream).** Keep
   this for contributors, debugging, unsupported platforms, or absence of a
   suitable published artifact. Require an exact release ref and a locked
   release build. Do not describe a full checkout as the ordinary recommended
   adopter path.

Suggested lead:

> Choose a published artifact unless you are contributing to AGF or no artifact
> exists for your platform. Ordinary downstream adopters should not need an AGF
> source checkout to run `agov`.

The explanation must preserve the authority boundary: a human or administrator
selects, verifies, installs, and pins the artifact; the AI uses the configured
CLI or MCP transport and must not silently choose an unpinned download or build
from source.

Before Gate 2, check cross-page coherence with `install-and-layouts.md`,
`mcp-server.md`, the README distribution paragraph, and release-artifact
references. Prefer cross-links over duplicated normative instructions. Do not
expand this slice into packaging implementation, new platform promises, signing
claims, or release mechanics.
