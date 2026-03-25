# Adversarial Test Summary: `lemma_map_set_singleton_auto`

## Specification Under Test

```
forall |x: A, f: spec_fn(A) -> B| set![x].map(f) == set![f(x)]
```

Maps a function `f` over a singleton set `{x}` and asserts the result equals `{f(x)}`.
No preconditions (`requires` clause absent).

---

## Results Overview

| Category               | Tests | Failed (expected) | Passed (unexpected) |
|------------------------|-------|--------------------|---------------------|
| Boundary Tests         | 3     | 3 ✅                | 0                   |
| Behavioral Mutation    | 3     | 3 ✅                | 0                   |
| Logical Tests          | 3     | 3 ✅                | 0                   |
| **Total**              | **9** | **9 ✅**            | **0**               |

All tests failed verification as intended — the specification correctly rejects every adversarial query.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|------|------------------|--------|
| 1 | `test_boundary_empty_set_map_not_singleton` | `∅.map(f) == {1}` — empty set map produces singleton | FAILED ✅ |
| 2 | `test_boundary_two_element_set_map_collapses` | `{1,2}.map(id) == {1}` — multi-element set collapses | FAILED ✅ |
| 3 | `test_boundary_singleton_map_not_empty` | `{5}.map(λx.0) == ∅` — singleton map produces empty set | FAILED ✅ |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Property Queried | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_wrong_result_value` | `{3}.map(λx.x+1) == {5}` — wrong output value (should be {4}) | FAILED ✅ |
| 2 | `test_mutation_result_equals_input` | `{7}.map(λx.x+1) == {7}` — output equals input (should be {8}) | FAILED ✅ |
| 3 | `test_mutation_different_functions_same_result` | `{3}.map(λx.x+1) == {3}.map(λx.x*2)` — different fns same result | FAILED ✅ |

## Logical Tests (`logical_tests.rs`)

| # | Test | Property Queried | Result |
|---|------|------------------|--------|
| 1 | `test_logical_injectivity_not_implied` | `f(1)==f(2) ⟹ 1==2` — injectivity from equal mapped sets | FAILED ✅ |
| 2 | `test_logical_multi_element_map_is_singleton` | `{1,2}.map(id) == {1}` — singleton lemma extends to larger sets | FAILED ✅ |
| 3 | `test_logical_no_reverse_mapping` | `{6}.map(λx.x+1) == {5}` — inverse/reverse mapping | FAILED ✅ |

---

## Conclusion

The specification for `lemma_map_set_singleton_auto` is **consistent** with respect to all tested adversarial queries:

- **Boundary correctness**: The spec does not leak to empty sets or multi-element sets; singleton mapping does not produce empty results.
- **Behavioral correctness**: The spec rejects all mutated output values and distinguishes between different mapping functions.
- **Logical soundness**: The spec does not entail injectivity, does not generalize to non-singleton sets, and does not support reverse/inverse mapping reasoning.

No specification weaknesses were detected. The ensures clause `set![x].map(f) == set![f(x)]` is tight and well-scoped for its intended purpose.
