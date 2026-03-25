# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__definitions_u__impl2__next_entry_base/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: result_equals_base → `ArchExec::next_entry_base`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If next_entry_base with idx=0 equals base, entries would start at the same address as the base, meaning no address space is actually mapped by the first entry.

### φ2: result_less_than_base → `ArchExec::next_entry_base`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If next_entry_base returned a value less than base, page table entries would map backwards into already-mapped address space, corrupting translations.

### φ3: entry_size_zero → `inv`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If inv permitted zero-sized entries, page table walks would never make progress and every entry would degenerate to a zero-length region.

### φ4: layer_sizes_inverted → `entry_size_is_next_layer_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If coarser layers had entry sizes no larger than finer layers, the hierarchical decomposition would be flat or inverted, breaking multi-level address translation.

### φ5: num_entries_always_one → `num_entries`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If every layer were limited to a single entry, the page table could only map one contiguous region per level, making the entire translation structure useless.

