# Adversarial Test Summary: `zero_leads_is_empty_page_entry`

## Target Specification
The spec proves that `spec_usize2page_entry(0).is_empty()` — converting a zero usize to a `PageEntry` yields an empty entry (all fields zero/false). The conversion functions extract page table entry fields via bitmask operations on x86-64-style page table entries.

## Test Execution Results

All **16 adversarial tests** correctly **FAILED verification**, indicating the specification properly rejects invalid properties.

### Boundary Tests (5/5 FAILED ✓)
| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_present_bit_set_is_empty` | `spec_usize2page_entry(1).is_empty()` | FAIL ✓ |
| `test_addr_bit_set_is_empty` | `spec_usize2page_entry(0x1000).is_empty()` | FAIL ✓ |
| `test_max_usize_is_empty` | `spec_usize2page_entry(MAX).is_empty()` | FAIL ✓ |
| `test_execute_disable_bit_is_empty` | `spec_usize2page_entry(1<<63).is_empty()` | FAIL ✓ |
| `test_ps_bit_set_is_empty` | `spec_usize2page_entry(0x80).is_empty()` | FAIL ✓ |

**Conclusion**: The spec correctly distinguishes zero from non-zero inputs; setting any permission bit or address bit causes `is_empty()` to be false.

### Behavioral Mutation Tests (6/6 FAILED ✓)
| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_present_extraction_mutated` | `usize2present(1) == false` | FAIL ✓ |
| `test_write_extraction_mutated` | `usize2write(2) == false` | FAIL ✓ |
| `test_user_extraction_mutated` | `usize2user(4) == false` | FAIL ✓ |
| `test_ps_extraction_mutated` | `usize2ps(0x80) == false` | FAIL ✓ |
| `test_pa_extraction_mutated` | `spec_usize2pa(0x2000) == 0` | FAIL ✓ |
| `test_execute_disable_extraction_mutated` | `usize2execute_disable(1<<63) == false` | FAIL ✓ |

**Conclusion**: Each bit-extraction function correctly maps its designated bit position; mutating any expected output is rejected.

### Logical Tests (5/5 FAILED ✓)
| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_mem_valid_for_arbitrary_value` | `MEM_valid(0x1)` (low bits → invalid) | FAIL ✓ |
| `test_pa_injectivity_false` | `spec_usize2pa(0x1001) != spec_usize2pa(0x1002)` (both mask to 0x1000) | FAIL ✓ |
| `test_pa_is_identity` | `spec_usize2pa(0x1FFF) == 0x1FFF` (masking strips low bits) | FAIL ✓ |
| `test_is_empty_implies_zero_input` | `is_empty(0x800) ==> 0x800 == 0` (converse of spec) | FAIL ✓ |
| `test_addr_not_page_aligned` | `spec_usize2pa(v) % 0x1000 != 0` (masking guarantees alignment) | FAIL ✓ |

**Conclusion**: The spec correctly handles logical edge cases:
- `MEM_valid` rejects addresses with bits outside the mask.
- The PA mapping is non-injective (many-to-one due to masking) and the spec correctly reflects this.
- **Notable finding**: `test_is_empty_implies_zero_input` shows that `is_empty()` holds for `v=0x800` (bit 11), yet `v != 0`. This means `is_empty` is NOT equivalent to `v == 0` — bits 3-6, 8-11, and 52-62 are "don't care" bits. The spec correctly captures this gap rather than incorrectly assuming bijectivity.

## Overall Assessment

**The specification is consistent** with respect to all 16 adversarial queries. It correctly:
1. Rejects invalid edge-case inputs as non-empty (boundary)
2. Rejects mutated field extraction results (behavioral)
3. Rejects unintended logical inferences including false injectivity and converse claims (logical)

No specification weaknesses were detected by these tests.
