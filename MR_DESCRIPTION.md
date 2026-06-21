# docs: remove Elixir implementation references from public docs

## Summary

- Rewrites four public documentation pages to reflect that `agov` is the only supported CLI — a single static Rust binary, no Elixir fallback.
- Removes dual-implementation framing, Elixir Mix task references, and "(Rust)" / "(Elixir reference)" column qualifiers that implied a parallel implementation still existed.
- Verified clean: `grep -rni "elixir" agf/docs/site/docs/` produces no matches; `mkdocs build --strict` produces zero errors and zero warnings.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/adopt/tooling-and-tokens.md` | Removed dual-implementation framing, Elixir fallback chain, `mix agov.token.roi report`, "Why Rust" section. Single `agov` binary description; merged command table; simplified setup to three options (cargo build, release archive, CI). |
| `agf/docs/site/docs/advanced/contracts-and-validators.md` | Removed "(Rust)" / "(Elixir reference)" column qualifiers; replaced Elixir-image CI example with Rust binary installation snippet; removed "Mix remains the reference" framing. |
| `agf/docs/site/docs/operate/review-and-release.md` | Removed "release tooling still in Mix-backed reference implementation" note from AGF v2 layout admonition. |
| `agf/docs/site/docs/start/adopter-faq.md` | Simplified upstream contribution validation step to `agov validate` / `agov validate learnings` only. |

## Fast-track waiver

Documentation only. No routes, models, migrations, tests, or security boundaries affected. Waiver recorded in commit message.

## Gate 2 handoff

`docs/coordination/20260621T182550Z-planner-architect-4m57.md`
