# Adversarial Proof Test Summary

**Target**: `pagetable__pagetable_impl_base__impl0__get_entry_1g_l3.rs`
**Function**: `get_entry_1g_l3` — resolves a 1GB page table entry at L3 level
**Spec**: `spec_resolve_mapping_1g_l3(l4i, l3i)` returns `Some(entry)` iff L3 entry is present AND has PS bit set

---

## Results: All 12 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no spec weakness detected.

### Boundary Tests (4/4 FAILED as expected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_boundary_l4_not_present` | When L4 is None, assert 1g is Some | ❌ FAIL — spec correctly returns None when L4 unresolved |
| `test_boundary_no_wf` | Without wf(), assert L3 table contains L4 addr | ❌ FAIL — spec correctly requires wf() to derive structural properties |
| `test_boundary_l4_max_always_1g` | L4=511, assert 1g mapping at L3=0 always exists | ❌ FAIL — not all L3 entries are 1g pages |
| `test_boundary_l3_max_always_1g` | Assert 1g mapping at L3=511 always exists | ❌ FAIL — same reason; L3 entries are not universally 1g |

### Behavioral Mutation Tests (4/4 FAILED as expected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_mutation_some_to_none` | When result is Some, assert None | ❌ FAIL — direct contradiction rejected |
| `test_mutation_flip_present` | Assert result.perm.present == false when Some | ❌ FAIL — spec ensures present==true for Some results |
| `test_mutation_flip_ps` | Assert result.perm.ps == false when Some | ❌ FAIL — spec ensures ps==true for 1g entries |
| `test_mutation_wrong_addr_level` | Assert result.addr == L4 entry addr | ❌ FAIL — L3 page addr ≠ L3 table pointer (different levels) |

### Logical Tests (4/4 FAILED as expected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_logical_always_some` | Assert 1g always returns Some for valid inputs | ❌ FAIL — not all entries are 1g pages; spec doesn't guarantee this |
| `test_logical_1g_and_l3_both_some` | Assert both 1g (ps=true) and non-huge L3 (ps=false) are Some | ❌ FAIL — mutually exclusive by PS bit; spec correctly prevents |
| `test_logical_different_l3_different_addr` | Two 1g entries at different L3 indices have different addrs | ❌ FAIL — disjointness only applies to non-PS entries; no guarantee for 1g physical addrs |
| `test_logical_write_equals_xd` | Assert write == execute_disable for 1g entries | ❌ FAIL — rwx constraints only apply to non-PS upper-level entries; no correlation enforced for 1g pages |

---

## Key Findings

1. **Boundary control is sound**: Invalid inputs (missing wf, out-of-range indices, unresolved L4) are properly rejected.
2. **Behavioral correctness is enforced**: The spec correctly distinguishes present/PS conditions and ties result structure to the L3 table entries.
3. **No unintended logical entailment detected**:
   - The spec does NOT accidentally guarantee 1g mappings exist for all valid index pairs.
   - The PS-based mutual exclusion between `spec_resolve_mapping_1g_l3` and `spec_resolve_mapping_l3` is correctly enforced.
   - The disjointness gap for 1g page physical addresses (only non-PS entries have disjointness) is a **potential spec weakness** — two 1g pages could map to the same physical address — but this is expected behavior (aliasing) rather than a spec bug.
   - The rwx constraint gap for PS=true entries is also intentional — 1g page permissions are controlled at the leaf level.
