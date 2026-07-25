---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260723T064152Z-contribution-auditor-mm7k
timestamp: 2026-07-23T06:41:52Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T062944Z-contribution-auditor-g6h6
topic: issue58-logo-use-guidelines
requires-ack: true
artifacts_owned: []
---

# Add logo-use guidelines to the #58 identity-system deliverable

The original AGF identity work must include a concise adopter-facing logo-use
guide rather than shipping unexplained asset files. Plan the guide alongside
the SVG master and derived variants.

At minimum, specify:

- approved forms: symbol, horizontal `agents-govern` lockup, stacked lockup,
  monochrome mark, favicon/avatar, and when each is appropriate;
- canonical spelling and typography: `AGF` and `agents-govern`, with the full
  framework name reserved for titles/metadata rather than compressed into the
  small wordmark;
- clear space based on a simple measurable unit from the mark;
- minimum sizes, including independently validated 16, 32, and 48 px favicon
  behavior and a minimum readable header-lockup width;
- approved light, dark, monochrome, and print colours using named palette
  tokens plus exact HEX/RGB values;
- permitted backgrounds and contrast requirements;
- alignment, aspect-ratio, and scaling rules;
- accessibility: the decorative/linked-logo distinction, meaningful link
  name, text alternative where needed, keyboard focus, and no reliance on the
  mark alone to communicate a critical instruction;
- prohibited modifications: stretching, rotation, recolouring outside the
  palette, rearranging letters/arrows, adding effects, placing on busy or
  low-contrast backgrounds, or embedding the slogan inside the mark;
- third-party and community use, including how to describe compatibility or
  adoption without implying endorsement;
- source, license, authorship/provenance, and deterministic export commands;
  and
- a compact visual “do / do not” section using accessible explanatory text,
  with every example explained before its figure under the #51 convention.

Treat the current Lovable draft and Codex raster refinement as provenance-tagged
concept inputs only. The approved production identity must be recreated as an
editable deterministic SVG, reviewed by the governor, and accompanied by the
usage guide in the same governed #58 implementation slice or an explicitly
sequenced child slice.
