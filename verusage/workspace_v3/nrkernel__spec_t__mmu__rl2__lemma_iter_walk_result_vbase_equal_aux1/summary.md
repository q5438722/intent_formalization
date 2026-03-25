# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal_aux1/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: align_is_noop_for_unaligned → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If alignment returns the original value even when the input is not aligned, the function fails its purpose.

### φ2: align_changes_already_aligned → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligning an already-aligned value changes it, the function incorrectly modifies valid inputs.

### φ3: align_exceeds_input → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Downward alignment must never produce a value larger than the original input.

### φ4: align_result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The result of alignment to b should always be a multiple of b; violating this breaks all page-table address invariants.

### φ5: align_not_idempotent → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Alignment should be idempotent; applying it twice must yield the same result as once, otherwise downstream walk computations diverge.

