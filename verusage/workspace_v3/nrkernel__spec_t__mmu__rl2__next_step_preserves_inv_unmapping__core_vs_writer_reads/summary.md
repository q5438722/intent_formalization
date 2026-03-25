# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_unmapping__core_vs_writer_reads/original.rs`
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

### φ2: align_not_idempotent → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning an already-aligned value should be the identity; changing it would be unsound

### φ3: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce a b-aligned result; failing that breaks all downstream alignment invariants

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every nonzero number, so aligned(0, size) must be true for all size > 0

### φ5: aligned_disagrees_with_align → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If aligned says addr is aligned then align_to_usize must be the identity on it; disagreement means the two specs are inconsistent

