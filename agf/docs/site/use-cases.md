# Use cases and non-use cases

*Status: Released. Current framework version: see [Release information](reference/release-info.md). Use cases below trace to
learning records in the framework's evidence base; codenames refer to
anonymized adopter projects.*

Use these scenarios to picture a pilot. They are examples from the evidence
base, not guaranteed results. [Should you adopt?](docs/start/should-you-adopt.md)
owns the Adopt / Wait / Skip decision and the costs you accept.

## Credible use cases

**Solo maintainer with AI assistants doing most implementation.**
The framework's own development (codename *foundry*) runs this way: a
human governor approves plans and merges; AI agents plan, implement, and
review under role definitions. The gates provide recorded checkpoints for the maintainer's review; they
do not demonstrate equivalence to an independent second human reviewer.

**Small team adding AI assistants to an existing quality culture.**
Adopters in the evidence base (e.g. *silver-birch*, *copper-rapids*)
used the rule library to make existing conventions — migration hygiene,
test tiers, CI scoping — binding on assistants across Elixir and Python
application stacks. The intended benefit is consistency: assistants receive the same written
rules across sessions, while adherence still needs review.

**Projects with audit or compliance pressure.**
The append-only coordination channel and audit-log rules give projects
under regulatory regimes a reconstructible decision trail for
machine-generated changes.

**Projects producing visible or generated artifacts.**
Adopters generating CAD drawings and documents (*blue-marlin*) used the
visual-verification and file-generator rules to catch defect classes that
unit tests structurally miss.

**One assistant across sessions, or several assistants working in parallel.**
The framework places no requirement on how many AI tools are involved. A single
assistant — Claude Code, Codex CLI, Cursor, Windsurf, or any tool that loads
the rule files — can play eligible AI roles one after another across work
sessions, within each role's limits. Loading files does not assign a role or
grant approval authority; the human governor remains accountable for approvals.
The coordination channel preserves continuity between sessions.

When several assistants are active, each works within an authorized role
assignment and a separate work boundary. Non-trivial tasks need claims, and any
required temporary allocation must be recorded and acknowledged. They exchange
work through the same channel. One assistant commits a handoff entry and pushes it; another pulls and picks it up. The medium is git: plain
files in the repository, no shared runtime, no message API, no session
continuity required between the tools. Each assistant is configured
independently with the framework rules and reads only what the coordination
channel contains. Role boundaries and claims make ownership and required handoffs explicit;
they do not themselves prevent conflicting writes.

## Non-use cases

Use the [Wait and Skip criteria](docs/start/should-you-adopt.md#wait-if)
before choosing a pilot. They cover disposable work, human-only development,
absent governor capacity, and expectations of a hosted service or agent runtime.

For very large organisations, the evidence boundary is still one project with
one governor. Multi-project, multi-governor federation remains a
[hypothesis](future/evolution.md), not a shipped capability. AGF supplies a
repository decision trail; it does not certify compliance or replace external
security and organisational controls.

## Maturity, honestly

Some capabilities exist only as ideas; others are implemented, tested, or
available through a supported release. AGF records that difference with
**maturity levels** instead of presenting every claim as equally complete. The
single-project, single-governor workflow is the validated core. Several rules
are explicitly **Provisional** (single-adopter
evidence) and say so in their text. If a page on this site does not state a
maturity level for a claim, treat that as a documentation bug and report it.

For enterprise evaluators, this boundary is deliberate: AGF can be piloted in
one repository at a time with local branch protection, local CI, and a named
governor. Central policy distribution, cross-repository gate state, and
multi-governor federation remain roadmap or local-integration work until they
are implemented and validated.

## Next

Found a credible match → use [Should you adopt?](docs/start/should-you-adopt.md)
to decide whether to adopt now, wait, or skip. Found a non-use case → do not
force AGF into it; follow the lighter or runtime-specific path named above.
