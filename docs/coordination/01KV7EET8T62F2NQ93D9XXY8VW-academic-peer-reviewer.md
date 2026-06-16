---
kind: review
from: academic-peer-reviewer
to: governor-reviewer
id: 01KV7EET8T62F2NQ93D9XXY8VW
timestamp: 2026-06-16T05:28:52Z
immutable: true
references-to:
  - 01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md
topic: governing-the-governors-academic-review-2026-06-16
requires-ack: false
artifacts_owned: []
---

# Academic peer review: governing-the-governors.tex

**Artifact:** `docs/academic/governing-the-governors.tex`
**Snapshot reviewed:** 2026-06-16 (post-refresh, post-Gate-7-cleanup commits)
**Prior reviews consulted:** `docs/academic/reviews/2026-06-13-academic-peer-review.md`,
`docs/academic/reviews/2026-06-13-academic-content-readability-layout-review.md`

Prior reviews addressed: method evidence underspecification, conclusion
overclaiming, citation corrections, readability, and evidence traceability. This
review covers gaps those passes did not address, primarily methodology framing,
epistemological design, and research question structure.

---

## CRITICAL — Missing design science methodology anchor

The paper is a design-case study of an artifact (the governance framework) built
using the same artifact. This is design science research as defined by Hevner,
March, Park, and Ram (2004), which established the two-cycle principle: a design
cycle (build/evaluate the artifact) and a relevance cycle (ground the problem in
real-world context) \cite{hevner2004design}. The paper describes both cycles but
never names the methodology, which leaves it exposed to a predictable reviewer
complaint: "How do we know what kind of study this is?"

**Required:** Cite Hevner et al. (2004) and/or Wieringa (2014) *Design Science
Methodology for Information Systems and Software Engineering* in Section 4
(Method). One sentence is enough: "The self-application method follows the
design science research tradition." The absence is conspicuous given how
precisely the paper otherwise defines its evidence claims.

**Secondary:** The paper's evaluation criteria ("a governance mechanism is useful
in this self-study when it makes an obligation observable, gives reviewers a
durable artifact to inspect, and is either checked by a deterministic tool or
clearly identified as convention-only") is a design-evaluation rubric, not an
empirical measure. That is fine — but the paper should label it as such.

---

## HIGH — Epistemological circularity deserves sharper treatment

The paper acknowledges circular-validation risk in two places (Sections 3 and 8)
but treats it as a limitation rather than engaging with it analytically. The
circularity is more interesting than the paper implies: the framework defines
what counts as evidence (typed artifacts, coordination records, learning records),
the self-study collects exactly that evidence, and the paper then uses that
evidence to support claims about the framework. An independent reviewer cannot
escape this circle using the paper's own resources.

The paper's current response is: "Its value is exposure: before release,
governance mechanisms must encounter stale handoffs, wrong-branch work, missing
advisory roles…" This is correct but insufficient. A stronger treatment would:

1. Acknowledge that self-application evidence is internally valid but
   methodologically bounded (it tells you the framework can handle its own
   work style; it tells you nothing about other work styles).
2. State explicitly what external evidence would be required to move beyond
   internal validity: at minimum, one independent downstream project, independent
   role holders, or one failed adoption attempt.
3. Reference the design science evaluation tradition's distinction between
   formative evaluation (during design) and summative evaluation (after release).
   The paper is reporting formative evidence; readers should know that.

**Actionable:** Add two to three sentences to Section 5.1 (or Section 8) that
name the validity boundary explicitly. "This self-study provides formative
evaluation evidence. External validity requires independent adoption, independent
role holders, and evidence from at least one non-successful case."

---

## HIGH — Research questions are design outcomes, not research questions

RQ1 ("what becomes observable"), RQ2 ("which failures become detectable"), RQ3
("which capabilities are enforced") are answered entirely by inspecting the
design artifact. They cannot be falsified by any evidence other than evidence
the designers produce. This is correct for design-case questions, and the paper
calls them "design-case questions" in Section 1.2 — but then answers them in an
Observations section structured like an empirical results section.

The paper would be stronger with one of two adjustments:

**Option A:** Rename the RQs as Design Goals (DG1–DG3) and frame Section 5 as
a design evaluation against those goals. This aligns with design science
convention and drops the pretense that the questions could have been answered
otherwise.

**Option B:** Keep them as RQs but rewrite the answers in Section 5 to include
explicit falsification conditions — "RQ1 would be answered negatively if artifact
classes X, Y, Z were not retrievable from the repository by a naive reader." This
gives external reviewers something to check.

Either option closes the gap. Option A is lower-effort.

---

## MEDIUM — Abstract overloads the literature listing

The abstract currently lists nine literature areas in one sentence ("software
evolution, multi-agent communication, software quality gates, continuous
improvement, health systems governance assessment, AI governance documentation,
responsible AI governance tools, and maturity models for healthcare AI
governance"). For an IEEE conference abstract (typically read under time
pressure), this reads as a keyword dump rather than a positioning statement.

**Recommended:** Collapse to three anchors that map to the paper's actual
contributions: (1) multi-agent coordination, (2) governance maturity and
assessment, (3) AI documentation and responsible AI. Remove healthcare AI
governance from the abstract — it appears in related work but is not the
paper's target domain.

---

## MEDIUM — Table II maturity categories are ad hoc

The maturity table (Table II, "Governance mechanism maturity") uses the
categories: schema-validated, validator-enforced, documented/schema-shaped,
planned/partial, warning-only, conventional/rule-candidate. These are coherent
but not grounded in any maturity framework. A reviewer will ask why these
specific levels were chosen.

Two options:

1. Explicitly reference the project's own five-level domain maturity model
   (Planned → Schema-defined → Instantiated → Validated → Deployable) and show
   that Table II maps to it. This is actually already defined in the adopted
   rules (`implementation-honesty-rule.md`) and would give the table a citable
   internal reference.

2. Compare briefly to CMMI or ISO/IEC 15504 (SPICE) to show why a project-
   specific scale is needed, then define the custom levels.

Option 1 is better for consistency: the paper should be an example of applying
its own documentation rules.

---

## MEDIUM — Corpus discussion belongs in Future Work

Section 6 (Discussion) contains a five-element list proposing "a curated corpus
of governed-agent behavior." This proposal adds no analytical weight to the
Discussion — it is a future deliverable, not an interpretation of current results.

**Recommended:** Move the corpus proposal (lines beginning "A near-term
opportunity…") to Section 7 (Future Work), where it fits naturally alongside
the corpus evaluation asset paragraph already present. The Discussion section
will then stay focused on interpreting the observations against the literature.

The federated-learning paragraph should move with it or be cut. The explicit
qualifier ("Federated learning provides only a loose structural analogy") is
correct but invites distraction from the paper's actual contributions.

---

## LOW — Contributions 4 and 5 need honesty about status

Contribution 4 ("argues that continuous improvement requires structured learning
records and explicit promotion paths") and Contribution 5 ("situates the
framework within recent AI governance literature") are framing contributions,
not empirical findings. The paper already acknowledges this ("the final two are
framing and agenda-setting contributions"), which is good — but in the
Contributions section this distinction should be stated before the list, not
after, so readers arrive at the evidence-supported contributions knowing what
kind of paper this is.

Suggested reordering: state contributions 1–3 first with the evidence note, then
contributions 4–5 under a subhead "Framing contributions." This prevents a
reviewer from counting five novel contributions and finding only three.

---

## LOW — Role/actor separation undersells a genuine contribution

Section 4.2 contains what may be the paper's most distinctive practical
contribution: the separation of governance roles from actors and the principle
that "durable coordination routes through roles rather than model identities."
This is a design principle with concrete implications for AI governance (a
specific AI model or session can be replaced without losing governance history;
role authority is defined by artifact, not by capability). The current text
buries this in a paragraph rather than foregrounding it as a named contribution.

**Recommended:** Name it explicitly in the Contributions section ("separates
governance roles from actors, enabling durable coordination that survives actor
substitution") and add a brief illustrative example in Section 4.2: what
concretely happens when the model that held the coder-tester role is replaced
mid-project?

---

## LOW — Two writing issues in current version

1. **Section 6, Discussion opening:** "Foundry's main design choice is to treat
   governance as process data." — this is the paper's best one-line summary.
   It should appear in the Abstract and Introduction, not debut in Discussion.

2. **Section 4, Method reproducibility:** "Some evidence remains branch-local
   until merge" contradicts "reproducibility is limited but concrete." If key
   evidence is branch-local, reproducibility is not yet concrete for an external
   reader. Remove "but concrete" or qualify it more precisely ("concrete for a
   reviewer with repository access after merge").

---

## What to do next

Priority order for next revision pass:

1. Add design science citation (Hevner et al. 2004) — one sentence, closes the
   methodology gap for reviewers.
2. Add explicit validity-boundary statement to Section 5.1 or Limitations —
   two to three sentences, addresses circularity analytically rather than
   defensively.
3. Rename RQs to Design Goals or add falsification conditions — either option
   is acceptable.
4. Move corpus discussion to Future Work.
5. Tighten Abstract (drop five literature areas).
6. Add Table II grounding (map to implementation-honesty-rule levels).

Items 1–4 are blockers before external submission. Items 5–6 are improvements
that raise the paper's chance of acceptance.

The paper's intellectual core is sound: the self-application result, the
episode table, and the maturity distinction are genuine contributions. The
revision needed is to the framing of epistemological status, not to the
substance.
