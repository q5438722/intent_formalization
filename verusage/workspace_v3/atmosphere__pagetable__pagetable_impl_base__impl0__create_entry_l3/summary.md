# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__create_entry_l3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: new_l3_entry_not_present → `create_entry_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A newly created L3 entry with present==false would be invisible to hardware address translation

### φ2: new_l3_entry_not_user → `create_entry_l3`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An L3 entry in user space without the user bit set would violate user/kernel isolation

### φ3: other_l3_mapping_disappears → `create_entry_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Creating an L3 entry at one index must not destroy a sibling L3 entry at a different index

### φ4: table_page_becomes_data_mapped → `create_entry_l3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A page used as a table structure appearing in data mappings would break memory safety invariants

### φ5: unrelated_l4_entry_changes → `create_entry_l3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Modifying an L3 slot under one L4 index must not corrupt the L4 resolution at a different index

