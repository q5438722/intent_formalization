# Adversarial Test Summary: `temp_pred_equality.rs`

## Specification Under Test

The specification defines temporal predicate equality via mutual entailment:
- **`temp_pred_equality`**: `p.entails(q) ∧ q.entails(p) ⟹ p == q`
- **`implies_apply`** (axiom): modus ponens on temporal predicates
- **`implies_contraposition_apply`** (axiom): contraposition on temporal predicates

## Results

**All 15 adversarial tests FAILED verification as expected.** ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (5/5 failed ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_equality_missing_backward_entailment` | Only `p.entails(q)`, missing `q.entails(p)` | ✅ Rejected |
| `test_equality_missing_forward_entailment` | Only `q.entails(p)`, missing `p.entails(q)` | ✅ Rejected |
| `test_implies_apply_missing_antecedent` | Implication holds but `p` not satisfied | ✅ Rejected |
| `test_contraposition_missing_negation` | Implication holds but `¬q` not given | ✅ Rejected |
| `test_equality_no_preconditions` | No preconditions at all | ✅ Rejected |

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_equality_mutated_to_negation` | Conclude `p == not(q)` instead of `p == q` | ✅ Rejected |
| `test_implies_apply_negated_output` | Conclude `not(q)` instead of `q` from modus ponens | ✅ Rejected |
| `test_contraposition_non_negated_output` | Conclude `p` instead of `not(p)` from contraposition | ✅ Rejected |
| `test_equality_mutated_to_negation_of_first` | Conclude `not(p) == q` instead of `p == q` | ✅ Rejected |
| `test_implies_apply_mutated_to_negate_antecedent` | Conclude `not(p)` instead of `q` from modus ponens | ✅ Rejected |

### Logical Tests (5/5 failed ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_arbitrary_equality` | Any two predicates are equal | ✅ Rejected |
| `test_entailment_symmetry` | Entailment is symmetric | ✅ Rejected |
| `test_valid_implies_equality_with_arbitrary` | Valid predicate equals any predicate | ✅ Rejected |
| `test_entailment_transitivity_yields_equality` | Transitive entailment implies equality | ✅ Rejected |
| `test_arbitrary_satisfaction` | Any predicate is satisfied by any execution | ✅ Rejected |

## Conclusion

The specification for `temp_pred_equality` is **well-constrained**:
- **Preconditions are necessary**: removing either direction of mutual entailment, or any axiom precondition, breaks the proof.
- **Postconditions are precise**: mutating any output (negating, swapping) is rejected.
- **No unintended entailments**: the spec does not admit arbitrary equality, entailment symmetry, or universal satisfaction.

No specification weaknesses were detected.
