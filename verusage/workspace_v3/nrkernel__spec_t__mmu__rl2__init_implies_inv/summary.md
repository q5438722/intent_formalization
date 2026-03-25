# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__init_implies_inv/original.rs`
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
- **Why flagged:** Aligning down should never produce a value strictly greater than the original input

### φ2: align_identity_broken → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to a 1-byte boundary should be the identity operation for any input

### φ3: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every positive number and should always be considered aligned

### φ4: aligned_not_fixpoint → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If an address is already aligned, align_to_usize should return it unchanged

### φ5: align_result_not_aligned → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize must itself be aligned to the given boundary

