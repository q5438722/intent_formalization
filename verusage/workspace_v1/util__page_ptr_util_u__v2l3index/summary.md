# Adversarial Proof Test Summary: `v2l3index`

**Target**: `util__page_ptr_util_u__v2l3index.rs`
**Function under test**: `v2l3index(va: usize) -> L3Index` — extracts L3 page table index (bits [38:30]) from a virtual address.

## Specification Summary

- **Precondition**: `va_4k_valid(va) || va_2m_valid(va) || va_1g_valid(va)` — VA must be page-aligned and in kernel memory space (L4 index ≥ 1).
- **Postcondition**: `ret == spec_v2l3index(va)` and `ret <= 0x1ff` — result is the 9-bit L3 index.

## Results Overview

| Category | Tests | All Failed? | Verdict |
|----------|-------|-------------|---------|
| Boundary | 4 | ✅ Yes | Preconditions correctly reject invalid inputs |
| Behavioral | 4 | ✅ Yes | Postconditions correctly specify behavior |
| Logical | 4 | ✅ Yes | Spec does not entail unintended properties |

**Total: 12/12 tests correctly rejected by the verifier.**

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Invalid Input | Failure Mode | Result |
|---|------|--------------|--------------|--------|
| 1 | `test_boundary_zero_valid` | `va = 0` | L4 index = 0 < 1 | ✅ FAILED |
| 2 | `test_boundary_unaligned_address` | `va = 0x0000_0080_0000_0001` | Bit 0 set, misaligned for all page sizes | ✅ FAILED |
| 3 | `test_boundary_low_l4_index` | `va = 0x1000` | 4K-aligned but L4 index = 0 | ✅ FAILED |
| 4 | `test_boundary_upper_bits_set` | `va = 0xffff_0080_0000_0000` | Upper 16 bits set, fails alignment mask | ✅ FAILED |

## Behavioral Mutation Tests (`behavioral_tests.rs`)

| # | Test | Valid Input | Mutation | Result |
|---|------|-----------|----------|--------|
| 1 | `test_behavioral_wrong_value` | `va = 0x80_4000_0000` (L3=1) | Assert L3 == 2 | ✅ FAILED |
| 2 | `test_behavioral_exceeds_bound` | `va = 0xff_c000_0000` (L3=0x1ff) | Assert L3 > 0x1ff | ✅ FAILED |
| 3 | `test_behavioral_off_by_one` | `va = 0x80_4000_0000` (L3=1) | Assert L3 == 0 | ✅ FAILED |
| 4 | `test_behavioral_negate_result` | `va = 0x80_0000_0000` (L3=0) | Assert L3 != 0 | ✅ FAILED |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property | Rationale | Result |
|---|------|-------------------|-----------|--------|
| 1 | `test_logical_4k_implies_1g` | 4K-valid ⟹ 1G-valid | Finer alignment doesn't imply coarser | ✅ FAILED |
| 2 | `test_logical_always_nonzero` | L3 index always > 0 | L3 index can be 0 for some valid VAs | ✅ FAILED |
| 3 | `test_logical_stronger_bound` | L3 index ≤ 0xff | Spec guarantees ≤ 0x1ff, not stronger | ✅ FAILED |
| 4 | `test_logical_injective` | v2l3index is injective | Many VAs share the same L3 index | ✅ FAILED |

## Conclusion

The specification for `v2l3index` is **consistent** with respect to all tested adversarial queries:

1. **Preconditions are tight**: Invalid inputs (zero, unaligned, wrong L4 index, upper bits set) are correctly rejected.
2. **Postconditions are precise**: Mutated output values, bound violations, and off-by-one results are all rejected.
3. **No unintended entailments**: The spec does not allow reasoning about cross-granularity validity, universal non-zero indices, stronger bounds, or injectivity.

No spec weaknesses were detected.
