# Test Summary: `share_mapping` Adversarial Proof Tests

**Target**: `kernel__create_and_share_pages__impl0__share_mapping.rs`
**Function under test**: `Kernel::share_mapping` — shares a page mapping from a source process to a target process.

---

## Results Overview

| Test Category | Total | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary Tests | 10 | 10 ✅ | 0 |
| Behavioral Mutation Tests | 10 | 10 ✅ | 0 |
| Logical Tests | 10 | 10 ✅ | 0 |
| **Total** | **30** | **30** | **0** |

All 30 tests failed verification as intended, meaning the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical inferences.

---

## Boundary Tests (`boundary_tests.rs`)

All tests violate preconditions of `share_mapping`:

| # | Test | Violated Precondition | Result |
|---|---|---|---|
| 1 | `test_boundary_src_proc_not_in_domain` | `proc_dom().contains(src_proc_ptr)` | FAIL ✅ |
| 2 | `test_boundary_target_proc_not_in_domain` | `proc_dom().contains(target_proc_ptr)` | FAIL ✅ |
| 3 | `test_boundary_target_va_zero` | `va_4k_valid(target_va)` — VA=0 is kernel space | FAIL ✅ |
| 4 | `test_boundary_entry_addr_misaligned` | `page_ptr_valid(entry.addr)` — 0x1001 not aligned | FAIL ✅ |
| 5 | `test_boundary_entry_addr_out_of_range` | `page_ptr_valid(entry.addr)` — index ≥ NUM_PAGES | FAIL ✅ |
| 6 | `test_boundary_target_va_already_mapped` | `get_address_space(target).dom().contains(va) == false` | FAIL ✅ |
| 7 | `test_boundary_src_va_not_mapped` | `get_address_space(src).dom().contains(src_va)` | FAIL ✅ |
| 8 | `test_boundary_ref_counter_overflow` | `get_physical_page_reference_counter ≤ usize::MAX - 1` | FAIL ✅ |
| 9 | `test_boundary_target_va_not_aligned` | `va_4k_valid(target_va)` — 0x1001 not 4k-aligned | FAIL ✅ |
| 10 | `test_boundary_page_not_mapped` | `page_alloc.page_is_mapped(entry.addr)` | FAIL ✅ |

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All tests start from valid postconditions and mutate expected results:

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_free_pages_decreased` | Free pages decreased by 1 (should be unchanged) | FAIL ✅ |
| 2 | `test_mutation_ref_counter_incremented_by_2` | Ref counter +2 (should be +1) | FAIL ✅ |
| 3 | `test_mutation_address_space_unchanged` | Target address space unchanged (should have insert) | FAIL ✅ |
| 4 | `test_mutation_other_proc_space_changed` | Other proc's space changed (should be preserved) | FAIL ✅ |
| 5 | `test_mutation_proc_dom_grew` | proc_dom grew (should be preserved) | FAIL ✅ |
| 6 | `test_mutation_page_mapping_dom_grew` | page_mapping domain grew (should be preserved) | FAIL ✅ |
| 7 | `test_mutation_other_page_ref_changed` | Other page's ref counter changed (should be preserved) | FAIL ✅ |
| 8 | `test_mutation_other_page_mapping_changed` | Other page's mapping set changed (should be preserved) | FAIL ✅ |
| 9 | `test_mutation_target_page_mapping_unchanged` | Target page's mapping unchanged (should have insert) | FAIL ✅ |
| 10 | `test_mutation_container_dom_shrank` | Container domain shrank (should be preserved) | FAIL ✅ |

---

## Logical Tests (`logical_tests.rs`)

All tests probe properties NOT explicitly guaranteed by the specification:

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_double_share_ref_counter` | Two shares only increment ref counter by 1 | FAIL ✅ |
| 2 | `test_logical_share_symmetry` | Sharing is symmetric (A→B implies B→A) | FAIL ✅ |
| 3 | `test_logical_self_share_removes_src` | Self-share uses move semantics (removes src) | FAIL ✅ |
| 4 | `test_logical_shared_entry_read_only` | Shared entry is forced read-only | FAIL ✅ |
| 5 | `test_logical_free_pages_increase` | Sharing frees a page | FAIL ✅ |
| 6 | `test_logical_mapped_status_changed` | Sharing changes page_is_mapped status | FAIL ✅ |
| 7 | `test_logical_endpoint_dom_changed` | Sharing creates a new endpoint | FAIL ✅ |
| 8 | `test_logical_pcid_mapping_changed` | Sharing changes pcid_to_proc_ptr mapping | FAIL ✅ |
| 9 | `test_logical_container_owned_pages_changed` | Sharing changes container's owned pages | FAIL ✅ |
| 10 | `test_logical_double_insert_in_single_call` | Single call inserts two VAs | FAIL ✅ |

---

## Conclusion

The specification for `share_mapping` is **consistent** with respect to all 30 adversarial queries tested. The spec:
- Properly rejects all invalid inputs (boundary violations)
- Correctly distinguishes intended from incorrect behaviors (mutation detection)
- Does not allow unintended logical inferences (logical property rejection)

No specification weaknesses were detected.
