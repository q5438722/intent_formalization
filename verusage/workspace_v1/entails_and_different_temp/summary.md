# Adversarial Test Summary: `entails_and_different_temp`

**Target**: `source-projects/anvil-library/verified/temporal_logic/entails_and_different_temp.rs`  
**Date**: 2026-03-22  
**Result**: ✅ All 15 tests FAILED verification as expected — specification is consistent.

## Specification Under Test

```
entails_and_different_temp(spec1, spec2, p, q):
  requires: spec1.entails(p), spec2.entails(q)
  ensures:  spec1.and(spec2).entails(p.and(q))
```

Also tested: `implies_apply(ex, p, q)` (modus ponens axiom).

---

## Results by Category

### Boundary Tests (5/5 rejected ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_missing_first_precondition` | Missing `spec1.entails(p)` | precondition error ✅ |
| 2 | `test_boundary_missing_second_precondition` | Missing `spec2.entails(q)` | precondition error ✅ |
| 3 | `test_boundary_missing_both_preconditions` | Missing both preconditions | precondition error ✅ |
| 4 | `test_boundary_implies_apply_no_implication` | Missing implication in `implies_apply` | precondition error ✅ |
| 5 | `test_boundary_implies_apply_no_antecedent` | Missing antecedent in `implies_apply` | precondition error ✅ |

### Behavioral Mutation Tests (5/5 rejected ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_spec1_alone_entails_conj` | `spec1.entails(p∧q)` instead of `spec1∧spec2.entails(p∧q)` | postcondition error ✅ |
| 2 | `test_mutation_spec2_alone_entails_conj` | `spec2.entails(p∧q)` instead of `spec1∧spec2.entails(p∧q)` | postcondition error ✅ |
| 3 | `test_mutation_wrong_pairing_spec1_q` | `spec1.entails(q)` (wrong pairing) | postcondition error ✅ |
| 4 | `test_mutation_wrong_pairing_spec2_p` | `spec2.entails(p)` (wrong pairing) | postcondition error ✅ |
| 5 | `test_mutation_valid_instead_of_entails` | `valid(p∧q)` (too strong) | postcondition error ✅ |

### Logical Tests (5/5 rejected ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_entails_not_symmetric` | Entails symmetry: `p.entails(spec1)` | postcondition error ✅ |
| 2 | `test_logical_valid_not_from_entails` | Valid from entails: `valid(p)` | postcondition error ✅ |
| 3 | `test_logical_entails_arbitrary_pred` | Arbitrary entailment: `spec1∧spec2.entails(r)` | postcondition error ✅ |
| 4 | `test_logical_no_conjunct_elimination` | Conjunct elimination: `spec1.entails(p∧q)` from `spec1∧spec2.entails(p∧q)` | postcondition error ✅ |
| 5 | `test_logical_no_cross_transitivity` | Cross-transitivity: `p.entails(q)` from unrelated entailments | postcondition error ✅ |

---

## Conclusion

The specification for `entails_and_different_temp` is **well-scoped**:

- **Preconditions are tight**: both `spec1.entails(p)` and `spec2.entails(q)` are necessary; removing either causes failure.
- **The conclusion is precise**: no weakened or strengthened variant of the conclusion can be derived.
- **No unintended logical consequences**: symmetry, universal validity, arbitrary entailment, conjunct elimination, and cross-transitivity are all correctly rejected.
- **The `implies_apply` axiom is safe**: both of its preconditions are independently necessary.

No specification weaknesses detected.
