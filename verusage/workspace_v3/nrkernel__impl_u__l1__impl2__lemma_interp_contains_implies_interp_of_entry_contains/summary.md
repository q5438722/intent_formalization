# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_contains_implies_interp_of_entry_contains/original.rs`
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
- **Why flagged:** Different indices should map to distinct base addresses; collision would allow overlapping memory regions

### φ2: arch_inv_allows_zero_layers → `Arch::inv`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A valid architecture with zero layers is degenerate and maps no address space, suggesting inv is too permissive

### φ3: interp_maps_below_base → `Directory::interp`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Interpreted mappings should only cover addresses at or above the directory's base_vaddr; mapping below would be unsound

### φ4: page_zero_frame_size → `Directory::pages_match_entry_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A page frame with zero size under a valid invariant means the page maps no memory, which defeats the purpose of the mapping

### φ5: well_formed_exceeds_max_entries → `Directory::well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A well-formed directory should not have more entries than the x86 architectural maximum of 512

