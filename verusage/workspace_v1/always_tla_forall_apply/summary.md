# Adversarial Test Results: `always_tla_forall_apply`

## Target Specification

```
proof fn always_tla_forall_apply<T, A>(spec, a_to_p, a)
    requires spec.entails(always(tla_forall(a_to_p)))
    ensures  spec.entails(always(a_to_p(a)))
```

Plus two helper lemmas (`entails_trans`, `entails_preserved_by_always`).

---

## Results Summary

| # | File | Test | Expected | Actual | Status |
|---|------|------|----------|--------|--------|
| 1 | boundary_tests.rs | `test_call_without_precondition` | FAIL | precondition not satisfied | ✅ |
| 2 | boundary_tests.rs | `test_missing_always_in_precondition` | FAIL | precondition not satisfied | ✅ |
| 3 | boundary_tests.rs | `test_entails_trans_missing_first_premise` | FAIL | precondition not satisfied | ✅ |
| 4 | behavioral_mutation_tests.rs | `test_mutate_entails_to_valid` | FAIL | assertion failed | ✅ |
| 5 | behavioral_mutation_tests.rs | `test_reverse_entailment_direction` | FAIL | assertion failed | ✅ |
| 6 | behavioral_mutation_tests.rs | `test_single_shot_implies_always` | FAIL | assertion failed | ✅ |
| 7 | logical_tests.rs | `test_entails_symmetry` | FAIL | assertion failed | ✅ |
| 8 | logical_tests.rs | `test_particular_to_universal` | FAIL | assertion failed | ✅ |
| 9 | logical_tests.rs | `test_entailment_does_not_imply_validity` | FAIL | assertion failed | ✅ |

**All 9/9 adversarial tests correctly rejected.**

---

## Analysis by Category

### Boundary Tests (3/3 rejected)
The spec correctly enforces its preconditions:
- Cannot invoke `always_tla_forall_apply` without proving `spec ⊨ □(∀a. P(a))`
- The weaker `spec ⊨ ∀a. P(a)` (without □) is insufficient
- `entails_trans` cannot be called with a missing premise

### Behavioral Mutation Tests (3/3 rejected)
Mutated conclusions are all rejected:
- `valid(□P(a))` (universal validity) is strictly stronger than `spec ⊨ □P(a)` — rejected
- Reversed entailment `□P(a) ⊨ spec` does not follow — rejected
- `P(a) ⊨ □P(a)` (single-shot implies forever) is not derivable — rejected

### Logical Tests (3/3 rejected)
No unintended logical inferences are possible:
- Entails is correctly non-symmetric
- The converse (particular → universal) is not derivable
- `spec ⊨ P` does not collapse to `valid(P)` for arbitrary `spec`

---

## Conclusion

The specification for `always_tla_forall_apply` and its helper lemmas is **consistent** with respect to all tested adversarial queries. The preconditions are necessary and sufficient, the postcondition cannot be strengthened or reversed, and no unintended logical reasoning is admitted.
