# Adversarial Proof Test Summary

**Target**: `page_map_set_kernel_entry_range` (pagetable/pagemap_util_t)
**Date**: 2026-03-21

## Overview

14 adversarial tests across 3 categories — all **FAILED verification** as expected, meaning the specification correctly rejects every invalid property tested.

---

## (1) Boundary Tests — 4/4 FAILED ✅

| Test | Property Asserted (should be false) | Result |
|---|---|---|
| `test_mem_valid_accepts_low_bit_address` | `MEM_valid(0x1)` — bit 0 is outside MEM_MASK | FAIL ✅ |
| `test_zero_entry_has_present` | `spec_usize2page_entry(0).perm.present` — zero value has no present bit | FAIL ✅ |
| `test_usize2pa_preserves_low_bits` | `spec_usize2pa(1) != 0` — bit 0 is masked off | FAIL ✅ |
| `test_mem_valid_accepts_low12_bits` | `MEM_valid(0x0FFF)` — all low 12 bits are outside MEM_MASK | FAIL ✅ |

**Conclusion**: The spec correctly enforces address validity boundaries. Invalid addresses (with bits outside MEM_MASK) are rejected.

---

## (2) Behavioral Mutation Tests — 6/6 FAILED ✅

| Test | Property Asserted (should be false) | Result |
|---|---|---|
| `test_present_bit_negated` | `!entry(1).perm.present` — value 1 has bit 0 set → present=true | FAIL ✅ |
| `test_write_bit_negated` | `!entry(2).perm.write` — value 2 has bit 1 set → write=true | FAIL ✅ |
| `test_user_bit_negated` | `!entry(4).perm.user` — value 4 has bit 2 set → user=true | FAIL ✅ |
| `test_address_mutated_to_zero` | `entry(0x1000).addr == 0` — 0x1000 is in MEM_MASK → addr=0x1000 | FAIL ✅ |
| `test_kernel_entry0_wrongly_preserved` | entry[0] unchanged after set_kernel_entry_range | FAIL ✅ |
| `test_kernel_entry1_wrongly_changed` | entry[1] changed after set_kernel_entry_range | FAIL ✅ |

**Conclusion**: The spec correctly encodes permission bit extraction and the kernel entry range update semantics. Mutated outputs are all rejected.

---

## (3) Logical Tests — 4/4 FAILED ✅

| Test | Property Asserted (should be false) | Result |
|---|---|---|
| `test_usize2pa_is_identity` | `spec_usize2pa(0xFFFF) == 0xFFFF` — masking drops low bits → 0xF000 | FAIL ✅ |
| `test_mem_valid_implies_zero` | `MEM_valid(0x1000) ⟹ 0x1000 == 0` — valid nonzero addresses exist | FAIL ✅ |
| `test_usize2pa_injective_on_low_bits` | `spec_usize2pa(0x1001) != spec_usize2pa(0x1002)` — both map to 0x1000 | FAIL ✅ |
| `test_different_entries_equal` | `entry(0x1000) =~= entry(0x2000)` — different PAs → different entries | FAIL ✅ |

**Conclusion**: The spec does not entail overly strong properties (identity, injectivity on low bits, uniqueness collapse). The masking semantics are tight.

---

## Final Assessment

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 4 | 4 | 0 |
| Behavioral Mutation | 6 | 6 | 0 |
| Logical | 4 | 4 | 0 |
| **Total** | **14** | **14** | **0** |

**Result**: The specification for `page_map_set_kernel_entry_range` is **consistent** with respect to all 14 adversarial queries. It correctly:
- Rejects invalid inputs (addresses with bits outside MEM_MASK)
- Rejects incorrect behaviors (mutated permission bits and wrong addresses)
- Rejects unintended logical inferences (identity, over-restrictiveness, false injectivity, false equality)

No specification weaknesses were detected.
