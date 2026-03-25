# Adversarial Test Summary: `va_4k_range_valid`

## Target
`source-projects/atmosphere/verified/util/util__page_ptr_util_u__va_4k_range_valid.rs`

## Specification Under Test
- `spec_va_4k_valid(va)`: VA is 4K-page-aligned (bits 0–11 = 0), within 48-bit space (bits 48–63 = 0), and L4 index ≥ 1.
- `spec_va_add_range(va, i)`: Computes `(va + i * 4096) as usize`.
- `spec_va_4k_range_valid(va, len)`: All pages `va + i*4096` for `i ∈ [0, len)` are 4K-valid.
- `va_4k_range_valid(va, len)`: Executable version; **requires** `va_4k_valid(va)`.

---

## Results: ALL 15 TESTS FAILED (as expected)

### Boundary Tests (5/5 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_boundary_unaligned_va` | `spec_va_4k_valid(1)` — unaligned addr | **FAIL** ✅ |
| `test_boundary_zero_va` | `spec_va_4k_valid(0)` — L4 index = 0 | **FAIL** ✅ |
| `test_boundary_upper_bits_set` | `spec_va_4k_valid(0x1_0000_0000_0000)` — bit 48 set | **FAIL** ✅ |
| `test_boundary_invalid_base_range` | `spec_va_4k_range_valid(0, 1)` — invalid base | **FAIL** ✅ |
| `test_boundary_low_bits_all_set` | `spec_va_4k_valid(0xFFF)` — all low bits set | **FAIL** ✅ |

**Conclusion**: The spec correctly rejects all invalid inputs — unaligned addresses, sub-kernel-space addresses, and addresses with upper bits set.

### Behavioral Mutation Tests (5/5 failed ✅)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| `test_mutation_empty_range_is_invalid` | Assert len=0 range is NOT valid | **FAIL** ✅ |
| `test_mutation_add_zero_changes_address` | Assert adding 0 pages changes addr | **FAIL** ✅ |
| `test_mutation_wrong_step_size` | Assert step size is 1 instead of 4096 | **FAIL** ✅ |
| `test_mutation_add_one_equals_base` | Assert va+4096 == va | **FAIL** ✅ |
| `test_mutation_two_pages_equals_one` | Assert va+2*4096 == va+4096 | **FAIL** ✅ |

**Conclusion**: The spec correctly rejects all mutated behaviors — wrong step sizes, identity violations, and arithmetic errors.

### Logical Tests (5/5 failed ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_all_valid_vas_equal` | All valid VAs are the same | **FAIL** ✅ |
| `test_logical_add_preserves_validity` | Adding pages always preserves validity | **FAIL** ✅ |
| `test_logical_range_extensible` | Range valid for n ⟹ valid for n+1 | **FAIL** ✅ |
| `test_logical_global_injectivity` | va_add_range is globally injective | **FAIL** ✅ |
| `test_logical_stronger_alignment` | 4K-valid ⟹ 2MB-aligned | **FAIL** ✅ |

**Conclusion**: The spec does not entail any of the unintended logical properties tested — no false uniqueness, no false closure under extension, no false injectivity, and no false stronger alignment.

---

## Overall Assessment

The specification for `va_4k_range_valid` is **consistent** with respect to all 15 adversarial queries:
- **Boundary**: Invalid inputs are properly rejected (5/5).
- **Behavioral**: Mutated outputs/relations are properly rejected (5/5).
- **Logical**: Unintended properties are not entailed (5/5).

No spec weakness was detected by this test suite.
