# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__va_4k_range_valid/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: range_always_valid → `va_4k_range_valid`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the range check is trivially always true regardless of length, making the validation meaningless

### φ2: empty_range_invalid → `va_4k_range_valid`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, empty ranges are rejected even though the universal quantifier should be vacuously true for len=0

### φ3: range_not_monotonic → `va_4k_range_valid`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, a valid range of size n implies a smaller sub-range is invalid, violating monotonicity of the forall

### φ4: no_nonempty_valid_range → `va_4k_range_valid`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, no non-empty range is ever valid even when the base address is valid, making page mapping impossible

### φ5: next_page_always_invalid → `va_4k_range_valid`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, no valid address ever has a valid successor page, meaning contiguous multi-page ranges can never exist

