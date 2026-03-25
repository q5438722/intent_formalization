# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__init_implies_tlb_inv/original.rs`
**Date:** 2026-03-24T22:24:25Z

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

Of the five candidate properties, only `aligned_zero_size_vacuous` is a true positive. The `aligned` function uses `spec(checked)` semantics, meaning that when `size == 0` the division-by-zero check fails and the result becomes unconstrained in the SMT solver, allowing any address to appear "aligned to 0." This is a genuine spec gap — a `requires size > 0` guard or an explicit `size == 0 ==> false` clause is needed. The remaining four properties (φ2–φ5) are false positives: they test standard modular arithmetic identities that the `addr % size == 0` definition handles correctly when the divisor is non-zero, and none are actually provable.

## True Positives

### aligned_zero_size_vacuous
- **Confidence:** high
- **Reasoning:** The function is `spec(checked)`, so `42 % 0` triggers a division-by-zero check failure, making the result unconstrained in the SMT solver. This allows the solver to satisfy `aligned(42, 0)` vacuously. The spec should either require `size > 0` or explicitly return `false` when `size == 0`.

## All Candidates

### φ1: aligned_zero_size_vacuous → `aligned`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If size==0 satisfies alignment, the spec fails to guard against division-by-zero, letting any address appear "aligned to 0"
- **Verdict:** TRUE_POSITIVE (high)

### φ2: aligned_odd_to_even → `aligned`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** 3 is not a multiple of 2; if provable, the modular-arithmetic check is fundamentally broken
- **Verdict:** FALSE_POSITIVE (high)

### φ3: aligned_implies_zero_addr → `aligned`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Many nonzero addresses (e.g. 4096, 8192) are page-aligned; if the spec collapses them all to zero, it is far too restrictive
- **Verdict:** FALSE_POSITIVE (high)

### φ4: aligned_sum_off_by_one → `aligned`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Adding 1 to a sum of two aligned values must break alignment for any size>1; provability would mean the spec ignores offsets
- **Verdict:** FALSE_POSITIVE (high)

### φ5: aligned_cross_size → `aligned`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Alignment to 2 does not imply alignment to 3 (e.g. addr==4); if provable, the spec conflates coprime alignment classes
- **Verdict:** FALSE_POSITIVE (high)

