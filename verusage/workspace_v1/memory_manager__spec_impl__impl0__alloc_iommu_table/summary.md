# Test Execution Summary: `alloc_iommu_table`

**Target**: `memory_manager__spec_impl__impl0__alloc_iommu_table.rs`
**Function under test**: `MemoryManager::alloc_iommu_table(&mut self, new_proc_ptr: ProcPtr) -> IOid`

## Results Overview

| Test Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 5 | 5 ✅ | 0 |
| Behavioral Mutation Tests | 5 | 5 ✅ | 0 |
| Logical Tests | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15** | **0** |

**Verdict**: All 15 adversarial tests were correctly **rejected** by Verus. The specification is **consistent** with respect to the tested properties — it does not entail any of the undesirable properties queried.

---

## Boundary Tests (`boundary_tests.rs`)

All 5 tests FAILED verification ✅ (as expected):

| # | Test | Property Challenged | Result |
|---|---|---|---|
| 1 | `test_boundary_execute_disable_not_empty` | `PageEntry` with `execute_disable=true` should not be `is_empty()` | FAIL ✅ |
| 2 | `test_boundary_ioid_at_max` | `ioid_active(IOID_MAX)` — off-by-one boundary | FAIL ✅ |
| 3 | `test_boundary_present_entry_not_empty` | `PageEntry` with `present=true` should not be `is_empty()` | FAIL ✅ |
| 4 | `test_boundary_nonzero_addr_not_empty` | `PageEntry` with `addr=4096` should not be `is_empty()` | FAIL ✅ |
| 5 | `test_boundary_pcid_at_max` | `pcid_active(PCID_MAX)` — off-by-one boundary | FAIL ✅ |

**Analysis**: The spec correctly rejects out-of-range indices and properly enforces all `is_empty()` field constraints.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All 5 tests FAILED verification ✅ (as expected):

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_wrong_proc_ptr` | Assert returned IOID maps to wrong `proc_ptr` | FAIL ✅ |
| 2 | `test_mutation_ret_not_active` | Assert returned IOID is NOT active | FAIL ✅ |
| 3 | `test_mutation_ret_was_active` | Assert returned IOID WAS active before alloc | FAIL ✅ |
| 4 | `test_mutation_nonempty_mapping` | Assert new IOID has non-empty mapping | FAIL ✅ |
| 5 | `test_mutation_other_ioid_proc_ptr_changed` | Assert another IOID's proc_ptr changed | FAIL ✅ |

**Analysis**: The spec correctly rejects all mutated behaviors — wrong proc_ptr bindings, incorrect activation status, non-empty initial mappings, and side-effects on unrelated IOIDs.

---

## Logical Tests (`logical_tests.rs`)

All 5 tests FAILED verification ✅ (as expected):

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_ret_always_zero` | Return value is always 0 (deterministic constant) | FAIL ✅ |
| 2 | `test_logical_ret_bounded_small` | Return value is always < 10 (stronger bound) | FAIL ✅ |
| 3 | `test_logical_determinism` | Two calls with same free list yield same IOID | FAIL ✅ |
| 4 | `test_logical_pcid_changes` | Allocating an IOID changes PCID active status | FAIL ✅ |
| 5 | `test_logical_free_ioids_unchanged` | `free_ioids` list is unchanged after alloc | FAIL ✅ |

**Analysis**: The spec correctly avoids entailing unintended structural assumptions — the return value is non-deterministic, unbounded within `[0, IOID_MAX)`, cross-domain isolation is maintained (PCID not affected by IOID allocation), and the free list is properly acknowledged to change.

---

## Conclusion

The `alloc_iommu_table` specification demonstrates strong consistency:
- **Input boundaries** are properly enforced (no off-by-one or field-level leaks)
- **Behavioral correctness** is maintained (mutated outputs are rejected)
- **Logical soundness** holds (no unintended entailments like determinism or cross-domain effects)

No specification weaknesses were detected in this round of adversarial testing.
