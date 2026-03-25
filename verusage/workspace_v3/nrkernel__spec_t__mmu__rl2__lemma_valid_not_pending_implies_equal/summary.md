# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_valid_not_pending_implies_equal/original.rs`
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
- **Why flagged:** Aligning down should never produce a value larger than the original input

### φ2: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize must itself be aligned to the given size

### φ3: aligned_value_changes → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An already-aligned value must be a fixed point of align_to_usize

### φ4: consecutive_both_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Two consecutive addresses cannot both be aligned to a stride of 2 or more

### φ5: aligned_sum_nonaligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Adding a sub-stride positive offset to an aligned address must break alignment

