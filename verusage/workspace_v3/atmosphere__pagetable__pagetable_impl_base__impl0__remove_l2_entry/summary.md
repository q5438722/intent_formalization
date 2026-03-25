# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__remove_l2_entry/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: closure_loses_l2_table_page → `remove_l2_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Removing an L2 entry should only evict the L1 page from the closure, not the L2 table page itself

### φ2: l1_page_still_in_closure → `remove_l2_entry`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** After removal the L1 page must not remain in the page closure or the page table leaks a page frame

### φ3: mapping_4k_gains_entry → `remove_l2_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Removing an L2 entry must never introduce a new 4K mapping that did not previously exist

### φ4: mapping_2m_loses_entry → `remove_l2_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Removing an L2-to-L1 link must not destroy any existing 2M large-page mappings

### φ5: closure_gains_new_page → `remove_l2_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A removal operation must never grow the page closure with pages that were not previously tracked

