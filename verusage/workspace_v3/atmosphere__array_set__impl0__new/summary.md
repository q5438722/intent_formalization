# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__array_set__impl0__new/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: new_contains_zero → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty set should not contain element 0; if provable, the spec fails to exclude membership.

### φ2: new_len_nonzero → `new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A freshly constructed empty ArraySet must have len == 0; positive len would violate the invariant.

### φ3: new_set_nonempty → `new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The empty set has cardinality 0; a positive cardinality would mean the spec is inconsistent.

### φ4: new_data_element_true → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** All backing-array entries should be false for an empty set; a true entry implies a phantom member.

### φ5: new_contains_arbitrary → `new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** No valid index should be a member of the empty set; if provable, the wf/emptiness specs are too weak.

