# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_pt_walk_with_indexing_bits_match/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_exceeds_input → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning down should never produce a value larger than the original input

### φ2: result_not_multiple → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The core contract of alignment is that the result is a multiple of b; violating this means alignment is broken

### φ3: not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Aligning an already-aligned value again must be a no-op; failure means the function is not a true projection

### φ4: already_aligned_changes → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the input is already a multiple of b the result must equal the input; any deviation is a bug

### φ5: gap_too_large → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The distance between the input and its aligned value must be strictly less than b; a gap >= b means a closer aligned address was skipped

