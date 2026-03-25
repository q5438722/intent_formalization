# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_aux_equal/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: arch_inv_allows_empty_layers → `inv (Arch)`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If inv() is satisfiable with zero layers, the architecture is vacuously valid but maps no address space — likely a degenerate case the spec should exclude.

### φ2: arch_inv_forces_equal_entry_sizes → `entry_size (Arch)`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If inv() forced adjacent layers to have equal entry sizes, the hierarchical size decomposition would be broken — each layer should cover a strictly larger region than the next.

### φ3: entry_base_collision_distinct_indices → `entry_base (Arch)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two distinct entry indices produced the same base address, page table entries would alias and the MMU mapping would be ambiguous.

### φ4: well_formed_forces_layer_zero → `well_formed`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If well_formed() could only be satisfied at layer 0, subdirectories at deeper layers could never be well-formed, breaking the recursive directory invariant.

### φ5: inv_allows_zero_size_page_frame → `pages_match_entry_size`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a directory satisfying inv() could contain a page with a zero-size frame, it would represent an empty memory mapping — violating the expectation that every mapped page covers a positive region.

