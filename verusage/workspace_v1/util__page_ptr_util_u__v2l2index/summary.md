# Adversarial Test Summary: `util__page_ptr_util_u__v2l2index`

## Target Specification

The function `v2l2index` extracts the L2 page table index (bits 29:21) from a virtual address. It requires the VA to satisfy at least one validity predicate (`va_4k_valid`, `va_2m_valid`, or `va_1g_valid`), which enforce alignment and a minimum L4 index (≥ 1). The postcondition guarantees `ret == (va >> 21) & 0x1ff` and `ret <= 0x1ff`.

## Results Overview

| Category               | Tests | All Failed (Expected) |
|------------------------|-------|-----------------------|
| Boundary Tests         | 6     | ✅ Yes                |
| Behavioral Mutation    | 5     | ✅ Yes                |
| Logical Tests          | 6     | ✅ Yes                |
| **Total**              | **17**| ✅ **All rejected**   |

## Boundary Tests (`boundary_tests.rs`) — 6/6 FAILED ✅

All invalid inputs were correctly rejected by the validity predicates.

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_zero_va_is_4k_valid` | VA=0 satisfies 4K validity (L4 index=0 < 1) | FAILED ✅ |
| `test_unaligned_va_is_4k_valid` | Non-4K-aligned VA (bit 0 set) satisfies 4K validity | FAILED ✅ |
| `test_high_bits_va_is_4k_valid` | VA with bits 48-63 set satisfies 4K validity | FAILED ✅ |
| `test_low_aligned_va_is_4k_valid` | 4K-aligned VA with L4 index=0 satisfies 4K validity | FAILED ✅ |
| `test_non_2m_aligned_is_2m_valid` | 4K-aligned (not 2M) VA satisfies 2M validity | FAILED ✅ |
| `test_non_1g_aligned_is_1g_valid` | 2M-aligned (not 1G) VA satisfies 1G validity | FAILED ✅ |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✅

All mutated output relationships were correctly rejected.

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_wrong_l2_index_value` | L2 index of VA with bits 29:21=1 equals 0 | FAILED ✅ |
| `test_off_by_one_l2_index` | L2 index of max-L2 VA (0x1ff) equals 0x1fe | FAILED ✅ |
| `test_wrong_shift_amount` | `(va >> 12) & 0x1ff` equals `(va >> 21) & 0x1ff` | FAILED ✅ |
| `test_wrong_mask_width` | `(va >> 21) & 0xff` equals `(va >> 21) & 0x1ff` | FAILED ✅ |
| `test_l2_index_exceeds_upper_bound` | L2 index > 0x1ff for any VA | FAILED ✅ |

## Logical Tests (`logical_tests.rs`) — 6/6 FAILED ✅

All unintended semantic properties were correctly rejected.

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_l2_index_always_zero` | ∀ valid VA: L2 index = 0 | FAILED ✅ |
| `test_l2_index_injective` | Different VAs → different L2 indices | FAILED ✅ |
| `test_stronger_upper_bound` | ∀ valid VA: L2 index < 256 (tighter than ≤ 511) | FAILED ✅ |
| `test_4k_valid_implies_2m_valid` | 4K validity ⟹ 2M validity | FAILED ✅ |
| `test_l2_index_always_nonzero` | ∀ valid VA: L2 index > 0 | FAILED ✅ |
| `test_2m_valid_implies_1g_valid` | 2M validity ⟹ 1G validity | FAILED ✅ |

## Conclusion

The specification for `v2l2index` is **consistent** with respect to all 17 adversarial queries. The validity predicates correctly enforce alignment and L4 index constraints, the output relation precisely captures the bit-extraction logic, and the spec does not entail unintended properties (injectivity, stronger bounds, cross-predicate implications). No spec weaknesses were found.
