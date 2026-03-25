# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_l3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_none → `get_entry_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If L3 resolution always returns None, the function is vacuous and no L3/L2/L1 page walks can ever succeed

### φ2: first_l3_index_none → `get_entry_l3`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the first L3 index always resolves to None, boundary entries at index 0 are unreachable

### φ3: some_implies_ps_set → `get_entry_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The function returns Some only when ps is false; if the spec entails ps is true for Some results, the ensures are contradictory

### φ4: l3_index_irrelevant → `get_entry_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the L3 index has no effect on the result, the page table cannot distinguish different L3 entries within the same L4 slot

### φ5: some_addr_zero → `get_entry_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A resolved present L3 entry with a null physical address would point to invalid memory, making lower-level walks unsound

