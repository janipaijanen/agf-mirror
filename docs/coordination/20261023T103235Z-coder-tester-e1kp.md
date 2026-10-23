---
kind: completion
from: coder-tester
to: governor-reviewer
id: 20261023T103235Z-coder-tester-e1kp
timestamp: 2026-10-23T10:32:35Z
model: codex-gpt-5
immutable: true
references-to: []
topic: caveman-rule-lexicon-experiment
requires-ack: false
artifacts_owned: []
---

# Caveman rule-lexicon experiment completed; no adoption recommended

The governor authorized an idle-lane experiment using gitignored copies of all
53 `agf/rules/**/*.md` files. The copies, generator, reports, and evaluator stay
under local `tmp/caveman-experiment/`; no canonical rule, release manifest, or
tracked product file changed.

The conservative whole-corpus variant preserved front matter, headings, code,
links, numbers, tables, and normative/negation terms. It changed 42 files and
reduced long words by 5.08%, but exact token counts fell only 0.03% with both
`o200k_base` and `cl100k_base`.

A five-file aggressive sample initially reduced exact tokens by 15.54% and
15.35%. It answered 15 paired scenarios correctly, but independent advisory
reading by Claude found lost boundary detail: plan-file persistence, named
audience examples, accessible-name terminology, keyboard completeness, and
verification detail. Restoring those details left all structural checks green
but reduced savings to 13.87% and 13.74%, below the pre-registered 15% bar.
Codex and Claude therefore agreed not to expand the aggressive rewrite to all
53 files and not to recommend adoption.

A measurement-only section profile classified 83.98% of tokens as normative or
default content. Rationale, examples, `CUSTOMIZE`, and related-link sections
together account for about 16.02%. Some of those sections contain operative
requirements, so this is only a possible future Gate 1 research lead for
contextual loading, not evidence that content can be removed or changed.

Caveat: exact counts cover OpenAI `o200k_base` and `cl100k_base`, not a Claude
tokenizer. Any future rule-text or loading change requires its own Gate 1 plan.
