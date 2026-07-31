# Decommissioning agents-govern

How to remove an agents-govern adoption from your project cleanly and
verifiably. Removal is a normal, supported outcome — a project that has
outgrown the framework, or found it too heavy, should be able to exit
without residue.

Before removing everything, consider the lighter options:

| Option | What it means | When to choose it |
|---|---|---|
| **Downgrade compliance** | Keep the files, lower your compliance profile (see [compliance-profile.md](compliance-profile.md)) | Governance overhead is too high but the audit trail still has value |
| **Pause** | Stop running gates and validators; leave files in place | Temporary capacity problem; you expect to resume |
| **Remove** | Delete all framework files, hooks, and CI jobs | The project no longer benefits from the framework |

The rest of this page covers full removal.

## 1. Record the current state

Run the validators once before removal so the final governed state is
captured in your project history:

```bash
agov validate agents
agov validate contributions
```

Commit any outstanding governed work first. Do not mix in-flight feature
work with the removal commit.

## 2. Files to delete

Delete the files you copied at adoption time. Depending on your layout
(A: root-level, B: subdirectory), paths vary; the standard set is:

| Artifact | Typical location |
|---|---|
| Agent definitions | `governance/agents/*.yaml` (incl. `MANIFEST.yaml`) |
| Governance rules | `agf/rules/` |
| AI assistant loader | Tool-specific root file pointing at `agf/rules/`, if created |
| AI assistant skills | `agf/skills/` plus any optional tool wrapper, if adopted |
| Stack-specific rule examples | Project-local rule files derived from `agf/project-specific-examples/`, if adopted |
| Upstream learning records | `learnings/foundry.yaml` |
| Framework declaration | `framework.yaml` (your tuned copy) |
| Governance charter | `GOVERNANCE.md` (if based on the template) |
| Coordination channel | `docs/coordination/` (see history note below) |
| Framework tooling | Rust `agov` binary/wrapper, or legacy Python/Elixir tooling if vendored from an older release |

**Keep your own learning record** (`learnings/<codename>.yaml`) unless you
have a reason to delete it — it documents decisions made while governed and
remains useful project history. The same applies to plan files and ADRs
produced under governance: they describe your project, not the framework.

## 3. Hooks to remove

- `.pre-commit-config.yaml` — remove the agents-govern hook entries (or the
  whole file if it only existed for the framework), then run
  `pre-commit uninstall`.
- `.claude/settings.json` / assistant configuration — remove framework rule
  references, skill registrations, and any `agov-*` command allowances.
- Git hooks (`.git-hooks/`, `core.hooksPath`) — remove any hook installed
  for gate enforcement and reset `git config core.hooksPath` if it was set.

## 4. CI changes to revert

- Remove validator jobs (`agov validate*` or legacy validator
  invocations) from your CI config.
- Remove path-scoped trigger rules that existed only to scope governance
  changes (see `ci-trigger-scoping-rule`); re-evaluate whether the scoping
  is still wanted for other reasons before deleting it.
- Remove docs/figure jobs that built framework-related artifacts.

## 5. Git history

Commit the removal as a **single atomic commit** with a clear message,
for example:

```
chore: decommission agents-govern adoption

Removes agent definitions, governance rules, skills, hooks, and CI
validator jobs. Learning record learnings/<codename>.yaml retained
as project history. Decision recorded in <ADR/issue reference>.
```

Do not rewrite history to erase the governed period. The adoption happened;
the history is the audit trail of that period and erasing it creates the
false impression that past changes were reviewed under rules that no longer
exist. Record the exit decision itself (an ADR, an issue, or a final
learning-record entry) so future maintainers know governance ended
deliberately.

## 6. Verify removal

After the removal commit, confirm nothing is left behind:

```bash
# No framework tooling references in CI
grep -rn "agov-" .gitlab-ci.yml .github/workflows/ 2>/dev/null

# No copied rule or skill references
grep -rn "agents-govern" --include="*.md" --include="*.yaml" --include="*.yml" . \
  | grep -v learnings/ | grep -v docs/adr/

# No leftover hooks
cat .pre-commit-config.yaml 2>/dev/null | grep -i agov
git config core.hooksPath
```

All searches should come back empty (except intentional history artifacts
such as your learning record and ADRs). If your CI previously ran validator
jobs, watch the first post-removal pipeline to confirm it passes without
them.

## See also

- [compliance-profile.md](compliance-profile.md) — downgrade instead of remove
- [ADOPTING.md](../../ADOPTING.md) — what adoption installed (the inverse of this page)
