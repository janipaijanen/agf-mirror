<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Anonymization Notice for Existing Adopters

**Effective date:** 2026-04-06
**Applies to:** All projects with learning records in `learnings/`

---

## What changed

Starting with this notice, **all learning records must use codenames** instead
of real project, product, or organisation names. This is a mandatory
requirement for all submissions — new and existing.

See the full policy in [LEARNINGS.md](https://gitlab.com/janipaijanen/agent-governance/-/blob/main/LEARNINGS.md)
under "Submission Privacy Policy".

---

## Why

Learning records are shared across all adopters and referenced in public ADRs,
sentinel reviews, and changelog entries. Real names expose adopters' technology
choices, compliance posture, and internal architecture. Codenames protect
commercial interests while preserving the analytical value of the data.

---

## What existing adopters must do

### 1. Choose a codename

Pick a memorable, non-descriptive codename for your project. Avoid names that
hint at the real product (e.g., don't use "finance-app" if your product is a
billing system).

Good examples: "blue-heron", "project-aurora", "iron-gate", "quiet-river"

### 2. Notify the Sentinel

Contact the framework Sentinel (currently Jani) with:
- Your current filename in `learnings/`
- Your chosen codename
- A contact email for future correspondence

The Sentinel will:
- Rename your learning file from `learnings/<real-name>.yaml` to
  `learnings/<codename>.yaml`
- Update the `project.name` field (now `project.codename`) inside the file
- Scrub any real names from free-text fields (descriptions, notes,
  recommendations)
- Update any Architecture Decision Record (ADR) or sentinel review references
- Record the mapping in the private (non-committed) adopter mapping file

### 3. Review the scrubbed file

After migration, the Sentinel will share the scrubbed file for your review.
Verify that:
- No real project, product, or organisation names remain
- No internal URLs, hostnames, or infrastructure identifiers remain
- The analytical value of your learnings is preserved (domain, stack,
  regulatory context, quantitative data are still present)

### 4. Update your local references

If your project references the learning file by name (e.g., in CI scripts,
documentation, or agent definitions), update those references to use the new
codename-based filename.

---

## Git history: when NOT to rewrite

**Pre-flight check before running `git filter-repo` on your project:**

Does the project/product name appear in any of the following?

- [ ] URLs or domain names (e.g., `myproject.example.com`)
- [ ] Package names (e.g., `pip install myproject`, `mix.exs :myproject`)
- [ ] Email addresses (e.g., `admin@myproject.io`)
- [ ] Database names or schema references
- [ ] Cascading Style Sheets (CSS) themes, asset paths, or branding files
- [ ] Deployment configuration (Docker image names, service names)
- [ ] CI/CD pipeline variables or job names

**If any box is checked: do NOT rewrite git history.** A blanket text
replacement will break your codebase — URLs, package references, database
connections, themes, and deployment configs will all be corrupted.

Instead, anonymize only the **learning record content** (the Yet Another Markup Language (YAML) file
submitted upstream). Your project's own source code keeps its real name;
only the record you share with the framework uses the codename.

**If no box is checked** (the project name appears only in governance
files, learnings, and agent definitions): `git filter-repo` is safe.
This typically applies to documentation-only repositories or governance
frameworks (like agents-govern itself).

*Evidence: copper-rapids (2026-03-29, critical) — blanket filter-repo
on a project where the product name appeared in URLs, package names,
CSS themes, and database names. Caught before force push; would have
broken CI, deployment, all URLs, and the package registry.*

---

## Affected projects

The following existing learning records will be migrated:

| Current filename | Status |
|------------------|--------|
| `iron-gate.yaml` | **Migrated** → `iron-gate.yaml` |
| `ember-clock.yaml` | **Migrated** → `ember-clock.yaml` |
| `pine-ridge.yaml` | **Migrated** → `pine-ridge.yaml` |
| `glass-key.yaml` | **Migrated** → `glass-key.yaml` |
| `silver-birch.yaml` | **Migrated** → `silver-birch.yaml` |
| `copper-rapids.yaml` | **Migrated** → `copper-rapids.yaml` |
| `north-wind.yaml` | **Migrated** → `north-wind.yaml` |
| `quiet-river.yaml` | **Migrated** → `quiet-river.yaml` |

---

## Timeline

- **Immediate:** New submissions must use codenames. The updated template
  enforces this.
- **Within 30 days (by 2026-05-06):** All existing adopters should contact the
  Sentinel with their chosen codename.
- **Within 60 days (by 2026-06-05):** Migration of all existing records will be
  complete. Any records not yet migrated will be migrated by the Sentinel using
  an assigned codename, and the adopter will be notified.

---

## Questions?

Contact the framework Sentinel or open an issue in the framework repository.
