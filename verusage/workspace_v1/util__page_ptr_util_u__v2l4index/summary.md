# Adversarial Test Results: `util__page_ptr_util_u__v2l4index`

## Target Specification

The `v2l4index` function extracts the L4 page table index (bits 39–47) from a virtual address.

- **Precondition**: `va_4k_valid(va) || va_2m_valid(va) || va_1g_valid(va)` — address must be page-aligned and have L4 index ≥ `KERNEL_MEM_END_L4INDEX` (1).
- **Postcondition**: `ret == spec_v2l4index(va)` and `1 <= ret <= 0x1FF`.

## Results Summary

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15** | **0** |

**All 15 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

| # | Test | Input | Violation | Result |
|---|---|---|---|---|
| 1 | `test_zero_address_valid` | `va = 0` | L4 index 0 < 1 | REJECTED ✅ |
| 2 | `test_unaligned_address_4k_valid` | `va = 1` | Not 4K-aligned (bit 0 set) | REJECTED ✅ |
| 3 | `test_high_bits_set_address_valid` | `va = 0xFFFF_0080_0000_0000` | Bits 48–63 nonzero | REJECTED ✅ |
| 4 | `test_l4index_zero_4k_aligned_valid` | `va = 0x1000` | 4K-aligned but L4 index = 0 | REJECTED ✅ |
| 5 | `test_max_usize_valid` | `va = 0xFFFF_FFFF_FFFF_FFFF` | Unaligned + high bits set | REJECTED ✅ |

The precondition correctly rejects all edge-case invalid inputs.

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✅

| # | Test | Mutation | Expected | Asserted | Result |
|---|---|---|---|---|---|
| 1 | `test_result_off_by_one` | +1 offset | 1 | 2 | REJECTED ✅ |
| 2 | `test_result_zero_for_valid_input` | Below bound | 1 | 0 | REJECTED ✅ |
| 3 | `test_wrong_result_max_index` | Wrong mask | 0x1FF | 0x100 | REJECTED ✅ |
| 4 | `test_result_exceeds_upper_bound` | Above bound | ≤ 0x1FF | > 0x1FF | REJECTED ✅ |
| 5 | `test_negated_correct_result` | Negation | 1 | ≠ 1 | REJECTED ✅ |

The postcondition correctly pins the output to exact bit-extraction semantics.

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✅

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_non_injectivity` | Different L4 indices forced equal | REJECTED ✅ |
| 2 | `test_strictly_greater_lower_bound` | `ret > 1` instead of `ret >= 1` | REJECTED ✅ |
| 3 | `test_4k_implies_2m` | 4K validity ⟹ 2M validity | REJECTED ✅ |
| 4 | `test_monotonicity` | `va₁ < va₂ ⟹ index₁ < index₂` | REJECTED ✅ |
| 5 | `test_validity_equivalence` | 4K-valid ⟺ 2M-valid | REJECTED ✅ |

The spec correctly rejects unintended logical inferences including false injectivity, false monotonicity, over-strong bounds, and false cross-predicate implications.

## Conclusion

The specification for `v2l4index` is **consistent** with respect to all 15 adversarial queries:

- **Boundary integrity**: Invalid inputs (zero, unaligned, high-bits, wrong L4 index) are all rejected by the precondition.
- **Behavioral precision**: The `spec_v2l4index` function is fully determined by bit-extraction, preventing any mutated output from being accepted.
- **Logical soundness**: The spec does not entail false monotonicity, injectivity, stronger bounds, or cross-predicate implications.

No specification weaknesses were detected.
