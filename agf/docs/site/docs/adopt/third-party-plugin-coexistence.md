<!-- audience: A (ai-operator); answers: "how AGF fits beside assistant tooling without giving away governance authority" -->
# Coexisting with third-party assistant tooling

*Status: guidance only. Capability maturity: Level 2 — Surface-defined.
Evidence status: Theoretical for coexistence. No adopter has reported running
AGF alongside the named tools. The dated foundry evidence below is bounded to
one repository and does not establish compatibility.*

## AGF's place in the stack

AGF is the governance layer: it records who may decide what, the gates a change
must cross, and the coordination evidence behind those decisions. Coding-style
tools work inside an approved change, below that layer.

Tools may assist work inside an approved change while AGF retains the authority
and evidence boundary. That is a proposed coexistence pattern, not demonstrated
compatibility. The [dated evidence below](#evidence-and-limits) records what was
inspected or tried and what remains untested.

This boundary is the whole claim. It does not cover a tool that replaces an
assistant loader, edits plans or coordination records, changes gate criteria,
or claims approval authority. It is not certification, support, integration,
endorsement, or a promise that two instruction sets cannot conflict.

| Layer | Owns | Authority |
| --- | --- | --- |
| AGF governance | Roles, plans, gates, decisions, coordination records | Binding only where the applicable AGF rule or human governor says so |
| Assistant tooling | How work is drafted or reviewed inside approved scope | Advisory; it cannot approve Gate 1 or Gate 2 |

The official [agov MCP server](mcp-server.md) is a local adapter for AGF's
own CLI commands. It is Level 4 — Validated, with the documented `dir` argument
limitation on its reference page. An external MCP server, plugin, hook, or
coding-style tool is a separate trust surface; it does not inherit that status
or gain approval authority by using the same protocol.

## Keep the boundary explicit

Add a short precedence statement to the shared assistant loader:

```markdown
## Third-party assistant tooling

1. Human governor decisions and AGF role authority.
2. AGF rules, schemas, plans, and coordination records.
3. Third-party instructions, only for work inside an approved change.

Third-party review output is advisory evidence. It never approves Gate 1 or
Gate 2, and its preference for less output never removes a required AGF record.
```

Then verify four things before enabling the tool:

1. **Loader safety.** Its installer must not replace root `AGENTS.md`,
   `.claude/CLAUDE.md`, or an AGF rule. Review any loader edit as a governed
   change.
2. **Trust surfaces.** Hooks, installers, Model Context Protocol (MCP) servers,
   network calls, telemetry, and automatic updates are supply-chain surfaces.
   Pin a version, inspect the exact release, and let a human decide whether to
   trust them.
3. **Role scope.** State which role uses the tool and whether it is active for
   reviewers. A user-level installation may affect every session on the
   machine.
4. **Evidence.** Record the tool name, exact version, enabled hooks or modes,
   and any review command in the plan or completion record. File its findings
   as advisory review evidence.

If the tool writes its own debt markers or ledger, define how those relate to
AGF completion messages and learning records. A marker does not replace a
required record at a trust boundary or outside approved scope.

## Evidence and limits

Review date: **2026-10-27**. Coverage period: **2026-10-23 to 2026-10-27**.
Coverage: **one repository, foundry**. There is **no adopter evidence** and no
source-code audit of either tool. Vendor facts below are attributed to the
projects' own immutable tagged documents. They are not AGF endorsements.

### Evidence entry: Ponytail 5.3.2

Immutable source commit:
[`16352ac229eb08a9598dcae48fe9f2b342567d86`](https://github.com/DietrichGebert/ponytail/commit/16352ac229eb08a9598dcae48fe9f2b342567d86).

Vendor-stated facts at that commit:

- the repository license is
  [MIT](https://github.com/DietrichGebert/ponytail/blob/16352ac229eb08a9598dcae48fe9f2b342567d86/LICENSE);
- the [hook manifest](https://github.com/DietrichGebert/ponytail/blob/16352ac229eb08a9598dcae48fe9f2b342567d86/hooks/claude-codex-hooks.json)
  has three event groups: `SessionStart`, `SubagentStart`, and
  `UserPromptSubmit`; the vendor README separately describes two lifecycle
  hooks, so this page reports the manifest and leaves the inconsistency
  unresolved;
- the [plugin configuration](https://github.com/DietrichGebert/ponytail/blob/16352ac229eb08a9598dcae48fe9f2b342567d86/.claude-plugin/plugin.json)
  enables a local per-turn task ledger by default;
  its optional webhook URL is empty by default, so no webhook sends by default.
  When configured, the webhook receives the task row as JSON—including cost,
  turns, agent time, tokens, reuse, and test status—and task names default to
  being included.

Foundry evidence: the Phase A
[context-delivery report](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/9ef652c7e5b1a29f2841b2d26b4fcc9c07b7252f/docs/reviews/context-delivery-measurement/20261026T155400Z-context-delivery-measurement-report.md)
records one read-only package inspection dated 2026-10-26. It did not exercise
the plugin beside AGF, audit hook code, or verify the code behind the data
statements. Its earlier install-record and signature observations are not used
here because the tagged primary documents do not support them. Vendor benchmark
figures were not reproduced and are not AGF evidence.

Result: the coexistence inference remains **Theoretical**.

### Evidence entry: Caveman 4.0.1

Immutable source commit:
[`d262ab3882952a59312ff3537cd86d894523c5ab`](https://github.com/JuliusBrussee/caveman/commit/d262ab3882952a59312ff3537cd86d894523c5ab).

Vendor-stated facts at that commit:

- [Apache-2.0 covers the repository from 3.0.0](https://github.com/JuliusBrussee/caveman/blob/d262ab3882952a59312ff3537cd86d894523c5ab/LICENSING.md);
  `LICENSE-MIT` preserves the earlier MIT notice;
- the [Claude plugin manifest](https://github.com/JuliusBrussee/caveman/blob/d262ab3882952a59312ff3537cd86d894523c5ab/.claude-plugin/plugin.json)
  has four event groups: `SessionStart`,
  `SubagentStart`, `UserPromptSubmit`, and `SessionEnd`;
- the instruction-only skill and classic output hooks do not directly call a
  Caveman service; `/caveman-compress` is a separate path that sends the
  selected file to the configured model provider;
- its tagged [security document](https://github.com/JuliusBrussee/caveman/blob/d262ab3882952a59312ff3537cd86d894523c5ab/SECURITY.md)
  says, **“Telemetry is on by default and
  opt-out.”** The first eligible interactive run discloses the scope and can
  persist the default-on decision without affirmative opt-in; non-interactive
  runs do not persist that default. A native `SessionStart` can then send a
  background `session_start` event. The listed event
  fields can include install and account state, locale and timezone, session,
  command and outcome, token and proxy aggregates, aggregate first-run scan
  counts from local Claude Code or Codex history, MCP tool metadata, and the
  sender's IP address. The same document warns that desktop or background hooks
  may miss a shell-only `DO_NOT_TRACK`; `caveman telemetry off` persists a
  lasting opt-out;
- the tagged security document says Git tags were not protected and GitHub
  immutable releases were not enabled at that snapshot. It separately says
  runtime binaries are checked against a signed manifest. These are
  vendor-stated document claims, not verification of current repository
  settings or of an installation.

Foundry evidence: two durable experiments on 2026-10-23
([first](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/coordination/20261023T051418Z-coder-tester-47qq.md),
[second](https://gitlab.com/janipaijanen/agents-govern-framework/-/blob/main/docs/coordination/20261023T103235Z-coder-tester-e1kp.md))
rewrote local copies of AGF rule text and measured token and fidelity effects.
They did not run the plugin beside AGF and concluded no adoption
recommendation. The engine, proxy, memory, MCP, shrink, browse, installer,
telemetry implementation, and hook code were not audited.

Result: the coexistence inference remains **Theoretical**.

## Recheck before adoption

These entries are dated snapshots, not current-version guidance. Before using
either tool—or another tool of the same kind—review its exact version, license,
hooks, data flows, telemetry defaults, update path, and loader changes. Keep AGF
authority above it and record what you enabled.

## Next

- [Assistant loader configuration](assistant-loader-configuration.md)
- [Tooling and token savings](tooling-and-tokens.md)
- [Roles and gates](../concepts/roles-and-gates.md)
