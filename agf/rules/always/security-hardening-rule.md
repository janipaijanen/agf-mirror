<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Security Hardening Standards
#
# Framework principle: Quality gate — security boundaries
# See: framework.md — ADOPTING.md Step 3.2 (Security checklist)

## Secrets and Environment

- **Never** commit `.env`, `.env.*`, or `*.env` files. All are gitignored.
- Secret keys must be strong random values. The application **refuses to start** in production if secrets are unset or use defaults. Do not weaken this guard.
- Use a secrets detection tool (e.g. `detect-secrets`, `gitleaks`, `trufflehog`) as a pre-commit hook. Never silence real secret detections.

## Supply Chain

- **Pin installer versions** — use versioned URLs or pinned versions in CI, scripts, and container builds. Never use unversioned bootstrap scripts.
- **Pin image tags** — no `:latest` or floating tags in compose files or CI. Use specific versioned tags.
- **Pin tool versions in CI** — install tools with explicit version constraints.
- **Lockfiles** — keep dependency lockfiles in version control. Never run upgrade commands without explicit user request.
- **Image scanning** — run a container vulnerability scanner on version tags. Address HIGH/CRITICAL CVEs before release.

<!-- CUSTOMIZE: add project-specific supply chain rules (e.g. registry mirrors, signing) -->

## Reverse Proxy and TLS

- If using a reverse proxy middleware, apply it automatically in production. Do not add duplicate wraps or remove existing ones.
- Trust one proxy hop by default. Adjust only with documented rationale.
- HSTS and secure session cookies should be automatic in production — do not add manual overrides that conflict.

## Container Image Hygiene

- Secrets are excluded via `.dockerignore` — verify this stays intact when adding new environment files.
- Application runs as a non-root user. Do not change without documented reason.
- Base image tag should be pinned to a specific version or digest for production builds.

<!-- CUSTOMIZE: adapt container runtime specifics (Docker, Podman, etc.) -->

## CSRF and Input Validation

- CSRF protection must be enabled by default. Disable only in test config — never in production.
- All request bodies use schema validation before processing. Do not bypass with raw request body access.
- Apply SSRF guards to all outbound HTTP calls.

<!-- CUSTOMIZE: adapt validation framework (Pydantic, Zod, marshmallow, etc.) -->

## Audit Logging

- All meaningful views and edits must be logged. Use designated audit log functions — never append audit records directly via raw database calls.

## Verify

After any security-relevant change:
<!-- CUSTOMIZE: adapt to your test runner -->
```bash
# Run security test suite
```
