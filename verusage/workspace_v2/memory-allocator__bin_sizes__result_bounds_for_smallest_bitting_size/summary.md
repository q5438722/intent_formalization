# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__result_bounds_for_smallest_bitting_size.rs`
**Date:** 2026-03-24T10:41:48Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives. The more significant finding is that `size_of_bin(BIN_HUGE)` is artificially capped at a finite value while `smallest_bin_fitting_size` routes all large sizes to BIN_HUGE, breaking the `property_bounds_for_smallest_bitting_size` invariant for sizes above 4194312 bytes. The `pow2` negative-exponent issue is a lower-severity design choice that doesn't affect valid bin indices but represents a mathematically incorrect definition that could mask bugs if called with unexpected arguments.

## True Positives (Spec Issues)

### size_of_bin_huge_is_not_infinite
- **Confidence:** high
- **Reasoning:** `smallest_bin_fitting_size` maps any size with wsize > 524288 to BIN_HUGE, but `size_of_bin(BIN_HUGE)` returns a finite cap of 4194312. For any size > 4194312, `property_bounds_for_smallest_bitting_size` requires `size_of_bin(BIN_HUGE) >= size`, which fails. This is a real spec gap where the bounds property breaks for large allocations.

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically wrong (2^(-n) should be fractional). Since `size_of_bin` uses `pow2(group + 1)` where `group = (bin_idx - 9) / 4`, group is non-negative for valid bin indices 9–72, so this doesn't directly affect correctness in practice — but the definition silently masks errors if ever called with negative arguments.

## All Candidates

### φ1: size_of_bin_not_monotone
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The transition from linear bins (1–8) to exponential bins (9+) could have a non-monotone gap — bin 8 gives `8*8=64` while bin 9 gives `8*5*pow2(1)=80`, but if the formula is wrong the invariant `size_of_bin(i) <= size_of_bin(i+1)` could break

### φ2: smallest_bin_negative_size_is_bin1
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Negative and zero sizes map to bin 1 — callers passing invalid negative sizes get silently allocated instead of being rejected

### φ3: size_of_bin_huge_is_not_infinite
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** BIN_HUGE's size is artificially capped at `8 * 524289 = 4194312` bytes rather than being unbounded — any allocation request above this value maps to BIN_HUGE but `size_of_bin(BIN_HUGE)` claims it fits, violating `size_of_bin(smallest_bin_fitting_size(size)) >= size` for sizes above 4194312
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `smallest_bin_fitting_size` maps any size with wsize > 524288 to BIN_HUGE, but `size_of_bin(BIN_HUGE)` returns a finite cap of 4194312. For any size > 4194312, `property_bounds_for_smallest_bitting_size` requires `size_of_bin(BIN_HUGE) >= size`, which fails. This is a real spec gap where the bounds property breaks for large allocations.

### φ4: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers accidentally passing negative group indices to `size_of_bin` get silently wrong bin sizes
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically wrong (2^(-n) should be fractional). Since `size_of_bin` uses `pow2(group + 1)` where `group = (bin_idx - 9) / 4`, group is non-negative for valid bin indices 9–72, so this doesn't directly affect correctness in practice — but the definition silently masks errors if ever called with negative arguments.

### φ5: bounds_property_fails_above_huge
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Tests that the boundary case `size = 8*524288+1 = 4194305` just barely fits in BIN_HUGE's artificial cap — but `size = 8*524289+1 = 4194313` would exceed `size_of_bin(BIN_HUGE)`, breaking the `property_bounds_for_smallest_bitting_size` invariant for large sizes

