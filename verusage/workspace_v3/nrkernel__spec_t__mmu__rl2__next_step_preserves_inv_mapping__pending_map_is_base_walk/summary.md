# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_mapping__pending_map_is_base_walk/original.rs`
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
- **Why flagged:** Downward alignment must never produce a value greater than the original address

### φ2: align_zero_nonzero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning zero to any positive granularity must still yield zero

### φ3: aligned_not_fixpoint → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned address must be a fixpoint of align_to_usize; disagreement indicates inconsistent specs

### φ4: align_result_not_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The output of align_to_usize must itself satisfy the aligned predicate for the same granularity

### φ5: aligned_zero_divisor → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment to size zero is undefined; a nonzero address should never be considered 0-aligned

