# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_l1/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: map_entry_addr_always_zero → `page_entry_to_map_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the conversion always loses the physical address, making all mappings point to address 0

### φ2: map_entry_write_always_false → `page_entry_to_map_entry`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, write permission is never propagated to MapEntry, silently making all pages read-only

### φ3: map_entry_xd_equals_write → `page_entry_to_map_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, execute-disable and write are always conflated, preventing independent writable-but-executable or read-only-non-executable pages

### φ4: l1_lookup_always_none → `get_entry_l1`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, no 4K page can ever be mapped through L1, rendering the entire 4K mapping layer vacuous

### φ5: l1_mapping_always_readonly → `get_entry_l1`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, all resolved 4K mappings are forced read-only, meaning writable user pages can never exist at 4K granularity

