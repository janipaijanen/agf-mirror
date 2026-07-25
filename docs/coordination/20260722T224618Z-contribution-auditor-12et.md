---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T224618Z-contribution-auditor-12et
timestamp: 2026-07-22T22:46:18Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T204718Z-contribution-auditor-mj2g
  - 20260722T215402Z-governor-reviewer-srqj
  - 20260722T215455Z-governor-reviewer-mkth
topic: marketing-assess-critical-review-public-message
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Add intentional critical review to the joint ELI5 marketing and pedagogy assessment.
    - Decide whether this safeguard has useful, honest public value and where it belongs in first-contact documentation.
    - Propose plain wording and identify claims or terms that marketing should reject.
  must_read:
    - docs/coordination/20260722T204718Z-contribution-auditor-mj2g.md
    - docs/coordination/20260722T215402Z-governor-reviewer-srqj.md
    - docs/coordination/20260722T215455Z-governor-reviewer-mkth.md
    - docs/coordination/20260722T224125Z-contribution-auditor-gq5a.md on branch coord/claude-next-expectations and MR !403
  do_not_touch:
    - README.md
    - agf/docs/site/
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Should AGF publicly explain intentional critical review?

Add this question to the joint marketing, customer-success, and pedagogical
screen of the ELI5 baseline:

> Should AGF explain that important proposals are intentionally stress-tested
> by a critical reviewer who tries to find counterexamples, expose unsupported
> assumptions, narrow claims, require changes, or recommend rejection before a
> human approves them?

This is potentially a concrete end-user benefit. It explains why AGF uses a
second review perspective: not to generate more agreement, but to reduce
confirmation bias and discover weaknesses while a proposal is still cheap to
change.

## Marketing and pedagogy questions

1. Does this help a newcomer understand AGF, or does it add governance jargon
   before the basic value is clear?
2. Is **critical review**, **stress-test**, or **challenge review** clearer than
   **adversarial assessment**? Test the terms with non-native-English and solo
   maintainer framing. “Adversarial” may sound hostile or imply security
   penetration testing.
3. Where, if anywhere, should the message appear: the short ELI5, main README,
   a deeper “how review works” page, or only evaluator material?
4. What observable behavior must exist before public prose can make the claim?
   Consider reviewer independence, explicit objections, response records, and
   the authority to recommend rejection.
5. How should AGF say that critical review improves scrutiny without claiming
   that a proposal which survives review is correct, safe, or proven?
6. Should the public explanation say that review can **weaken or narrow** a
   proposal, or would **change or reject** be more natural wording?

## Claim boundaries

Marketing must reject wording such as:

- “AGF proves proposals are sound.”
- “Adversarial review removes bias.”
- “Two AI reviewers provide independent assurance” when they share context,
  model lineage, operator incentives, or evidence.
- “A passed critical review is approval.” Binding approval remains the human
  decision-maker's act.

Prefer a restrained explanation along these lines for testing, not automatic
publication:

> Before you approve an important change, another reviewer is asked to look
> for what is wrong with it. The reviewer can challenge assumptions, ask for a
> smaller claim, require changes, or recommend rejection. This does not prove
> the proposal is correct; it makes disagreement and unresolved risks visible
> before the decision.

The terminology screen in coordination
`20260722T224125Z-contribution-auditor-gq5a` separately asks whether AGF should
use recognized review-result vocabulary. Keep the public benefit explanation
plain even if the underlying records later gain structured terms.

Return the conclusion as part of the existing ELI5 consensus synthesis on MR
!350. Distinguish consensus, minority objections, and governor choices. Do not
edit public prose before the ELI5 baseline is jointly reviewed and accepted.
This remains post-v2.4.0 work and must not displace release blockers.
