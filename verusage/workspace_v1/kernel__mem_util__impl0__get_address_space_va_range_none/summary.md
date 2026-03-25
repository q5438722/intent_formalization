# Adversarial Proof Test Summary

**Target**: `kernel__mem_util__impl0__get_address_space_va_range_none`

**Function under test**: `Kernel::get_address_space_va_range_none(target_proc_ptr, va_range) -> bool`

**Specification**:
- **Requires**: `self.wf()`, `self.proc_dom().contains(target_proc_ptr)`, `va_range.wf()`
- **Ensures**: `ret == (∀ i: 0 ≤ i < va_range.len → ¬(get_address_space(proc_ptr).dom().contains(va_range@[i])))`

---

## Results: All 15 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification, indicating the spec is sufficiently tight to prevent the tested classes of incorrect reasoning.

### Boundary Tests (5/5 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_no_kernel_wf` | Missing `kernel.wf()` precondition — asserts address space property | FAIL ✅ |
| `test_boundary_proc_not_in_domain` | `proc_ptr` not in domain — asserts address space property | FAIL ✅ |
| `test_boundary_va_zero_not_4k_valid` | VA=0 is not 4k-valid (L4 index < KERNEL_MEM_END_L4INDEX) | FAIL ✅ |
| `test_boundary_va_range_len_no_wf` | Without `va_range.wf()`, `view.len() == len` is not guaranteed | FAIL ✅ |
| `test_boundary_va_range_no_dup_no_wf` | Without `va_range.wf()`, `no_duplicates()` is not guaranteed | FAIL ✅ |

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_mapped_va_asserted_unmapped` | VA is mapped but we assert all-unmapped forall | FAIL ✅ |
| `test_mutation_unmapped_but_assert_exists_mapped` | All VAs unmapped but we assert exists-mapped | FAIL ✅ |
| `test_mutation_last_index_mapped_breaks_forall` | Last VA mapped, assert full forall still holds | FAIL ✅ |
| `test_mutation_wrong_proc_address_space` | Transfer unmapped property from proc1 to proc2 | FAIL ✅ |
| `test_mutation_off_by_one_index` | Access `va_range@[len]` (out of specified range) | FAIL ✅ |

### Logical Tests (5/5 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_different_procs_same_address_space` | Different procs must have identical address spaces | FAIL ✅ |
| `test_logical_range_none_implies_empty_space` | Unmapped range implies entirely empty address space | FAIL ✅ |
| `test_logical_valid_va_always_mapped` | All valid VAs must be in every proc's address space | FAIL ✅ |
| `test_logical_diff_pcid_implies_disjoint_space` | Different pcids imply disjoint address spaces | FAIL ✅ |
| `test_logical_unmapped_range_not_generalizable` | Unmapped range implies arbitrary VA is unmapped | FAIL ✅ |

---

## Conclusion

The specification for `get_address_space_va_range_none` correctly:
1. **Rejects boundary violations**: Invalid inputs (missing wf, proc not in domain, invalid VAs) cannot be used to derive postcondition properties.
2. **Rejects behavioral mutations**: Incorrect input-output relationships (mapped VAs claimed unmapped, wrong proc, off-by-one) are properly rejected.
3. **Rejects logical overreach**: Unintended global properties (address space equality, complete emptiness, universal mapping, disjointness, generalization beyond range) are not entailed.

No specification weaknesses were detected in these test categories.
