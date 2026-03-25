# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_unmapping__inflight_walks/original.rs`
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
- **Why flagged:** Aligning down should never produce a value exceeding the original input

### φ2: align_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 should be the identity function since every integer is 1-aligned

### φ3: aligned_not_fixpoint → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned value must be a fixpoint of align_to_usize

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every positive number so it must always be aligned

### φ5: align_result_unaligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize must itself be aligned to b by construction

