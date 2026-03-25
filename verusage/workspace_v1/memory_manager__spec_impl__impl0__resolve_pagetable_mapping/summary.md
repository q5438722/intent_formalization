# Adversarial Proof Test Summary

**Target**: `memory_manager__spec_impl__impl0__resolve_pagetable_mapping.rs`  
**Function**: `MemoryManager::resolve_pagetable_mapping(pcid, va) -> Option<PageEntry>`  
**Specification**: Requires `wf()`, `pcid_active(pcid)`, `va_4k_valid(va)`. Ensures mapping_4k contains va iff result is Some, and the returned entry matches the stored MapEntry.

---

## Results: All 15 tests FAIL verification ✅

All adversarial tests were correctly rejected by the specification, indicating the spec is sufficiently strong in the areas tested.

### Boundary Tests (5/5 FAIL ✅)

| Test | Description | Result |
|------|-------------|--------|
| B1 | Inactive PCID (in free set) asserted as active | FAIL ✅ |
| B2 | `va_4k_valid(0)` — VA 0 has L4 index 0 < KERNEL_MEM_END_L4INDEX | FAIL ✅ |
| B3 | `pcid_active(PCID_MAX)` — PCID at upper boundary | FAIL ✅ |
| B4 | `va_4k_valid(0x1001)` — non-4K-aligned address | FAIL ✅ |
| B5 | PageEntry with `present=true` asserted as `is_empty()` | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Description | Result |
|------|-------------|--------|
| M1 | L1 resolution is None but mapping_4k asserted to contain VA | FAIL ✅ |
| M2 | `spec_page_entry_to_map_entry` addr mutated (assert ≠ 4096) | FAIL ✅ |
| M3 | Write permission flipped (assert write=false when true) | FAIL ✅ |
| M4 | Execute_disable flipped (assert false when true) | FAIL ✅ |
| M5 | Both L3 and 1G-L3 asserted as Some (mutually exclusive) | FAIL ✅ |

### Logical Tests (5/5 FAIL ✅)

| Test | Description | Result |
|------|-------------|--------|
| L1 | mapping_4k asserted always non-empty (not guaranteed) | FAIL ✅ |
| L2 | Different PCIDs asserted to have same mapping domain | FAIL ✅ |
| L3 | L4 entry addr asserted to be zero (arbitrary) | FAIL ✅ |
| L4 | One L4 None asserted to imply another L4 None | FAIL ✅ |
| L5 | All mappings asserted writable (not guaranteed) | FAIL ✅ |

---

## Conclusion

The specification for `resolve_pagetable_mapping` correctly:
- **Rejects invalid inputs**: Inactive PCIDs, out-of-range PCIDs, invalid VAs, and unaligned addresses are all properly guarded by preconditions.
- **Rejects incorrect behaviors**: Mutated outputs (wrong addresses, flipped permissions, contradictory resolution results) are caught by the spec.
- **Rejects unintended reasoning**: Stronger logical claims (universal non-emptiness, cross-PCID equivalence, arbitrary address values, monotonicity of None results, universal writability) are not entailed.

No spec weaknesses were found in the tested areas.
