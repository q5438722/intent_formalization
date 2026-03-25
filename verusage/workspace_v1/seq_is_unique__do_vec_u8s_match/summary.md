# Adversarial Test Results — `do_vec_u8s_match`

**Target**: `seq_is_unique__do_vec_u8s_match.rs`
**Specification**: `ensures eq == (e1@ == e2@)` (no preconditions)

## Summary

| Category | Tests | All Rejected? | Verdict |
|---|---|---|---|
| Boundary | 3 | ✅ Yes | Spec correctly handles edge cases |
| Behavioral Mutation | 3 | ✅ Yes | Spec rejects incorrect behaviors |
| Logical | 3 | ✅ Yes | Spec prevents unintended reasoning |

**Overall: 9/9 adversarial tests correctly FAILED verification.** The specification is consistent.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_boundary_empty_vecs_not_equal` | Empty vectors claimed unequal | ❌ REJECTED |
| 2 | `test_boundary_same_single_element_not_equal` | Identical single-element vecs claimed unequal | ❌ REJECTED |
| 3 | `test_boundary_different_length_seqs_equal` | Different-length seqs claimed equal | ❌ REJECTED |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_equal_vecs_return_false` | Equal → false (negate output) | ❌ REJECTED |
| 2 | `test_mutation_different_vecs_return_true` | Different → true (negate output) | ❌ REJECTED |
| 3 | `test_mutation_first_element_differs_still_match` | First-element-diff → true (partial match) | ❌ REJECTED |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_match_implies_nonempty` | Matching implies non-empty | ❌ REJECTED |
| 2 | `test_logical_nondeterministic` | Same inputs → different outputs | ❌ REJECTED |
| 3 | `test_logical_asymmetric` | match(a,b) ≠ match(b,a) | ❌ REJECTED |

## Conclusion

The specification `ensures eq == (e1@ == e2@)` is **fully deterministic and complete**: it uniquely determines the return value for all inputs. The lack of preconditions correctly allows any pair of vectors. All boundary conditions, behavioral mutations, and logical queries were properly rejected, indicating the spec neither admits invalid behaviors nor supports unintended reasoning.
