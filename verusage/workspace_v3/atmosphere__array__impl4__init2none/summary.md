# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__array__impl4__init2none/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_is_noop → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, set never modifies the array, making the operation a no-op regardless of the value written

### φ2: set_corrupts_other_index → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, set has unintended side effects on indices other than the target index

### φ3: set_ignores_value → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, set does not actually store the provided value at the target index

### φ4: init2none_first_is_some → `init2none`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, init2none leaves the first element as Some, directly contradicting its all-None guarantee

### φ5: init2none_empties_seq → `init2none`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, init2none collapses the array to an empty sequence instead of preserving length while filling with None

