# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_unmapping__notin_nonpos/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Aligning down should never produce a value strictly greater than the original input

### φ2: align_result_not_aligned → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize should always be a multiple of b; failing this means alignment is broken

### φ3: aligned_identity_broken → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a is already aligned to b, align_to_usize should be the identity; disagreement indicates inconsistency

### φ4: align_gap_exceeds_modulus → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The distance from a to its aligned-down value must be less than b; a gap >= b means a full unit was lost

### φ5: zero_never_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every nonzero value, so it must always be considered aligned; denying this is unsound

