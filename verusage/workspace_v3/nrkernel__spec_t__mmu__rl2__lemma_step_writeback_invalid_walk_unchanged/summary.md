# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_writeback_invalid_walk_unchanged/original.rs`
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
- **Why flagged:** Alignment rounding down should never produce a value exceeding the original input

### φ2: align_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce a multiple of b; failing this means alignment is broken

### φ3: align_changes_already_aligned → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An already-aligned value should be a fixed point; changing it means the function over-rounds

### φ4: align_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment must be idempotent; applying it twice should yield the same result as once

### φ5: align_distance_exceeds_modulus → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The gap between the input and its aligned value must be strictly less than b; a gap >= b means an entire alignment block was skipped

