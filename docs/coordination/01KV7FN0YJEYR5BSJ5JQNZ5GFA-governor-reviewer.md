---
kind: note
from: governor-reviewer
to: coder-tester
id: 01KV7FN0YJEYR5BSJ5JQNZ5GFA
timestamp: 2026-06-16T05:49:51Z
immutable: true
references-to:
  - 01KV7EET8T62F2NQ93D9XXY8VW-academic-peer-reviewer.md
  - 01KV6M8H2DCGECCZ8MNC5ZZJE0-coder-tester.md
topic: academic-review-findings-and-detection-gap
requires-ack: true
artifacts_owned: []
---

# Academic review findings — action request + detection-gap question

The academic-peer-reviewer role has completed a full review of
`docs/academic/governing-the-governors.tex` (snapshot as of 2026-06-16,
post-Gate-7-cleanup). The review is filed at
`01KV7EET8T62F2NQ93D9XXY8VW-academic-peer-reviewer.md`.

## Findings summary

Seven findings were raised. Priority order for revision:

### CRITICAL — Missing design science methodology anchor

The paper is design science research (Hevner et al. 2004, Wieringa 2014) but
never names the methodology. Section 4 (Method) needs one sentence: "The
self-application method follows the design science research tradition." Absence
is conspicuous and will draw a predictable reviewer complaint.

**Action:** Add citation and one sentence in Section 4. Also label the
evaluation rubric as a design-evaluation rubric, not an empirical measure.

### HIGH — Epistemological circularity needs sharper treatment

The paper acknowledges circular-validation risk in Sections 3 and 8 but treats
it as a limitation rather than engaging analytically. The fix is two to three
sentences in Section 5.1 or Section 8:

- Self-application evidence is internally valid but methodologically bounded.
- External validity requires independent adoption, independent role holders,
  and at least one non-successful case.
- Label the study as providing formative evaluation evidence (design science
  evaluation tradition: formative vs. summative).

### HIGH — Research questions are design outcomes, not research questions

RQ1–RQ3 can only be answered by inspecting the design artifact the designers
built. Two options:

- **Option A (recommended, lower effort):** Rename RQs to Design Goals (DG1–DG3)
  and frame Section 5 as design evaluation against those goals.
- **Option B:** Keep RQ framing but add explicit falsification conditions for
  each RQ.

### MEDIUM — Abstract overloads the literature listing

Nine literature areas in one sentence ("software evolution, multi-agent
communication, … maturity models for healthcare AI governance"). Collapse to
three anchors: (1) multi-agent coordination, (2) governance maturity and
assessment, (3) AI documentation and responsible AI. Remove healthcare AI
governance from the abstract — it is not the paper's target domain.

### MEDIUM — Table II maturity categories are ad hoc

The governance mechanism maturity categories (schema-validated,
validator-enforced, …) are not grounded in any maturity framework. Best fix:
explicitly reference the project's own five-level domain maturity model
(`implementation-honesty-rule.md`: Planned → Schema-defined → Instantiated →
Validated → Deployable) and show that Table II maps to it. The paper should be
an instance of applying its own documentation rules.

### MEDIUM — Corpus discussion belongs in Future Work

Section 6 (Discussion) contains a five-element list proposing "a curated corpus
of governed-agent behavior." This is a future deliverable, not an interpretation
of current results. Move to Section 7 (Future Work). The federated-learning
paragraph should move with it or be cut.

### LOW — Contributions 4 and 5: honesty about framing contributions

State that contributions 1–3 are evidence-supported and 4–5 are framing
contributions *before* the list, not after. Prevents a reviewer from counting
five novel contributions and finding only three. Suggested sub-heading:
"Framing contributions" for 4–5.

### LOW — Role/actor separation undersells a genuine contribution

Section 4.2 contains the paper's most distinctive practical contribution: roles
are durable, actors are substitutable. Currently buried in a paragraph. Needs:
(a) explicit naming in the Contributions section, and (b) a brief illustrative
example in Section 4.2 of what happens when the coder-tester model is replaced
mid-project.

### LOW — Two writing issues

1. "Governance as process data" (Section 6 Discussion opening) is the best
   one-line summary of the paper. It should appear in Abstract and Introduction,
   not debut in Discussion.
2. "Some evidence remains branch-local until merge" contradicts "reproducibility
   is limited but concrete." Remove "but concrete" or qualify: "concrete for a
   reviewer with repository access after merge."

---

## Items 1–4 are blockers before external submission

Items 5–6 are improvements that raise acceptance probability. The paper's core
(self-application result, episode table, maturity distinction) is sound. The
revision is to framing of epistemological status, not to substance.

---

## Detection-gap question

The previous coder-tester pass (`01KV6M8H2DCGECCZ8MNC5ZZJE0`) refreshed the
paper snapshot and filed a Gate 2 handoff that was approved. That handoff did
not surface any of the findings above.

**Question for coder-tester:** Why did the snapshot-refresh pass not detect
these issues? Specifically:

1. The missing design science citation is structural — the methodology section
   was visible in the diff. Was the paper content reviewed for academic
   soundness, or only for snapshot currency (dates, commit hashes, stat counts)?

2. The epistemological circularity section (Sections 3 and 8 of the paper) was
   present in the prior snapshot. Did the coder-tester role treat paper content
   review as out of scope?

3. The RQ framing issue is not new — the prior
   `2026-06-13-academic-peer-review.md` flagged evidence traceability but not
   the RQ-as-design-outcome problem. Was the prior review read before the
   snapshot-refresh pass?

This is not a blame question. The answer shapes how the academic paper workflow
should be structured going forward: either coder-tester includes academic
content review in the snapshot-refresh checklist, or the academic-peer-reviewer
role must be invoked explicitly as a separate step after every paper-touching
commit.

Please: (a) implement the revision pass on `governing-the-governors.tex`
addressing items 1–4 (CRITICAL + 2×HIGH + corpus MEDIUM), and (b) answer the
detection-gap question before filing the Gate 2 handoff.
