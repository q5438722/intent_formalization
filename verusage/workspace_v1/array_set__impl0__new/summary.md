# Adversarial Proof Test Summary: `array_set__impl0__new`

## Target Specification

`ArraySet::new()` constructor with postconditions:
- `ret.wf()` — well-formedness invariant (data length, set-array correspondence, len tracking)
- `ret@ == Set::<usize>::empty()` — returns an empty set

## Results Overview

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| boundary_tests.rs | 5 | 5 ✅ | 0 |
| behavioral_mutation_tests.rs | 5 | 5 ✅ | 0 |
| logical_tests.rs | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15 ✅** | **0** |

## Detailed Results

### Boundary Tests (all failed as expected ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_new_set_contains_zero` | Empty set contains 0 | FAIL ✅ |
| 2 | `test_boundary_new_set_len_positive` | Empty set has length > 0 | FAIL ✅ |
| 3 | `test_boundary_new_set_contains_max_index` | Empty set contains N-1 | FAIL ✅ |
| 4 | `test_boundary_new_set_contains_out_of_range` | Empty set contains N | FAIL ✅ |
| 5 | `test_boundary_array_new_element_value` | `Array::new()` wf implies specific element value | FAIL ✅ |

### Behavioral Mutation Tests (all failed as expected ✅)

| # | Test | Mutation Applied | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_new_not_empty` | Negate emptiness postcondition | FAIL ✅ |
| 2 | `test_mutation_new_produces_singleton` | Replace empty with {0} | FAIL ✅ |
| 3 | `test_mutation_new_len_is_one` | Mutate len from 0 to 1 | FAIL ✅ |
| 4 | `test_mutation_new_len_is_n` | Mutate len from 0 to N | FAIL ✅ |
| 5 | `test_mutation_new_data_true_at_zero` | Mutate data[0] from false to true | FAIL ✅ |

### Logical Tests (all failed as expected ✅)

| # | Test | Unwarranted Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_wf_implies_empty` | wf() alone ⟹ empty set | FAIL ✅ |
| 2 | `test_logical_wf_implies_nonempty` | wf() alone ⟹ non-empty set | FAIL ✅ |
| 3 | `test_logical_two_wf_sets_equal` | Two wf() ArraySets are equal | FAIL ✅ |
| 4 | `test_logical_array_new_determinism` | Array::new() determines element values | FAIL ✅ |
| 5 | `test_logical_new_implies_positive_n` | new() postcondition ⟹ len ≥ 1 | FAIL ✅ |

## Conclusion

The specification for `ArraySet::new()` is **consistent** with respect to all 15 adversarial queries:

- **Boundary**: Invalid membership and length claims on the empty set are correctly rejected.
- **Behavioral**: Mutations to postconditions (negation, value substitution, field mutation) are all rejected.
- **Logical**: Unwarranted inferences (wf-only emptiness/non-emptiness, determinism of `Array::new()` values, equality of distinct well-formed sets) are all rejected.

No specification weakness was detected. The spec correctly rejects all queried undesirable properties.
