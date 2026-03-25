# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_iter_walk_result_vbase_equal_aux2/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: result_not_multiple_of_alignment → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Aligning down should always produce a multiple of the alignment value

### φ2: result_exceeds_input → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning down must never produce a value larger than the original input

### φ3: not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment should be idempotent — aligning an already-aligned value must be a no-op

### φ4: nonaligned_unchanged → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-aligned input must be rounded down, so the result should differ from the input

### φ5: gap_ge_alignment → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The distance from the aligned value to the original must be strictly less than b (i.e. the remainder)

