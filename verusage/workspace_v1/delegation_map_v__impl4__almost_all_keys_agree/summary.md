# Adversarial Proof Test Summary

**Target**: `delegation_map_v__impl4__almost_all_keys_agree.rs`  
**Function under test**: `almost_all_keys_agree` on `DelegationMap<K>`  
**Result**: ✅ All 9 adversarial tests correctly FAILED verification (spec is robust)

---

## Boundary Tests (`boundary_tests.rs`) — 3/3 FAILED ✅

| Test | Violation | Verus Error |
|------|-----------|-------------|
| `test_boundary_lo_greater_than_hi` | `lo=5 > hi=3` violates `0 <= lo <= hi` | precondition not satisfied |
| `test_boundary_hi_equals_len` | `hi=5` with `len=5` violates `hi < len` | precondition not satisfied |
| `test_boundary_missing_agreement` | Agreement condition omitted | precondition not satisfied |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3/3 FAILED ✅

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `test_mutation_wrong_id` | Assert `range_consistent` with wrong endpoint | assertion failed |
| `test_mutation_wider_range` | Assert `range_consistent` for `[keys[0], keys[5])` instead of `[keys[1], keys[4])` | assertion failed |
| `test_mutation_hi_key_included` | Assert `dm@[keys[4]] == id@` (hi key excluded by `between`) | assertion failed |

## Logical Tests (`logical_tests.rs`) — 3/3 FAILED ✅

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| `test_logical_global_consistency` | All keys (entire domain) map to `id` | assertion failed |
| `test_logical_extend_mismatched_ids` | `extend_range_consistent` with different IDs across ranges | precondition not satisfied |
| `test_logical_empty_range_nonempty` | `empty_key_range_is_consistent` when `lo < hi` | precondition not satisfied |

---

## Conclusion

The specification of `almost_all_keys_agree` is **consistent** with respect to all tested semantic boundaries:

1. **Precondition completeness**: All three preconditions (`lo <= hi`, `hi < len`, agreement condition) are necessary — removing any one allows invalid inputs.
2. **Postcondition precision**: The postcondition `range_consistent` cannot be extended to wrong IDs, wider ranges, or the excluded hi-key endpoint.
3. **Logical tightness**: The spec does not entail global consistency, does not allow extending across mismatched IDs, and does not allow vacuous empty-range reasoning on non-empty ranges.
