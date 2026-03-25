# Adversarial Test Summary: `transform_leads_to_with_until`

## Overview

12 adversarial proof tests were generated across 3 categories to probe the semantic boundary of the `transform_leads_to_with_until` specification. **All 12 tests failed verification as expected**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Results

### Boundary Tests (4/4 FAILED ✅)

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| `test_missing_leads_to_precondition` | Missing `spec ⊨ p1 ~> q1` | precondition not satisfied |
| `test_missing_always_next` | Missing `spec ⊨ □next` | precondition not satisfied |
| `test_missing_inductive_step` | Missing inductive condition on p2 | precondition not satisfied |
| `test_weakened_always_to_eventually` | `◇next` instead of `□next` | precondition not satisfied |

**Conclusion**: All three preconditions are strictly enforced. Weakening `always` to `eventually` is also correctly rejected.

### Behavioral Mutation Tests (4/4 FAILED ✅)

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `test_drop_q2_disjunct` | Removed `.or(q2)` from conclusion | postcondition not satisfied |
| `test_drop_p2_from_antecedent` | Weakened antecedent from `p1∧p2` to `p1` | postcondition not satisfied |
| `test_and_instead_of_or` | Changed `.or(q2)` to `.and(q2)` | postcondition not satisfied |
| `test_swap_q1_q2` | Swapped q1↔q2 roles in conclusion | postcondition not satisfied |

**Conclusion**: The postcondition is tight. Strengthening (dropping disjuncts, replacing `or` with `and`), weakening the antecedent, and swapping operand roles are all rejected.

### Logical Tests (4/4 FAILED ✅)

| Test | Unintended Property Tested | Verus Error |
|------|---------------------------|-------------|
| `test_leads_to_preserves_conjunction` | `p1 ~> q1` alone implies `p1∧p2 ~> q1∧p2` | postcondition not satisfied |
| `test_always_next_alone_insufficient` | `□next` without inductive step suffices | postcondition not satisfied |
| `test_swapped_roles_invalid` | Swapped precondition roles yield original conclusion | postcondition not satisfied |
| `test_strengthen_eventually_to_always` | `leads_to` (◇) strengthened to `always` (□) | postcondition not satisfied |

**Conclusion**: The spec does not admit unintended logical inferences. Key findings:
- **leads_to does NOT preserve conjunctions** without an explicit inductive invariant for the conjunct — the spec correctly requires this.
- **The inductive step is essential** — `□next` alone is insufficient.
- **Parameter roles are non-interchangeable** — swapping p1/p2 and q1/q2 gives a different theorem.
- **eventually cannot be promoted to always** — the temporal modality in the conclusion is precise.

---

## Specification Quality Assessment

The specification of `transform_leads_to_with_until` is **well-constrained**:

1. **Input validation**: All preconditions are necessary and enforced (no redundant requirements).
2. **Output precision**: The postcondition is tight — no spurious strengthenings or weakenings pass.
3. **Logical soundness**: No unintended entailments were discovered; the spec rejects invalid reasoning.

**No specification weaknesses were identified.** The spec correctly captures the temporal logic theorem: combining a liveness property (`p1 ~> q1`) with an inductive "until" condition on `p2` yields the combined liveness result with the proper disjunctive outcome.
