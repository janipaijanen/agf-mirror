<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# Coexisting with third-party assistant plugins

*Status: Draft. Capability maturity: Level 1 — Planned (guidance only, no
tooling). Evidence status: Theoretical — no adopter has yet reported running
AGF alongside a coding-style plugin. Audience: adopters who want to use AGF
together with a third-party skill, plugin, or rules pack in the same
repository, and the assistants preparing that setup. After this page you can
decide whether a plugin fits beside AGF, set precedence between them, and keep
the audit trail intact.*

## Why this matters

Many assistant plugins ship always-on instructions, slash commands, hooks, or
their own `AGENTS.md`. AGF is also always-on. Two always-on instruction sets
with no stated precedence leave the assistant to resolve conflicts by itself,
usually in favour of whichever text it read last or found more convenient.

Most plugins do not compete with AGF. They work on a different layer:

| Layer | Governs | Example |
| ----- | ------- | ------- |
| Governance (AGF) | Who may decide what, gates, the coordination record | Gate 1, Gate 2, role authority |
| Coding style (plugin) | How code is written inside an approved change | Minimal code, reuse first, test placement |

Coexistence works when the boundary between the layers is written down. It
fails when the plugin's instructions reach into the governance layer, or when
the plugin's installer replaces AGF's loader.

## The precedence rule

Add the following to the shared loader (root `AGENTS.md`), below the AGF
loader text:

```markdown
## Third-party plugins

This repository also uses <plugin name> for coding style.

Precedence:

1. Human governor decisions and AGF role authority.
2. AGF rules, schemas, plans, and coordination records.
3. <plugin name> instructions, for how code is written inside an approved
   change only.

AGF artifacts (plans, coordination messages, learning records, decision
records) are mandatory process records, not code. Plugin instructions to
minimise, skip, or avoid writing things do not apply to them.

Plugin review or audit output is advisory evidence. It never approves Gate 1
or Gate 2.
```

The second paragraph matters most for minimalist plugins. A plugin that asks
"does this need to exist?" before every artifact will, without this line,
eventually apply that question to a coordination message.

## Checklist

### 1. Keep one loader, and use the install path the plugin actually offers for this assistant

Do not let a plugin installer overwrite root `AGENTS.md`, `.claude/CLAUDE.md`,
or other loader files. A plugin with a native marketplace install for the
assistant in use (for example a Claude Code or Codex plugin, installed
through that tool's own plugin manager) is preferable to a copied rules file,
because the install stays versioned and removable through the tool itself
rather than as untracked repository content.

Many plugins do not ship a native install for every assistant. The remaining
hosts typically get one of two lighter mechanisms instead, and either is
acceptable when no native install exists:

- a skill file the assistant loads on demand (for example a
  `skills/<plugin>/SKILL.md`), for assistants with skill support but no
  plugin manager;
- an instructions-only rules file (commonly `AGENTS.md`) copied into the
  project, for assistants that read repository-root instruction files but
  have neither a plugin manager nor skill support.

For the instructions-only case, merge the plugin's file under the AGF loader
by hand — do not let it replace the loader — and review the diff like any
other governed change. Check the plugin's own install documentation for the
assistant actually in use; do not assume a single install mechanism covers
every supported host, and do not guess which hosts are plugin-capable versus
instructions-only without checking.

### 2. Scope the plugin to roles

Decide which AGF roles the plugin serves. A coding-style plugin usually
belongs to **coder-tester**, sometimes to **planner-architect** for scope
reduction. It does not change **governor-reviewer** authority, and it does not
redefine the criteria of a domain reviewer.

A plugin installed at user level applies to every session on that machine. If
a role must run without it, run that role in a separate session with the
plugin switched off, and say so in the plan.

### 3. File plugin reviews as evidence

A plugin review command (for example a diff review or repository audit) can be
useful Gate 2 input. Treat its output like any other domain reviewer's
findings:

- the role that ran it files the findings as a review entry in the
  coordination channel;
- the entry names the plugin, its version, and the command used;
- the governor-reviewer decides; the plugin does not.

### 4. Choose one record for deferred work

Some plugins track deferred work in their own format, such as marker comments
collected into a ledger. AGF tracks recurring problems as learning records.
Pick a split and write it in the loader. A workable default:

- in-code markers are allowed for local, low-risk shortcuts;
- a shortcut that touches a trust boundary, data contract, or approved plan
  scope is recorded in the completion message, not only in a code comment;
- a shortcut pattern that recurs becomes a learning record.

If the plugin lets a project rename its marker word (some do, via a loader
file setting), record the chosen word in the loader section above so a
reviewer searching for deferred work knows which token to grep for.

### 5. Record what shaped the code

A user-level plugin is not versioned in the repository, so the audit trail
does not show it unless someone writes it down. In the Gate 1 plan or the
completion message, record:

- plugin name and version;
- mode or intensity, if the plugin has levels;
- whether its hooks were active, and on which assistant.

### 6. Treat every hook, on every assistant, as supply chain

Plugin hooks execute code on the developer's machine and inside the agent
loop. This applies per-assistant, not only to whichever assistant the plugin
documents most prominently — a plugin that offers hooks on more than one host
(for example a lifecycle hook on one assistant and a command-switching hook on
another) needs the same review on each one it is enabled for. Install only
from the plugin's documented official source, pin the version, and route the
first adoption and every major-version upgrade — on each assistant where
hooks are enabled — through the security-sentinel role. Hook installation and
trust changes stay on the red list: an agent does not grant them to itself.

### 7. Keep vendor claims at their evidence level

Plugin benchmarks published by the plugin's own author are useful but
self-reported. Until reproduced in your project, record them as no stronger
than *Empirically informed* in the [maturity
vocabulary](../../reference/maturity-vocabulary.md), and do not cite them as
evidence for an AGF rule.

## Verify the setup

Ask the assistant:

```text
Which AGF files and which third-party plugin instructions did you load?
State the precedence between them. Then say whether you may skip a
coordination message because the plugin prefers minimal output.
```

Good answer: names the AGF paths and the plugin, states AGF precedence, and
says coordination messages are mandatory.

Bad answer: treats both instruction sets as equal, or says it may skip a
record to keep output small.

## Exit

Removing the plugin is a normal governed change: uninstall it, delete the
"Third-party plugins" section from the loader, and decide what happens to any
plugin-specific markers left in the code. AGF records that mention the plugin
stay as written; the coordination channel is append-only.

## Worked example: Ponytail

[Ponytail](https://github.com/DietrichGebert/ponytail) is an MIT-licensed
coding-style plugin that pushes the assistant toward the smallest working
change: reuse existing code, standard library, and platform features before
writing new code, while keeping validation, security, error handling, and a
small test for logic. Notes below describe Ponytail 5.3 (release tag
`v5.3.2`); check its README for current behaviour before relying on these
specifics.

Ponytail's own install story already spans all three mechanisms from
checklist item 1: a native plugin install for Claude Code and Codex; a skill
file for several other assistants with skill support (its README names, among
others, Devin CLI, OpenCode, Gemini, Antigravity CLI, and Grok Build); and an
instructions-only `AGENTS.md` copy for assistants with neither (for example
Cursor, Windsurf, Cline, Copilot, and Kiro). Use whichever mechanism the
README documents for the assistant actually in use — do not assume the
plugin-install path applies project-wide just because it is available for one
host.

| Ponytail feature | AGF handling |
| ---------------- | ------------ |
| Always-on minimal-code instructions | Add the precedence rule; AGF artifacts are not code to minimise |
| Rules-file install copies its own `AGENTS.md` (non-plugin, non-skill hosts) | Use the plugin's native plugin or skill install where available; otherwise merge the copied file by hand under the AGF loader |
| Levels (`lite`, `full`, `ultra`, `off`) | Record the level used in the plan or completion message |
| `/ponytail-review`, `/ponytail-audit` | Advisory evidence; file findings as a review entry |
| `shortcut:` comments and `/ponytail-debt` ledger | Apply the deferred-work split from checklist item 4; the marker word is configurable per project |
| `/ponytail-gain` benchmark scoreboard | Self-reported; keep at *Empirically informed* per checklist item 7, not cited as AGF evidence |
| Codex lifecycle hooks, Cursor's level-switching hook | Security-sentinel review on each assistant where a hook is enabled; human grants trust |
| Local cost and token band (Claude Code only) | Stays outside the AGF evidence chain; optional input to `agov` token telemetry comparisons |
| Self-reported benchmark figures | No stronger than *Empirically informed* until reproduced |

Fit with the fast track: smaller diffs mean more changes meet the fast-track
limits. The limits themselves do not change, and gate-mechanics changes still
take the full path whatever the diff size.

## Planned tooling

Not implemented. A possible `agov validate` advisory finding when the
loader file lacks the AGF loader section, for example after a plugin
installer replaced it. Like other loader findings it would not change the
validator's exit status.

## Next

- [Assistant loader configuration](assistant-loader-configuration.md)
- [Adopter applicability catalog](applicability-catalog.md)
- [Roles and gates](../concepts/roles-and-gates.md)
