# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl5__delegate_for_key_range_is_host_impl/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: valid_dm_unsatisfiable → `delegate_for_key_range_is_host_impl`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If valid() is unsatisfiable, all ensures of delegate_for_key_range_is_host_impl hold vacuously

### φ2: delegation_always_true → `delegate_for_key_range_is_host_impl`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If delegation check is always true, the function never rejects an incorrect host assignment for a key range

### φ3: is_end_always_true → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If is_end is always true then no KeyIterator holds a valid key, making get's precondition unsatisfiable

### φ4: get_all_keys_zero → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If get always yields the zero key, the iterator cannot distinguish different keys and delegation lookups collapse

### φ5: clone_collapses_keys → `clone`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If distinct SHTKey values cannot exist, clone is trivially correct but the entire key-based delegation system is degenerate

