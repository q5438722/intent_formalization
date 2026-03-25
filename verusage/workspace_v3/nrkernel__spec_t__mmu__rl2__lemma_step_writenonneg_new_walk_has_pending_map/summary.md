# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_writenonneg_new_walk_has_pending_map/original.rs`
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

### φ2: align_zero_nonzero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning zero down to any positive alignment must yield zero

### φ3: align_result_not_aligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize must itself be aligned to the given boundary

### φ4: aligned_input_changes → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned address must be a fixed point of align_to_usize

### φ5: align_gap_exceeds_step → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The rounding-down distance must be strictly less than the alignment granularity

