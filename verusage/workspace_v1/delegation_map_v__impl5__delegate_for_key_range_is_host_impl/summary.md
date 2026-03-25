# Adversarial Proof Test Summary

**Target**: `delegation_map_v__impl5__delegate_for_key_range_is_host_impl.rs`

## Results: All 10 tests FAILED verification ✅ (as intended)

The specification correctly rejects all invalid properties, indicating it is well-constrained.

---

### Boundary Tests (4/4 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `boundary_empty_range_is_vacuously_true` | Empty range (lo==hi) → `!delegate` | FAILED ✅ — spec correctly recognizes vacuous truth |
| `boundary_reversed_range_is_vacuously_true` | Reversed range (hi<lo) → `!delegate` | FAILED ✅ — reversed ranges are also vacuously true |
| `boundary_address_at_limit` | `id.len() == 0x100000` → `valid_physical_address` | FAILED ✅ — strict `<` boundary enforced |
| `boundary_none_lo_empty_range` | `lo=None` (end) → `!delegate` | FAILED ✅ — None lo yields empty range |

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `mutation_wrong_host` | Delegate for id₁ → delegate for id₂ ≠ id₁ | FAILED ✅ — wrong host correctly rejected |
| `mutation_key_outside_range` | Delegate for [5,10) → key 15 maps to same host | FAILED ✅ — out-of-range keys unconstrained |
| `mutation_extend_range` | Delegate for [5,10) → delegate for [5,11) | FAILED ✅ — range extension not entailed |

### Logical Tests (3/3 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `logical_global_uniformity` | Complete map → all keys map to same host | FAILED ✅ — completeness ≠ uniformity |
| `logical_range_extension_left` | Delegate for [5,10) → delegate for [2,10) | FAILED ✅ — left extension not entailed |
| `logical_disjoint_ranges_different_hosts_not_contradictory` | Disjoint ranges with different hosts → false | FAILED ✅ — valid configuration not contradictory |

## Conclusion

The `delegate_for_key_range_is_host` specification is **consistent**: it correctly rejects all tested invalid inputs (boundary violations), incorrect behaviors (mutated outputs), and unintended logical inferences (global uniformity, range extension, false contradictions). No spec weaknesses were detected.
