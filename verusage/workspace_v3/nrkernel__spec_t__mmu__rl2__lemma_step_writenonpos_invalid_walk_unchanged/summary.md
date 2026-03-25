# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_writenonpos_invalid_walk_unchanged/original.rs`
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

### φ2: align_result_not_multiple → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The result of downward alignment must be a multiple of the alignment boundary

### φ3: aligned_idempotent_fails → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the input is already aligned, align_to_usize must be the identity—returning a different value is a spec inconsistency

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is trivially divisible by any positive number, so aligned(0, size) must be true

### φ5: align_noop_when_unaligned → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the input is not aligned, align_to_usize must adjust it downward—returning it unchanged means alignment is broken

