---
id: readable-coordination-ids
status: pending-gate1
topic: readable-coordination-ids
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      ID generation and watch-state tooling affect coordination integrity,
      collision resistance, auditability, and potential disclosure of local
      operational context. The plan keeps secrets and machine-local paths out
      of generated IDs and watch records.
  - role: contribution-auditor
    status: consulted
    reason: >
      The plan changes coordination conventions, AI operating guidance, and
      downstream adopter ergonomics. Contribution review is needed to ensure
      the readable format improves usability without weakening governance or
      compatibility.
---

# Gate 1 Plan: Readable Coordination IDs and Tooling-First ID Generation

**Date:** 2026-06-20
**Author:** coder-tester draft for planner-architect review

## 1. Intent and Scope

Introduce a human-readable coordination ID and filename format while preserving
the current ULID format forever.

The target format is:

```text
20260620T183914Z-coder-tester-r7k2
```

The goals are:

- keep coordination and generated governance artifact IDs collision-resistant;
- make coordination files easier for humans to scan;
- preserve chronological sorting;
- guide AI agents toward tooling-generated IDs instead of ad hoc invention;
- keep manual authoring possible for humans and recovery scenarios;
- avoid breaking existing coordination records, references, handoff tracking,
  branch visibility checks, or adopters using current ULIDs.

This plan covers:

- readable coordination ID and filename policy;
- tooling-first generation policy for AIs;
- RFC 2119/8174 keyword scoping for the normative policy text;
- interim branch-watch ownership protocol;
- validator changes needed for uniqueness and warnings;
- backward compatibility for existing ULID records.

This plan does not cover:

- changing historical coordination filenames;
- removing ULID support;
- implementing a persistent daemon or background watcher;
- making branch-watch state a blocking CI check;
- changing release artifact naming.

## 2. Readable ID Format and Tooling Commands

### 2.1 Format

Readable coordination IDs use:

```text
<utc-timestamp>-<role-slug>-<random-suffix>
```

Where:

- `<utc-timestamp>` is `YYYYMMDDThhmmssZ`;
- `<role-slug>` is the coordination sender role, such as `coder-tester`;
- `<random-suffix>` is 4 to 6 lowercase base32/base36 characters;
- the complete ID is ASCII and filename-safe.

Example:

```text
20260620T183914Z-coder-tester-r7k2
```

The filename convention becomes:

```text
docs/coordination/<id>-<sender>.md
```

For the example above:

```text
docs/coordination/20260620T183914Z-coder-tester-r7k2-coder-tester.md
```

The frontmatter `id` remains the authoritative record identity. The filename
is a discoverability and sorting convention.

### 2.2 Tooling Commands

Existing command:

```bash
agov coord brief
```

Required behavior:

- generate a valid `id`;
- populate `timestamp`;
- populate `from` and filename sender suffix when role is known;
- emit a coordination skeleton that validates before body edits except for
  placeholder fields.

Future command:

```bash
agov id new --kind coordination --role coder-tester
```

Required behavior:

- generate the readable ID without a full coordination skeleton;
- support future generated governance artifacts, not only coordination files;
- avoid secrets, paths, hostnames, branch names, or user identifiers in the ID.

The first implementation may keep ULID as the default generator if needed, but
must support an explicit readable-ID mode. A later policy change may make the
readable format the default.

## 3. Policy Text with RFC 2119/8174 Declaration

Normative AGF rules, schemas, and tooling contracts may use RFC 2119/8174
style keywords. The keywords are binding only when the document explicitly
declares the convention.

Normative surfaces for this feature should include:

```text
The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
document are to be interpreted as described in BCP 14 [RFC2119] [RFC8174]
when, and only when, they appear in all capitals.
```

Policy:

```text
AI agents MUST use AGF tooling to generate coordination IDs unless tooling is
unavailable. Humans SHOULD use AGF tooling. Manually authored IDs MAY be used
when they pass schema and uniqueness validation.
```

Explanatory docs should use ordinary guidance instead:

```text
Use the tooling to generate IDs whenever possible.
```

This prevents tutorials, examples, and rationale prose from accidentally
creating downstream compliance obligations.

## 4. AI Operational Rituals

Readable IDs are part of a broader tooling-first operating model for AIs:
durable governance state should be generated or recorded through AGF tooling
where possible, not kept only in ephemeral chat.

### 4.1 ID Generation Ritual

Before writing a coordination message, an AI agent should:

1. Run the approved ID/skeleton generator.
2. Fill the message body.
3. Validate the coordination file.
4. Commit or hand off only after validation passes.

If the generator is unavailable, the AI may manually create a valid ID and
must still validate the file before relying on it.

### 4.2 Branch-Watch Ownership Protocol

Interim protocol:

When an agent starts, changes, or stops a polling timer, it records the watch
state in coordination or in the next relevant coordination message.

Minimum fields:

- watcher agent;
- branch;
- upstream ref and current head;
- purpose;
- interval;
- expiry or duration;
- active or passive status;
- handoff or stop condition.

Policy:

- one branch should have one named primary watcher;
- additional watchers should mark themselves passive or scoped to another
  branch;
- status reports should include watched branch and upstream head;
- when work shifts branches, the watcher should either move the timer or record
  that the old timer intentionally remains scoped to the old branch.

Long-term target:

```bash
agov watch start --branch <branch> --interval 300 --duration 180m
agov watch status
agov watch stop
```

The watch commands are out of scope for the first readable-ID implementation,
but the plan should reserve their shape and document the interim manual
protocol.

## 5. Validator Changes Needed

### 5.1 Schema

Update coordination message schema to accept both:

- current 26-character ULIDs;
- readable IDs matching:

```text
^[0-9]{8}T[0-9]{6}Z-[a-z0-9][a-z0-9-]*-[a-z0-9]{4,6}$
```

The exact regex may be tightened during implementation.

### 5.2 Filename Validation

Validators should accept both existing filename styles:

```text
<ULID>-<sender>.md
<readable-id>-<sender>.md
```

Validators should warn, not block, when:

- filename timestamp differs from frontmatter `timestamp`;
- filename sender suffix differs from frontmatter `from`;
- readable ID role slug differs from frontmatter `from`.

Warnings are preferred because branch rebases, clock skew, and manual recovery
can produce legitimate edge cases.

### 5.3 Uniqueness

Validators must provide a channel-level uniqueness check for frontmatter `id`.

Required behavior:

- duplicate IDs are blocking;
- duplicate filenames remain naturally blocked by Git/filesystem behavior;
- uniqueness checks are format-agnostic;
- existing ULID records and readable records share the same identity namespace.

### 5.4 References and Handoff Tracking

References should continue to resolve by explicit strings in `references-to`
and existing filename mentions. The implementation should not require
historical references to be rewritten.

If future tooling adds ID lookup, it should support both ULID and readable IDs.

## 6. Migration and Backward Compatibility

Existing ULID-based coordination records remain valid forever.

No historical file rename is authorized by this plan. Renaming immutable
coordination files would damage auditability and make historical references
harder to trust.

Backward-compatibility requirements:

- existing coordination validators continue to accept ULIDs;
- existing handoff tracking continues to work;
- existing branch visibility checks continue to work;
- existing `agov coord brief` behavior may remain available via a compatibility
  option;
- documentation explains both formats during the transition.

Recommended rollout:

1. Add schema and validator support for both formats.
2. Add generator support for readable IDs behind an explicit option.
3. Update AI/contributor guidance to prefer tooling generation.
4. Consider making readable IDs the default after at least one release cycle.

## 7. Risks and Open Questions

| Risk | Impact | Mitigation |
|---|---|---|
| Suffix too short causes collisions | Duplicate IDs block coordination | Use 4-6 random base32/base36 chars and validate uniqueness |
| Human-readable IDs leak context | Operational metadata exposure | Include role only; exclude hostnames, usernames, branches, paths, secrets |
| Two accepted ID formats confuse adopters | Documentation and tooling complexity | Present ULID as legacy-compatible and readable format as preferred going forward |
| Filename and frontmatter drift | Confusing audit trail | Warn on drift; keep frontmatter `id` authoritative |
| AIs bypass tooling | Inconsistent IDs | Make tooling use a normative AI ritual and validate all records |
| Global RFC keywords overconstrain downstreamers | Accidental compliance burden | Require explicit RFC 2119/8174 declaration per normative surface |
| Branch-watch protocol becomes noisy | Coordination clutter | Record watch state only for multi-agent or long-running polling sessions |

Open questions:

- Should readable IDs become the default immediately or after one release
  cycle?
- Should `agov coord brief` gain `--id-format readable|ulid`, or should
  readable be a separate command first?
- Should uniqueness be checked by `agov coord validate <file>` only when a
  coordination directory is supplied, or by a separate channel-level command?
- Should readable IDs be used for plan files and other generated governance
  artifacts in the same implementation, or should coordination messages land
  first?

## 8. Test Plan

- Schema tests for ULID IDs and readable IDs.
- Validator tests for:
  - readable ID acceptance;
  - duplicate ID blocking;
  - filename/frontmatter timestamp warning;
  - filename/frontmatter sender warning;
  - ULID backward compatibility.
- Generator tests for:
  - timestamp format;
  - role slug inclusion;
  - random suffix length and alphabet;
  - generated skeleton validation.
- Handoff/reference tests proving existing ULID records still resolve.
- Documentation validation:
  - `agov plan validate docs/plans/readable-coordination-ids.md`
  - coordination examples validate.

## 9. Acceptance Criteria

- Gate 1 approval is recorded before implementation begins.
- Existing coordination records validate without migration.
- New readable-ID coordination fixtures validate.
- Duplicate frontmatter IDs are blocking.
- Timestamp/sender filename drift produces advisory warnings, not blocking
  failures.
- AI guidance clearly states tooling-first ID generation.
- RFC 2119/8174 scope is declared only on normative surfaces.
