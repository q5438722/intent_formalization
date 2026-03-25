# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_entries_remove_implies_interp_remove/original.rs`
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
- **Why flagged:** An arch with zero layers is degenerate and should be excluded by inv, but the constraint `layers.len() <= X86_NUM_LAYERS` does not enforce a positive lower bound

### φ2: entry_base_equals_next_entry_base → `Arch::entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entry_base equals next_entry_base for a valid arch, entries have zero size contradicting the positive entry_size invariant

### φ3: well_formed_allows_zero_entries → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A directory with zero entries is degenerate; inv should prevent this via num_entries > 0 from arch.inv

### φ4: next_entry_base_not_greater → `Arch::next_entry_base`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** next_entry_base should always be strictly greater than entry_base under a valid arch; if not, address regions would be empty or inverted

### φ5: subdirectory_base_vaddr_overlap → `Directory::directories_are_in_next_layer`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two distinct subdirectory entries sharing the same base_vaddr would mean overlapping address spaces, violating isolation between page table entries

