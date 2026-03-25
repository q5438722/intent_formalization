# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_equal/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: arch_inv_allows_zero_layers → `Arch::inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, inv() is vacuously satisfied by empty architectures, making the invariant too weak to be meaningful.

### φ2: entry_base_zero_index_differs_from_base → `Arch::entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Entry base at index 0 must equal the base address; a mismatch breaks the fundamental address decomposition.

### φ3: well_formed_allows_zero_entries → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, well-formed directories can be empty, yielding page table levels with no mappable regions.

### φ4: page_frame_size_zero_under_inv → `Directory::pages_match_entry_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the invariant permits zero-size page frames, which represent physically meaningless memory regions.

### φ5: subdir_base_equals_parent_base → `Directory::directories_are_in_next_layer`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, non-first subdirectories share the parent's base address, causing overlapping virtual address ranges.

