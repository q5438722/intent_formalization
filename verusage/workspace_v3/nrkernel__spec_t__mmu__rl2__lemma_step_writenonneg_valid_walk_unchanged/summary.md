# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_writenonneg_valid_walk_unchanged/original.rs`
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

### φ2: align_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 should be the identity since every integer is 1-aligned

### φ3: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The output of align_to_usize should always be aligned to the given size

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every positive integer and should always be considered aligned

### φ5: align_gap_too_large → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The distance from input to its aligned-down value must be strictly less than the alignment size

