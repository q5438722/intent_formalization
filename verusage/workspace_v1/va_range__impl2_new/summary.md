# Adversarial Test Results: `va_range__impl2_new.rs`

## Target

`VaRange4K::new` constructor and supporting spec functions for 4K-aligned virtual address ranges.

---

## Test Summary

| File | Tests | Failed (expected) | Passed (unexpected) |
|------|-------|--------------------|----------------------|
| `boundary_tests.rs` | 4 | 4 ✅ | 0 |
| `mutation_tests.rs` | 4 | 4 ✅ | 0 |
| `logical_tests.rs` | 4 | 3 ✅ | **1 ⚠️** |

---

## Boundary Tests (4/4 FAILED ✅)

All invalid inputs correctly rejected by the specification:

| Test | Input | Violation | Result |
|------|-------|-----------|--------|
| `test_boundary_non_aligned` | `va = 1` | Not 4K-aligned | FAILED ✅ |
| `test_boundary_user_space` | `va = 0x1000` | Bits 39-47 = 0 (not kernel space) | FAILED ✅ |
| `test_boundary_upper_bits_set` | `va = 0xFFFF_0080_0000_0000` | Bits 48-63 ≠ 0 | FAILED ✅ |
| `test_boundary_zero_address` | `va = 0` | Zero (all checks fail) | FAILED ✅ |

**Conclusion**: `spec_va_4k_valid` correctly rejects all tested invalid inputs.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

All incorrect behaviors correctly rejected:

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_zero_offset_differs` | Assert `add_range(va, 0) ≠ va` | FAILED ✅ |
| `test_mutation_one_page_equals_base` | Assert `add_range(va, 1) == va` | FAILED ✅ |
| `test_mutation_wrong_stride` | Assert stride is 8192 not 4096 | FAILED ✅ |
| `test_mutation_reversed_ordering` | Assert `add_range(va, 2) < add_range(va, 1)` | FAILED ✅ |

**Conclusion**: `spec_va_add_range` correctly pins down identity, offset change, stride, and ordering.

---

## Logical Tests (3/4 FAILED ✅, 1/4 PASSED ⚠️)

| Test | Property | Result |
|------|----------|--------|
| `test_logical_universal_validity` | All usize values are valid 4K addresses | FAILED ✅ |
| `test_logical_range_extension` | Range valid for 5 ⟹ valid for 6 | FAILED ✅ |
| `test_logical_universal_monotonicity` | `add_range` is universally monotone | FAILED ✅ |
| **`test_logical_lemma_unsoundness`** | **Derive `false` via lemma bug** | **PASSED ⚠️** |

---

## ⚠️ Specification Weakness Found

**`test_logical_lemma_unsoundness` PASSED — the `external_body` lemma is UNSOUND.**

### Root Cause

`va_range_lemma` has a typo in its ensures clause:

```
0 <= i < len && 0 <= i < len   ← BUG: checks i twice
```

Should be:

```
0 <= i < len && 0 <= j < len   ← FIX: j must also be in range
```

Because `j` is unconstrained, the lemma claims injectivity of `spec_va_add_range(va, ·)` for **all** `j`, not just `j ∈ [0, len)`.

### Exploit

When `j = 2^52`, `spec_va_add_range(va, j) = (va + 2^52 × 4096) as usize = (va + 2^64) mod 2^64 = va`, causing a modular wrap-around. The lemma then concludes:

```
(0 == 2^52) == (va == va)  →  false == true  →  false
```

From `false`, anything can be derived (`assert(false)` succeeds), making the entire proof system unsound for any code using this lemma.

### Impact

Any proof that depends on `va_range_lemma` is potentially unsound. The no-duplicates guarantee in `VaRange4K::wf()` and any range-based reasoning may be compromised.

---

## Overall Assessment

- **Boundary completeness**: ✅ Strong — invalid inputs are rejected
- **Behavioral correctness**: ✅ Strong — incorrect behaviors are rejected
- **Logical soundness**: ⚠️ **Weak** — `external_body` lemma allows deriving `false` due to unconstrained quantifier variable (`j` not bounded)
