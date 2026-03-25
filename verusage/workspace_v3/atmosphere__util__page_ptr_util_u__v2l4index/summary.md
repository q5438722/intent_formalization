# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__v2l4index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_minimum_index → `v2l4index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the L4 index is always 1 and the function is degenerate, unable to address multiple L4 page-table entries.

### φ2: upper_bound_unreachable → `v2l4index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the maximum L4 index 0x1ff is unreachable, meaning the ensures upper bound is vacuously loose and the top page-table slot is inaccessible.

### φ3: lower_bound_unreachable → `v2l4index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the lower bound KERNEL_MEM_END_L4INDEX is never actually achieved, making the ensures lower bound vacuously loose.

### φ4: result_always_even → `v2l4index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, odd L4 indices are unreachable, halving the effective address space coverage of the page table.

### φ5: injective_on_valid_addresses → `v2l4index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, every distinct valid VA maps to a distinct L4 index, meaning each index covers at most one address — contradicting the purpose of page-table indexing which groups many addresses under one entry.

