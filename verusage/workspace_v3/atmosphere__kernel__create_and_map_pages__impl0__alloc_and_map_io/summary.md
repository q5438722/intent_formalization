# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_map_pages__impl0__alloc_and_map_io/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pa_always_zero → `alloc_and_map_io`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If PA extraction collapses all nonzero values to 0, alloc_and_map_io always returns a null physical address for mapped IO pages.

### φ2: mem_valid_vacuous → `alloc_and_map_io`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If MEM_valid is trivially true for every address, usize2pa's postcondition provides no real constraint on physical pages returned by alloc_and_map_io.

### φ3: zero_entry_present → `alloc_and_map_io`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If zero-valued entries decode as present, uninitialized L2 slots spuriously satisfy alloc_and_map_io's L2-resolution precondition, enabling mapping into garbage page tables.

### φ4: perm_always_user → `alloc_and_map_io`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If every page-entry permission has the user bit set, IO pages allocated by alloc_and_map_io are accessible from unprivileged code, breaking device-isolation guarantees.

### φ5: pa_non_injective → `alloc_and_map_io`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If distinct present page-table entries always decode to the same physical address, alloc_and_map_io could alias a new IO mapping onto an already-used physical frame.

