# Adversarial Proof Test Summary

**Target**: `pagetable__pagetable_impl_base__impl0__create_entry_l3.rs`
**Function**: `PageTable::create_entry_l3` — Creates a new L3 page table entry, inserting a fresh L2 page map.

---

## Results: All 15 tests FAILED verification ✅

This confirms the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 failed as expected)

| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `page_ptr_valid(1)` | Unaligned pointer rejected | FAIL ✅ |
| 2 | `page_ptr_valid(ptr)` where `ptr/0x1000 == NUM_PAGES` | Upper bound rejected | FAIL ✅ |
| 3 | `PageEntry{present:true}.is_empty()` | Present entry not empty | FAIL ✅ |
| 4 | `page_ptr_1g_valid(0x200000)` | 2M-aligned ≠ 1G-aligned | FAIL ✅ |
| 5 | `MEM_valid(1)` | Low bits violate mask | FAIL ✅ |

### Behavioral Mutation Tests (5/5 failed as expected)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `is_empty()` with addr=0x1000 | Nonzero addr breaks is_empty | FAIL ✅ |
| 2 | `is_empty()` with user=true | User bit breaks is_empty | FAIL ✅ |
| 3 | page_closure insert wrong pointer | Closure growth target mutated | FAIL ✅ |
| 4 | L3 mapping addr ≠ page_map_ptr | Resolved address mutated | FAIL ✅ |
| 5 | VA in old mapping_4k absent in new | Mapping preservation violated | FAIL ✅ |

### Logical Tests (5/5 failed as expected)

| # | Test | Unguaranteed Property | Result |
|---|------|----------------------|--------|
| 1 | `page_ptr_valid ⟹ page_ptr_2m_valid` | Stronger alignment not implied | FAIL ✅ |
| 2 | `MEM_valid ⟹ page_ptr_valid` | MEM validity ≠ page validity | FAIL ✅ |
| 3 | `page_ptr_valid(ptr) ⟹ ptr > 0` | Zero is a valid page ptr | FAIL ✅ |
| 4 | `spec_index2va` injectivity | Uses `&` (AND) — not injective | FAIL ✅ |
| 5 | `page_ptr_valid(ptr) ⟹ ptr == 0` | Valid ptr not unique | FAIL ✅ |

---

## Notable Findings

1. **`spec_index2va` uses bitwise AND (`&`) instead of OR (`|`)** — This causes all distinct index tuples like `(1,0,0,0)` and `(2,0,0,0)` to map to the same VA (0). This is likely a spec extraction artifact or bug, since the function should compose indices via OR to form distinct virtual addresses.

2. **`page_ptr_valid(0)` is true** — The specification admits pointer value 0 as a valid page pointer, which may conflict with null-pointer conventions.

3. **All postcondition constraints of `create_entry_l3` are correctly enforced** — The specification properly preserves mapping_4k/2m/1g, constrains the new L3 entry's address, and correctly models page_closure growth.
