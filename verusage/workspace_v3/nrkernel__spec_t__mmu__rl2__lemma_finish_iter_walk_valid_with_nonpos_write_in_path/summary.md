# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_finish_iter_walk_valid_with_nonpos_write_in_path/original.rs`
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
- **Why flagged:** Aligning down should never produce a value strictly greater than the original input

### φ2: zero_aligns_to_nonzero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is a multiple of every positive integer so aligning zero must yield zero

### φ3: result_not_multiple_of_alignment → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of alignment is to produce a multiple of b; failing that means the spec is wrong

### φ4: not_idempotent → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Aligning a value that is already a multiple of b should be a no-op; changing it indicates a broken spec

### φ5: offset_within_block_differs → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Any address within the same aligned block should round down to the block base; disagreement reveals a spec error

