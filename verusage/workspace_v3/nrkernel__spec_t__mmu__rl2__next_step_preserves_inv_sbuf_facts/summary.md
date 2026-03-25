# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_step_preserves_inv_sbuf_facts/original.rs`
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
- **Why flagged:** Floor-alignment should never produce a value greater than the original input.

### φ2: align_result_not_divisible → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The result of floor-alignment must always be divisible by the alignment boundary.

### φ3: aligned_identity_broken → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If an address is already aligned, align_to_usize should return it unchanged; inconsistency between aligned and align_to_usize would be a spec bug.

### φ4: align_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Floor-alignment must be idempotent; applying it twice should yield the same result as once.

### φ5: align_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Every integer is 1-aligned, so aligning to 1 must always return the input unchanged.

