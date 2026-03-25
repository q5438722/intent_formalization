# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__step_Writeback_preserves_inv_unmapping__inflight_walks/original.rs`
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

### φ2: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of align-down must itself be a multiple of the alignment; if not, the function is broken

### φ3: aligned_input_changed → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the input is already aligned, align_to_usize must be the identity; changing it would violate consistency with aligned

### φ4: align_unit_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment to 1 must be the identity since every integer is 1-aligned; failure here indicates a fundamental flaw

### φ5: aligned_predecessor_also_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr) implied aligned(addr-1) for size>1, the predicate would be vacuously true for all addresses

