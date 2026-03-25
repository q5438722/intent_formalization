# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__lemma_bits_align_to_usize/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: result_always_zero → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment collapsing every address to zero would destroy all address information

### φ2: aligned_input_changes → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Already-aligned addresses must be fixed points; changing them violates idempotency

### φ3: result_exceeds_input → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Downward alignment must never produce a value larger than the original address

### φ4: alignment_not_multiple → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The fundamental invariant of alignment is that the result is a multiple of the alignment size

### φ5: off_by_one_subtraction → `align_to_usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Subtracting the full alignment size instead of just the remainder would be an off-by-one bug

