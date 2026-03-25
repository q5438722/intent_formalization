# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__indexing__lemma_index_from_base_and_addr/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_successor → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Alignment should not be preserved by incrementing the address by 1 for any size > 1

### φ2: aligned_implies_double_size → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to s should not imply alignment to 2*s (e.g., 2 is aligned to 2 but not to 4)

### φ3: aligned_pred → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The value s-1 should never be aligned to s when s > 1 (e.g., 1 is not aligned to 2)

### φ4: aligned_sum_off_by_one → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Sum of two aligned values plus 1 should not be aligned for size > 1, since alignment is closed under addition but not under +1

### φ5: aligned_product_distrib → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to different moduli should not distribute over addition into their product (e.g., aligned(2,2) and aligned(3,3) but 5 is not aligned to 6)

