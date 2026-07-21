---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260721T133538Z-governor-reviewer-bm5t
timestamp: 2026-07-21T13:35:38Z
model: codex-gpt-5
immutable: true
references-to:
  - docs/plans/v231-recovery-release.md
topic: release-asset-consumer-naming
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Critically assess replacing Rust target triples in consumer-facing release asset filenames with clear platform slugs.
    - Recommend whether the change belongs in v2.3.1 and identify any compatibility or deprecation path required for existing package URLs.
  must_read:
    - docs/plans/rust-agov-port.md
    - agf/docs/site/reference/release-artifacts.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Review consumer-facing release asset names

The governor-reviewer finds `unknown` unsuitable in public release filenames.
Although `unknown` is the technically correct vendor component of Rust targets
such as `x86_64-unknown-linux-musl`, non-Rust consumers can reasonably read it
as uncertainty or an incomplete deliverable.

Critically assess using the earlier consumer-facing slugs recorded in
`docs/plans/rust-agov-port.md`:

- `agov-<version>-linux-x86_64.tar.gz`
- `agov-<version>-linux-aarch64.tar.gz`

Their checksum and provenance sidecars should use the same public basename.
Retain exact canonical Rust target triples in build configuration and inside
provenance metadata where that technical precision belongs.

Before changing MR !317, report:

1. every affected contract and consumer surface, including documentation,
   install commands, package URLs, provenance subjects, CI, and tests;
2. whether changing names in v2.3.1 is within the approved recovery scope;
3. whether old-name aliases or an explicit deprecation period are required.

The GitLab notes on MR !317 are the visible bridge for this record. They do not
authorize silently expanding !317; the governor-reviewer retains the scope and
compatibility decision.
