# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__mod_mult_zero_implies_mod_zero/original.rs`
**Date:** 2026-03-24T22:05:32Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property claims that offsetting an aligned address by `size - 1` preserves alignment, but this is trivially false by counterexample (e.g., 0 aligned to 4, offset to 3, which is not aligned to 4). Since Verus would reject the empty proof body, the property does not actually verify and therefore reveals no spec issue. It is a false positive both because it cannot be proved and because it targets a spec function rather than an executable function.

## All Candidates

### φ1: zero_size_alignment_trivially_true → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** aligned(addr, 0) should not hold because alignment to zero is undefined, but nat%0 may be 0 in SMT making it vacuously true

### φ2: shift_by_one_preserves_alignment → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Adding 1 to an aligned address should break alignment for any size > 1, so this entailment would indicate the spec is vacuous or wrong

### φ3: double_size_always_preserves_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to size does not imply alignment to 2*size (e.g. 4 is aligned to 4 but not to 8), so this holding would mean the spec is too weak

### φ4: nonmultiple_is_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** size+1 is never a multiple of size when size>1, so this being provable would mean aligned accepts non-multiples

### φ5: offset_by_size_minus_one_still_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** addr+size-1 is one less than the next aligned address, so it should NOT be aligned; provability would indicate the spec conflates adjacent boundaries
- **Verdict:** FALSE_POSITIVE (high)

