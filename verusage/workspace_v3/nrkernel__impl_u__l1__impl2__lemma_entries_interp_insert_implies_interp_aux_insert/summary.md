# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_aux_insert/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: arch_inv_vacuously_satisfiable → `Arch::inv`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If an Arch with zero layers satisfies inv(), a degenerate architecture with no address translation is considered valid, defeating the purpose of the page table structure.

### φ2: entry_base_collision_different_indices → `Arch::entry_base`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two distinct entry indices in the same layer produce the same base address, page table entries would map overlapping virtual address regions, breaking isolation.

### φ3: well_formed_allows_zero_entries → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a well_formed directory can have zero entries, the interp_aux interpretation would always be empty, allowing vacuously valid but useless directories that waste a page table level.

### φ4: page_frame_zero_size_under_inv → `Directory::pages_match_entry_size`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a page within a valid directory can have a zero-sized frame, the mapping represents no actual memory, which would silently create unusable page table entries.

### φ5: directory_child_same_layer → `Directory::directories_are_in_next_layer`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a child directory can reside at the same layer as its parent, the decreasing measure on layer breaks, enabling infinite recursion in the page table walk and a non-terminating interpretation.

