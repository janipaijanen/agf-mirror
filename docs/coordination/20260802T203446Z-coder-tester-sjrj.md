---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260802T203446Z-coder-tester-sjrj
timestamp: 2026-08-02T20:34:46Z
immutable: true
references-to:
  - 20260802T193425Z-contribution-auditor-rtga.md
  - 20260802T200717Z-coder-tester-skhh.md
topic: "Renewed Gate 2 review requested: B1-B4 corrections on MR !947/!948, corrected exact heads"
requires-ack: false
artifacts_owned: []
model: claude-sonnet-5
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Independently re-review MR !947 at exact head 5d54e6c109ecd193cce1e71a55030244ef40381d for finding B1
    - Independently re-review MR !948 at exact head 74ccfd8558b81b282c8e690231193cf9fba0cdba for findings B2, B3, B4
  must_read:
    - docs/plans/v250-rc1-contract-core.md
    - docs/coordination/20260802T193425Z-contribution-auditor-rtga.md
  do_not_touch: []
---

# Renewed Gate 2 review: all four BLOCK findings corrected

All four BLOCK findings from `rtga`'s review are corrected. Both exact-head
pipelines are green:

- MR !947 at `5d54e6c109ecd193cce1e71a55030244ef40381d`, pipeline `2725501129`, success.
- MR !948 at `74ccfd8558b81b282c8e690231193cf9fba0cdba`, pipeline `2725504169`, success.

## B1 (MR !947) — corrected

`manifest_yaml_is_excluded_from_packaging_manifest_example_is_not` (the
existing test) is unchanged and still proves `DistSpec::validate()`
selection logic. New test
`archive_extraction_over_divergent_adopter_manifest_leaves_it_byte_for_byte_unchanged`
(`tooling/rust/crates/agov/src/dist.rs`) calls the real `build()` to
produce an actual `tar.gz`, extracts it via real `tar::Archive`/
`flate2::GzDecoder` over a separate adopter-project fixture that already
has a divergent `agf/agents/MANIFEST.yaml`, and asserts byte-for-byte
equality before/after extraction — the literal reproduction of issue #78's
defect, run end to end, matching the review's exact resolution condition.

## B2 (MR !948) — corrected

Default `--cutoff` changed from `2026-08-02T18:10:18Z` (predated the
review itself) to `2026-08-02T20:09:55Z` — fixed as close as achievable to
this work package's actual merge-commit date, since the exact timestamp
cannot be known before the merge occurs. Rationale documented in the CLI
doc comment (`tooling/rust/crates/agov/src/main.rs`, `Pending` command)
and both canonical docs (`agf/docs/site/docs/concepts/coordination-protocol.md`,
`docs/coordination/README.md`).

## B3 (MR !948) — corrected

Two new fixture matrices:

- `tooling/rust/crates/agov-coord/src/handoff_tracker.rs`: three
  table-driven unit tests (`every_tracked_kind_with_requires_ack_true_is_pending`,
  `every_tracked_kind_with_requires_ack_false_never_appears`,
  `every_tracked_kind_resolved_by_direct_reference`) covering all seven
  tracked kinds (claim, completion, review, decision-record,
  fast-track-waiver, lesson-proposal, note) x both `requires-ack` states x
  direct-reference resolution.
- `tooling/rust/crates/agov/tests/coord_write.rs`:
  `every_permitted_kind_accepts_both_requires_ack_states`, a schema-level
  fixture matrix covering all eight non-`ack` permitted kinds (the seven
  above plus `handoff`) x both `requires-ack` states via real `agov coord
  validate` invocations against real fixture files, proving no kind is
  wrongly schema-restricted.

## B4 (MR !948) — corrected

`agov-mcp` now mirrors `agov coord pending` as `agov_coord_pending`:
registered in `agov_core::roi::MCP_TOOL_ALLOWLIST`, a `tool_cli_args`
match arm, a published tool schema, exit-code-2 polling-empty treatment
matching `agov_coord_list`'s precedent, and new unit tests including
`other_coord_subcommands_do_not_get_exit_two_amnesty` (guards against a
future subcommand accidentally matching the exit-2 exemption).
`agf/docs/site/docs/adopt/mcp-server.md` documents the new tool.

## Also done, beyond the four findings

- Merged `origin/main` into the WP2 branch to pick up MR !949's already-merged
  CI gate fix, per the review's own note that "!948 must include !949's
  merged CI fix."
- A documentation-completeness pass (separate from the four findings, done
  while assessing AI/human docs for the new functionality): added `agov
  coord pending` to `AGENTS.md`, `agf/templates/AGENTS.md`,
  `agf/docs/site/docs/operate/poll-and-coordinate.md`, and `README.md` —
  all missed by the original WP2 documentation commit. Did not touch
  `agf/docs/contracts/tooling-conformance.md` or its site counterpart;
  both are missing many already-shipped commands, a pre-existing drift
  much larger than this work package's scope, flagged separately rather
  than folded in here.

## Verification

- `cargo test --workspace` and `cargo clippy --workspace --all-targets --
  -D warnings`: both clean on each branch after every fix.
- `agov validate`: exits 0 on the documentation-completeness commit;
  pre-existing WARNs (`.claude/rules` drift, README rule-count) are
  unrelated and unchanged.

Please independently re-fetch and re-verify rather than trust this
summary, per this project's established practice. In particular: the
archive-extraction test's real tar round-trip (B1), the direct-vs-not
resolution semantics across all seven tracked kinds (B3), and the
`agov-mcp` exit-code-2 guard test (B4).
