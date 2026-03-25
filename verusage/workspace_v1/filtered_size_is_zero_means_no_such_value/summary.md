# Adversarial Test Summary: `filtered_size_is_zero_means_no_such_value`

## Target Specification

The specification proves a biconditional:
```
(m.filter(f).len() == 0) == (forall |v: V| !(m.contains(v) && f(v)))
```
It also relies on an axiom `len_is_zero_means_count_for_each_value_is_zero` asserting that a multiset has length 0 iff all values have count 0.

## Results Summary

| File | Tests | Failed (expected) | Passed (unexpected) |
|------|-------|--------------------|----------------------|
| `boundary_tests.rs` | 5 | 5 | 0 |
| `behavioral_mutation_tests.rs` | 5 | 5 | 0 |
| `logical_tests.rs` | 7 | 7 | 0 |
| **Total** | **17** | **17** | **0** |

**Combined (`correctness_tests.rs`):** 1 verified (original function), 17 errors (all tests correctly rejected).

## Boundary Tests (5/5 FAILED ✓)

| # | Test | Property Challenged | Result |
|---|------|---------------------|--------|
| 1 | `test_boundary_empty_multiset_nonzero_filter_len` | Empty multiset filter has len > 0 | REJECTED ✓ |
| 2 | `test_boundary_singleton_satisfying_filter_is_zero` | Singleton with matching element has filter len 0 | REJECTED ✓ |
| 3 | `test_boundary_singleton_not_satisfying_still_contains` | Singleton doesn't contain its own element | REJECTED ✓ |
| 4 | `test_boundary_empty_multiset_contains_value` | Empty multiset contains a value | REJECTED ✓ |
| 5 | `test_boundary_always_true_filter_on_nonempty` | Always-true filter on non-empty gives len 0 | REJECTED ✓ |

## Behavioral Mutation Tests (5/5 FAILED ✓)

| # | Test | Property Challenged | Result |
|---|------|---------------------|--------|
| 1 | `test_mutation_flip_biconditional` | filter len == 0 implies ∃v with m.contains(v) ∧ f(v) | REJECTED ✓ |
| 2 | `test_mutation_negate_postcondition` | filter len ≠ 0 when no value satisfies f | REJECTED ✓ |
| 3 | `test_mutation_filter_len_equals_multiset_len` | filter length always equals multiset length | REJECTED ✓ |
| 4 | `test_mutation_swap_contains_and_f` | m.contains(v) ↔ f(v) for all v | REJECTED ✓ |
| 5 | `test_mutation_filter_exceeds_multiset` | filter length exceeds multiset length | REJECTED ✓ |

## Logical Tests (7/7 FAILED ✓)

| # | Test | Property Challenged | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_derive_false` | Axiom is unsound (derives false) | REJECTED ✓ |
| 2 | `test_logical_filter_is_identity` | filter(f) is always the identity | REJECTED ✓ |
| 3 | `test_logical_filter_predicate_irrelevance` | Different predicates give same filter length | REJECTED ✓ |
| 4 | `test_logical_filter_len_at_most_one` | filter length is always ≤ 1 | REJECTED ✓ |
| 5 | `test_logical_all_multisets_empty` | All multisets have length 0 | REJECTED ✓ |
| 6 | `test_logical_double_filter_equals_single` | filter preserves all element counts | REJECTED ✓ |
| 7 | `test_logical_filter_injective` | Same filter length implies equal multisets | REJECTED ✓ |

## Conclusion

The specification correctly rejects all 17 adversarial queries across all three categories. No boundary violations, behavioral mutations, or unentailed logical properties were accepted. The specification appears **consistent** — it does not entail any of the tested undesirable properties.
