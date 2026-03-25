# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_insert_implies_interp_aux_insert/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: adjacent_entry_overlap → `Arch::entry_base, Arch::next_entry_base`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Adjacent entries overlapping would mean two page table entries map the same virtual address, destroying isolation.

### φ2: upper_vaddr_leq_base → `Directory::upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A directory whose upper bound does not exceed its base covers zero or negative address space, making it useless.

### φ3: update_mutates_layer → `Directory::update`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If updating a single entry changes the directory's layer, the tree structure is silently corrupted.

### φ4: entry_base_zero_idx_not_base → `Arch::entry_base`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The first entry (index 0) not starting at the directory's base address would create an unmapped hole at the start of every directory.

### φ5: entry_size_layer_inversion → `Arch::entry_size_is_next_layer_size, Arch::inv`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A higher (deeper) layer having larger entry sizes than its parent would invert the page table hierarchy, making subdivision meaningless.

