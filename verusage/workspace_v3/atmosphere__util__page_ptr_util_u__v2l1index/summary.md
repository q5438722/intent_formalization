# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__v2l1index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_zero → `v2l1index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the L1 index is always zero for every valid address, the page table index extraction is broken and all mappings collapse to a single entry.

### φ2: tighter_upper_bound → `v2l1index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the index is always below 256, the claimed upper bound of 0x1ff is too loose and only half the L1 page table entries are reachable.

### φ3: constant_output → `v2l1index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If any two valid addresses always yield the same L1 index, the function is effectively constant and cannot distinguish different page table entries.

### φ4: max_index_unreachable → `v2l1index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the maximum L1 index 0x1ff is unreachable, the last page table entry can never be addressed, indicating an off-by-one or masking error in the validity predicates.

### φ5: index_equals_full_page_number → `v2l1index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the 9-bit mask is redundant and the index equals the full shifted page number, higher-order address bits are leaking into the L1 index, causing incorrect page table lookups.

