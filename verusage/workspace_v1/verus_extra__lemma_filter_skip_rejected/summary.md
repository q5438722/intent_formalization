# Test Summary: `lemma_filter_skip_rejected`

## Specification Under Test

```rust
pub proof fn lemma_filter_skip_rejected<A>(s: Seq<A>, pred: spec_fn(A) -> bool, i: int)
    requires
        0 <= i <= s.len(),
        forall |j| 0 <= j < i ==> !pred(s[j]),
    ensures
        s.filter(pred) == s.skip(i).filter(pred)
```

**Semantics**: If the first `i` elements of sequence `s` are all rejected by predicate `pred`, then filtering `s` equals filtering `s` after skipping those `i` elements.

---

## Results Overview

| Test Category         | Total | Failed (as expected) | Passed (unexpected) |
|-----------------------|-------|----------------------|---------------------|
| Boundary Tests        | 5     | 5                    | 0                   |
| Behavioral Mutations  | 5     | 5                    | 0                   |
| Logical Tests         | 5     | 5                    | 0                   |
| **Total**             | **15**| **15**               | **0**               |

✅ **All 15 tests correctly failed verification.** The specification is consistent — it rejects all tested invalid inputs, incorrect behaviors, and unintended logical inferences.

---

## Boundary Tests (5/5 failed ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_negative_index` | `i = -1`, violates `0 <= i` | Failed: precondition not satisfied |
| `test_boundary_i_exceeds_length` | `i = 4` on len-3 seq, violates `i <= s.len()` | Failed: precondition not satisfied |
| `test_boundary_middle_element_satisfies_pred` | `s[1] = 20 > 10`, violates forall rejection | Failed: precondition not satisfied |
| `test_boundary_first_element_satisfies_pred` | `s[0] = 100 > 10`, violates forall rejection | Failed: precondition not satisfied |
| `test_boundary_all_before_i_satisfy_pred` | All elements satisfy pred, violates forall rejection | Failed: precondition not satisfied |

## Behavioral Mutation Tests (5/5 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_negate_postcondition` | Assert filter results differ (`!==`) | Failed: assertion failed |
| `test_mutation_filter_equals_original` | Assert filter equals original sequence | Failed: assertion failed |
| `test_mutation_filter_result_is_empty` | Assert filter result is empty when it shouldn't be | Failed: assertion failed |
| `test_mutation_skip_one_more` | Assert `skip(i+1).filter` equals `s.filter` when `s[i]` satisfies pred | Failed: assertion failed |
| `test_mutation_filter_len_equals_original` | Assert filter length equals original length | Failed: assertion failed |

## Logical Tests (5/5 failed ✅)

| Test | Unwarranted Property | Result |
|------|---------------------|--------|
| `test_logical_skip_equals_original` | `s.skip(i) =~= s` (skip is identity) | Failed: assertion failed |
| `test_logical_filter_must_be_empty` | Filter result has length 0 | Failed: assertion failed |
| `test_logical_different_predicate` | Conclusion transfers to a different predicate | Failed: assertion failed |
| `test_logical_extend_sequence` | Conclusion extends to `s.push(100)` | Failed: assertion failed |
| `test_logical_seq_is_empty` | Original sequence must be empty | Failed: assertion failed |

---

## Conclusion

The specification for `lemma_filter_skip_rejected` is **well-formed and consistent** across all three testing dimensions:

1. **Preconditions are tight**: Invalid index ranges and predicate violations on prefix elements are correctly rejected.
2. **Postcondition is precise**: Mutated output relations (negation, wrong equality, empty result, off-by-one skip, length mismatch) are all rejected.
3. **No unintended entailments**: The spec does not leak conclusions about skip identity, filter emptiness, cross-predicate transfer, sequence extension, or sequence emptiness.
