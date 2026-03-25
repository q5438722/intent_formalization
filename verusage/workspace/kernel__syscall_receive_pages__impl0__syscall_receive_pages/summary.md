# Adversarial Test Summary: `syscall_receive_pages`

## Target
`kernel__syscall_receive_pages__impl0__syscall_receive_pages.rs`

## Results: 15/15 tests FAIL verification ✅

All adversarial tests were correctly rejected by the specification, indicating the spec is **consistent** for the tested properties.

---

### Boundary Tests (5/5 FAIL ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| B1 | `page_ptr2page_index(1)` | Non-aligned ptr violates `ptr % 0x1000 == 0` | ✅ Precondition rejected |
| B2 | `page_index2page_ptr(NUM_PAGES)` | Out-of-range index violates `i < NUM_PAGES` | ✅ Precondition rejected |
| B3 | `page_ptr2page_index(0x1001)` | Off-by-one alignment violation | ✅ Precondition rejected |
| B4 | `page_index2page_ptr(usize::MAX)` | Extreme out-of-range index | ✅ Precondition rejected |
| B5 | `page_ptr2page_index(0)` then assert `ret != 0` | Valid input, wrong assertion on output | ✅ Assertion rejected |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| M1 | `page_ptr2page_index(0x2000)` assert `ret == 3` | Mutated expected result (correct: 2) | ✅ Assertion rejected |
| M2 | `page_index2page_ptr(5)` assert `ret == 0` | Mutated expected result (correct: 20480) | ✅ Assertion rejected |
| M3 | `NoSwitchNew(Error)` assert `!is_error()` | Error status negated | ✅ Assertion rejected |
| M4 | `NoSwitchNew(Else)` assert `is_error()` | Non-error claimed as error | ✅ Assertion rejected |
| M5 | `NoSwitchNew(Error)` assert `switch_decision == Switch` | Switch decision mutated | ✅ Assertion rejected |

### Logical Tests (5/5 FAIL ✅)

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| L1 | ∀ aligned ptr: `spec_page_ptr2page_index(ptr) < NUM_PAGES` | Unbounded result range (spec doesn't bound output) | ✅ Assertion rejected |
| L2 | `va_4k_valid(0) == true` | VA=0 validity (0 >> 39 & 0x1ff < 1) | ✅ Assertion rejected |
| L3 | Arbitrary struct with `pcid=Some(42)` assert `pcid.is_None()` | Spec scope: NoSwitchNew guarantees ≠ universal property | ✅ Assertion rejected |
| L4 | `spec_page_ptr2page_index(0x1000) == spec_page_ptr2page_index(0x2000)` | False collision claim (different ptrs → different indices) | ✅ Assertion rejected |
| L5 | `NoSwitchNew(Else).cr3.is_Some()` | cr3 must be None per spec | ✅ Assertion rejected |

---

## Conclusion

The specification correctly rejects all 15 adversarial queries across all three categories:
- **Boundary**: Preconditions properly guard against invalid inputs (alignment, range)
- **Behavioral**: Postconditions correctly specify input-output relationships
- **Logical**: Spec does not over-generalize; it correctly handles scope boundaries

**No spec weaknesses detected** in the tested surface area. The `page_ptr2page_index`, `page_index2page_ptr`, `va_4k_valid`, and `NoSwitchNew` functions have adequate specifications. The main `syscall_receive_pages` function could not be directly tested in isolation due to its complex `Kernel` state preconditions, but its helper functions and return type specifications are sound.
