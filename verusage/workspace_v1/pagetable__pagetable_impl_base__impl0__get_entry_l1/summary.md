# Adversarial Proof Test Summary: `get_entry_l1`

**Target**: `pagetable__pagetable_impl_base__impl0__get_entry_l1.rs`
**Function**: `PageTable::get_entry_l1` — resolves a 4K page entry at level 1 of a 4-level page table.

## Results

All **12 tests** across 3 files **failed verification** as expected, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended logical claims.

| File | Tests | Failures | Status |
|------|-------|----------|--------|
| `boundary_tests.rs` | 4 | 4 | ✅ All rejected |
| `behavioral_mutation_tests.rs` | 4 | 4 | ✅ All rejected |
| `logical_tests.rs` | 4 | 4 | ✅ All rejected |

---

## Boundary Tests (4/4 FAILED ✅)

| # | Test | Property Probed | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_kernel_space_l4i` | L4 index in kernel space (l4i=0) — mapping_4k unconstrained | REJECTED |
| 2 | `test_boundary_l1i_overflow` | L1 index = 512 (out of bounds) — behavior undefined | REJECTED |
| 3 | `test_boundary_no_wf_cascade` | No `wf()` precondition — mapping_4k unconstrained | REJECTED |
| 4 | `test_boundary_kernel_l4_end_value` | Pin `kernel_l4_end` to specific value (0) — not entailed | REJECTED |

## Behavioral Mutation Tests (4/4 FAILED ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_not_present_returns_some` | L1 not present → assert Some (should be None) | REJECTED |
| 2 | `test_mutation_present_returns_none` | L1 present → assert None (should be Some) | REJECTED |
| 3 | `test_mutation_mapping_exists_when_none` | L1 None → assert mapping_4k contains VA | REJECTED |
| 4 | `test_mutation_wrong_mapping_addr` | Assert mapping address is `addr + 0x1000` | REJECTED |

## Logical Tests (4/4 FAILED ✅)

| # | Test | Unentailed Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_determinism` | Two wf page tables resolve identically | REJECTED |
| 2 | `test_logical_totality` | All valid L1 indices resolve (total function) | REJECTED |
| 3 | `test_logical_l1_injective` | Different L1 entries → different physical addrs | REJECTED |
| 4 | `test_logical_l2_implies_all_l1` | L2 resolved → all 512 L1 entries present | REJECTED |

## Conclusion

The specification for `get_entry_l1` is **consistent** with respect to all 12 adversarial queries:
- **Boundary**: Invalid inputs (kernel space, overflow, missing wf) are not admitted.
- **Behavioral**: Mutated output relations (flipped Some/None, wrong addresses) are rejected.
- **Logical**: Unentailed properties (determinism, totality, injectivity, universal L1 presence) cannot be derived.

No specification weaknesses were detected.
