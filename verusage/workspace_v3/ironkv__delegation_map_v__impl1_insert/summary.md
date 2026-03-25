# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl1_insert/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: element_not_at_index → `insert`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the element at the returned insertion index is not k, the spec fails to place k correctly

### φ2: always_appended_at_end → `insert`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the insertion index is always the last position, sorting is not actually constraining placement

### φ3: old_element_lost → `insert`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If a pre-existing element disappears after insert, the spec fails to preserve old contents

### φ4: set_unchanged_after_insert → `insert`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the set view is unchanged, k was never actually added — the spec's set postcondition is vacuous

### φ5: first_element_always_changes → `insert`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the first element always changes, insertion incorrectly disrupts prefix stability when k is not minimal

