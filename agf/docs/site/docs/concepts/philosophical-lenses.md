# Philosophical lenses on AI governance

*Audience: developers, evaluators, and AI systems evaluating this or any
AI governance framework. After this page you have twelve structured
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
own vocabulary. The twelve lenses below come from organizational theory,
economics, human-factors research, decision theory, and knowledge
management — not from AI governance specifically — each names a way that
*any* accountability, measurement, oversight, or learning system can
fail, independent of what it is overseeing.

The first six focus on oversight and accountability failure modes. The
last six focus specifically on how governance systems learn — or fail to
learn — from their own evidence.

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

## Observe-Orient-Decide-Act (OODA)

**The idea.** Boyd's decision cycle — observe, orient, decide, act — came
from analyzing why US F-86 pilots beat the technically superior MiG-15 in
Korean War dogfights: not better aircraft, but faster cycling through the
decision loop than the opponent could manage. The richest step is
Orient — raw observations don't inform a decision until they're filtered
through prior experience, mental models, and context. Skip Orient and
you're not deciding, you're just reacting.

**The failure mode it exposes.** A governance system built for speed can
collapse Observe straight into Act, skipping real orientation — review
becomes pattern-matching against a summary instead of evidence filtered
through independent judgment. This is specifically a fast-loop failure:
it shows up when several actors are reacting to each other's output in
near-real time and the pressure is to keep pace, not to reason.

**The design question.** When this system reacts quickly to new input —
a new finding, a new message, a new review — does the reaction pass
through genuine orientation (independent verification, an actual model
of the situation), or does it move straight from observing the input to
acting on it?

**Evidence status:** Theoretical. Boyd's model is a widely cited
strategic-decision framework, but its origin is adversarial-tempo
decision-making — out-cycling an opponent. Applying it to non-adversarial,
collaborative multi-agent review is an analogical extension this lens
should be honest about, not a direct transplant.

## Plan-Do-Check-Act (PDCA)

**The idea.** A four-step iterative improvement cycle — Plan, Do, Check,
Act — refined by Walter Shewhart in the 1920s as an analogy to the
scientific method (hypothesis, experiment, evaluation), and popularized
by Deming as the "Shewhart Cycle," later PDCA. Deming later relabeled it
PDSA — Plan-Do-**Study**-Act — specifically because "Check" invited
superficial verification instead of genuine analysis of what happened.

**The failure mode it exposes.** The cycle degrades if any step runs in
name only. Most commonly it's Check: comparing outcome to plan without a
real baseline or an honest count of what changed turns "continuous
improvement" into a self-congratulatory loop that never tests whether the
Act step actually worked — the exact drift Deming's own PDCA-to-PDSA
rename was trying to correct.

**The design question.** For this system's own "we learned something, we
changed a rule" cycle — is there a real, pre-stated Check (a named
baseline, a measurable comparison) before a learning record moves to Act,
or does it jump straight from "we noticed something" to "we changed the
rule"?

**Evidence status:** Theoretical. PDCA/the Shewhart cycle is long
established in quality management and manufacturing, but that track
record is domain-specific. Applying its discipline to AI-governance rule
evolution is a reasoned analogy, not something independently validated in
this domain.

## Single-loop versus double-loop learning

**The idea.** Argyris and Schön distinguish two kinds of organizational
learning. Single-loop learning corrects an error while leaving the
underlying rule or assumption unquestioned — do the same thing, better,
next time. Double-loop learning questions and revises the assumption
itself (Argyris & Schön, *Organizational Learning: A Theory of Action
Perspective*, Addison-Wesley, 1978).

**The failure mode it exposes.** A learning system can look productive —
records accumulate, findings get filed — while every single one is
single-loop: fixing the instance, never revising the rule or process that
produced it. This is invisible from inside, because both kinds of
learning produce documented output; only one of them changes the
governing assumption.

**The design question.** When a learning record gets filed, does it
change the specific defect, or does it ever change the rule or process
that made the defect possible in the first place — and how would a
reader tell the difference just from the record?

**Evidence status:** Theoretical. A well-established organizational-theory
framework; whether this or any specific learning-record mechanism
actually produces double-loop learning at any measurable rate is not
something this page tests or claims.

## Espoused theory versus theory-in-use

**The idea.** People and organizations hold two theories of their own
action: the espoused theory — what they say they do, consciously held
and stated — and the theory-in-use — what actually guides behavior,
visible only by inferring it from what actually happens. The two can
diverge without anyone noticing (Argyris & Schön, *Theory in Practice:
Increasing Professional Effectiveness*, Jossey-Bass, 1974).

**The failure mode it exposes.** A governance system can state its
process precisely — the plan, the rule, the claim "applied in full" —
while the actual diff, commit, or log tells a different story. The gap is
dangerous specifically because both sides are sincere: no one is lying,
the divergence is just unexamined.

**The design question.** For a specific claim this system makes about
itself — "this rule is enforced," "this fix is applied," "this gate
always runs" — has anyone actually checked the artifact against the
claim, or does the claim rest on the claim alone?

**Evidence status:** Theoretical. Well established in
organizational-behavior research; its application here is a direct
structural analogy (a documentation claim versus the actual diff), not
something separately tested for AI-governance systems.

## The Cynefin framework

**The idea.** Snowden and Boone sort problems into five domains: Simple
(cause and effect clear to everyone, best practice applies), Complicated
(cause and effect knowable but requires expertise, good practice
applies), Complex (cause and effect only clear in retrospect, requires
probing and sensing rather than a pre-set process), Chaotic (no clear
cause and effect, requires acting first to establish order), and Disorder
(not knowing which of the other four domains you're actually in) (Snowden
& Boone, "A Leader's Framework for Decision Making," *Harvard Business
Review*, November 2007).

**The failure mode it exposes.** A governance process built for
Complicated problems — a fixed checklist, a fixed gate sequence —
assumes the situation is knowable in advance by an expert. Applied to a
genuinely Complex or novel situation, following the checklist produces
false confidence: the gate passed, but passing it was never evidence the
actual, unprecedented risk was addressed.

**The design question.** Is the situation this gate is being applied to
actually Complicated — an expert could work out the right answer in
advance — or Complex — the right answer only becomes clear after trying
something and observing the result? Does the process allow probing and
adapting when the answer is Complex, or does it force a
Complicated-style checklist regardless?

**Evidence status:** Theoretical. The framework itself is well regarded
(HBR, Academy of Management recognition); classifying this system's own
governed changes by Cynefin domain, or testing whether its gates actually
adapt to domain, is not something this page does.

## Knowledge creation and the missing "ba" (Nonaka, Takeuchi, Konno)

**The idea.** Organizational knowledge moves through four conversions, in
a spiral, not a line: Socialization (tacit knowledge shared through
direct experience), Externalization (tacit knowledge articulated into
explicit concepts, models, documents), Combination (separate explicit
pieces systematized into something coherent), Internalization (explicit
knowledge practiced until it becomes fluent judgment again) (Nonaka &
Takeuchi, *The Knowledge-Creating Company: How Japanese Companies Create
the Dynamics of Innovation*, Oxford University Press, 1995). Each
conversion mode requires *ba* — a shared context for emerging
relationships, which can be physical, virtual, or mental, but must
actually be shared and actually evolving (Nonaka & Konno, "The Concept of
'Ba': Building a Foundation for Knowledge Creation," *California
Management Review*, 40, 1998, 40–54).

**The failure mode it exposes.** A governance system can be excellent at
Externalization and Combination — articulating judgment into documents,
systematizing documents into rules — while starving Socialization,
because Socialization is the one mode that produces no artifact of its
own. An append-only, asynchronous coordination channel is not
automatically *ba* just because it's shared: *ba* requires an evolving
relational context, and a channel built for durable, immutable, one-way
records is a specific, weak instance of that — well suited to
Externalization, poorly suited to the tacit-to-tacit transfer that lets a
new participant actually absorb *why* a decision was made, not just
*what* was decided.

**The design question.** When a new participant — human or AI — joins
this governance process, do they inherit working judgment (why this
call, not just what call), or only the explicit artifacts (the rules,
the records) with the tacit reasoning behind them never actually
transferred? What, if anything, in this system does the job Socialization
does?

**Evidence status:** Theoretical. The SECI model and the concept of *ba*
are well-established, widely cited contributions to organizational-
knowledge research. Their application here — that an artifact-only
coordination channel may be structurally weak at Socialization
specifically — is a structural inference from the model, not something
this page tests empirically against this or any governance channel.

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
  [Evidence and learnings](evidence-and-learnings.md)
