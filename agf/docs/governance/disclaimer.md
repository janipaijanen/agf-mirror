<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Disclaimer

## No warranty

This framework is provided **"as-is" and "as-available"** without
warranties of any kind, whether express, implied, statutory, or
otherwise. This includes, without limitation, any warranties of
merchantability, fitness for a particular purpose, non-infringement,
and absence of latent or other defects, accuracy, or the presence or
absence of errors, whether or not known or discoverable.

See [CC BY-SA 4.0, Section 5a](https://creativecommons.org/licenses/by-sa/4.0/legalcode#s5a)
for the full warranty disclaimer.

## Limitation of liability

To the extent possible, the licensor shall not be liable for any direct,
special, indirect, incidental, consequential, punitive, exemplary, or
other losses, costs, expenses, or damages arising out of this framework
or use of the licensed material, even if the licensor has been advised
of the possibility of such losses, costs, expenses, or damages.

See [CC BY-SA 4.0, Section 5b](https://creativecommons.org/licenses/by-sa/4.0/legalcode#s5b)
for the full limitation of liability.

## Framework guidance is advisory

The governance patterns, agent definitions, rule templates, anonymization
procedures, and all other guidance in this framework are **advisory, not
authoritative**. They are based on evidence from a limited number of
adopting projects and may not be appropriate for your context.

In particular:

- **Anonymization procedures** (including `git filter-repo` instructions)
  may damage your codebase if applied without the pre-flight checks
  described in `docs/ANONYMIZATION-NOTICE.md`. Always verify the
  procedure against your project before executing. Always back up first.

- **Security guidance** (including the InfoSec Sentinel's detection
  heuristics) does not constitute a security audit. The framework does
  not guarantee that following its guidance will prevent information
  leaks, data breaches, or regulatory violations.

- **Agent definitions and rule templates** are starting points, not
  complete solutions. They must be adapted to your project's domain,
  regulatory context, and risk profile.

- **AI assistants and local tooling** must be run only on systems,
  repositories, model weights, and data where you have the necessary
  authorization. AGF does not grant tool-call permissions, bypass local access
  controls, authorize use of third-party code or data, or make it safe to send
  confidential material to external AI services.

## Your responsibility

You are solely responsible for:

- Evaluating whether this framework is appropriate for your project
- Adapting the guidance to your specific context and risk profile
- Backing up your data and codebase before applying any procedure
- Complying with applicable laws, regulations, and contractual obligations
- Reviewing all changes before committing or pushing them
- Confirming that any AI assistant, local model, hardware, runtime, or
  connector you use is permitted by your project, employer, customer, and
  applicable license terms
- Obtaining independent professional advice where appropriate

## Independence and third-party brands

AGF-NG is developed independently. Unless a separate written agreement says
otherwise, the project has not received external funding from, and has no
dependency, endorsement, reseller, partnership, or governance relationship
with, companies or brands whose tools, models, hardware, or runtimes are
mentioned in the documentation.

References to products such as AI assistants, model providers, local runtimes,
hardware platforms, editors, CI systems, or package managers are compatibility
and evaluation examples only. They do not imply sponsorship, endorsement,
certification, or that those vendors approve the framework.

## No professional advice

Nothing in this framework constitutes legal, security, compliance, or
professional advice. If your project handles regulated data (GDPR, HIPAA,
PCI-DSS, etc.), consult qualified professionals for your specific
obligations.
