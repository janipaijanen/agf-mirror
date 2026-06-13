<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Template Translatability
#
# Example: Python + Flask-Babel / Django / FastAPI
# Copy and adapt for your project.

- **Requirement:** Every user-visible string in templates (labels, buttons, titles, tooltips, messages, link text, placeholders, aria-labels) must be **translatable** and **translated**.

- **Translatable:** Wrap strings in your framework's i18n function. Examples:
  - Flask-Babel / Jinja2: `{{ _('...') }}`
  - Django: `{% trans "..." %}` or `{{ _("...") }}`
  - FastAPI / Starlette: use your chosen i18n library's equivalent
  Do not hardcode user-facing text.

- **Translated:** Each extracted message ID must have a non-empty translation in every supported locale. Enforce this via a pre-commit hook or CI check.

- Use aria-labels on all interactive elements; no hardcoded colour values in templates.

## Examples (Flask-Babel / Jinja2)

```html
<!-- Good -->
<label>{{ _('Role') }}</label>
<button>{{ _('Save') }}</button>
<title>{{ _('Settings') }} -- Your App</title>
<span aria-label="{{ _('Dismiss') }}">x</span>

<!-- Bad: hardcoded -->
<label>Role</label>
<button>Save</button>
```

## Workflow
1. Add strings with the i18n function -> extract messages -> update locale catalogs -> add translations -> compile. Enforce completeness via pre-commit hook or CI check.
