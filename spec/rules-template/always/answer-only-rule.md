# Answer Questions; Do Not Also Implement
#
# Framework principle: Boundary enforcement — interaction clarity
# Evidence: foundry (2026-06-10, Governor-Reviewer directive)

**Scope:** Every interaction where a human or agent asks a question.

## Rule

When a question is asked, answer it. Stop.

Do not implement, fix, or change anything conceived while formulating
the answer. If the answer reveals a problem or opportunity, state the
answer and stop. Implementation requires a separate, explicit request.

## Why this exists

Answering a question and implementing a solution are two distinct
interactions serving two distinct needs. Collapsing them:

- Removes the human's opportunity to redirect before work is done
- Produces unrequested changes the human must then evaluate and undo
- Conflates information (the answer) with action (the change)
- Trains humans to ask fewer questions to avoid triggering unwanted work

The cost of stopping at the answer is zero. The cost of unrequested
implementation is review burden, potential revert, and loss of trust.

## Applies to

- Every question in every context: design questions, clarification
  questions, "what do you think?" questions, architecture questions
- Applies regardless of how obvious the fix seems while answering
- Applies regardless of how small the implementation would be

## Does not apply to

- Explicit implementation requests ("fix this", "proceed", "implement")
- Tasks that are requests, not questions, even if phrased as questions
  ("can you add X?" where intent is clearly a request)

When in doubt: answer, then ask "should I also implement this?"
