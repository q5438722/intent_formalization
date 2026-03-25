# Adversarial Test Summary: `v2l1index` Specification

**Target**: `util__page_ptr_util_u__v2l1index.rs`
**Function under test**: `v2l1index(va) → L1Index` — extracts 9-bit L1 page table index (bits [20:12]) from a virtual address.
**Specification**:
- **Requires**: `va_4k_valid(va) || va_2m_valid(va) || va_1g_valid(va)`
- **Ensures**: `ret == spec_v2l1index(va)` and `ret <= 0x1ff`

---

## Results: 18/18 tests FAILED verification ✅

All adversarial tests were correctly rejected by Verus, indicating the specification is **consistent** with respect to the tested properties.

---

### Boundary Tests (6/6 FAILED as expected)

| Test | Input | Failure Mode | Result |
|------|-------|-------------|--------|
| B1 | `va = 0` | L4 index = 0 (below kernel threshold) | ❌ precondition rejected |
| B2 | `va = 1` | Not aligned to any page size | ❌ precondition rejected |
| B3 | `va = 0x1000` | 4K aligned but L4 index = 0 | ❌ precondition rejected |
| B4 | `va = 0xFFFF_FFFF_FFFF_FFFF` | Upper bits set, unaligned | ❌ precondition rejected |
| B5 | `va = 0x0000_0080_0000_0001` | Kernel space but not 4K aligned | ❌ precondition rejected |
| B6 | `va = 0xFFFF_0080_0000_0000` | Upper 16 bits set (outside 48-bit range) | ❌ precondition rejected |

**Conclusion**: The precondition correctly rejects all invalid inputs — addresses must be properly aligned AND in kernel space (L4 index ≥ 1) AND within the 48-bit address range.

---

### Behavioral Mutation Tests (6/6 FAILED as expected)

| Test | Mutation | Failure Mode | Result |
|------|----------|-------------|--------|
| M1 | Claim L1=0 for address with L1=1 | Wrong concrete value | ❌ bitvector rejected |
| M2 | Claim result > 0x1ff | Violates upper bound postcondition | ❌ bitvector rejected |
| M3 | Claim L1=2 for address with L1=1 | Off-by-one error | ❌ bitvector rejected |
| M4 | Claim result ≠ spec definition | Negated correctness | ❌ bitvector rejected |
| M5 | Claim L1=0x1ff for 2M-aligned address (actual=0) | Wrong value for aligned address | ❌ bitvector rejected |
| M6 | Claim L1=0x200 (out of 9-bit range) | Impossible value | ❌ bitvector rejected |

**Conclusion**: The spec correctly ties the output to the bitwise computation `(va >> 12) & 0x1ff` and enforces the upper bound `<= 0x1ff`.

---

### Logical Tests (6/6 FAILED as expected)

| Test | Unentailed Property | Why It's Wrong | Result |
|------|-------------------|----------------|--------|
| L1 | Injectivity of L1 index | Different addresses can share L1 bits | ❌ bitvector rejected |
| L2 | Stronger bound (< 256) | L1 uses 9 bits (up to 511) | ❌ bitvector rejected |
| L3 | 4K-valid ⟹ 2M-valid | 4K addresses need not be 2M aligned | ❌ bitvector rejected |
| L4 | L1 index always even | Odd indices are valid | ❌ bitvector rejected |
| L5 | L1 index determines address (bijection) | Many-to-one mapping | ❌ bitvector rejected |
| L6 | L1 index always 0 | Non-zero indices exist for 4K addresses | ❌ bitvector rejected |

**Conclusion**: The spec does not entail any of the tested unintended properties. It correctly models L1 index extraction as a non-injective, non-surjective 9-bit field extraction.

---

## Overall Assessment

The `v2l1index` specification is **well-formed and consistent**:
- **Preconditions** are strong enough to reject all invalid inputs (unaligned, out-of-range, non-kernel).
- **Postconditions** precisely characterize the output (exact bitwise formula + upper bound).
- **No unintended entailments** were found — the spec does not over-promise on injectivity, bounds, or cross-function implications.
