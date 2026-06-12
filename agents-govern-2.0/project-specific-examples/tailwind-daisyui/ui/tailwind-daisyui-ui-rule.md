<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Tailwind + DaisyUI UI Rule
#
# Framework principle: UX-Accessibility cross-cutting concern
# See: framework.md -- Tier 3 (Cross-Cutting Agents)
# Evidence: silver-birch and copper-rapids (validated across Elixir/Phoenix and Python/Flask)
#
# Stack: Any web application using Tailwind CSS + DaisyUI

<!-- CUSTOMIZE: Replace version numbers and build commands with your project's setup -->

Use **Tailwind CSS 3.x** and **DaisyUI 4.x** for all HTML UI.

## Stack

- Tailwind 3.x + DaisyUI 4.x; use a project-specific theme matching your palette
- Build via your framework's asset pipeline
- **Do not add new CSS packages** without Governor-Reviewer approval
- **Do not change the DaisyUI version** without explicit approval

## Conventions

- Prefer DaisyUI semantic classes: `btn`, `btn-primary`, `card`, `navbar`, `menu`,
  `alert`, `badge`, `modal`
- Set `data-theme` on the root `<html>` element
- Minimal custom CSS -- if Tailwind utilities or DaisyUI components cover it, use them
- All interactive elements must have `aria-label` or visible text
- All form inputs use your framework's form component (not raw `<input>`/`<select>`)
  unless there is a documented reason
