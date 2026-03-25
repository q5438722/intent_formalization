# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__lemma_candidate_mapping_inflight_vmem_overlap_os_implies_hl/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) is provable, the spec fails to guard against zero-size alignment (division by zero)

### φ2: upper_vaddr_equals_base → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals base under inv, the mapped address range is zero-sized, meaning no virtual addresses are covered

### φ3: entry_base_collision → `entry_base`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If distinct indices yield the same entry base, page table entries would collide and the translation structure is unsound

### φ4: entry_size_hierarchy_inversion → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a child layer's entry size is >= its parent's, the hierarchical page table decomposition is broken

### φ5: inv_allows_zero_num_entries → `inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If inv permits a layer with zero entries, the architecture admits degenerate empty layers that cannot map any addresses

