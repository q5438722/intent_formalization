# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__result_sbin.rs`
**Date:** 2026-03-24T10:49:31Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

One false positive: the `size_of_sbin` opacity candidate is a trivial tautology proving nothing. Two true positives: `smallest_sbin_fitting_size` silently returns negative (invalid) sbin indices for negative inputs without any precondition guard, and `pow2` returning 1 for negative exponents is a non-standard design choice that could mask bugs if called with unexpected arguments, though it doesn't affect valid sbin index computations.

## True Positives (Spec Issues)

### smallest_sbin_negative_input
- **Confidence:** medium
- **Reasoning:** `smallest_sbin_fitting_size(-5) == -5` means negative slice counts produce negative sbin indices that violate `valid_sbin_idx`. While callers should validate inputs, the function silently returns invalid indices for negative inputs with no precondition guard, which could propagate into `size_of_sbin` with out-of-range arguments.

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). Within `size_of_sbin`, `group + 1` is always ≥ 1 for valid sbin indices 9–31 so it doesn't directly cause issues, but the definition silently masks errors if ever called with negative arguments in other contexts.

## All Candidates

### φ1: size_of_sbin_closed_spec_opaque
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `size_of_sbin` is `closed spec` — callers cannot reason about its definition, so monotonicity (larger bin index → larger size) cannot be established outside this module, potentially breaking bin-fitting correctness proofs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `size_of_sbin(i) <= size_of_sbin(j) || size_of_sbin(i) > size_of_sbin(j)` is a tautology (P ∨ ¬P) that holds for any expression. It demonstrates nothing about the closed spec opacity — the ensures would verify regardless of `size_of_sbin`'s visibility.

### φ2: sbin8_discontinuity
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The transition at `i=9` switches from identity (`i <= 8` branch returns `i`) to the bit-manipulation formula, which may produce a different value than 9 — a discontinuity in the sbin mapping could skip bin indices or assign incorrect bins

### φ3: smallest_sbin_negative_input
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** For negative inputs, `smallest_sbin_fitting_size` returns the input directly (since `-5 <= 8`), producing a negative sbin index that violates `valid_sbin_idx` — callers passing invalid slice counts get silently invalid indices
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `smallest_sbin_fitting_size(-5) == -5` means negative slice counts produce negative sbin indices that violate `valid_sbin_idx`. While callers should validate inputs, the function silently returns invalid indices for negative inputs with no precondition guard, which could propagate into `size_of_sbin` with out-of-range arguments.

### φ4: property_sbin_vacuous_for_large
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For very large slice counts, `smallest_sbin_fitting_size` may produce an index exceeding `SEGMENT_BIN_MAX=31`, making `property_sbin` fail — there is no BIN_HUGE equivalent to catch oversized inputs like the regular bin allocator has

### φ5: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so if `size_of_sbin` ever computes a negative `group + 1`, the size silently collapses to `(inner + 5) * 1` instead of producing an error
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). Within `size_of_sbin`, `group + 1` is always ≥ 1 for valid sbin indices 9–31 so it doesn't directly cause issues, but the definition silently masks errors if ever called with negative arguments in other contexts.

