# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_2m_l2/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ret_some_but_not_present → `get_entry_2m_l2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A resolved 2MB mapping should always have its present bit set; returning a non-present entry as Some indicates a spec inconsistency.

### φ2: ret_some_but_ps_not_set → `get_entry_2m_l2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A 2MB large-page entry must have the ps (page-size) bit set; returning an entry without ps would confuse large-page and table-pointer semantics.

### φ3: always_returns_none → `get_entry_2m_l2`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the spec forces every 2MB lookup to return None, the function is vacuously correct but useless—no 2MB page could ever be resolved.

### φ4: ret_some_addr_zero → `get_entry_2m_l2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A successfully resolved 2MB mapping pointing to physical address 0 is almost certainly a null-pointer bug in the page table specification.

### φ5: result_ignores_l2_index → `get_entry_2m_l2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the L2 index has no effect on the resolved mapping, the spec fails to distinguish distinct L2 slots and would collapse all 512 entries into one.

