# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__os_invariant__lemma_candidate_mapping_inflight_vmem_overlap_hl_implies_os/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size_trivially_true → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If alignment to size 0 holds for arbitrary nonzero addresses, the spec fails to guard against meaningless zero-size alignment checks

### φ2: entry_base_nonzero_at_zero_index → `entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The first entry (idx=0) should start exactly at the base address; if entry_base diverges from base at index 0, the address calculation is fundamentally broken

### φ3: upper_vaddr_not_greater_than_base → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Under a valid invariant (entry_size > 0, num_entries > 0), upper_vaddr must strictly exceed base; equality or less would mean the layer maps zero or negative address space

### φ4: child_entry_size_exceeds_parent → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The hierarchical constraint entry_size(i) == entry_size(i+1) * num_entries(i+1) with num_entries >= 1 means a child layer's entry size can never exceed its parent's; proving otherwise reveals an inconsistent hierarchy

### φ5: inv_allows_zero_num_entries → `inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The invariant explicitly requires 0 < num_entries for every layer; if a zero-entry layer is permitted, address space subdivision collapses and upper_vaddr degenerates

