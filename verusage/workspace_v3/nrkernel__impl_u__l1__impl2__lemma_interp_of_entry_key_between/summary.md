# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_key_between/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: arch_inv_unsatisfiable → `Arch::inv`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If Arch::inv is unsatisfiable, every spec depending on it is vacuously true and provides no guarantees

### φ2: entry_base_equals_next_entry_base → `Arch::entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entry_base equals next_entry_base, entries have zero width and address regions collapse to empty intervals

### φ3: arch_inv_forces_single_layer → `entry_size_is_next_layer_size`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If multi-layer architectures are impossible under inv, the hierarchical page table structure is unmodelable

### φ4: directory_inv_forbids_subdirectories → `Directory::directories_are_in_next_layer`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If inv forbids all subdirectories, the page table cannot represent a multi-level hierarchy

### φ5: well_formed_forces_single_entry → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If well_formed forces at most one entry, directories cannot map the full address space with 512-entry page tables

