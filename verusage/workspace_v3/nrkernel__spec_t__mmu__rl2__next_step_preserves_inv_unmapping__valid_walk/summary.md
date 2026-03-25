# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_unmapping__valid_walk/original.rs`
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
- **Why flagged:** Aligning down should never produce a value greater than the original input

### φ2: align_not_idempotent → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning a value that is already a multiple of b should return the same value (idempotency)

### φ3: align_result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of aligning down to a multiple of b should itself be divisible by b

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by any nonzero number so aligned(0, size) should always be true

### φ5: aligned_disagrees_with_align → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a value is aligned to b then aligning it down should return itself; disagreement implies a spec inconsistency

