# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal/original.rs`
**Date:** 2026-03-24T22:18:25Z

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidate properties target `align_to_usize`, which is a pure spec function (`open spec(checked) fn`), not an executable function, making them out-of-scope for exec-spec consistency analysis. Additionally, both properties assert mathematically false claims about the standard floor-alignment operation `sub(a, a % b)`: φ1 claims the result is not a multiple of `b` (it always is), and φ2 claims consecutive addresses within the same alignment block map to different bases (they always map to the same base). Neither property could verify and neither reveals a real spec issue.

## All Candidates

### φ1: align_always_zero → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If alignment collapses every value >= b to zero, the function destroys address information and is trivially wrong.

### φ2: align_exceeds_input → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Floor alignment must never produce a value strictly greater than the input; that would map addresses beyond their original range.

### φ3: result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The fundamental contract of alignment is that the result is a multiple of b; violating this breaks all page-boundary assumptions in the MMU.
- **Verdict:** FALSE_POSITIVE (high)

### φ4: not_idempotent → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment must be idempotent — re-aligning an already-aligned value should be a no-op; failure means vbase recomputation in Walk::result is unstable.

### φ5: same_block_different_align → `align_to_usize`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two addresses within the same alignment block must map to the same base; otherwise iter_walk could assign different vbases to addresses on the same page.
- **Verdict:** FALSE_POSITIVE (high)

