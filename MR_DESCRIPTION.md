# plan: identity, accountability, and gate enforcement (issues #35, #45, #34, #47)

Branch: `plan/identity-accountability-gate-enforcement`

## Summary

Four issues planned as one cluster because they all touch the same
artifact: the Gate 1/Gate 2 approval record. Defines one coherent
approval-artifact shape rather than four independently-invented ones.

## What's in this branch

`docs/plans/identity-accountability-gate-enforcement.md` covers:

- **#45** (human accountability): new `kind: gate-approval` coordination
  message with a `gate_approval` object carrying `accountable_human` —
  named human who owns the decision, distinct from the acting role/session.
- **#35** (sender identity): git-native attribution — role identity
  mapped in `agov-config.yaml`, verified against commit authorship;
  explicitly rejects any external key registry or attestation service
  (would violate the removability principle).
- **#34** (enforceable gates): `agov coord gate-check` subcommand design
  + an opt-in GitLab CI job (`gate2:enforce`) that exact-SHA-matches a
  valid approval artifact before allowing merge. States explicitly what
  this does and doesn't cover (presence-of-artifact only, not
  authorship-fraud — that's #35's layer).
- **#47** (verification independence): `independence` sub-object on
  `gate_approval` recording session/model/vendor independence honestly,
  including the valid low-independence case.

## Reconciliation applied (see second commit)

Cross-cluster review
(`docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md`,
Finding 1) found this plan's `owner` field (on `rule.schema.json`) and
the incident-amendment cluster's `approved_by`/`proposed_by` described
the same concept with different names. Resolved to one canonical name,
`accountable_human`, reused verbatim across both artifact families —
applied in the second commit on this branch.

## Ground truth used

Extended existing mechanisms rather than inventing new ones:
`coordination-message.schema.json` (`from`/`session`/`model` already
exist), `docs/coordination/agov-config.yaml` (already tooling-consumed,
extended with `role_identities`), `deputy-governor-rule.md`'s
Delegation Artifact (generalized its named-human concept).

## Review checklist

- [x] Plan validates against `agf/schemas/gate1-plan.schema.json`
- [ ] `gate_approval` schema shape reviewed against #46/#43's actual
      accountability-field usage (see reconciliation note)
- [ ] CI reference job's stated non-goals (no authorship-fraud
      detection, no rebase-invalidation handling) are acceptable scope
- [ ] No `Co-authored-by` trailers in commit
