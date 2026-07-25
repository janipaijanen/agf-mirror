---
kind: handoff
from: governor-reviewer
to: fact-checker
id: 20260722T232217Z-governor-reviewer-n65h
timestamp: 2026-07-22T23:22:17Z
model: human-jani
immutable: true
references-to:
  - 20260722T232217Z-governor-reviewer-7s22
topic: agov-command-map-subject-matter-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - audit the proposed agov command-map concept for command and output accuracy
    - distinguish released capability from repository-only or planned capability
    - route findings to contribution-auditor for the marketing and pedagogy synthesis
  must_read:
    - agf/docs/contracts/tooling-conformance.md
    - agf/docs/site/reference/tooling-conformance.md
    - agf/docs/site/reference/maturity-vocabulary.md
    - agf/docs/site/docs/adopt/mcp-server.md
  do_not_touch:
    - agf/docs/site/
    - tooling/rust/
---

# Subject-matter review: `agov` command-map accuracy

Review the command-map proposal in `7s22` from the `agov` subject-matter and
fact-checking perspective. The eventual map must not advertise a command,
option, output, MCP surface, or automation level that is absent from the
supported release path.

Check that a prototype or recommendation would:

1. derive its command inventory from the supported Rust CLI and the versioned
   tooling-conformance contract rather than legacy Elixir or development-only
   sources;
2. distinguish command availability from capability maturity and evidence
   status;
3. describe output shapes and side effects accurately, including read-only
   commands such as session resume versus commands that write coordination;
4. preserve the tool-authority boundary: validators and summaries do not
   approve gates or replace the human decision-maker;
5. distinguish the `agov` CLI from the optional `agov-mcp` transport without
   implying feature parity where none has been verified; and
6. include versioned provenance sufficient for a later reader to trace every
   command and claim back to its release contract or generated source.

Return corrections, missing command families, dangerous simplifications, and
a minimum accuracy checklist to the contribution-auditor. Do not implement
the map. This consultation is post-v2.4.0 and must not interrupt the Class 4
and Class 5 release chain.
