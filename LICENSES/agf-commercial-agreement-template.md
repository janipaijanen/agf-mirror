# AGF Commercial Distribution Agreement

**Effective Date:** [DATE]

**Between:**

- **Licensor:** Jani Päijänen
  Email: jani.paijanen@protonmail.com
  Address: [LICENSOR ADDRESS]

- **Licensee:** [COMPANY NAME]
  Contact Name: [CONTACT NAME]
  Email: [CONTACT EMAIL]
  Address: [COMPANY ADDRESS]

*This is a template showing the structure of the agreement, not a filled
or executed instance. Bracketed fields are completed per signed
agreement.*

## 1. Licence Reference

This agreement grants rights under **LicenseRef-AgentsGovern-Commercial**
for the Agents Govern Framework (AGF). The full text of that licence is
at `LICENSES/LicenseRef-AgentsGovern-Commercial.txt` in the AGF
repository.

The executed agreement must identify the exact licence and corpus artifacts
reviewed by the parties:

| Binding field | Exact value at signing |
|---------------|------------------------|
| Licence text Git blob | `[LICENCE TEXT GIT BLOB SHA]` |
| Commercial corpus manifest version | `[CORPUS VERSION]` |
| Commercial corpus manifest Git blob | `[MANIFEST GIT BLOB SHA]` |
| Corpus set SHA-256 | `[CORPUS SET SHA-256]` |

These fields are required. A Git blob can be recovered with `git cat-file
blob <sha>` even if a repository path later changes. The unfilled template's
own blob is not an executed-agreement identifier.

## 2. Grant Scope

The Licensor grants the Licensee the rights described in the
LicenseRef-AgentsGovern-Commercial licence, including the right to:

- use, copy and modify AGF Framework Content;
- redistribute Framework Content and Modified Versions under terms of
  the Licensee's choosing (including proprietary terms);
- combine Framework Content with other works and distribute the
  combination under the Licensee's chosen terms.

This grant exempts the Licensee from the ShareAlike obligations of
CC-BY-SA-4.0 for material used under this commercial licence.

## 3. Term and Fee

This licence is available from 1 January 2027; no agreement under this
template is dated earlier than that.

| Item | Details |
|------|---------|
| Tier | [Starter / Standard / Enterprise] |
| Annual fee | [EUR 100 / EUR 500 / as agreed] |
| Term | 12 months commencing [START DATE] |

**Payment.** Payment must be made in full to the account designated by
the Licensor, per the invoice issued for this agreement.

**Effectiveness.** This agreement and the commercial licence grant
become effective only on the date the Licensor has received the full
cleared payment; until that date no commercial redistribution rights are
granted.

**Failed or reversed payment.** Failure to make the required payment, or
any later bounce, charge-back, reversal, or failure of the payment to
remain cleared, constitutes a material breach. In that case the Licensor
may immediately suspend or terminate the commercial licence grant
without waiting for the thirty-day cure period in Section 7.

This agreement does not auto-renew. Continuing past its stated term
requires a new agreement.

## 4. Attribution

By default, attribution is satisfied by retaining the original
copyright notice for the Framework Content in the product's credits or
documentation. Any stronger attribution requirement must be stated
here: [None / Specify]

## 5. Reporting

- [ ] No reporting required.
- [ ] Licensee shall provide a brief annual summary of products/services that incorporate redistributed AGF content.
- [ ] Other: [SPECIFY]

## 6. Permitted Redistribution Scope

This agreement covers:

- [ ] Redistribution in commercial products or services supplied to clients.
- [ ] OEM / embedded distribution.
- [ ] Other: [SPECIFY]

Pure internal use with no redistribution to third parties does not
require this commercial licence.

## 7. Term and Termination

The agreement's term is as stated in Section 3. Either party may
terminate for material breach if the breach is not cured within thirty
(30) days after written notice. Non-payment, or any reversal/failure of
payment to remain cleared, may be treated as an immediate material
breach (see Section 3).

On termination or expiry, the Licensee must cease **new** distributions
under this commercial licence. End users may continue to use copies
already supplied to them.

## 8. Governing Law

This agreement is governed by the laws of Finland, without regard to
conflict-of-law principles.

## 9. Entire Agreement

This agreement and the referenced LicenseRef-AgentsGovern-Commercial
text constitute the entire understanding between the parties concerning
the commercial licence grant. Amendments must be in writing and signed
by both parties.

---

**Licensor**
Name: Jani Päijänen
Date: _______________
Signature: _______________

**Licensee**
Name: _______________
Title: _______________
Date: _______________
Signature: _______________
Company: [COMPANY NAME]

---

*This agreement does not affect the MIT licence that applies to AGF
tooling code.*
