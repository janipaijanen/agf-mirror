# learn: incorporate external reviewer fit assessment (#17) + implement doc proposals

Branch: `learn/external-review-17`

## Summary

Incorporates the external LLM fit assessment from GitLab issue #17 and implements
the four highest-impact improvement proposals identified in that review.

**Learning recorded:** `20260624T214609Z-30gz` — external reviewer confirmed AGF
2.2.4's strengths (authority model, two-gate design, coordination protocol, evidence
discipline) and surfaced 8 actionable doc improvement proposals. Verdict: "Adopt now"
for the validated core (single repo, single governor, AI in the loop, long-lived work).

## Proposals implemented

| Proposal | Change | File |
|---|---|---|
| #1 — Versioning clarity | Added "Versioning note" section explaining documentation line vs framework content version | `agf/docs/build/hooks/release_info.py` (generated into release-info.md at build time) |
| #3 — Gate 2 checklist | Added 7-item copyable Gate 2 review checklist | `agf/docs/site/docs/operate/review-and-release.md` |
| #4 — Daily operation checklist | Added 6-item daily checklist + AI agent session-resume note | `agf/docs/site/docs/operate/day-to-day.md` |
| #7 — First-week checklist | Added 7-item first-week adoption checklist | `agf/docs/site/docs/start/adopter-faq.md` |

Proposals #2 (visual customization zones), #5 (validator surface matrix), #6
(coordination thread example), and #8 (configuration drift in upgrades) deferred
to a follow-up — they require layout/visual work beyond prose additions.

## Files changed

| File | Change |
|---|---|
| `agf/learnings/foundry.yaml` | Added learning `20260624T214609Z-30gz` (single-adopter, external reviewer) |
| `agf/docs/build/hooks/release_info.py` | Versioning note added to generated release-info.md template |
| `agf/docs/site/docs/operate/review-and-release.md` | Gate 2 review checklist |
| `agf/docs/site/docs/operate/day-to-day.md` | Daily operation checklist |
| `agf/docs/site/docs/start/adopter-faq.md` | First-week adoption checklist |

## Validation

```
$ agov validate learnings
OK agov.validate.learnings
```

## Review checklist

- [ ] Learning content faithfully captures issue #17 assessment and proposals
- [ ] ID format: `YYYYMMDDThhmmssZ-<4-char-random>` ✓
- [ ] Gate 2 checklist aligns with existing review-and-release guidance
- [ ] Daily checklist aligns with existing day-to-day guidance
- [ ] First-week checklist matches the Day 1–5 prose it follows
- [ ] Versioning note appears in generated release-info.md (hook change, verified by MkDocs build)
- [ ] `agov validate learnings` passes
- [ ] No `Co-authored-by` trailers in commits
