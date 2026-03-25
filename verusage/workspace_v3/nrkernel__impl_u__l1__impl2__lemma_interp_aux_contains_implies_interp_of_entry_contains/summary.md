# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_aux_contains_implies_interp_of_entry_contains/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: entry_base_collision → `Arch::entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Two distinct entry indices mapping to the same base address would cause virtual address aliasing in the page table

### φ2: well_formed_allows_zero_entries → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A well-formed directory with zero entries is degenerate and would mean the page table maps no address space at that subtree

### φ3: inv_allows_subpage_entry_size → `Arch::inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A leaf-layer entry_size below 4096 bytes would violate the x86 minimum page size, making the architecture physically unrealizable

### φ4: dir_base_vaddr_mismatch → `Directory::directories_are_in_next_layer`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A child directory whose base_vaddr disagrees with its parent's layout would break the contiguous virtual address mapping invariant

### φ5: entry_size_not_decomposable → `Arch::entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a layer's entry_size is not the product of the next layer's entry_size and num_entries, the hierarchical address space decomposition is inconsistent

