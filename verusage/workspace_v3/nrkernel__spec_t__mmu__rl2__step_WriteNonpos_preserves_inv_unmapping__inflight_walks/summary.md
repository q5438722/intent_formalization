# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__step_WriteNonpos_preserves_inv_unmapping__inflight_walks/original.rs`
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

### φ2: align_result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of floor-alignment must always be a multiple of the alignment value

### φ3: aligned_identity_broken → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Aligning a value that is already aligned should be the identity operation

### φ4: aligned_always_true → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Not every nonzero address is aligned to every size; alignment should be selective

### φ5: align_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Floor-alignment must be idempotent — applying it twice should equal applying it once

