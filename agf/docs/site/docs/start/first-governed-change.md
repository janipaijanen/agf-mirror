# Your first governed change

*Status: Provisional (v2.0 documentation). Audience: developers who have
completed [install and layouts](../adopt/install-and-layouts.md). After
this page you have taken one real change through both gates and validated the
files that make the audit trail.*

Pick a small, real change: a modest bug fix or feature that is large enough to
need review, but small enough that the mechanics stay visible. The example
below uses a safer login error message. Replace the file names with your own
project's files.

Figure 1 shows the path you are about to walk: plan, Gate 1, implementation,
Gate 2, merge, and completion.

![Gate pipeline](../../reference/figures/gate-pipeline.svg)

## 1. Know where the files will go

AGF itself lives in `agf/`. The POC default keeps project-owned gate evidence
in root `docs/`:

```bash
mkdir -p docs/plans docs/coordination agf/learnings
```

You can run that command yourself, but the next step asks your assistant to
create any missing directories. If your project chooses another coordination
path, configure it before agents start writing files. Do not let agents split
records across two channels.

## 2. Ask your assistant to prepare the starter files

Do not spend your first AGF trial copying template files by hand. Let the AI do
the clerk work, then review what it produced.

Paste this into your coding assistant:

```text
Prepare the starter files for my first governed AGF change.

Use this example task:
"Use a safer, less revealing login error message."

Create or update:
- docs/plans/
- docs/coordination/
- agf/learnings/
- docs/plans/sample-login-error-message.md from agf/templates/gate1-plan.md
- one Gate 1 handoff coordination message
- agf/learnings/sample-project.yaml from agf/templates/learning-record.yaml

Use `agov coord brief --id-format readable --from <role>` for new coordination
messages when the tooling is available. New readable-ID coordination filenames
must be `<id>.md`.

Fill only the fields you can honestly fill for this trial. Leave obvious
placeholders where the governor or implementation evidence must be supplied
later. Do not invent tests, commits, approvals, or validation results.
```

The assistant should create the files and tell you what it changed. Review the
diff. The point of the first trial is not to test your ability to copy
templates; it is to see whether the assistant can work inside the governance
loop and leave an audit trail you can read.

If you are working without an assistant for the setup step, the manual fallback
is to copy the starter templates yourself and then rename the coordination
files using readable IDs from `agov coord brief`.

## 3. Validate the plan and first handoff

Use the `agov` command for core validation. In this repository it points at
`tooling/agov`; in another checkout, symlink that executable into
`~/.local/bin` or run `./tooling/agov` directly.

```bash
agov plan validate docs/plans/sample-login-error-message.md
agov coord validate docs/coordination/<gate-1-handoff-id>.md
```

The plan contains real YAML front matter, including the required advisory
consultation entries. The handoff contains the required handoff object. If
validation fails, fix the file before continuing.

Advisory consultation evidence is required for a valid non-trivial plan, but
advisors do not approve or reject Gate 1. The validator can block a malformed
or missing consultation record; the governor-reviewer records the binding gate
decision.

## 4. Gate 1: decide before implementation

The governor-reviewer reads:

- `docs/plans/sample-login-error-message.md`
- the Gate 1 handoff coordination message

Then the governor files the decision. For this trial, the assistant may prepare
a decision-record skeleton, but the governor writes the actual decision and
conditions. Do not let the assistant approve its own plan.

Validate it:

```bash
agov coord validate docs/coordination/<gate-1-decision-id>.md
```

Nothing is implemented before Gate 1 approval lands.

## 5. Branch and implement test-first

```bash
git checkout -b fix/sample-login-error-message
```

Write or update the failing test, make it pass, then refactor. The TDD rule in
your assistant rules is binding on the assistant even when the change feels
small.

## 6. Gate 2: review before merge

After implementation, update the prepared Gate 2 handoff with the real changed
files and validation commands:

```bash
$EDITOR docs/coordination/<gate-2-handoff-id>.md
agov coord validate docs/coordination/<gate-2-handoff-id>.md
```

The governor-reviewer reviews the implementation diff, the tests, and the
handoff evidence. Approval is a new `decision-record` naming the branch and any
conditions. Do not merge without Gate 2 approval.

## 7. Merge and close the loop

Merge through your normal reviewed merge path. Then update and file the
completion entry:

```bash
$EDITOR docs/coordination/<completion-id>.md
agov coord validate docs/coordination/<completion-id>.md
```

The chain is now reconstructible:

```text
plan -> Gate 1 handoff -> Gate 1 decision -> implementation -> Gate 2 handoff -> Gate 2 decision -> completion
```

That chain is the audit trail. Six months later, "why did this land?" is answered
by files, not memory.

## 8. Record what you learned

If the first run exposed friction, record it in
`agf/learnings/sample-project.yaml` under your codename. Do not include real
project, company, customer, credential, or private URL data in externally shared
learning records.

## Fast track after the first run

For the first walkthrough, use the full ceremony once. After that, check the
fast-track criteria before writing a plan:

- no more than three files;
- no shared interface, data model, security boundary, or migration change;
- no conflicting claim;
- no scope uncertainty.

If every criterion holds, Gate 1 may be waived, but Gate 2 still applies.

## Next

- Mental model behind the ceremony: [roles and gates](../concepts/roles-and-gates.md)
- Daily rhythm without the training wheels: [day-to-day operation](../operate/day-to-day.md)
- Exit path if the trial does not pay off: [exit and decommission](../adopt/exit-and-decommission.md)
