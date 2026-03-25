# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_finish_iter_walk_invalid_after_nonpos_write/original.rs`
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
- **Why flagged:** Floor-alignment must never exceed the original value; if it does, the subtraction logic is inverted

### φ2: align_one_not_identity → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning to 1 should be the identity (every integer is 1-aligned); failure means the modular arithmetic is wrong

### φ3: align_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment must be idempotent; an already-aligned value re-aligned should not change

### φ4: align_remainder_ge_modulus → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The gap between the input and its alignment must be strictly less than b; otherwise a closer aligned address was skipped

### φ5: align_multiple_changes_value → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An input already divisible by b is already aligned; the function must return it unchanged

