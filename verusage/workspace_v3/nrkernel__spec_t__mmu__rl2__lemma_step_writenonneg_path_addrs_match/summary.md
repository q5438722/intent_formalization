# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_writenonneg_path_addrs_match/original.rs`
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

### φ2: align_not_identity_when_aligned → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the input is already aligned, align_to_usize should return it unchanged

### φ3: align_result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of down-alignment must itself be a multiple of the alignment value

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is trivially divisible by any positive number so aligned(0, size) must be true

### φ5: align_and_aligned_disagree → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The output of align_to_usize must satisfy the aligned predicate for the same divisor

