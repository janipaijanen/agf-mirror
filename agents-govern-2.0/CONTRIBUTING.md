<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Contributing to agents-govern

Thank you for your interest in improving the governance framework. This
document explains how to contribute effectively.

## Ways to contribute

| Contribution | Where | How |
|-------------|-------|-----|
| **Learning record** | `learnings/<codename>.yaml` | Record what worked, what didn't, and what's missing in your project |
| **Rule template** | `rules-template/` | A governance rule validated across tech stacks |
| **Stack-specific rule** | `project-specific-examples/` | A rule tied to a specific framework or tool |
| **Agent definition** | `agents/` | A new agent role validated by project evidence |
| **Bug report** | GitLab issue | Something is wrong in the framework docs or templates |
| **Improvement proposal** | GitLab issue | A concrete suggestion backed by evidence |

## Before you submit

### Anonymization (mandatory)

All submissions must use **codenames** instead of real project, product,
or organisation names. This protects adopters' commercial interests.

- See `learnings/_template.yaml` for what to mask and what to keep
- Run `uv run agov-validate-learnings` on your learning YAML before submitting
- Submissions containing real names will be returned for anonymization

### Evidence requirement

The framework changes based on field evidence, not theory. Every
proposed change must cite:

- Which project codename encountered the issue
- What category of learning it falls under (gap, overkill, validation,
  adaptation, tension)
- What the concrete outcome was (prevented a loss, added overhead, etc.)

Changes corroborated by **two or more independent projects** are
prioritized. Single-project evidence is noted and watched.

## Submitting a learning record

### Quick path: GitLab issue (recommended for updates)

Paste the learning details into a GitLab issue using the "Learning"
issue template. The Sentinel will integrate your entries into the YAML
file safely. **Use this path when updating an existing learning record**
— editing 400+ lines of whitespace-sensitive YAML in a browser is
error-prone.

### Full path: Merge request (recommended for new projects)

Use this when submitting a **new** project's first learning record.
For updates to existing records, prefer the quick path above.

Step-by-step for submitting learnings via MR:

```bash
# 1. Clone or fork the repo
git clone git@gitlab.com:janipaijanen/agent-governance.git
cd agent-governance

# 2. Create a branch named learnings/<codename>
git checkout -b learnings/<codename>

# 3. Copy the template (if first submission for this project)
cp learnings/_template.yaml learnings/<codename>.yaml

# 4. Fill in the project context section and add learning entries
#    - Use your codename everywhere — never real project/org names
#    - Keep the private codename→real-name mapping in YOUR records, not here

# 5. Run the InfoSec Sentinel validation (mandatory — CI will block without it)
uv run agov-validate-infosec learnings/<codename>.yaml
#    Fix any BLOCK findings. For false positives, add sentinel_ack entries
#    (see learnings/_template.yaml for the format). Only a human can ack.

# 6. Commit and push
git add learnings/<codename>.yaml
git commit -m "learnings: <codename> — N learnings from <domain> adoption (vX.Y.Z)"
git push -u origin learnings/<codename>

# 7. Create the MR on GitLab
#    Target branch: main
#    Title: learnings: <codename> — N learnings from <domain> adoption (vX.Y.Z)
#    Description: list the framework-scoped signals and their severity
```

**MR description template:**

```markdown
## New adopter: <codename>

<One-line project description, anonymized>. Adopted agents-govern vX.Y.Z on YYYY-MM-DD.

### Learnings summary

| # | Category | Scope | Severity | Title |
|---|----------|-------|----------|-------|
| 1 | ... | project/framework | ... | ... |

### Framework-scoped signals for Sentinel review

- **Signal A:** <description and recommendation>

### InfoSec sentinel
- validate-infosec.sh: all clear / N findings acked
```

### Full path via GitLab Web IDE (no CLI required)

If you do not have a local Git setup, you can submit via the browser:

1. Open the repository at
   `https://gitlab.com/janipaijanen/agent-governance`
2. Click **Fork** (top right) to create your own copy
3. In your fork, click **Code → Open in Web IDE**
4. In the file explorer, right-click the `learnings/` folder →
   **New file** → name it `<codename>.yaml`
5. Copy the contents of `learnings/_template.yaml` and paste into your
   new file. Fill in the project context and learning entries.
6. In the left sidebar, click the **Source Control** icon (branch symbol)
7. Enter a commit message:
   `learnings: <codename> — N learnings from <domain> adoption (vX.Y.Z)`
8. Click **Commit to new branch** — name the branch `learnings/<codename>`
9. When prompted, click **Create MR** to open a merge request against
   the upstream repository

**Note:** The Web IDE has no YAML validator. For new files (copy-paste
from the template) this works well. For editing existing YAML files
with many entries, prefer the quick path (GitLab issue) to avoid
indentation errors.

### Anonymization checklist (before pushing)

- [ ] No real project, product, or organisation names
- [ ] No real person names (except in `sentinel_ack.acknowledged_by` which requires a human name)
- [ ] No URLs, hostnames, or infrastructure identifiers
- [ ] No customer or partner names
- [ ] `validate-infosec.sh` passes (exit code 0)

## Reporting overkill findings

An overkill finding means a part of the framework added overhead
without delivering value for your project. These are **the most
valuable contributions** right now — zero overkill at ~177 learnings
is statistically suspicious and likely reflects single-developer bias.

To report overkill:

1. Record a learning with `category: "overkill"` in your learning YAML
2. Be specific: which rule, agent, or process step was the problem?
3. Explain what you tried, how long you gave it, and why it didn't work
4. Submit as a learning record (quick path or merge request — see above)

Overkill evidence from independent adopters will trigger a Sentinel
review of the affected framework section.

## Submitting a rule or agent

1. Validate the rule/agent in your own project first
2. Anonymize -- replace real names with codenames
3. Add `<!-- CUSTOMIZE -->` markers where adopters need to adapt
4. Ensure it's self-contained (makes sense without your project's code)
5. Submit as a merge request with the evidence that validates it

See [README.md](README.md#proposing-rules-or-agents-for-the-framework)
for the full promotion criteria.

## Merge request guidelines

- One logical change per MR
- Reference the learning(s) or issue(s) that motivate the change
- If touching `framework.md`, explain which section and why
- If adding a rule, place it in the correct subdirectory:
  - `rules-template/always/` -- rules that apply to every change
  - `rules-template/workflow/` -- process and pipeline rules
  - `rules-template/docs/` -- documentation quality rules
  - `rules-template/ui/` -- UI and accessibility rules
  - `project-specific-examples/<stack>/` -- stack-specific rules

## Code of conduct

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contributions are licensed under
[CC BY-SA 4.0](LICENSE), the same license as the framework's operational
artifacts (rules, agents, templates, CI configuration).

You also grant the project maintainer the right to distribute your
contribution under additional licenses, including a commercial license
offered as an alternative to CC BY-SA 4.0 (see [LICENSING.md](LICENSING.md)
and [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md)). This grant is
necessary to keep the dual-license offer coherent: every contribution
must be available under both license tracks, otherwise the project
cannot offer a uniform commercial alternative.

Contributions to the Python tooling (`src/agents_govern/`, `tests/`)
are licensed under [MIT](LICENSE-MIT). Contributions to
`project-specific-examples/` are likewise MIT, since adopters copy them
into their own (potentially proprietary) repositories as starter
content.

You retain copyright in your contributions. You are not transferring
ownership; you are granting the project the licenses described above.
