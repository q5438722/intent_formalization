# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_step_WriteNonpos_post_valid_pt_walk_no_wraddr_in_path/original.rs`
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
- **Why flagged:** Aligning down should never produce a value larger than the original input

### φ2: align_result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of aligning down must itself be a multiple of b; otherwise the alignment is broken

### φ3: aligned_not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An already-aligned address should be unchanged by align_to_usize; violating this means the two specs are inconsistent

### φ4: align_always_zero → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** align_to_usize should not collapse every value to zero; for a >= b it must return a positive aligned address

### φ5: align_unit_changes_value → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Every integer is 1-aligned so align_to_usize(a,1) must equal a; failure means aligned and align_to_usize disagree on trivial case

