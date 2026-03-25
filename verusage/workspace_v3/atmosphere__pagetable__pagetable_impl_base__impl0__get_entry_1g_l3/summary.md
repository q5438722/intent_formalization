# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_1g_l3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_some_1g → `get_entry_1g_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every L3 lookup unconditionally returns Some, non-present or non-PS entries would be treated as valid 1G mappings.

### φ2: always_none_1g → `get_entry_1g_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the result is always None, 1G large-page resolution is completely broken and no gigabyte page can ever be used.

### φ3: some_implies_addr_zero → `get_entry_1g_l3`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A resolved 1G mapping with physical address 0 would alias the null frame, corrupting low physical memory.

### φ4: some_implies_user_not_set → `get_entry_1g_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** User-space page table entries must have the user bit set; lacking it would prevent ring-3 access and violate user_only invariant.

### φ5: some_implies_ps_not_set → `get_entry_1g_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A 1G mapping must have PS set; without it hardware interprets the entry as an L2 table pointer, causing memory corruption.

