# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_l2/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: l2_always_none → `get_entry_l2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If L2 resolution always fails, the page table can never walk past L2 to reach 4K pages, making 4K mappings dead code.

### φ2: l2_entry_addr_zero → `get_entry_l2`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If every resolved L2 entry points to address 0, all L1 tables would alias to the null page, collapsing the entire 4K mapping space.

### φ3: l2_some_implies_writable → `get_entry_l2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If all resolvable L2 entries are forced writable, read-only page protections at L2 would be impossible, breaking memory safety.

### φ4: l2_none_empties_all_4k → `get_entry_l2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If L2 returning None forces the L3 entry address to zero, it conflates absent L2 sub-tables with null L3 pointers, over-constraining reachable states.

### φ5: l2_result_equals_l3 → `get_entry_l2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the L2 resolution always returns the L3 entry itself, the L2 table walk is a no-op and no independent L2-level translation occurs.

