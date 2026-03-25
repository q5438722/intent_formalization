# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__rl2__next_preserves_inv/original.rs`
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

### φ2: align_to_one_changes_value → `align_to_usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Every integer is a multiple of 1, so aligning to 1 must be a no-op

### φ3: align_result_not_aligned → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The whole point of align_to_usize is to produce a b-aligned result; failing that breaks its contract

### φ4: zero_not_aligned → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is divisible by every positive integer, so aligned(0, n) must be true for all n > 0

### φ5: aligned_implies_align_noop_fails → `align_to_usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a is already aligned to b then align_to_usize(a,b) must equal a; the contrary would mean alignment is lossy

