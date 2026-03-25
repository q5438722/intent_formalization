# Test Execution Summary: `lemma_seq_push_to_set`

## Specification Under Test

```rust
pub proof fn lemma_seq_push_to_set<A>(s: Seq<A>, x: A)
    ensures s.push(x).to_set() == s.to_set().insert(x)
```

No preconditions. The postcondition states that pushing `x` onto sequence `s` and converting to a set is equivalent to converting `s` to a set and inserting `x`.

---

## Results Overview

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15 ✅** | **0** |

All 15 adversarial tests were correctly **rejected** by the verifier, indicating the specification is consistent with respect to the queried properties.

---

## Boundary Tests (5/5 rejected ✅)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_boundary_1_push_empty_gives_empty_set` | Push onto empty seq cannot yield empty set |
| 2 | `test_boundary_2_push_preserves_seq_length` | Push increases seq length by 1 |
| 3 | `test_boundary_3_pushed_element_not_in_result` | Pushed element must be in result set |
| 4 | `test_boundary_4_push_empty_set_cardinality_zero` | Singleton set has cardinality 1, not 0 |
| 5 | `test_boundary_5_push_gives_wrong_singleton` | {1} ≠ {2} — wrong element in singleton |

## Behavioral Mutation Tests (5/5 rejected ✅)

| # | Test | Mutation Applied |
|---|---|---|
| 1 | `test_mutation_1_push_set_eq_original_set` | Omit insert — claim push-to-set = original set |
| 2 | `test_mutation_2_push_set_eq_remove` | Replace insert with remove |
| 3 | `test_mutation_3_pushed_element_not_in_set` | Negate element containment |
| 4 | `test_mutation_4_insert_wrong_element` | Insert wrong element (77 instead of 5) |
| 5 | `test_mutation_5_negate_ensures` | Negate the entire ensures clause |

## Logical Tests (5/5 rejected ✅)

| # | Test | Unintended Property Tested |
|---|---|---|
| 1 | `test_logical_1_set_size_always_increases` | Set cardinality always +1 (false with dups) |
| 2 | `test_logical_2_push_reversible_via_remove` | Push reversible via remove (false with dups) |
| 3 | `test_logical_3_seq_len_eq_set_len` | Seq length = set cardinality (false with dups) |
| 4 | `test_logical_4_to_set_injective` | to_set is injective (false: [1,2,1] ≈ [1,2]) |
| 5 | `test_logical_5_double_push_strictly_larger` | Double push gives strictly larger set (false) |

---

## Conclusion

The specification `s.push(x).to_set() == s.to_set().insert(x)` is **well-formed and consistent** with respect to all tested semantic queries. It correctly:

- **Rejects invalid edge cases** (boundary)
- **Rejects mutated input-output relations** (behavioral)
- **Rejects unintended logical inferences** such as injectivity of `to_set` and strict cardinality increase (logical)

No specification weaknesses were detected.
