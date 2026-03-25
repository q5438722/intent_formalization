# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__spec_t__mmu__defs__x86_arch_exec/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: first_layer_is_page_size → `x86_arch_exec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** L0 should map 512G regions, not 4K pages; equality with PAGE_SIZE would indicate reversed layer order

### φ2: l1_num_entries_not_512 → `x86_arch_exec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** All x86-64 page-table layers have exactly 512 entries; a mismatch would corrupt address translation

### φ3: adjacent_layers_equal_entry_size → `x86_arch_exec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Adjacent layers must have different granularities (factor of 512); equal sizes would collapse the hierarchy

### φ4: last_layer_not_page_size → `x86_arch_exec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The finest-granularity layer (L3) must map exactly PAGE_SIZE (4096); any other value breaks 4K page mapping

### φ5: layers_not_strictly_decreasing → `x86_arch_exec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Entry sizes must strictly decrease from L0 (512G) to L3 (4K); non-decreasing order would invert the page-table hierarchy

