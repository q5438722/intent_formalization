# Adversarial Test Summary: `entails_preserved_by_always`

## Target Specification

**Lemma**: `entails_preserved_by_always<T>(p, q)`  
**Requires**: `p.entails(q)`  
**Ensures**: `always(p).entails(always(q))`  

The lemma states that temporal entailment is preserved under the `always` operator: if `p` entails `q` for all executions, then `always(p)` entails `always(q)`.

---

## Results Summary

| # | Test Name | Category | Failure Mode | Result |
|---|-----------|----------|-------------|--------|
| 1 | `test_boundary_missing_precondition` | Boundary | No `p.entails(q)` precondition | ✅ FAILED (precondition not satisfied) |
| 2 | `test_boundary_reversed_precondition` | Boundary | `q.entails(p)` instead of `p.entails(q)` | ✅ FAILED (precondition not satisfied) |
| 3 | `test_boundary_unfold_without_always` | Boundary | `p.satisfied_by(ex)` instead of `always(p).satisfied_by(ex)` | ✅ FAILED (precondition not satisfied) |
| 4 | `test_boundary_modus_ponens_no_antecedent` | Boundary | `implies_apply` without antecedent `p.satisfied_by(ex)` | ✅ FAILED (precondition not satisfied) |
| 5 | `test_mutation_reversed_conclusion` | Behavioral | `always(q).entails(always(p))` — reversed direction | ✅ FAILED (precondition not satisfied) |
| 6 | `test_mutation_strengthen_to_always` | Behavioral | `p.entails(always(q))` — strengthened conclusion | ✅ FAILED (postcondition not satisfied) |
| 7 | `test_mutation_to_valid` | Behavioral | `valid(always(q))` — unconditional validity | ✅ FAILED (postcondition not satisfied) |
| 8 | `test_logical_entails_not_symmetric` | Logical | Entails is not symmetric | ✅ FAILED (postcondition not satisfied) |
| 9 | `test_logical_self_entails_valid` | Logical | Self-entailment ≠ validity | ✅ FAILED (postcondition not satisfied) |
| 10 | `test_logical_local_to_global` | Logical | Local `always` ≠ global `valid` | ✅ FAILED (postcondition not satisfied) |
| 11 | `test_logical_entails_not_valid` | Logical | Conditional entailment ≠ unconditional validity | ✅ FAILED (postcondition not satisfied) |

**Original lemma**: ✅ VERIFIED (1 verified)

---

## Conclusion

**All 11 adversarial tests were correctly rejected by the specification.** The spec demonstrates:

1. **Precondition rigor** (Boundary): The `requires p.entails(q)` clause is necessary — the lemma cannot be used without it, with a reversed version, or with weakened axiom preconditions.

2. **Output precision** (Behavioral): The conclusion `always(p).entails(always(q))` cannot be strengthened (e.g., to `p.entails(always(q))`), reversed, or generalized to unconditional validity.

3. **Logical soundness** (Logical): The spec does not admit unintended inferences — entails is not confused with symmetry, self-entailment is not conflated with validity, and local satisfaction is not confused with global truth.

**Verdict**: The specification for `entails_preserved_by_always` is **consistent** — it correctly rejects all tested adversarial properties across boundary, behavioral, and logical dimensions.
