# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l2_impl__lemma_aligned_addr_mask_facts/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: zero_size_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Size-zero alignment is meaningless; if entailed, the spec has a division-by-zero gap allowing vacuous alignment

### φ2: aligned_symmetry → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment is not symmetric; PAGE_SIZE is not a multiple of every page-aligned address (e.g. 4096 % 8192 != 0)

### φ3: nonzero_smaller_than_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A positive value strictly less than its alignment size cannot be divisible by it; would break all alignment-size invariants

### φ4: mul_closure → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to two sizes does not imply alignment to their product; e.g. aligned(12,4) && aligned(12,6) but not aligned(12,24)

### φ5: page_implies_l1 → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** 4KB page alignment does not imply 1GB L1 alignment; would conflate all page-table levels and break the translation hierarchy

