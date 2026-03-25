# Adversarial Test Results: `lemma_fold_left_on_equiv_seqs`

## Specification Under Test

```
requires: s1.len() == s2.len()
requires: ∀i. 0 ≤ i < s1.len() ⟹ eq(s1[i], s2[i])
requires: ∀b, a1, a2. eq(a1, a2) ⟹ f(b, a1) == f(b, a2)
ensures:  s1.fold_left(init, f) == s2.fold_left(init, f)
```

## Results Summary

| # | File | Test | Type | Result | Details |
|---|------|------|------|--------|---------|
| 1 | boundary_tests.rs | test_boundary_different_lengths | Boundary | ✅ FAIL | Precondition 1 violated: `s1.len() == s2.len()` rejected |
| 2 | boundary_tests.rs | test_boundary_non_equiv_elements | Boundary | ✅ FAIL | Precondition 2 violated: `eq(s1[i], s2[i])` rejected |
| 3 | boundary_tests.rs | test_boundary_f_not_respecting_eq | Boundary | ✅ FAIL | Precondition 3 violated: `eq(a1,a2) ⟹ f(b,a1)==f(b,a2)` rejected |
| 4 | behavioral_mutation_tests.rs | test_mutation_negate_postcondition | Behavioral | ✅ FAIL | Negated postcondition `fold(s1) != fold(s2)` rejected |
| 5 | behavioral_mutation_tests.rs | test_mutation_off_by_one | Behavioral | ✅ FAIL | Off-by-one `fold(s1) == fold(s2) + 1` rejected |
| 6 | behavioral_mutation_tests.rs | test_mutation_different_init | Behavioral | ✅ FAIL | Different init `fold(s1,0,f) == fold(s2,100,f)` rejected |
| 7 | logical_tests.rs | test_logical_extensional_equality | Logical | ✅ FAIL | Stronger claim `s1 =~= s2` from weak eq rejected |
| 8 | logical_tests.rs | test_logical_order_independence | Logical | ✅ FAIL | Commutativity `fold([1,2,3]) == fold([3,2,1])` rejected |
| 9 | logical_tests.rs | test_logical_cross_function_misuse | Logical | ✅ FAIL | Cross-function `fold(s1,g) == fold(s2,g)` for unrelated g rejected |

**All 9/9 tests failed verification as expected.**

## Conclusion

The specification is **well-constrained**:

- **Boundary**: All three preconditions are necessary and individually enforced. Invalid inputs (mismatched lengths, non-equivalent elements, non-congruent functions) are properly rejected.
- **Behavioral**: The postcondition is tight. Negation, off-by-one mutation, and init-value substitution are all rejected, confirming the fold-equality guarantee is precise.
- **Logical**: The spec does not leak stronger properties. Equivalence under `eq` does not imply actual sequence equality, fold_left is not treated as commutative, and fold equality for one function does not transfer to an arbitrary different function.

No spec weaknesses were detected.
