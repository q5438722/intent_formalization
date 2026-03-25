# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__rl2/spec_t__mmu__rl2__lemma_step_writenonneg_valid_walk_unchanged.rs`
**Date:** 2026-03-24T14:01:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: write_seq_pml4_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_write_seq` is `external_body` — pml4 preservation across write sequences is trusted without proof

### φ2: ptmem_view_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `PTMem::view` is `external_body` — the PTE interpretation function is entirely opaque and unverified

### φ3: pt_walk_unrelated_write_at_pml4
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Writing to an unrelated address when the L0 entry is non-present should not change the walk path length — tests that pt_walk only depends on addresses it actually reads

### φ4: memories_disjoint_no_overlap_boundary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The last byte of physical memory range should not be in ptmem range — tests that `range_mem.1 < range_ptmem.0` creates a genuine gap with no overlap

### φ5: in_ptmem_range_bounded_by_max_phyaddr
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** All ptmem addresses should be within MAX_PHYADDR — but MAX_PHYADDR depends on the underspecified MAX_PHYADDR_WIDTH (32..=52), so the actual bound is uncertain

