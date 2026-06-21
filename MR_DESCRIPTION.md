# docs: language polish — idiom replacements, adopt nav reorder, filename guidance (Slice 4)

Governor-approved remediation slice 4.
Plan: `docs/plans/agf-docs-language-polish.md`

## What changed

Four idiom replacements (exact substitutions):
- `should-you-adopt.md`: "are theatre" → "become ceremony"
- `adopter-faq.md`: "gives your review a spine" → "gives your review structure"
- `first-governed-change.md`: "chain is the product" → "chain is the audit trail"
- `downstream-impact.md`: "are theatre" → "become ceremony"

Adopt nav reorder (`mkdocs.yml` and `docs/index.md`):
- Downstream impact and Exit now appear before Install, so readers see
  cost and reversibility before they commit.

Coordination filename guidance (`concepts/roles-and-gates.md`):
- Readable IDs (`YYYYMMDDThhmmssZ-role-slug-suffix`) documented as the
  current preferred convention; earlier sequence and ULID formats
  acknowledged as historical but still valid.

## Verification

- `mkdocs build --strict`: clean (no broken links — nav reorder keeps all files
  in place, only changes ordering).

## Review Focus

Please verify:

- [ ] The idiom replacements read more professional without changing meaning.
- [ ] Adopt navigation puts impact and exit information before installation.
- [ ] Readable coordination IDs are presented as the current preferred format.
- [ ] Legacy sequence and ULID records are described as historical but still valid.

## Does not touch

`agf/docs/site/stylesheets/agf.css` and all other non-targeted files.
