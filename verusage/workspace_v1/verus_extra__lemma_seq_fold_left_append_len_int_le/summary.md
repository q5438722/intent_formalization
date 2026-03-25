# Adversarial Test Summary: `verus_extra__lemma_seq_fold_left_append_len_int_le`

## Target Specification

Three lemmas about `Seq::fold_left` with a length-summing accumulator:
- **`lemma_seq_fold_left_sum_right`**: Decomposes fold as `fold(init) == fold(init[..n-1]) + f(last)` (requires `s.len() > 0`)
- **`lemma_seq_fold_left_sum_len_int_positive`**: `fold(low, sum_len) >= 0` for `low: nat`
- **`lemma_seq_fold_left_append_len_int_le`**: For `0 <= i <= len` and `0 <= low`: (1) `fold >= 0`, (2) `fold(prefix[0..i]) <= fold(full)`

## Results: All 12 Tests FAILED Verification ✅

### Boundary Tests (4/4 rejected)
| Test | Property Violated | Result |
|------|------------------|--------|
| `test_boundary_negative_index` | `i = -1` violates `0 <= i` | ❌ FAIL (precondition) |
| `test_boundary_index_exceeds_len` | `i = 4` violates `i <= s.len()` | ❌ FAIL (precondition) |
| `test_boundary_negative_low` | `low = -1` violates `0 <= low` | ❌ FAIL (precondition) |
| `test_boundary_empty_seq_sum_right` | Empty seq violates `s.len() > 0` | ❌ FAIL (precondition) |

### Behavioral Mutation Tests (4/4 rejected)
| Test | Mutated Property | Result |
|------|-----------------|--------|
| `test_mutation_fold_negative` | Assert `fold < 0` (negates ensures #1) | ❌ FAIL (assertion) |
| `test_mutation_prefix_greater_than_full` | Assert `prefix_fold > full_fold` (negates ensures #2) | ❌ FAIL (assertion) |
| `test_mutation_sum_right_not_equal` | Assert decomposition `!=` (negates sum_right ensures) | ❌ FAIL (assertion) |
| `test_mutation_empty_prefix_greater` | Assert `fold(prefix(0,0)) > fold(full)` | ❌ FAIL (assertion) |

### Logical Tests (4/4 rejected)
| Test | Unentailed Property | Result |
|------|---------------------|--------|
| `test_logical_strict_inequality_at_len` | Strict `<` when `i == len` (only `<=` guaranteed) | ❌ FAIL (assertion) |
| `test_logical_low_independence` | `fold(low=0) == fold(low=10)` (fold depends on low) | ❌ FAIL (assertion) |
| `test_logical_fold_equals_low` | `fold == low` for non-empty seq (not guaranteed) | ❌ FAIL (assertion) |
| `test_logical_different_accumulator` | Subtraction accumulator satisfies same `<=` (not guaranteed) | ❌ FAIL (assertion) |

## Conclusion

The specification is **consistent** with respect to all tested adversarial properties:
- **Preconditions** properly reject invalid inputs (negative indices, out-of-bounds, negative low, empty sequences)
- **Postconditions** properly reject incorrect behavioral claims (negated inequalities, wrong values)
- **Logical boundaries** properly reject unentailed reasoning (strict vs non-strict inequality, low-independence, accumulator generalization)

No specification weaknesses were detected. The spec neither admits invalid inputs nor entails unintended properties.
