# Should you adopt?

*Status: Provisional (v2.0 documentation). Audience: evaluators. After
this page you can make an informed adopt / wait / skip decision.*

## Adopt now if…

- AI assistants write a significant share of your code, **and** you have
  (or are willing to be) a human governor who approves plans and merges.
- Your project will live long enough for an audit trail to matter —
  months, not days.
- You already have quality conventions you want assistants to follow
  consistently, and prompting alone has not held.
- You accept review overhead per change as a deliberate trade for
  reconstructible decisions.

## Wait if…

- You need multi-project or multi-governor federation — that is a
  [hypothesis](../../future/evolution.md), not a shipped capability.
- You want v2.0's structure specifically: v2.0 is Provisional; v1.x is
  canonical. Waiting for v2.0 promotion is reasonable.
- Your team has no capacity for the governor role right now. Gates without
  a governor are theatre; adopt when someone can own the role.

## Skip if…

- No AI assistants in your loop — conventional review and CI serve a
  human-only team better.
- The codebase is exploratory or disposable.
- You expect a hosted service or a runtime. This framework is files,
  rules, and conventions that your existing tools load and enforce.

## What adoption asks of you

| Commitment | Size |
|---|---|
| Initial install | Copy files, customize rules — an afternoon |
| Per-change overhead | Plan + two gates for non-trivial changes; fast track for small ones |
| Ongoing | Learning records when incidents happen; upgrade hygiene per release |
| Reversibility | Full removal is documented and verifiable — see [exit](../adopt/exit-and-decommission.md) |

The complete itemisation is on the [downstream impact](../adopt/downstream-impact.md)
page. If you read only one more page before deciding, read that one.

## Next

Decided to try it → [Install and layouts](../adopt/install-and-layouts.md).
