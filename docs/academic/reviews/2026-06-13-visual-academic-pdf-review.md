# Visual Academic PDF Review

Date: 2026-06-13

Reviewed artifact:

- `docs/academic/governing-the-governors.pdf`

Reviewer stance:

- Visual academic review of the rendered PDF.
- Focused on layout, figure legibility, caption usefulness, page flow,
  citation-link presentation, bibliography placement, and public-circulation
  readiness.
- Did not reassess the paper's full argument or factual claims.

## Recommendation

Revise before public circulation or submission.

The PDF is coherent and academically recognizable, but three visual issues
weaken credibility: Figure 1 interrupts a sentence across pages, all three
figures are under-scaled, and green boxed citation links visually dominate
dense pages.

## Findings

### High: Figure 1 interrupts sentence-level reading flow

Page 1 ends mid-sentence ("We do"), then page 2 opens with Figure 1 before the
sentence resumes. This is the strongest layout issue because it disrupts the
reader's basic path through the introduction.

Suggested fix: move Figure 1 after the paragraph or place it with a stricter
float policy so it cannot interrupt a sentence across pages.

### Medium: Figures are too small for academic reading

Figures 1, 2, and 3 support the argument conceptually, but their internal node
and edge labels are difficult to read in the rendered PDF. Figure 1 is the
weakest; Figures 2 and 3 are better placed but still compressed.

Suggested fixes:

- simplify labels;
- increase figure height or rotate selected diagrams;
- split long workflow diagrams into staged panels;
- consider using fewer nodes per figure for the paper, while retaining richer
  SVGs for HTML documentation.

### Medium: Green boxed citation links are visually distracting

The default hyperref boxes around citations are prominent on pages 2-9. They
are acceptable in a draft but look unpolished for academic submission or public
release.

Suggested fix: use `hidelinks` or a restrained link color configuration for
the PDF.

### Low: Figure placement is otherwise logical

Figure 2 appears near the gates discussion, and Figure 3 appears near learning
records and downstream impact. Captions are clear and argumentative.

### Low: Bibliography placement is sound

References begin cleanly and continue without obvious layout damage. No missing
figures, orphaned captions, or broken bibliography rendering were observed.

## Disposition

Do not block continued drafting, but treat the PDF as not yet ready for public
circulation until the three main visual issues are addressed.
