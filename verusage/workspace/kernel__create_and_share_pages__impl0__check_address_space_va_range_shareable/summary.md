# Test Summary: `check_address_space_va_range_shareable`

## Target Function
`Kernel::check_address_space_va_range_shareable` — checks whether a VA range in a process's address space is shareable (all VAs mapped, all physical page reference counters have room for `va_range.len` more references).

## Results Overview

| Category | Total | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 7 | 7 ✅ | 0 |
| Behavioral Mutation Tests | 5 | 5 ✅ | 0 |
| Logical Tests | 5 | 5 ✅ | 0 |
| **Total** | **17** | **17 ✅** | **0** |

**Verdict: All 17 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (7/7 failed ✅)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_boundary_invalid_proc_ptr_assert_shareable` | Invalid proc_ptr, assert true | FAIL ✅ |
| 2 | `test_boundary_invalid_proc_ptr_assert_not_shareable` | Invalid proc_ptr, assert false | FAIL ✅ |
| 3 | `test_boundary_invalid_va_range_assert_shareable` | va_range not wf, assert true | FAIL ✅ |
| 4 | `test_boundary_kernel_not_wf_assert_shareable` | Kernel not wf, assert true | FAIL ✅ |
| 5 | `test_boundary_zero_len_assert_not_shareable` | Zero-length range, assert false | FAIL ✅ |
| 6 | `test_boundary_max_len_assert_shareable` | Max len (usize::MAX), assert true | FAIL ✅ |
| 7 | `test_boundary_kernel_not_wf_assert_not_shareable` | Kernel not wf, assert false | FAIL ✅ |

## Behavioral Mutation Tests (5/5 failed ✅)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_mutation_high_ref_counter_assert_shareable` | Ref counter too high, assert shareable | FAIL ✅ |
| 2 | `test_mutation_unmapped_va_assert_shareable` | VA not mapped, assert shareable | FAIL ✅ |
| 3 | `test_mutation_shareable_implies_va_unmapped` | Shareable → VA not mapped | FAIL ✅ |
| 4 | `test_mutation_all_conditions_met_assert_not_shareable` | All conditions met, assert not shareable | FAIL ✅ |
| 5 | `test_mutation_ref_counter_at_boundary_assert_not_shareable` | Ref counter at exact boundary, assert not shareable | FAIL ✅ |

## Logical Tests (5/5 failed ✅)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_logical_shareable_cross_proc` | Shareability transfers across processes | FAIL ✅ |
| 2 | `test_logical_shareable_implies_zero_ref_counter` | Shareable → ref counter == 0 | FAIL ✅ |
| 3 | `test_logical_shareable_monotonicity` | Shareable for range N → shareable for N+1 | FAIL ✅ |
| 4 | `test_logical_shareable_strict_inequality` | Shareable → ref counter < bound (strict) | FAIL ✅ |
| 5 | `test_logical_shareable_unique_physical_pages` | Shareable → distinct physical pages | FAIL ✅ |

---

## Conclusion

The specification for `check_address_space_va_range_shareable` is **consistent** with respect to all tested properties:

- **Boundary**: Invalid inputs (bad proc_ptr, bad va_range, non-wf kernel) are properly unresolvable — the spec does not entail any result without preconditions.
- **Behavioral**: Incorrect input/output mutations are correctly rejected — the spec precisely characterizes when a range is shareable.
- **Logical**: Unintended inferences (cross-process transfer, strict inequalities, monotonicity, physical page uniqueness) are correctly not entailed.

No spec weaknesses were detected.
