# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_Writeback_post_valid_walk_unchanged/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: result_exceeds_input → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Floor-alignment must never produce a value larger than the input

### φ2: aligned_value_changes → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Aligning an already-aligned value should be a no-op; changing it indicates a broken spec

### φ3: not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Floor-alignment must be idempotent; a second application should not change the result

### φ4: excess_rounding → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The rounding-down distance must be strictly less than the alignment unit; rounding by b or more skips an entire block

### φ5: align_zero_nonzero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is a multiple of every positive integer, so aligning zero must yield zero

