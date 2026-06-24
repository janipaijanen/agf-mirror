# feat: expand contributions validator + add readme_stats validator

Branch: `feat/validate-contributions-readme-stats`

## Summary

Two validator improvements, both advisory (never blocking):

**1 — Expanded `contributions.rs`**
The previous implementation had a 2-pattern stub. This replaces it with
full adversarial pattern detection across three classes:

- **Absence-of-incident bias** (14 patterns) — "no issues", "worked fine",
  "without any problems", etc. Flags claims that cite absence of evidence
  as evidence of safety.
- **Governance relaxation** (6 patterns) — "skip the gate review",
  "unnecessary overhead", "bypass validation", etc. Flags proposals to
  weaken governance without explicit evidence.
- **Weak universal claims** (6 patterns) — "all projects should", "always
  works", "universally applicable", etc. Flags claims that require
  multi-project corroboration.

Uses a contributions-specific freetext extractor covering 8 YAML keys
(`description`, `text`, `summary`, `recommendation`, `notes`, `title`,
`customization_notes`, `conclusion`) — broader than the shared extractor
which covers only 4.

**2 — New `readme_stats.rs`**
Validates numeric claims in `README.md` against actual file counts:

| Claim | Counted by |
|---|---|
| `N governance rules` | `.md` files under `agf/rules/` (recursive, excl. `README.md`) |
| `N agent definitions` | `.yaml` files in `agf/agents/` (excl. `MANIFEST.yaml`) |
| `N skills` | Subdirectories in `agf/skills/` containing `SKILL.md` |

A mismatch or missing claim produces an advisory WARN. Always exit 0.

Added the matching stats line to `README.md` (44 rules, 12 agents, 7 skills).

**3 — Docs + gitignore cleanup**
- `contracts-and-validators.md` table was missing `adoption`, `contributions`,
  `framework-config`, and `readme-stats`. All four added.
- `.gitignore` collapsed three individual Elixir artifact entries to a single
  `tooling/elixir/` entry, suppressing the untracked directory that was
  causing `cargo build` to mark local binaries dirty.

## Files changed

| File | Change |
|---|---|
| `tooling/rust/crates/agov-validators/src/contributions.rs` | Expanded: 26 patterns, contributions-specific freetext, 9 unit tests |
| `tooling/rust/crates/agov-validators/src/readme_stats.rs` | New — README stat consistency validator, 5 unit tests |
| `tooling/rust/crates/agov-validators/src/lib.rs` | Added `pub mod readme_stats` |
| `tooling/rust/crates/agov/src/main.rs` | Wired `ReadmeStats` variant + `render_readme_stats` helper |
| `README.md` | Added stats claim line (44 rules, 12 agents, 7 skills) |
| `agf/docs/site/docs/advanced/contracts-and-validators.md` | Updated validator table: 7 subcommands now documented |
| `.gitignore` | Collapsed Elixir entries to `tooling/elixir/` |

## Test evidence

```
$ cargo test -p agov-validators 2>&1 | tail -5
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered-out
```

Negative cases per class:
- `absence_clean_on_neutral_text` — incident-reporting language does not fire
- `relaxation_clean_on_neutral_text` — governance-strengthening text does not fire
- `universal_clean_on_hedged_text` — hedged language does not fire
- `warns_when_rules_stale` — stale README count produces finding
- `warns_when_claim_missing` — missing stat produces three "not found" findings
- `manifest_excluded_from_agent_count` — MANIFEST.yaml not counted

## Smoke test on the foundry repo

```
$ agov validate contributions
OK agov.validate.contributions

$ agov validate readme-stats
OK agov.validate.readme_stats
```

Both pass clean on the foundry repo.

## Review checklist

- [ ] Pattern lists cover the three target classes without excessive false positives
- [ ] `readme_stats` tolerances are set to 0 (exact match required)
- [ ] MANIFEST.yaml and README.md correctly excluded from their respective counts
- [ ] All 34 tests pass; negative cases fire as expected
- [ ] docs table lists all 7 subcommands accurately
- [ ] No `Co-authored-by` trailers in commits
