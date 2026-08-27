---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260802T231019Z-coder-tester-pb2n
timestamp: 2026-08-02T23:10:19Z
immutable: true
references-to: []
topic: schema-versioning-learning-adversarial-review
requires-ack: true
artifacts_owned: []
model: claude-sonnet-5
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "The user explicitly asked for a hard, adversarial tear-down of a
      new learning record, not a normal pass/fail review -- please treat
      this request literally: try to break the finding and the
      recommendation, not just confirm they're plausible."
    - "New learning MR !986, branch
      learning/schema-and-record-versioning-gap, id
      20260802T230821Z-sv7q in agf/learnings/foundry.yaml. Claim: 8 of 9
      agf/schemas/*.json files have no per-record schema_version field,
      while release-provenance.schema.json already requires one
      (const-pinned) and has a sibling -legacy schema for pre-versioning
      records. Verified by direct grep + python3 json.load() inspection
      of all nine files, not from documentation or memory -- please
      re-verify that inspection independently rather than trust it."
    - "Specific things worth trying to break: (1) is
      framework.schema.json's contract_version actually a
      counterexample the learning wrongly dismisses -- does it, in fact,
      solve the same problem release-provenance's schema_version solves,
      just under a different name? (2) is the severity (minor) and
      business_impact (neutral) classification defensible, or does the
      live, in-flight learning.schema.json shape change (NASA LLIS
      comparison, MR !981) make this more urgent than 'minor'? (3) does
      the recommendation's 'decide per-schema, not one blanket change'
      hedge actually add value, or is it dodging a real design decision
      the learning should have made itself? (4) is there a ninth or
      tenth schema-like record surface this document missed entirely
      (e.g. agf/dist/release-manifest.yaml, or non-JSON-Schema YAML
      contracts) that would change the '8 of 9' count?"
    - "This is a learning record, not a plan -- no code or schema change
      is proposed or authorized by it. The point of the hard review is
      to stress-test the finding before it becomes the basis for any
      future schema-change plan, not to gate this MR's own merge."
  must_read:
    - agf/learnings/foundry.yaml
    - agf/schemas/release-provenance.schema.json
    - agf/schemas/release-provenance-legacy.schema.json
  do_not_touch: []
---

# Hard adversarial review requested: schema-versioning learning

Filed after the user asked, verbatim, to "tear down with codex by hard."
Please read that as license to be genuinely adversarial rather than
confirmatory -- if the finding or recommendation doesn't survive scrutiny,
say so plainly; a learning that only survives friendly review isn't
worth much.
