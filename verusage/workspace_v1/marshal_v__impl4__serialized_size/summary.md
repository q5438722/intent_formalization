# Adversarial Test Results: marshal_v__impl4__serialized_size

## Overview

Target: `source-projects/ironkv/verified/marshal_v/marshal_v__impl4__serialized_size.rs`

**12/12 tests correctly FAILED verification** — the specification properly rejects all adversarial queries.

---

## Boundary Tests (4/4 FAILED ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_empty_seq_sum_right` | Empty seq violates `s.len() > 0` | precondition not satisfied ✅ |
| `test_negative_index` | `i = -1` violates `0 <= i` | precondition not satisfied ✅ |
| `test_index_exceeds_length` | `i = 4` violates `i <= s.len()` (len=3) | precondition not satisfied ✅ |
| `test_negative_low` | `low = -1` violates `0 <= low` | precondition not satisfied ✅ |

**Conclusion**: All preconditions are properly enforced. Invalid inputs are correctly rejected.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_u64_serialize_wrong_length` | Assert `spec_u64_to_le_bytes` length == 4 (should be 8) | assertion failed ✅ |
| `test_sum_right_subtraction` | Mutate `+` to `-` in fold decomposition | assertion failed ✅ |
| `test_fold_append_len_off_by_one` | Assert fold-append-len equality with +1 offset | assertion failed ✅ |
| `test_fold_le_reversed` | Assert subrange fold `>` full fold (spec says `<=`) | assertion failed ✅ |

**Conclusion**: All behavioral mutations are correctly rejected. The spec distinguishes correct from incorrect behaviors.

---

## Logical Tests (4/4 FAILED ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_sum_right_not_unsound` | `assert(false)` after lemma — checks axiom soundness | assertion failed ✅ |
| `test_fold_strictly_greater_than_low` | `fold(5,...) > 5` (spec only gives `>= 0`) | assertion failed ✅ |
| `test_subrange_strict_less_than` | Strict `<` instead of `<=` with zero-contribution function | assertion failed ✅ |
| `test_cross_function_misuse` | Equate sum-of-values with count-of-elements across lemmas | assertion failed ✅ |

**Conclusion**: No unsoundness detected in external axioms. The spec does not entail stronger-than-stated properties or allow unintended cross-function reasoning.

---

## Overall Assessment

The specification for `marshal_v__impl4__serialized_size` is **consistent** with respect to the tested semantic boundaries:

1. **Input validity**: Preconditions correctly guard all lemma entry points.
2. **Behavioral correctness**: Mutated postconditions are properly rejected.
3. **Logical soundness**: External-body axioms do not introduce contradictions, and the spec does not entail unintended stronger properties.

No specification weaknesses were detected by these adversarial tests.
