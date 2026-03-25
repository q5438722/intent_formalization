# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__create_full/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: contains_out_of_upper_bound → `create_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** After create_full the set should be {0..511}; containing 512 would mean the upper bound is wrong

### φ2: contains_negative_index → `create_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** After create_full the set should have no negative elements; containing -1 would indicate a lower-bound error

### φ3: result_set_is_empty → `create_full`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** create_full should produce a full mask; being empty would mean the spec is trivially unsound or vacuous

### φ4: result_equals_half_range → `create_full`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The full mask covers 512 bits; equalling only the first 256 would indicate COMMIT_MASK_BITS is misused or halved

### φ5: missing_last_element → `create_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 511 is the last valid index in a 512-bit mask; its absence would indicate an off-by-one in the spec

