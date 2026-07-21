---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260721T091004Z-coder-tester-sj5g
timestamp: 2026-07-21T09:10:04Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T080233Z-governor-reviewer-qnbt
topic: post-v2-3-unified-release-provenance-schema
requires-ack: true
artifacts_owned:
  - agf/schemas
  - agf/docs/site/reference/release-artifacts.md
  - tooling/rust/crates/agov/src/dist.rs
  - .gitlab-ci.yml
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - prepare a post-v2.3 Gate 1 plan for a unified release-provenance contract and validator
  must_read:
    - agf/docs/site/reference/release-artifacts.md
    - docs/plans/release-pages-artifacts-policy.md
  do_not_touch:
    - agf/VERSION
    - tooling/rust/Cargo.toml
    - tooling/rust/Cargo.lock
---

# Future TODO: unified release-provenance contract and validation

After v2.3.0 completes, prepare a Gate 1 plan for one explicit provenance
contract shared by framework source archives, Rust tooling archives, and OCI
release evidence. This is not v2.3.0 scope and must not delay the current
release.

## Current gap

Framework provenance is relatively self-describing: it includes version,
source commit, tag, build timestamp, pipeline/job identity, manifest path,
archive filenames, checksums, implementation, and packaging. Rust tooling
provenance contains version/build/target fields but does not embed its archive
filename or checksum, relying on an adjacent `.sha256` file and filename
convention. OCI evidence uses labels, digest, binary output, and scan evidence
without a single shared machine-validated contract.

There is no JSON Schema or one validator that proves these surfaces agree.

## Planning requirements

Compare and propose the smallest compatible contract with:

- a common required core: schema version, product/artifact kind, version,
  source commit, tag, build timestamp, implementation, and packaging;
- build identity limited to public/reproducible pipeline and job identifiers
  when available;
- artifact-specific extensions for source archives, target binaries, and OCI;
- archive filename, size, and SHA-256 linkage inside each provenance record;
- target platform for tooling artifacts;
- OCI digest and required OCI-label agreement;
- executable self-report agreement (`version`, `commit`, `dirty: false`);
- deterministic validation of tag/version/commit/checksum relationships;
- backward-compatible handling of existing v2 provenance records; and
- tests for missing, mismatched, dangling, duplicated, and malformed evidence.

Assess whether validation belongs under `agov dist validate`, a new
`agov release validate-provenance` command, or both at different lifecycle
stages. Keep build-time generation separate from post-publication verification:
the latter must validate artifacts downloaded from the public release surface.

## Hard privacy boundary

The provenance contract MUST NOT persist client, assistant, model-session,
conversation, correlation, or request identifiers; prompts or responses; raw
MCP/JSON-RPC envelopes; local usernames/home paths; host fingerprints; secrets;
or raw environment dumps. Use a controlled allowlist, not an open metadata map.

## Done condition

This TODO is complete only after an approved plan, implementation MR, schema
and negative tests, documentation, and a real release-path validation prove the
generated provenance is self-consistent. A schema file alone is not completion.
