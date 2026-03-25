# Adversarial Test Results Summary

**Target**: `memory_manager__spec_impl__impl0__create_pagetable_l3_entry.rs`
**Function under test**: `MemoryManager::create_pagetable_l3_entry`

## Overall Results

| Test File | Tests | Expected Failures | Actual Failures | Passes (spec weakness) |
|---|---|---|---|---|
| boundary_tests.rs | 5 | 5 | 5 | 0 |
| behavioral_mutation_tests.rs | 5 | 5 | 5 | 0 |
| logical_tests.rs | 5 | 5 | 5 | 0 |
| **Total** | **15** | **15** | **15** | **0** |

## Detailed Results

### Boundary Tests (5/5 FAIL ✓)

All boundary tests correctly fail, confirming the spec rejects invalid inputs:

| Test | Property Tested | Result |
|---|---|---|
| `test_boundary_unaligned_page_ptr` | `page_ptr_valid(0x1001)` — unaligned pointer | FAIL ✓ |
| `test_boundary_ptr_below_page` | `page_ptr_valid(0xFFF)` — just below 4K boundary | FAIL ✓ |
| `test_boundary_l4i_kernel_range` | `KERNEL_MEM_END_L4INDEX <= 0` — L4 index in kernel range | FAIL ✓ |
| `test_boundary_present_not_empty` | `entry.is_empty()` with `present=true` | FAIL ✓ |
| `test_boundary_mem_valid_low_bits` | `MEM_valid(1)` — address with low bits set | FAIL ✓ |

### Behavioral Mutation Tests (5/5 FAIL ✓)

All mutation tests correctly fail, confirming the spec rejects incorrect behaviors:

| Test | Postcondition Mutated | Result |
|---|---|---|
| `test_mutation_new_mapping_appears` | mapping_4k preserved → new VA appears | FAIL ✓ |
| `test_mutation_closure_unchanged` | page_closure grows → stays same | FAIL ✓ |
| `test_mutation_l3_addr_wrong` | L3 entry addr == page_map_ptr → different addr | FAIL ✓ |
| `test_mutation_1g_becomes_some` | 1g L3 resolve is None → is Some | FAIL ✓ |
| `test_mutation_pcid_active_flips` | pcid_active preserved → status flips | FAIL ✓ |

### Logical Tests (5/5 FAIL ✓)

All logical tests correctly fail, confirming the spec does not entail unintended properties:

| Test | Unintended Property Probed | Result |
|---|---|---|
| `test_logical_null_ptr_invalid` | `!page_ptr_valid(0)` — is null pointer excluded? | FAIL ✓ |
| `test_logical_present_implies_write` | present + non-PS → write must be true | FAIL ✓ |
| `test_logical_different_l3_resolves` | one L3 resolves → another L3 also resolves | FAIL ✓ |
| `test_logical_ptpages_value_unconstrained` | domain constraint → determines stored value | FAIL ✓ |
| `test_logical_l3_user_unconstrained` | L3 addr + present + non-PS → user must be true | FAIL ✓ |

## Analysis

The specification for `create_pagetable_l3_entry` correctly:

1. **Rejects invalid inputs**: Boundary conditions on `page_ptr_valid`, `KERNEL_MEM_END_L4INDEX`, `MEM_valid`, and `is_empty` are properly enforced.

2. **Rejects incorrect behaviors**: All postcondition mutations (mapping preservation, page closure growth, L3 resolve correctness, 1g resolve absence, pcid stability) are correctly constrained.

3. **Does not entail unintended properties**: Permission bits (write, user) are appropriately unconstrained. L3 indices are independent. The domain-only constraint on `page_table_pages` does not leak value information.

### Notable Observations

- **Null pointer is valid**: `page_ptr_valid(0)` evaluates to `true` (0 is 4K-aligned and 0 < NUM_PAGES). This is a design choice — address 0 is a valid page pointer in this memory model. Whether this is intentional depends on the system's memory layout.

- **page_table_pages value unconstrained**: The postcondition only constrains the domain of `page_table_pages` (adds `page_map_ptr`), not the value stored. The actual value (`target_pcid`) is set in the function body but not exposed in the postcondition. This is by design — the `wf()` invariant (`no_memory_leak`) indirectly constrains it.
