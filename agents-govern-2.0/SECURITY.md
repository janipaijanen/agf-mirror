<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Security Policy

## Scope

agents-govern is a governance framework -- it consists of documentation,
YAML agent definitions, and rule templates. It does not contain
executable code that runs in production environments (except a bash
validation script for learning records).

Security concerns for this project fall into two categories:

1. **Framework guidance that could lead adopters to insecure practices**
   (e.g., a rule template that encourages bypassing security checks)
2. **Information disclosure in learning records** (e.g., real project
   names, internal infrastructure details, or credentials)

## Reporting a vulnerability

If you discover a security issue in the framework guidance or an
information disclosure in a learning record:

1. **Do not open a public issue.** Security reports should not be
   visible to the public before they are addressed.
2. **Email the maintainer** at the address listed in the git commit
   history (use `git log --format='%ae' -1`).
3. Include:
   - Which file and section is affected
   - What the security impact is (who could be harmed and how)
   - A suggested fix, if you have one

## Response timeline

- **Acknowledgement:** within 48 hours
- **Assessment:** within 7 days (is this a real security issue?)
- **Fix:** depends on severity, but information disclosure issues
  (leaked real names, credentials) will be treated as urgent and
  addressed within 24 hours of confirmation

## Information disclosure

The anonymization policy (see `learnings/_template.yaml` and
`docs/governance/anonymization-notice.md`) exists specifically to prevent
information disclosure. If you find a real project, product, or
organisation name anywhere in the repository -- including in git
history -- that is a security issue and should be reported.

## Security-relevant changes — what they mean for adopters

### What counts as security-relevant

This framework is not executable software — it has no CVEs. A
security-relevant change means one of:

| Category | Example | Impact on adopters |
|----------|---------|-------------------|
| **Anonymization bypass** | Stale branch exposed real project names (v0.23.1) | Adopters who submitted learning records may have been de-anonymized |
| **Insecure guidance** | A rule template that permitted or encouraged insecure code patterns | Adopters following the rule may have introduced vulnerabilities |
| **Agent boundary gap** | A forbidden_actions list that was incomplete, allowing an agent to bypass a security control | Adopters relying on the boundary may have undetected violations |
| **Infosec sentinel bypass** | A detection gap in the screening script that missed sensitive content | Learning records may contain information that should have been flagged |

### How security-relevant changes are communicated

1. **CHANGELOG.md** — entries for security-relevant releases are prefixed
   with `**SECURITY:**`
2. **GitLab release notes** — security-relevant releases are tagged in
   the release description
3. **This file** — significant incidents are recorded in the incident
   log below

### Expected adopter response

| Severity | Meaning | Expected response |
|----------|---------|-------------------|
| **Critical** | Active information disclosure or guidance that causes insecure deployments | Upgrade within 7 days. Review your project for impact. |
| **Significant** | Boundary gap or detection bypass that could be exploited under specific conditions | Upgrade at your next planned release or within 30 days. |
| **Minor** | Cosmetic or low-impact gap with no realistic exploit path | Upgrade at your convenience. No urgency. |

These are expectations, not mandates. The framework is advisory (see
docs/governance/disclaimer.md). Adopters are responsible for evaluating whether a
security-relevant change affects their project.

### Incident log

| Date | Version | Category | Summary |
|------|---------|----------|---------|
| 2026-04-09 | v0.23.1 | Anonymization bypass | Stale `version-file` branch on public GitLab exposed all 8 codename-to-real-name mappings. Branch deleted. Post-migration branch audit added to dist.sh. |

## Supported versions

Only the latest release is supported. Older releases are not patched.
Adopters should upgrade to the latest version when a security-relevant
change affects them (see response guidance above).

| Version | Supported |
|---------|-----------|
| Latest (see VERSION) | Yes |
| Older releases | No |
