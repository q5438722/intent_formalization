# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__extra__lemma_aligned_iff_eq_mul_div/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_implies_double_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment to size does not imply alignment to double the size (e.g., 4 is 4-aligned but not 8-aligned)

### φ2: odd_aligned_to_even → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 1 is not 2-aligned (1 % 2 == 1), so the spec must not entail this

### φ3: alignment_symmetric → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment is not symmetric (e.g., aligned(4,2) holds but aligned(2,4) does not)

### φ4: successor_preserves_alignment → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Adding 1 to an aligned address breaks alignment for any size > 1

### φ5: aligned_to_sum_of_divisors → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to b and c individually does not imply alignment to b+c (e.g., 6 is aligned to 2 and 3 but not to 5)

