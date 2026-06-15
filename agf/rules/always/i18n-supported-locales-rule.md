---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Supported Locales: Fixed Set -- No New Additions Without Approval
#
# Framework principle: Escalation over Improvisation
# See: framework.md -- Principle 5 (Escalation over Improvisation)
# Evidence: silver-birch (rolled back partial German translation in predecessor project)

<!-- CUSTOMIZE: Replace the locale list below with your project's supported locales -->

**Supported locales:** `en`

**Rule:** Do NOT add, scaffold, or partially implement any new locale or translation
catalog beyond the approved set listed above.

## When the user asks for a new locale

**Stop and warn before doing anything:**

> "The i18n-supported-locales rule restricts supported locales to the approved set.
> Adding a new locale is not allowed under the current rule. To proceed, the rule
> must be explicitly updated first."

Do not create new translation files, add locale entries to configuration, or add
locale keys to translation modules for any locale outside the approved set.

## Rationale

A partial translation is worse than no translation -- the UI shows raw message IDs
to users where translations are missing. New locales require: complete translation
of all message strings, build verification, smoke-test, and native-speaker review.
These cannot be done incrementally by an AI agent.

## Customization

Replace the locale list at the top of this file with your project's supported set.
If your project uses Gettext (.po), i18next (.json), or similar, update the file
format references in your project-specific copy of this rule.
