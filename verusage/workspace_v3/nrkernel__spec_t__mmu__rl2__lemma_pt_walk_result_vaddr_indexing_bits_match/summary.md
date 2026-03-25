# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_pt_walk_result_vaddr_indexing_bits_match/original.rs`
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
- **Why flagged:** Alignment-down should never produce a value larger than the original input

### φ2: unaligned_input_unchanged → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If an unaligned value is returned unchanged, the function fails its core purpose of aligning down

### φ3: result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize must always be a multiple of b; if not, the alignment is broken

### φ4: different_granularity_equal → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Coarser and finer alignment granularities collapsing to the same value would indicate the spec conflates page sizes

### φ5: gap_exceeds_alignment → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The rounding-down gap must always be strictly less than b; a gap >= b means the result skipped past a valid aligned address

