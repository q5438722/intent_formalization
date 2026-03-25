# Test Summary: `check_address_space_va_range_free`

## Target Function
```rust
pub fn check_address_space_va_range_free(&self, target_proc_ptr: ProcPtr, va_range: &VaRange4K) -> (ret: bool)
    requires self.wf(), self.proc_dom().contains(target_proc_ptr), va_range.wf(),
    ensures  ret == self.address_space_range_free(target_proc_ptr, va_range),
```

## Results: ALL 15 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no spec weakness detected.

### Boundary Tests (5/5 failed ✅)
| Test | Property Asserted | Result |
|------|-------------------|--------|
| B1 | `address_space_range_free` with proc_ptr outside `proc_dom` | FAILED ✅ |
| B2 | Zero-length range is NOT free (vacuously true) | FAILED ✅ |
| B3 | `spec_va_4k_valid(0)` holds | FAILED ✅ |
| B4 | `page_ptr_valid(1)` (unaligned) holds | FAILED ✅ |
| B5 | `page_index_valid(NUM_PAGES)` (off-by-one) holds | FAILED ✅ |

### Behavioral Mutation Tests (5/5 failed ✅)
| Test | Property Asserted | Result |
|------|-------------------|--------|
| M1 | First VA mapped but range still free | FAILED ✅ |
| M2 | Free range returns false | FAILED ✅ |
| M3 | Last VA mapped but range still free | FAILED ✅ |
| M4 | PageEntry with `present=true` is empty | FAILED ✅ |
| M5 | PageEntry with `addr=4096` is empty | FAILED ✅ |

### Logical Tests (5/5 failed ✅)
| Test | Property Asserted | Result |
|------|-------------------|--------|
| L1 | Different proc_ptrs yield same range_free result | FAILED ✅ |
| L2 | Single VA free implies entire range free | FAILED ✅ |
| L3 | Range free implies entire address space empty | FAILED ✅ |
| L4 | Range free on one kernel implies free on another | FAILED ✅ |
| L5 | page_index↔page_ptr roundtrip breaks | FAILED ✅ |

## Conclusion

The specification for `check_address_space_va_range_free` is **consistent** with respect to all 15 adversarial queries:
- **Boundary**: Invalid inputs are correctly rejected by preconditions.
- **Behavioral**: Incorrect output mutations are correctly rejected by the postcondition.
- **Logical**: Unintended inferences (cross-process, cross-kernel, partial-to-total) are correctly not entailed.
