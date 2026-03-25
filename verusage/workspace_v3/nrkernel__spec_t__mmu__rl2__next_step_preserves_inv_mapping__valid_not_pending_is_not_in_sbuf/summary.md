# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_mapping__valid_not_pending_is_not_in_sbuf/original.rs`
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

### φ2: align_zero_nonzero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning zero to any positive alignment must yield zero since 0 mod b == 0

### φ3: aligned_identity_broken → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned address must be a fixed point of align_to_usize; otherwise the spec is inconsistent with aligned

### φ4: one_alignment_false → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Every address is trivially 1-aligned since addr % 1 == 0 for all addr

### φ5: align_result_unaligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole purpose of align_to_usize is to produce an aligned result; if the output is unaligned the spec is wrong

