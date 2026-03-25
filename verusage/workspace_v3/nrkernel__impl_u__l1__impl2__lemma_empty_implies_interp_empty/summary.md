# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_empty_implies_interp_empty/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: entry_base_zero_idx_not_base → `entry_base (Arch)`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** entry_base at index 0 should equal the base address; if this proves, the address calculation is fundamentally broken

### φ2: entry_base_not_strictly_increasing → `entry_base (Arch)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** consecutive entry bases must be strictly increasing for entries to map disjoint regions; provability implies overlapping or reversed address layout

### φ3: inv_page_forced_zero_size → `pages_match_entry_size`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** under inv, page frame sizes must equal entry_size which is positive; if this proves, the invariant erroneously forces zero-size frames

### φ4: empty_dir_has_mapping → `interp`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** an empty directory should have no mappings; provability means the interp of an all-Invalid directory is non-empty, violating the empty semantics

### φ5: child_entry_size_exceeds_parent → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** parent layer entries cover larger regions than children (entry_size(i) = entry_size(i+1) * num_entries(i+1)); provability means the size hierarchy is inverted

