# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_unmapping__pt_walk_valid_in_post_unchanged/original.rs`
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

### φ2: align_result_not_multiple → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The result of aligning to b should always be a multiple of b; failing this means the function is broken

### φ3: aligned_identity_disagree → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If addr is already aligned to size, align_to_usize should be the identity; disagreement indicates inconsistency

### φ4: not_aligned_to_one → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Every natural number is aligned to 1; if this verifies, the aligned spec is vacuously broken

### φ5: align_always_zero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For a >= b > 0, the aligned-down result should be at least b, not zero; verifying this signals a degenerate spec

