---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Accessibility: WCAG AA Compliance
#
# Framework principle: Quality gate — WCAG AA compliance
# See: framework.md — ADOPTING.md Step 3.3 (Accessibility checklist)

**Scope:** All user-facing HTML in templates, CSS, and JavaScript interactions.

## Per-change (every commit touching templates or UI)

- **Labels:** Every interactive element (`<button>`, `<a>`, `<input>`, `<select>`, `<textarea>`) must have an accessible name -- either visible text, `aria-label`, or `<label for="...">`.
- **Form fields:** Every `<input>`, `<select>`, `<textarea>` must have an associated `<label>` element with matching `for` attribute, or be wrapped in a `<label>`.
- **Images:** Every `<img>` must have `alt` text (descriptive) or `alt=""` + `role="presentation"` (decorative).
- **Colour:** Colour must not be the sole means of conveying information. Use icons, text, or patterns alongside colour indicators (e.g. status indicators must show text labels, not just red/green).
- **Contrast:** Text must meet WCAG AA minimum contrast ratio: 4.5:1 for normal text, 3:1 for large text (>=18px or >=14px bold). Use your CSS framework's semantic classes where they are designed for this.
- **Keyboard:** Every interactive element must be reachable and operable via keyboard (Tab, Enter, Space, Escape). Do not use `tabindex` values > 0.
- **No hardcoded colours:** Use theme/design-system classes, not inline styles or hex values in templates. Exceptions must be documented.

## Per-milestone (before release)

- [ ] No scaffold or placeholder text visible in production UI
- [ ] App-level navigation works on every page (navbar, sub-nav)
- [ ] Navigation is consistent across all pages
- [ ] No internal IDs (UUIDs) exposed as user-editable fields
- [ ] Full keyboard-only walkthrough of primary user journeys

## Verify with

Visual inspection of key pages. Recommended: automated a11y scan in CI (e.g. `pa11y` or `axe-core`).

<!-- CUSTOMIZE: add project-specific a11y verification commands or CI steps -->
