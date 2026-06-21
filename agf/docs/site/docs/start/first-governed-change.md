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

## 1. Create the working directories

AGF itself lives in `agf/`. The POC default keeps project-owned gate evidence
in root `docs/`:

```bash
mkdir -p docs/plans docs/coordination agf/learnings
```

If your project chooses another coordination path, configure it before agents
start writing files. Do not let agents split records across two channels.

## 2. Copy the starter files

```bash
cp agf/templates/gate1-plan.md docs/plans/sample-login-error-message.md
cp agf/templates/coord-handoff.md      docs/coordination/COORD-20260614-001.md
cp agf/templates/coord-decision-record.md docs/coordination/COORD-20260614-002.md
cp agf/templates/coord-gate2-handoff.md  docs/coordination/COORD-20260614-003.md
cp agf/templates/coord-completion.md     docs/coordination/COORD-20260614-004.md
cp agf/templates/learning-record.yaml agf/learnings/sample-project.yaml
```

For current tooling, prefer generating coordination skeletons with
`agov coord brief --id-format readable --from <role>`. Readable IDs use
`YYYYMMDDThhmmssZ-<role-slug>-<random-suffix>` and new readable-ID files are
named `<id>.md`. Existing template `COORD-YYYYMMDD-XXX` examples and ULID
records remain valid compatibility forms; do not use sequential integers for
new messages.

## 3. Validate the plan and first handoff

Use the `agov` command for core validation. In this repository it points at
`tooling/agov`; in another checkout, symlink that executable into
`~/.local/bin` or run `./tooling/agov` directly.

```bash
agov plan validate docs/plans/sample-login-error-message.md
agov coord validate docs/coordination/COORD-20260614-001.md
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
- `docs/coordination/COORD-20260614-001.md`

Then the governor files the decision. For this trial, the copied
`docs/coordination/COORD-20260614-002.md` is a valid approval example.
In real work, the governor writes the actual decision and conditions.

Validate it:

```bash
agov coord validate docs/coordination/COORD-20260614-002.md
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

After implementation, update the copied Gate 2 handoff with the real changed
files and validation commands:

```bash
$EDITOR docs/coordination/COORD-20260614-003.md
agov coord validate docs/coordination/COORD-20260614-003.md
```

The governor-reviewer reviews the implementation diff, the tests, and the
handoff evidence. Approval is a new `decision-record` naming the branch and any
conditions. Do not merge without Gate 2 approval.

## 7. Merge and close the loop

Merge through your normal reviewed merge path. Then update and file the
completion entry:

```bash
$EDITOR docs/coordination/COORD-20260614-004.md
agov coord validate docs/coordination/COORD-20260614-004.md
```

The chain is now reconstructible:

```text
plan -> Gate 1 handoff -> Gate 1 decision -> implementation -> Gate 2 handoff -> Gate 2 decision -> completion
```

That chain is the product. Six months later, "why did this land?" is answered
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
