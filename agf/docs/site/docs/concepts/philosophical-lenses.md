# Philosophical lenses on AI governance

*Audience: developers, evaluators, and AI systems evaluating this or any
AI governance framework. After this page you have six structured
questions to bring to a governance design review.*

!!! note "Consultative, not normative"
    These lenses are consultative review prompts, not AGF rules. They help
    any reader — human or AI — challenge a governance design, including
    this one. They create no obligations and do not grant blocking
    authority. Binding authority lives in named rules (`agf/rules/`) and
    human gate decisions, never in this page.

## Why this page exists

Governance systems are usually reviewed against their own stated goals:
did the gate catch the bug, did the audit trail record the change. That
review misses failure modes that are invisible from inside the system's
own vocabulary. The six lenses below come from organizational theory,
economics, and human-factors research, not from AI governance
specifically — each names a way that *any* accountability, measurement,
or oversight system can fail, independent of what it is overseeing.

Each lens is presented as: the idea, the failure mode it names, a design
question a reviewer can ask of any governance system, and an evidence
status per [Maturity vocabulary](../../reference/maturity-vocabulary.md#evidence-status).
An evidence status of *Theoretical* means the lens is grounded in a citable
source but its application to governance-system design specifically has
not been independently tested — that is normal for organizational theory
applied to a new domain, not a weakness unique to any one lens.

## The legibility critique

**The idea.** Governments and organizations often try to make complex,
locally-adapted practices "legible" — standardized, measured, and
observable from the center — in order to plan and control them (Scott,
*Seeing Like a State: How Certain Schemes to Improve the Human Condition
Have Failed*, Yale University Press, 1998).

**The failure mode it exposes.** Making work legible to a governance
system can distort the work itself and destroy the tacit knowledge that
made it effective. What a metric or record captures is not the same as
what the work actually required — and once people work *to the record*
instead of *to the goal*, the record stops reflecting reality.

**The design question.** What evidence does this gate actually require,
and does producing that evidence change how the work gets done in ways
the gate itself cannot see?

**Evidence status:** Theoretical.

## Accountability sinks

**The idea.** Complex organizational and technical systems can absorb
responsibility for a decision so thoroughly that no individual can be
held accountable for the outcome — "the process approved it" becomes the
entire explanation (Davies, *The Unaccountability Machine: Why Big
Systems Make Terrible Decisions — and How the World Lost Its Mind*,
Profile Books, 2024).

**The failure mode it exposes.** A governance process can become an
accountability sink: every step was followed, every box was checked, and
yet no named person can say they could have decided otherwise. When the
feedback loop that would catch a bad outcome never closes on a specific
decision-maker, the system stops learning from its own mistakes.

**The design question.** For a given gate or approval, is there a
specific named person who could have decided otherwise — and would that
person actually find out if the decision turned out to be wrong?

**Evidence status:** Theoretical.

## Automation bias and de-skilling

**The idea.** When a task is automated and a human is left only to
monitor and approve the automated output, the human's own skill at the
underlying task tends to atrophy — while the task that remains, noticing
when the automation is wrong, is often harder than the one that was
automated away (Bainbridge, "Ironies of Automation," *Automatica*, 19,
775–779, 1983, describing industrial process-control operators).

**The failure mode it exposes.** A human approval gate can quietly become
a rubber stamp. The approver has less hands-on practice catching problems
than the person who used to do the work directly, and the review step is
exactly the moment automation bias — trusting the automated output more
than the evidence warrants — is most likely to strike.

**The design question.** Does this approval step give the reviewer real
independent judgment to exercise — forced justification, sampling,
spot-verification against source evidence — or does it just ask them to
click yes on a summary they did not generate and cannot easily check?

**Evidence status:** Theoretical. Bainbridge's paper is a peer-reviewed,
widely cited finding within its original domain — industrial process
control. This page cites that finding only; it does not cite the
separate aviation- or medicine-specific automation-bias literature, so it
does not claim corroboration beyond process control. Application to
AI-assisted software governance gates specifically is reasoned from
Bainbridge's original finding, not independently established in this
domain.

## Goodhart's law

**The idea.** Once a measure is turned into an enforced target, it tends
to stop being a reliable measure of the thing it was meant to track,
because people begin optimizing for the measure itself rather than the
underlying goal. The idea is attributed to economist Charles Goodhart,
from a 1975 paper on UK monetary policy, and is commonly restated as
"when a measure becomes a target, it ceases to be a good measure."

**The failure mode it exposes.** Any metric a governance system starts
enforcing — pass rate, review turnaround time, findings per review — will
tend to be gamed once people know it is being watched, even without
anyone consciously deciding to cheat.

**The design question.** If everyone in this system started optimizing
directly for this specific metric, would the thing the metric was
supposed to protect actually get better, or worse?

**Evidence status:** Theoretical. This is a widely restated heuristic
rather than a single controlled empirical study — treat it as a strong
prior worth checking, not a proven law for every metric in every context.

## Psychological safety versus surveillance

**The idea.** In a study of 51 work teams, psychological safety — feeling
able to admit mistakes and uncertainty out loud — was associated with more
team learning behavior, which was in turn associated with better team
performance (Edmondson, "Psychological Safety and Learning Behavior in
Work Teams," *Administrative Science Quarterly*, 44, 350–383, 1999). The
mechanism: problems surface early instead of staying hidden until they
become expensive.

**The failure mode it exposes.** A comprehensive audit trail — recording
every action for governance purposes — can suppress the same candor it
depends on to work. People are less likely to say "I'm not sure this is
right" out loud when that uncertainty becomes a permanent, attributable
record that someone else can read later, out of context.

**The design question.** What exactly gets recorded, who can read it, for
how long, and could someone reasonably fear consequences for admitting a
mistake or an uncertainty inside that record?

**Evidence status:** Theoretical. Edmondson's underlying psychological-safety
research is a well-established, independently corroborated empirical
program. Its specific application here — that governance audit trails can
suppress candor — is a reasoned extension of that research to a context
(permanent, attributable, third-party-readable technical records) it was
not originally designed to test.

## Proceduralism versus outcome-based regulation

**The idea.** Some governance and regulatory approaches judge whether the
right process was followed (proceduralism); others judge whether the
actual outcome was acceptable (outcome- or risk-based regulation). These
reflect different bets about what is controllable and knowable in
advance.

**The failure mode it exposes.** A governance system can satisfy every
procedural requirement — every gate passed, every record filed — while
the outcome it was meant to protect against still goes wrong, because
following a process is not the same as achieving a result.

**The design question.** Does this system claim to guarantee outcomes, or
does it produce evidence that supports someone else's outcome judgment?
Is that distinction stated plainly to the people relying on it, or left
to be assumed?

**Evidence status:** Theoretical. This lens is a comparison between two
well-known regulatory design philosophies rather than a claim resting on
a single citable source — closer to a framing device than an empirical
finding — but it uses the same vocabulary as the other five lenses rather
than an invented category, per this page's own stated rule that every
lens carries a vocabulary-valid label.

## How to use these lenses

Pick a specific gate, rule, or approval step in the governance system
under review — this one or any other — and ask each design question
against it directly. A lens that produces no useful answer for a given
system is not a defect in the lens; not every governance design is
vulnerable to every failure mode described here. The point is to have
asked, and to be able to say what the answer was.

## Next

- How AGF scopes what it claims to guarantee and what it leaves to human
  judgment: [AGF and agent runtimes](position.md)
- How evidence accumulates and what evidence-status labels mean:
  [Learning loop and evidence](evidence-and-learnings.md)
