# Adversarial Test Results: `tla_exists_implies_equality1`

## Target Specification

The lemma `tla_exists_implies_equality1` proves:
```
∃a.(p → q(a))  =  p → (∃a.q(a))
```
This distributes existential quantification over implication, using three `external_body` axioms:
- `temp_pred_equality`: mutual entailment ⟹ equality
- `a_to_temp_pred_equality`: pointwise mutual entailment ⟹ function equality
- `tla_exists_or_equality`: existential distributes over disjunction

---

## Results Summary

| # | Category | Test | Result | Details |
|---|----------|------|--------|---------|
| 1 | Boundary | `test_boundary_both_entailments_violated` | ✅ FAIL | `temp_pred_equality(true, false)` — precondition `p.entails(q)` rejected |
| 2 | Boundary | `test_boundary_partial_entailment` | ✅ FAIL | `temp_pred_equality(false, true)` — precondition `q.entails(p)` rejected |
| 3 | Boundary | `test_boundary_a_to_equality_invalid` | ✅ FAIL | `a_to_temp_pred_equality(const_true, const_false)` — pointwise entailment rejected |
| 4 | Mutation | `test_mutation_swap_implies_direction` | ✅ FAIL | ∃a.(q(a)→p) = (∃a.q(a))→p correctly rejected |
| 5 | Mutation | `test_mutation_implies_to_or` | ✅ FAIL | ∃a.(p→q(a)) = p∨(∃a.q(a)) correctly rejected |
| 6 | Mutation | `test_mutation_drop_precondition` | ✅ FAIL | ∃a.q(a) = p→(∃a.q(a)) correctly rejected |
| 7 | Logical  | `test_logical_arbitrary_equality` | ✅ FAIL | p = q for arbitrary p,q correctly rejected |
| 8 | Logical  | `test_logical_arbitrary_valid` | ✅ FAIL | valid(p) for arbitrary p correctly rejected |
| 9 | Logical  | `test_logical_exists_elimination` | ✅ FAIL | ∃a.q(a) = q(a₀) for specific a₀ correctly rejected |

**All 9/9 adversarial tests were correctly rejected by the verifier.**

---

## Analysis

### Boundary Tests (precondition violations)
The `requires` clauses on `temp_pred_equality` and `a_to_temp_pred_equality` are sufficiently precise:
- They demand **both** directions of entailment, not just one
- Invalid inputs (always-true vs always-false) are properly rejected

### Behavioral Mutation Tests (output mutations)
The specification correctly rejects all three mutations of the main lemma:
- **Swapped direction**: ∃a.(q(a)→p) ≠ (∃a.q(a))→p — catches that implication is not symmetric
- **Wrong connective**: p∨Q ≠ p→Q — catches connective substitution
- **Dropped term**: ∃a.q(a) ≠ p→(∃a.q(a)) — catches removal of the antecedent

### Logical Tests (unintended reasoning)
The axiom system does not permit:
- Concluding arbitrary predicates are equal (no vacuous equality)
- Concluding arbitrary predicates are universally valid
- Eliminating existential quantification to a specific witness

## Conclusion

The specification for `tla_exists_implies_equality1` demonstrates **adequate strength** against all tested attack vectors. The preconditions are tight, behavioral mutations are rejected, and no unintended logical consequences were derivable from the axiom system.
