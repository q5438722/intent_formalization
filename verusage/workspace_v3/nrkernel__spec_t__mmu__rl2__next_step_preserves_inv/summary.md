# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv/original.rs`
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
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The result of align_to_usize should always be aligned to b; failing this means the function is wrong

### φ3: aligned_contradicts_modulus → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If aligned returns true, the modulus must be zero; entailment here means the spec is self-contradictory

### φ4: align_gap_exceeds_block → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The difference a - align_to_usize(a,b) equals a%b which must be strictly less than b; a gap >= b means over-alignment

### φ5: align_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Aligning an already-aligned value should be a no-op; failure of idempotence would indicate a broken alignment function

