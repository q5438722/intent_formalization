# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__get_entry_l4/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: present_entry_yields_none → `get_entry_l4`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A present L4 entry resolving to None would silently discard valid page table mappings

### φ2: absent_entry_yields_some → `get_entry_l4`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-present L4 entry resolving to Some would fabricate phantom mappings from empty slots

### φ3: some_entry_is_empty → `get_entry_l4`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A resolved Some entry being empty (addr=0, all perms false) contradicts it being present and meaningful

### φ4: some_addr_always_zero → `get_entry_l4`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** All L4 entries resolving to physical address 0 would collapse the entire address space to a single frame

### φ5: none_still_has_4k_mapping → `get_entry_l4`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A 4k mapping existing under a None L4 entry would mean memory is accessible without valid page table traversal

