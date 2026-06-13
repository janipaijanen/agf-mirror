---
provenance: inherited-v1x-unverified
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# File Generator: Lowest-Common-Denominator Output for Heterogeneous Viewers
#
# Framework principle: "the running consumer is the source of truth, not
# the format spec" — verify against actual consumer behavior, not against
# documentation
# See: framework.md — Pattern 4 (Tester Discipline);
#      always/environment-interrogation-rule.md (sibling pattern for
#      consuming heterogeneous environments)

> **Provisional status.** Single-adopter operational evidence
> (blue-marlin, 2026-04-26, framework v0.34.0 — issue #41) plus
> parallel observation from external AI review (Copilot,
> 2026-04-26 sentinel review). The blue-marlin evidence is from
> DXF for 3D-CAD viewers; the principle is well-known in
> document-exchange practice (PDF/A, JSON-LD profiles, MISRA-C
> subsets), but operational evidence inside the framework's
> learning corpus is single-adopter at the time of shipping.
> Adopters generating files for heterogeneous viewer ecosystems
> (any format: SVG, PDF, CSV, JSON-LD, OOXML, OpenDocument, GLB,
> STEP, etc.): please file a learning record so we can promote
> from provisional to recommended.

**Scope:** Any task that generates files consumed by tools the
project does not control — drawings (DXF, SVG, PDF), 3D models
(STEP, GLB, OBJ), spreadsheets (CSV, XLSX, ODS), structured data
(JSON-LD, RDF), document exchange (PDF/A, OOXML, ODF), captions
(SRT, VTT), or any other format with a published spec and multiple
independent renderers / parsers.

## The principle

**Lowest-common-denominator output.** When generating files for
unknown or heterogeneous consumer ecosystems, prefer the most
universally-rendered primitives over advanced format features —
even at the cost of more verbose code or larger files.

The format spec is *not* the source of truth for what consumers
actually render. Real consumers implement subsets, ignore optional
features, fall back silently, and disagree about marginal cases.
A generator that uses an "advanced" feature is betting that every
consumer in the target ecosystem implements it correctly. For
heterogeneous ecosystems, that bet is usually wrong somewhere.

## The two MUSTs

### MUST 1 — Audit EXHAUSTIVELY in one pass

When discovering that a feature isn't rendering in the target
consumer, do not iterate per defect class. Instead, **audit every
branch of the format usage tree at once**:

- entity types (DIMENSION, TEXT, MTEXT, HATCH, LWPOLYLINE, ...)
- block / group references (INSERT, BLOCK, ...)
- font / external resource references (txt.shx, fonts.css, ...)
- layer features (visibility toggles, line-weight, color modes)
- encoding (UTF-8 vs CP1252, BOMs, line endings)
- metadata / advanced format features (XData, OPC parts,
  embedded thumbnails, etc.)

Downgrade EACH branch to the most basic equivalent that all target
consumers render. Iterating per defect class wastes user-feedback
rounds. blue-marlin took two rounds to fix DXF dimensions
(DIMENSION → LINE+TEXT, then TEXT → LWPOLYLINE) — the second round
was avoidable if the audit had been exhaustive on the first pass.

### MUST 2 — Test against the actual consumer, not the spec

Generated output that "validates" against a format-spec linter or
parser is **not evidence of consumer-renderability**. Test by
opening the output in *each* consumer in the target ecosystem and
verifying the visible / parsed result matches expectation.

For heterogeneous ecosystems, this test is necessarily multi-tool:

- DXF deliverable for "any CAD viewer": open in Onshape AND
  AutoCAD AND LibreCAD AND FreeCAD Part AND FreeCAD Draft;
  the union of failures is the audit findings.
- SVG deliverable for "any browser": open in Chromium, Firefox,
  Safari (technology family), plus a non-browser SVG renderer
  (Inkscape, ImageMagick).
- PDF deliverable for "any reader": Acrobat AND a Linux reader
  (evince, mupdf) AND a mobile reader.
- JSON-LD for "any consumer": parse with at least two independent
  libraries from different language ecosystems.

This is the file-format analog of `always/environment-
interrogation-rule.md`'s rule about deployment instructions —
the running consumer is the source of truth, not the docs.

## Concrete examples (DXF case from blue-marlin)

| Format feature | Universal alternative | Trade-off |
|---|---|---|
| `DIMENSION` blocks | LINE + LINE arrowheads + TEXT/polyline | 7-8× more entities per dim |
| `TEXT` entities | LWPOLYLINE glyph outlines (via `text2path`) | ~10× more entities per character; rough Hershey-style fonts |
| `MTEXT` / formatted text | Plain TEXT or LWPOLYLINE | No formatting |
| `HATCH` fills | LWPOLYLINE outline + maybe stipple lines | No fills |
| Layered visibility | Single layer + line-weight differentiation | Loss of toggle-ability |

The pattern generalises beyond DXF:

- **SVG.** Prefer `<line>`, `<polyline>`, `<rect>` over `<filter>`,
  `<animate>`, `<feFlood>` if target audience uses heterogeneous
  viewers.
- **PDF.** PDF/A subset over full-feature PDF for archive /
  exchange.
- **CSV.** Avoid Excel-specific quirks (BOM, formula cells,
  embedded sep declarations); plain comma-separated UTF-8.
- **JSON-LD.** Limit to JSON-LD 1.0 features if consumers may not
  support 1.1 (no nested @context, no @container scope changes).
- **OOXML / ODF.** Avoid extension namespaces; restrict to the
  conformance profile your consumers all support.
- **GLB / STEP.** Restrict to common 3D-CAD profiles (e.g., AP242
  for STEP) and avoid implementation-specific extensions.

## Anti-patterns

- **"The format spec says it's required to render."** — Real
  consumers ignore "required" features regularly. Test, don't
  assume.
- **"It validates against the linter."** — Linters check syntax,
  not consumer behavior. Validation passes are necessary but not
  sufficient.
- **"We'll just document the supported viewer."** — For
  open-deliverable formats (DXF, SVG, PDF), the consumer is
  whatever the recipient already has. The deliverable must work
  in that consumer, not in the one the generator's author
  prefers.
- **"Iterate per failure."** — Each round of "fix this one
  feature, ship again, hear back about the next one" wastes a
  user-feedback round. Audit exhaustively.

## CUSTOMIZE

Adopters MUST customize:

- **Target consumer list** for each format the project generates.
  Not "any DXF viewer" — name them: Onshape, FreeCAD, AutoCAD, ...
- **Universal-primitive table** for each format used (the DXF
  table above is an example; build one for each format the project
  emits).
- **Verification command / harness** for opening the output in
  each named consumer and confirming the result.

Adopters MAY customize:

- **Acceptable trade-offs** — file size growth, entity count
  growth, formatting loss. Document the bound (e.g., "no more than
  20× entity count growth").
- **Programmatic verification** (image-capable model rendering the
  output and describing it; structural diff against a known-good
  baseline).

## Related rules

- `always/environment-interrogation-rule.md` — sibling rule for
  *consuming* heterogeneous environments (deployment targets);
  this rule covers *generating* for heterogeneous environments
  (file consumers).
- `workflow/visible-artifact-verification-rule.md` — sibling rule
  for whether the generation is correct in itself; this rule
  covers whether it survives the consumer-translation step.
- `workflow/tdd-rule.md` — the consumer-verification harness is
  testable. Build a fixture per consumer and run it on every
  generator change.
