# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__array__impl2__init2zero/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_changes_other_index → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, set would corrupt elements at indices other than the one being written.

### φ2: set_value_not_written → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, set would fail to actually store the provided value at the target index.

### φ3: set_length_changes → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, set would change the array length, violating the structural invariant.

### φ4: init2zero_forces_empty → `init2zero`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, the init2zero postcondition would be satisfiable only for empty arrays, making it useless.

### φ5: init2zero_nonzero_element → `init2zero`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, init2zero would allow a non-zero value at a valid index, contradicting its zeroing guarantee.

