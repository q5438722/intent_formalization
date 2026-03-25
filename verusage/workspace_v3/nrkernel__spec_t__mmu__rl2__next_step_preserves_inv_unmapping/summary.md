# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_unmapping/original.rs`
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
- **Why flagged:** Aligning down should never produce a value exceeding the original input

### φ2: align_to_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 must be the identity since every integer is a multiple of 1

### φ3: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce an aligned result; failing this breaks its contract

### φ4: aligned_implies_changed → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned address must be a fixed point of align_to_usize; changing it would corrupt mappings

### φ5: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every positive integer so it must always be considered aligned

