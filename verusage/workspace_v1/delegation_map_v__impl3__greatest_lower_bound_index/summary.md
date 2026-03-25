# Adversarial Proof Test Summary: `greatest_lower_bound_index`

**Target**: `delegation_map_v__impl3__greatest_lower_bound_index`
**Result**: ✅ All 9 tests correctly FAILED verification (spec rejects all invalid claims)

---

## Boundary Tests (3/3 failed ✅)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_boundary_empty_map` | GLB postcondition holds with 0 keys (no valid index in `[0, 0)`) | ❌ FAIL |
| `test_boundary_oob_index` | GLB postcondition holds at index `== keys.len()` (off-by-one) | ❌ FAIL |
| `test_boundary_negative_index` | GLB postcondition holds at index `-1` | ❌ FAIL |

**Conclusion**: The spec correctly bounds the result index to `[0, keys.len())`.

## Behavioral Mutation Tests (3/3 failed ✅)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_behavioral_glb_after_iter` | GLB is strictly after iter (`iter < glb`) — negates `glb ≤ iter` | ❌ FAIL |
| `test_behavioral_glb_not_in_map` | GLB key is NOT in the map (non-end iter) — negates membership guarantee | ❌ FAIL |
| `test_behavioral_not_greatest` | A map key below iter is NOT below GLB — negates "greatest" property | ❌ FAIL |

**Conclusion**: The spec correctly constrains: (1) GLB ≤ iter, (2) GLB key is in the map, (3) GLB is the greatest such lower bound.

## Logical Tests (3/3 failed ✅)

| Test | Property Asserted | Result |
|------|-------------------|--------|
| `test_logical_always_first` | GLB index is always 0 | ❌ FAIL |
| `test_logical_different_iters_different_glbs` | Different iterators must yield different GLB indices | ❌ FAIL |
| `test_logical_always_last` | GLB index is always `keys.len() - 1` | ❌ FAIL |

**Conclusion**: The spec does not over-constrain the GLB index — it correctly depends on iter position, and two distinct iterators may share the same GLB.

---

## Overall Assessment

The `greatest_lower_bound_spec` specification is **well-formed** against all tested adversarial queries:
- **Boundary**: Invalid indices are rejected.
- **Behavioral**: Mutated postcondition properties are rejected.
- **Logical**: Unentailed structural claims (fixed index, injectivity) are rejected.

No spec weakness was detected across the 9 adversarial tests.
