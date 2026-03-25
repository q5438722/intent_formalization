# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l2_impl__lemma_page_aligned_implies_mask_dir_addr_is_identity/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: size_zero_vacuously_true → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) holds for any addr, the spec is unsound on degenerate size=0 input (division by zero)

### φ2: trivially_true_for_all → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If aligned is true for all addr and size, the spec is vacuous and provides no alignment guarantees

### φ3: successor_preserves_page_alignment → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Adding 1 to a page-aligned address must NOT remain page-aligned; if it does the spec conflates all addresses

### φ4: odd_is_even_aligned → `aligned`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** 1 is not a multiple of 2; if aligned(1,2) holds the modular arithmetic in the spec is broken

### φ5: double_size_always_entailed → `aligned`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Alignment to s does not imply alignment to 2*s (e.g. 4096 is 4096-aligned but not 8192-aligned); provability would mean the spec over-generalizes

