---
title: "Governing the Governors: Self-Application as Evidence for Multi-Agent AI Development Governance"
codename: foundry
status: archived-wip-low-priority
date: 2026-06-13
audience: researchers, framework adopters, peer reviewers
destination: internal
note: |
  ARCHIVED: This Markdown file is retained as the prose-source provenance draft.
  The maintained academic source is governing-the-governors.tex with BibTeX
  references in references.bib.
  WIP / LOW PRIORITY: The academic paper is an internal working draft and
  background artifact. It should not displace higher-priority framework
  development, validator, CI, documentation-site, or downstream-adoption work.
  EXTERNAL ANONYMIZATION: This draft uses the project codename "foundry".
  Verify author names, publication metadata, and anonymization requirements
  before external circulation.
---

# Archived Markdown Draft

This file is archived and low priority. Use
`docs/academic/governing-the-governors.tex` and
`docs/academic/references.bib` for academic review and PDF generation. The
paper is a WIP internal draft, not an external submission artifact.

# Governing the Governors: Self-Application as Evidence for Multi-Agent AI Development Governance

## Abstract

Multi-agent AI development increasingly depends on roles, tools, and review
processes that are themselves difficult to inspect. This paper presents
*foundry*, a self-applied governance framework for multi-agent software
development. The project is both the object being built and the governance
environment used to build it. We argue that this recursive setup is not a
novelty but a useful evidence mechanism: governance rules, validators, and
coordination protocols are exercised under the same operational pressure they
are meant to regulate.

The framework combines immutable coordination records, machine-checkable
schemas, deterministic validators, gate-based review, advisory roles, and
structured learning records. We position this work against prior literature
on software evolution, multi-agent communication, software quality gates,
continuous improvement, health systems governance assessment, AI governance
documentation, responsible AI governance tools, and maturity models for
healthcare AI governance. The paper's central claim is intentionally narrow:
governance tooling must itself be observable, enforceable, revisable, and
governed by construction. Otherwise the process intended to produce assurance
can become an unvalidated source of drift.

## 1. Introduction

AI development governance is often described at the level of principles,
policies, or organizational responsibilities. Those descriptions matter, but
they leave a practical question open: what governs the governance work itself?
When a plan is approved, a role is activated, a validator is changed, or a
coordination protocol evolves, the change affects the process that will later
judge other changes. A governance framework that cannot inspect its own
evolution risks becoming a source of hidden policy rather than a control
surface.

The *foundry* project addresses this problem by using its current governance
system to build its successor. The development process is mediated through
structured coordination messages, Gate 1 planning review, Gate 2 implementation
review, role manifests, advisory consultation, schema validation, and learning
records. These artifacts are stored in version control and are intended to be
readable by humans and machine-checkable by validators.

This paper turns the project's development diary into an academic working
paper. The diary form was useful for discovery; the paper form is useful for
argument. We do not claim that foundry solves all AI governance problems, nor
that self-application proves external effectiveness. The claim is more modest:
self-application creates a high-friction, evidence-rich environment in which
governance mechanisms must demonstrate that they can survive ordinary
development work.

## 2. Contributions

This paper makes five contributions:

1. It frames self-application as an evidence mechanism for governance
   frameworks rather than merely as "dogfooding".
2. It describes a coordination protocol in which agent communication is an
   immutable, append-only, typed record.
3. It connects governance validators to software quality gates and data quality
   enforcement.
4. It argues that continuous improvement requires structured learning records
   and explicit promotion paths from incident to rule or validator.
5. It situates the framework within recent AI governance and broader governance
   assessment literature, including maturity models, documentation studies,
   actor/stage-specific governance tools, and health systems governance review.

## 3. Related Work

### 3.1 Software Evolution and Maintenance

Lehman's laws of software evolution establish that useful software systems
require continuing adaptation as their environments change. For governance
software, the implication is sharper: governance mechanisms must themselves be
governed, or they calcify while the work they regulate changes around them.
Swanson's corrective, adaptive, and perfective maintenance categories also map
onto governance work. A missing approval record is corrective maintenance at
the governance layer. A role vocabulary updated to match the coordination
schema is adaptive maintenance. A usage reporter that improves observability is
perfective maintenance.

### 3.2 Multi-Agent Communication and Commitments

The coordination protocol draws from speech-act theory and multi-agent systems.
Austin and Searle established that utterances can perform institutional acts,
not merely describe them. FIPA's agent communication language carried this
insight into deployed agent systems through typed communicative acts.
Wooldridge, Shoham, and Leyton-Brown emphasize that agent communication must
make commitments visible enough to coordinate action. Foundry applies this
idea directly: a `handoff`, `decision-record`, `review`, `claim`, or
`completion` is a typed action in a governance log.

### 3.3 Quality Gates and Machine-Checkable Governance

Software quality gate research argues that process controls become reliable
when they are executable and reproducible. Foundry treats schemas and
validators as governance gates. A malformed coordination message, missing
advisory consultation, inactive role, or stale handoff should not rely on human
memory; it should be detected by deterministic tooling with clear failure
messages.

### 3.4 Governance Assessment, Maturity, and Resource Sensitivity

Governance assessment frameworks predate the current AI wave. Pyone, Smith,
and van den Broek's systematic review of health systems governance assessment
frameworks provides an older grounding for the idea that governance can be
evaluated through structured dimensions rather than treated as an informal
property. Biesenthal and Wilden's review of multi-level project governance
similarly supports the transition from single-project governance to portfolio,
organizational, and enterprise settings.

Recent AI governance work makes resource sensitivity especially visible.
Hussein et al. propose HAIRA, a five-level healthcare AI governance maturity
model derived from a systematic review of 35 healthcare AI governance
frameworks. The value of HAIRA for foundry is not its clinical specificity but
its structure: governance maturity is staged, domain-specific, and constrained
by organizational resources. Foundry's adoption guidance should follow the same
principle. A small downstream project should not need an enterprise compliance
department to use a useful subset of the framework.

### 3.5 AI Governance Documentation, Actors, and Corpora

Robles and Mallinson argue that public AI governance remains fragmented and
requires more cohesive frameworks. Papagiannidis, Mikalef, and Conboy identify
the corresponding organizational problem: responsible AI literature has many
principles, but limited cohesion on how those principles become practices for
designing, executing, monitoring, and evaluating AI applications. Their
structural, procedural, and relational framing maps directly onto foundry's
role authority, validator and gate mechanics, and advisory coordination model.

Kuehnert et al. provide a systematic review and meta-analysis of actor- and
stage-specific responsible AI governance tools, which is directly relevant to
foundry's division of labor across roles, gates, validators, and documentation
surfaces. Winecoff and Bogen show that AI documentation is not peripheral
polish but an intervention that can improve governance outcomes when it connects
theory to practice.

The AGORA project is also important for foundry's future corpus strategy.
Arnold et al. describe a rigorously compiled and validated archive of AI laws,
policies, and governance concepts. Foundry needs an analogous, smaller corpus
for governed-agent behavior: canonical traces, adversarial examples, validator
fixtures, decision records, and examples of good and failed governance conduct.

Moon et al.'s relational commons framework comes from conservation rather than
AI or software, but it is useful as a broader governance warning: governance is
not only control over bounded artifacts. It is also stewardship of
relationships, shared resources, and power. This matters when foundry documents
downstream impact, exit paths, and decommissioning.

## 4. Method: Self-Application as Evidence

Foundry's method is self-application under version control. The framework is
built using the governance system it is replacing. Each non-trivial change is
expected to pass through:

1. A Gate 1 plan with structured front matter and advisory consultation.
2. Review by the governor-reviewer, supported by advisory roles.
3. Implementation on a scoped branch.
4. Gate 2 review before merge.
5. A coordination record that preserves the decision, completion, or handoff.
6. Learning records when operational friction reveals a reusable rule.

This structure creates evidence in three forms:

1. **Behavior traces.** Coordination messages and commits show what agents did,
   in what order, under what stated authority.
2. **Executable checks.** Validators encode schema and policy constraints that
   can be reproduced locally or in CI.
3. **Reflective records.** Learnings and decision records explain why rules
   changed, not only that they changed.

The method has limits. Self-application is not independent adoption, and it can
overfit the framework to its creators' work style. It is still useful because it
forces governance mechanisms to encounter real failure modes before release:
stale handoffs, wrong-branch work, missing advisory roles, role-status drift,
sequence collisions, and unclear downstream obligations.

## 5. Framework Artifacts

### 5.1 Coordination Records

Coordination messages are Markdown files with YAML front matter. The required
fields include kind, sender, recipient, sequence number, timestamp, immutability
marker, references, topic, acknowledgement requirement, and owned artifacts.
The message kind is a performative: it states what governance action is being
performed. For example, a `decision-record` changes institutional state, while
a `review` reports findings without itself approving a merge.

### 5.2 Schemas and Validators

Foundry uses JSON Schema and Elixir Mix tasks to validate coordination messages,
plans, role manifests, and handoff state. The design principle is conservative:
hardcode invariant governance requirements, but read configurable governance
participants from the declared authority. A recent validator change illustrates
this rule. Gate 1 plans must always include `infosec-sentinel` and
`contribution-auditor`, but additional advisory roles are allowed when they are
active in the manifest. Suspended, skipped, misspelled, or unknown roles remain
invalid.

### 5.3 Gates

Gate 1 verifies intent before implementation. Gate 2 verifies implementation
before merge. This separation mirrors the Plan-Do-Check-Act cycle: planning
reduces conceptual error early, and review prevents implementation drift late.
The gates are intentionally slower than ungoverned development, but their
purpose is to improve orientation quality and reduce hidden risk.

### 5.4 Learning Records

The framework treats operational friction as data. A repeated failure can move
from incident to learning record, from learning record to rule, and from rule to
validator when it can be made deterministic. Wrong-branch work, stale handoff
resolution, and advisory-role validation are examples. This is continuous
improvement with an audit trail: the framework changes because evidence
changed, and the change is itself governed.

### 5.5 Downstream Impact and Exit

A governance framework affects downstream projects by adding artifacts, review
steps, CI checks, and role expectations. Adoption documentation must therefore
state both the expected impact and the exit path. Downstream projects should be
able to identify which files are authoritative, which generated artifacts can be
removed, how CI validators are disabled, how coordination records are archived,
and what residual obligations remain after decommissioning.

## 6. Discussion

Foundry's main design choice is to treat governance as process data. A plan file
is not merely prose; it has validated front matter. A coordination message is
not merely chat; it is an immutable record with type and provenance. A learning
is not merely memory; it is structured evidence that can later justify a rule.
This framing connects AI governance to data governance: process artifacts need
schema, lineage, quality checks, and lifecycle policy.

The recent literature supports this direction. Governance maturity models such
as HAIRA show that capability should be staged and resource-aware. Governance
documentation research shows that documents can improve outcomes when they are
designed as operational tools rather than static declarations. Responsible AI
governance tool surveys show that roles, stages, and artifacts matter. AGORA
shows the value of curated, validated governance corpora. Foundry combines
these ideas at the development-process layer.

The strongest near-term opportunity is a curated corpus of governed-agent
behavior. Such a corpus should include:

1. canonical successful traces suitable for supervised fine-tuning or later
   evaluation;
2. adversarial examples showing common failures;
3. machine-checkable schemas;
4. deterministic validators and negative fixtures;
5. explanations of what good governed-agent conduct looks like;
6. decommissioning and exit examples for downstream projects.

This corpus should not be marketed as training data prematurely. Its first job
is evaluation and pedagogy. If it later becomes suitable for supervised
fine-tuning or reinforcement learning from human feedback, that should follow
from curation quality, licensing clarity, and privacy review.

## 7. Limitations

The project currently has one primary human governor and a small number of AI
agent roles. This limits the evidence for multi-team and enterprise-scale
claims. Self-application also risks circular validation: the framework may
reward behaviors it already expects. Independent downstream adoption is
therefore necessary before strong claims can be made about generality.

The framework also needs clearer measures of governance overhead. Gate latency,
review churn, validator failure rate, override frequency, stale handoff count,
and learning-record promotion rate are plausible metrics, but they require
longitudinal evidence. Finally, documentation must remain honest about maturity.
Some capabilities are planned, some are schema-defined, some are validated, and
some are deployable. These states should not be collapsed into a single
"supported" claim.

## 8. Conclusion

Foundry demonstrates a practical thesis: AI development governance should be
made inspectable as it operates. The framework's value is not that it eliminates
judgment, but that it records, structures, and validates the conditions under
which judgment is exercised. Self-application exposes governance tooling to
real work, creates canonical traces, and turns operational friction into
evidence for improvement.

The next step is not to claim completeness. It is to make the evidence base
stronger: better documentation, clearer exit paths, generated reference
artifacts, curated behavior traces, adversarial examples, and independent
adoption records. Governance that cannot learn from its own use will drift.
Governance that makes its own operation inspectable can improve.

## References

Agha, G. (1986). *Actors: A model of concurrent computation in distributed
systems*. MIT Press.

Argyris, C., & Schon, D. A. (1978). *Organizational learning: A theory of
action perspective*. Addison-Wesley.

Armstrong, J. (2007). *Programming Erlang: Software for a concurrent world*.
Pragmatic Bookshelf.

Arnold, Z., Schiff, D. S., Schiff, K. J., Love, B., Melot, J., Singh, N.,
Jenkins, L., Lin, A., Pilz, K., Enweareazu, O., & Girard, T. (2024).
Introducing the AI Governance and Regulatory Archive (AGORA): An analytic
infrastructure for navigating the emerging AI governance landscape.
*Proceedings of the AAAI/ACM Conference on AI, Ethics, and Society*, *7*(1),
39-48. https://doi.org/10.1609/aies.v7i1.31615

Austin, J. L. (1962). *How to do things with words*. Oxford University Press.

Bateson, G. (1972). *Steps to an ecology of mind*. University of Chicago
Press.

Beck, K. (2002). *Test driven development: By example*. Addison-Wesley.

Biesenthal, C., & Wilden, R. (2014). Multi-level project governance: Trends and
opportunities. *International Journal of Project Management*, *32*(8),
1291-1308.

Brooks, F. P. (1975). *The mythical man-month: Essays on software
engineering*. Addison-Wesley.

Deming, W. E. (1986). *Out of the crisis*. MIT Center for Advanced Engineering
Study.

FIPA. (2002). *FIPA communicative act library specification* (Document
SC00037J). Foundation for Intelligent Physical Agents.

Hewitt, C., Bishop, P., & Steiger, R. (1973). A universal modular ACTOR
formalism for artificial intelligence. In *Proceedings of the 3rd
International Joint Conference on Artificial Intelligence* (pp. 235-245).

Hussein, R., Zink, A., Ramadan, B., Howard, F. M., Hightower, M., Shah, S., &
Beaulieu-Jones, B. K. (2026). Advancing healthcare AI governance through a
comprehensive maturity model based on systematic review. *npj Digital
Medicine*, *9*, Article 236. https://doi.org/10.1038/s41746-026-02418-7

Jafari Meimandi, K., Reuel, A., Aranguiz-Dias, G., Rahama, H., Ayadi, A.-E.,
Boullier, X., Verdo, J., Montanie, L., & Kochenderfer, M. (2025). *An adaptive
responsible AI governance framework for decentralized organizations*. arXiv
preprint arXiv:2510.03368.

Ji, J., Qiu, T., Chen, B., Zhou, J., Zhang, B., Hong, D., Lou, H., Wang, K.,
Duan, Y., He, Z., Vierling, L., Zhang, Z., Zeng, F., Dai, J., Pan, X., Xu, H.,
O'Gara, A., Ng, K., Tse, B., Fu, J., McAleer, S., Wang, Y., Yang, M., Liu, Y.,
Wang, Y., Zhu, S.-C., Guo, Y., Yang, Y., & Gao, W. (2025). AI alignment: A
contemporary survey. *ACM Computing Surveys*. https://doi.org/10.1145/3770749

Khan, R., Joyce, D., & Habiba, M. (2025). *AGENTSAFE: A unified framework for
ethical assurance and governance in agentic AI*. arXiv preprint
arXiv:2512.03180.

Kuehnert, B., Kim, R., Forlizzi, J., & Heidari, H. (2025). The "who", "what",
and "how" of responsible AI governance: A systematic review and meta-analysis
of (actor, stage)-specific tools. In *Proceedings of the 2025 ACM Conference
on Fairness, Accountability, and Transparency* (pp. 2991-3005).
https://doi.org/10.1145/3715275.3732191

Lehman, M. M. (1980). Programs, life cycles, and laws of software evolution.
*Proceedings of the IEEE*, *68*(9), 1060-1076.

McMahan, H. B., Moore, E., Ramage, D., Hampson, S., & y Arcas, B. A. (2017).
Communication-efficient learning of deep networks from decentralized data. In
*Proceedings of the 20th International Conference on Artificial Intelligence
and Statistics*.

Moon, K., Marsh, D., Cooke, B., & Kingsford, R. (2025). Relational Commons: An
ontological and governance framework beyond protected areas and the boundaries
of conservation. *Conservation Letters*, *18*(5), e13137.
https://doi.org/10.1111/conl.13137

Papagiannidis, E., Mikalef, P., & Conboy, K. (2025). Responsible artificial
intelligence governance: A review and research framework. *The Journal of
Strategic Information Systems*, *34*(2), 101885.
https://doi.org/10.1016/j.jsis.2024.101885

Parnas, D. L. (1972). On the criteria to be used in decomposing systems into
modules. *Communications of the ACM*, *15*(12), 1053-1058.

Pyone, T., Smith, H., & van den Broek, N. (2017). Frameworks to assess health
systems governance: A systematic review. *Health Policy and Planning*, *32*(5),
710-722. https://doi.org/10.1093/heapol/czx007

Qin, X., Luan, S., See, J., Boukhers, Z., Yang, C., & Li, Z. (2026). *Harnessing
embodied agents: Runtime governance for policy-constrained execution*. arXiv
preprint arXiv:2604.07833.

Robles, P., & Mallinson, D. J. (2023). Catching up with AI: Pushing toward a
cohesive governance framework. *Politics & Policy*, *51*(3), 355-372.
https://doi.org/10.1111/polp.12529

Russell, S., & Norvig, P. (2021). *Artificial intelligence: A modern approach*
(4th ed.). Pearson.

Searle, J. R. (1969). *Speech acts: An essay in the philosophy of language*.
Cambridge University Press.

Shoham, Y., & Leyton-Brown, K. (2009). *Multiagent systems: Algorithmic,
game-theoretic, and logical foundations*. Cambridge University Press.

Simon, M., & Buhnova, B. (2024). Quality gates in software development:
Concepts, definition and tools. *Proceedings of the 12th Workshop on Software
Quality Analysis, Monitoring, Improvement, and Applications*.

Swanson, E. B. (1976). The dimensions of maintenance. In *Proceedings of the
2nd International Conference on Software Engineering* (pp. 492-497).

Winecoff, A., & Bogen, M. (2025). Improving governance outcomes through AI
documentation: Bridging theory and practice. In *Proceedings of the 2025 CHI
Conference on Human Factors in Computing Systems* (pp. 1-18).
https://doi.org/10.1145/3706598.3713814

Wooldridge, M. (2009). *An introduction to multiagent systems* (2nd ed.). John
Wiley & Sons.
