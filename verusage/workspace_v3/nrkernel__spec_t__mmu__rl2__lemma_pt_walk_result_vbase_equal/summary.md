# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_pt_walk_result_vbase_equal/original.rs`
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
- **Why flagged:** Alignment (round-down) must never produce a value larger than the original address

### φ2: aligned_input_changes → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Aligning an already-aligned value must be a no-op; changing it would corrupt valid base addresses

### φ3: result_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole point of alignment is that the result is divisible by the alignment size; violating this breaks page-boundary invariants

### φ4: excessive_rounding → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Rounding down by b or more means skipping an entire alignment block, losing a valid mapping region

### φ5: not_idempotent → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment must be idempotent; a double-align changing the result would mean walk result vbase addresses are unstable

