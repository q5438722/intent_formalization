# Adversarial Test Summary: `get_address_space_va_range_none`

## Target Function
```rust
pub fn get_address_space_va_range_none(&self, target_proc_ptr: ProcPtr, va_range: VaRange4K) -> (ret: bool)
    requires self.wf(), self.proc_dom().contains(target_proc_ptr), va_range.wf(),
    ensures ret == (forall|i: int| 0 <= i < va_range.len ==> !self.get_address_space(target_proc_ptr).dom().contains(va_range@[i]))
```

## Results: All 15 tests FAILED verification ✅

| # | Category | Test | Result | Meaning |
|---|----------|------|--------|---------|
| 1 | Boundary | `test_boundary_missing_wf` | ❌ FAIL | Spec correctly requires `kernel.wf()` |
| 2 | Boundary | `test_boundary_proc_not_in_domain` | ❌ FAIL | Spec correctly requires proc in domain |
| 3 | Boundary | `test_boundary_va_range_not_wf` | ❌ FAIL | Spec correctly requires `va_range.wf()` |
| 4 | Boundary | `test_boundary_va_range_oob` | ❌ FAIL | Out-of-bounds index rejected |
| 5 | Boundary | `test_boundary_va_range_view_len_no_wf` | ❌ FAIL | View/len consistency requires wf() |
| 6 | Mutation | `test_mutation_always_unmapped` | ❌ FAIL | Spec does not force addresses to be unmapped |
| 7 | Mutation | `test_mutation_always_mapped` | ❌ FAIL | Spec does not force addresses to be mapped |
| 8 | Mutation | `test_mutation_first_element_sufficient` | ❌ FAIL | Single-element check is insufficient |
| 9 | Mutation | `test_mutation_inverted_containment` | ❌ FAIL | Inverted containment is contradicted |
| 10 | Mutation | `test_mutation_off_by_one` | ❌ FAIL | Off-by-one range doesn't cover last element |
| 11 | Logical | `test_logical_cross_proc_determinism` | ❌ FAIL | Different procs have independent address spaces |
| 12 | Logical | `test_logical_range_implies_empty_space` | ❌ FAIL | Range unmapped ≠ all VAs unmapped |
| 13 | Logical | `test_logical_no_spatial_locality` | ❌ FAIL | No spatial locality guarantee |
| 14 | Logical | `test_logical_subrange_does_not_imply_full` | ❌ FAIL | One element unmapped ≠ all unmapped |
| 15 | Logical | `test_logical_cross_function_page_alloc` | ❌ FAIL | Address space state ≠ page allocator state |

## Conclusion

The specification for `get_address_space_va_range_none` is **robust** against all 15 adversarial queries:

- **Boundary**: All 3 preconditions (`wf()`, proc membership, VA range well-formedness) are necessary and cannot be dropped.
- **Behavioral mutations**: The postcondition correctly distinguishes mapped from unmapped, rejects partial checks, and prevents off-by-one errors.
- **Logical**: The spec does not permit cross-process inference, spatial locality assumptions, or unrelated subsystem reasoning.

No spec weaknesses (unintended entailments) were detected.

### Note on `spec_page_ptr2page_index`
During initial testing, `spec_page_ptr2page_index(0x1001) == 1` was verifiable because this spec function is pure integer division without alignment enforcement. This is by design — the alignment requirement lives on the executable function's `requires` clause, not the spec function. The test was replaced with a more relevant boundary test.
