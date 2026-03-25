# Adversarial Test Results: `simplify_predicate.rs`

## Target Specification

The file defines a temporal logic framework (`TempPred<T>`, `Execution<T>`) with:
- **`entails_apply`**: If `p ⊨ q` and `p(ex)`, then `q(ex)`
- **`temp_pred_equality`**: If `p ⊨ q` and `q ⊨ p`, then `p == q`
- **`entails_and_temp`**: If `spec ⊨ p` and `spec ⊨ q`, then `spec ⊨ p ∧ q`
- **`simplify_predicate`**: If `p ⊨ q`, then `p == p ∧ q`

---

## Results Summary

| Category | Test | Expected | Actual | Status |
|---|---|---|---|---|
| **Boundary** | `test_simplify_no_entailment` — call without `p.entails(q)` | FAIL | precondition error | ✅ |
| **Boundary** | `test_entails_apply_no_entailment` — missing `p.entails(q)` | FAIL | precondition error | ✅ |
| **Boundary** | `test_entails_apply_no_satisfaction` — missing `p.satisfied_by(ex)` | FAIL | precondition error | ✅ |
| **Boundary** | `test_equality_one_direction` — only one direction of entailment | FAIL | precondition error | ✅ |
| **Boundary** | `test_entails_and_temp_missing_q` — missing `spec.entails(q)` | FAIL | precondition error | ✅ |
| **Mutation** | `test_simplify_mutated_to_equality` — conclude `p == q` instead of `p == p∧q` | FAIL | postcondition error | ✅ |
| **Mutation** | `test_simplify_swapped_subjects` — conclude `q == q∧p` from `p⊨q` | FAIL | postcondition error | ✅ |
| **Mutation** | `test_entails_apply_reversed` — derive `p(ex)` from `q(ex)` and `p⊨q` | FAIL | postcondition error | ✅ |
| **Mutation** | `test_equality_mutated_extra` — derive `q == p∧q` from mutual entailment | FAIL | postcondition error | ✅ |
| **Mutation** | `test_and_operand_order_matters` — `p == q∧p` instead of `p == p∧q` | FAIL | postcondition error | ✅ |
| **Logical** | `test_entails_not_symmetric` — `p⊨q` implies `q⊨p` | FAIL | postcondition error | ✅ |
| **Logical** | `test_valid_arbitrary` — `valid(p)` for arbitrary `p` | FAIL | postcondition error | ✅ |
| **Logical** | `test_entails_arbitrary` — `p.entails(q)` for arbitrary `p,q` | FAIL | postcondition error | ✅ |
| **Logical** | `test_entails_skip_intermediate` — `p⊨q` implies `p⊨r` for arbitrary `r` | FAIL | postcondition error | ✅ |
| **Logical** | `test_and_commutativity` — `p∧q == q∧p` without proof | FAIL | postcondition error | ✅ |
| **Logical** | `test_cross_function_misuse` — `p∧q == q` from `p⊨q` | FAIL | postcondition error | ✅ |

**Total: 16/16 tests correctly rejected by the verifier.**

---

## Analysis

### Boundary (5/5 rejected)
All preconditions on the four proof functions are properly enforced. Callers cannot invoke any lemma without satisfying all `requires` clauses.

### Behavioral Mutation (5/5 rejected)
The specification distinguishes between:
- `p == p∧q` (correct) vs `p == q` (incorrect)
- Forward direction (`p(ex) → q(ex)`) vs reverse (`q(ex) → p(ex)`)
- Operand order in `and` (`p∧q` vs `q∧p`)

No mutated behavior was accepted.

### Logical (6/6 rejected)
The specification does not allow:
- Symmetry of entailment
- Validity of arbitrary predicates
- Transitivity shortcuts
- Structural commutativity of `and` (intensional inequality)
- Cross-function composition to derive unintended equalities

---

## Conclusion

The specification for `simplify_predicate` and its supporting axioms is **well-constrained**. All 16 adversarial queries were correctly rejected, indicating no detectable semantic gaps in the tested dimensions. The spec properly enforces its preconditions and does not entail unintended properties.
