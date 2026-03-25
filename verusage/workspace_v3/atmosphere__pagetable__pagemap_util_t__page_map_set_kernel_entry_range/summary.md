# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagemap_util_t__page_map_set_kernel_entry_range/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_no_requires_is_noop → `page_map_set_no_requires`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, the set operation is a no-op — the written entry always equals the old entry regardless of the value argument, meaning the spec fails to constrain the write.

### φ2: kernel_range_preserves_all → `page_map_set_kernel_entry_range`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, kernel entries are never actually written — the function is a no-op for the kernel range, meaning the spec doesn't distinguish old from new entries.

### φ3: set_modifies_other_entries → `page_map_set_no_requires`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, writing at one index corrupts a different index, violating the frame/non-interference property of the page map set operation.

### φ4: kernel_range_corrupts_above → `page_map_set_kernel_entry_range`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If verified, setting kernel entries in [0, KERNEL_MEM_END_L4INDEX) also corrupts entries at or above that boundary, breaking isolation of the non-kernel region.

### φ5: set_drops_present_bit → `page_map_set_no_requires`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, writing a present page entry silently clears its present bit, meaning the spec loses permission information and mapped pages would appear unmapped.

