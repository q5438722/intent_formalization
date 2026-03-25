# Adversarial Test Results: `always_and_equality.rs`

## Target Specification
Proves that `always(p ∧ q) == always(p) ∧ always(q)` using two axioms:
- `always_unfold`: unfolds the `always` quantifier (requires `always(p).satisfied_by(ex)`)
- `temp_pred_equality`: mutual entailment implies equality (requires both directions)

---

## Results Summary

| Test | Category | Property Tested | Result | Failure Mode |
|------|----------|----------------|--------|-------------|
| boundary_test_1 | Boundary | `always_unfold` without precondition | **FAILED** ✅ | Precondition `always(p).satisfied_by(ex)` not satisfied |
| boundary_test_2 | Boundary | `temp_pred_equality` with one-directional entailment | **FAILED** ✅ | Precondition `q.entails(p)` not satisfied |
| boundary_test_3 | Boundary | `always_unfold` proving wrong predicate `q` instead of `p` | **FAILED** ✅ | Postcondition about `q` not provable from knowledge of `p` |
| mutation_test_1 | Behavioral | `always(p ∧ q) == always(p)` (drop q) | **FAILED** ✅ | Cannot prove `always(p)` entails `always(p ∧ q)` |
| mutation_test_2 | Behavioral | `always(p ∧ q) == always(q)` (drop p) | **FAILED** ✅ | Cannot prove `always(q)` entails `always(p ∧ q)` |
| mutation_test_3 | Behavioral | `always(p) ∧ always(q) == always(p)` (weaken) | **FAILED** ✅ | Cannot prove `always(p)` entails `always(p) ∧ always(q)` |
| logical_test_1 | Logical | `always(p ⟹ q) == always(p) ⟹ always(q)` (distribute over implies) | **FAILED** ✅ | Mutual entailment not provable (reverse direction is invalid) |
| logical_test_2 | Logical | `always(p) == p` (always is identity) | **FAILED** ✅ | Cannot prove mutual entailment between `always(p)` and `p` |
| logical_test_3 | Logical | `always(p) == always(q)` for arbitrary p, q | **FAILED** ✅ | Cannot prove entailment between unrelated predicates |

**Total: 9/9 tests correctly rejected by the specification.**

---

## Analysis

### Boundary Tests
The specification's preconditions are **well-guarded**. Both axioms (`always_unfold` and `temp_pred_equality`) correctly reject calls that don't satisfy their full preconditions. Misuse of `always_unfold` on the wrong predicate is also caught via postcondition mismatch.

### Behavioral Mutation Tests
The `always_and_equality` theorem cannot be weakened. Dropping either conjunct from the equality breaks the reverse entailment direction — `always(p)` alone cannot recover `always(p ∧ q)` without knowing `q` holds everywhere. The spec correctly rejects all three mutated variants.

### Logical Tests
The specification does **not** admit unintended logical properties:
- **No distribution over implies**: `always(p ⟹ q) == always(p) ⟹ always(q)` is correctly rejected. The pointwise implication (LHS) is strictly stronger than the global implication (RHS).
- **No always-identity collapse**: `always(p) == p` is rejected, confirming `always` is a genuine temporal operator.
- **No arbitrary equality**: The axiom `temp_pred_equality` cannot be weaponized to equate unrelated predicates.

## Conclusion

The specification for `always_and_equality` is **consistent** with respect to all 9 adversarial queries. It properly rejects invalid inputs, incorrect behavioral mutations, and unintended logical inferences. No specification weaknesses were detected.
