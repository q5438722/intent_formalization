# Adversarial Proof Test Summary

## Target
`check_io_space_va_range_free` — checks whether an IO space VA range is free for a given process.

**Preconditions**: `kernel.wf()`, `proc_dom().contains(target_proc_ptr)`, `get_proc(target_proc_ptr).ioid.is_Some()`, `va_range.wf()`

**Postcondition**: `ret == io_space_range_free(target_proc_ptr, va_range)` — returns true iff no VA in the range exists in the process's IOMMU mapping.

---

## Results: All 10 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

### Boundary Tests (4/4 failed as expected)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_no_wf` | Omits `kernel.wf()` | ❌ Rejected — cannot prove IO space is free without well-formedness |
| `test_boundary_proc_not_in_domain` | `proc_ptr` not in `proc_dom` | ❌ Rejected — undefined IO space for unknown process |
| `test_boundary_ioid_none` | `ioid` is `None` | ❌ Rejected — cannot reason about IO space without a valid ioid |
| `test_boundary_zero_len_not_free` | Asserts empty range is NOT free | ❌ Rejected — empty range is vacuously free (correct) |

### Behavioral Mutation Tests (3/3 failed as expected)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_negate_when_free` | Negates result when all VAs are free | ❌ Rejected — spec correctly entails range IS free |
| `test_mutation_assert_free_when_mapped` | Claims free when first VA is mapped | ❌ Rejected — spec catches the mapped VA |
| `test_mutation_off_by_one` | All free except last VA, claims range free | ❌ Rejected — spec checks ALL elements, not len-1 |

### Logical Tests (3/3 failed as expected)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_free_implies_empty_io_space` | Free range ⇒ entire IO space empty | ❌ Rejected — free range is local, not global |
| `test_logical_cross_process` | Free for proc1 ⇒ free for proc2 | ❌ Rejected — processes have independent IO spaces |
| `test_logical_io_free_implies_regular_free` | IO free ⇒ regular page table free | ❌ Rejected — IOMMU and CPU page tables are independent |

---

## Conclusion

The specification for `check_io_space_va_range_free` is **consistent** with respect to all tested adversarial queries:
- **Preconditions are enforced**: Invalid inputs (missing wf, wrong proc, no ioid) are properly guarded.
- **Behavioral correctness is tight**: Mutated outputs (negation, off-by-one, false positives) are rejected.
- **No unintended logical consequences**: The spec does not over-generalize (no cross-process leakage, no conflation of IO and CPU mappings, no global emptiness from local freeness).
