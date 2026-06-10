# agents-govern 1.1.50

## Provenance

| Field | Value |
|-------|-------|
| Framework | agents-govern |
| Version | 1.1.50 |
| Commit | 47a1792523e453065d2c1aa557cab67cc5635b87 |
| Date | 2026-05-08 |
| Repository | https://gitlab.com/janipaijanen/agent-governance |
| License | CC-BY-SA-4.0 |

## Contents

This distribution contains the adoptable files from the agents-govern
framework. See [ADOPTING.md](ADOPTING.md) for the step-by-step adoption guide.

| Directory / File | Purpose | Action |
|------------------|---------|--------|
| `agents/` | Agent definitions (Phase 1 + Phase 3) | Copy to `governance/agents/`, customize |
| `rules-template/` | Governance rules | Copy to `.claude/rules/`, customize |
| `skills/` | AI assistant workflows (upgrade, learning, infosec) | Copy to `.claude/skills/`, customize (optional) |
| `project-specific-examples/` | Stack-specific rule examples | Copy if stack matches |
| `schemas/` | Strict-contract JSON Schemas and invariants | Use for validation and integration checks |
| `scripts/generate-schemas.py` | Schema regeneration helper | Use when maintaining schema artefacts |
| `learnings/_template.yaml` | Learning record template | Copy to `learnings/<codename>.yaml` |
| `learnings/.adopter-mapping.example` | Codename mapping format | Keep private (see anonymization policy) |
| `framework.yaml` | Machine-readable framework declaration | Copy and tune for adopter compliance profile |
| `framework.md` | Framework theory & reference | Read-only reference |
| `ADOPTING.md` | Adoption guide | Follow step by step |
| `GOVERNANCE.md` | Governance template | Basis for project-specific governance |
| `LEARNINGS.md` | Learnings guide & privacy policy | Read-only reference |
| `LESSONS-LEARNED.md` | Narrative lessons from adopters | Read-only reference |
| `README.md` | Overview and quick start | Read-only reference |
| `docs/binding-contract.md` | Agent-output binding contract | Read before enabling full compliance |
| `docs/integration-contract.md` | Validator and integration contract | Read before integrating tooling |
| `docs/contract-stability.md` | Contract versioning and stability promise | Read before upgrading |
| `docs/adopt/compliance-profile.md` | Compliance target definitions | Use to choose adoption depth |
| `docs/ANONYMIZATION-NOTICE.md` | Anonymization policy details | Read before submitting learnings |
| `docs/adr/` | Architecture decision records | Informational |
| `pyproject.toml` + `src/agents_govern/` | CLI tools (`uv run agov-*`) | Validation, bumping, health check |
| `.pre-commit-config.yaml` | Pre-commit hook config | Optional: `pre-commit install` |
| `CONTRIBUTING.md` | Contribution guidelines | Read before submitting |
| `CODE_OF_CONDUCT.md` | Community standards | Read-only reference |
| `.gitlab/issue_templates/` | Issue templates (Learning, Rule Proposal, Bug) | Use when filing issues |
| `LICENSE` | CC BY-SA 4.0 | Include in project |

## Quick start

```bash
# 1. Copy agent definitions to your project
mkdir -p governance/agents
cp agents/*.yaml governance/agents/

# 2. Copy governance rules
cp -r rules-template/ .claude/rules/

# 2b. (Optional) Copy AI assistant skills
cp -r skills/ .claude/skills/

# 3. Create your project learning record (use a codename, not the real name)
mkdir -p learnings
cp learnings/_template.yaml learnings/<codename>.yaml

# 4. Copy license
cp LICENSE .

# 5. Customize
#    - governance/agents/*.yaml — set jurisdiction to your domain
#    - .claude/rules/ — search for <!-- CUSTOMIZE --> comments
#    - learnings/<codename>.yaml — fill in project context and assessments

# 6. Write your GOVERNANCE.md (use the included one as a starting point)
cp GOVERNANCE.md .
#    Edit to match your project's roles, pipeline, and gate rules.

# 7. (Optional) Copy stack-specific examples if applicable
#    cp -r project-specific-examples/python-flask/ .claude/rules/
```

For the full 7-step adoption process, see [ADOPTING.md](ADOPTING.md).

## Upgrading

When upgrading from a previous version, record the upgrade in your
`learnings/<project>.yaml` adoption section:

```yaml
upgrade_history:
  - date: "2026-05-08"
    from_version: "<previous>"
    to_version: "1.1.50"
    agent_changes: []
    notes: ""
```
