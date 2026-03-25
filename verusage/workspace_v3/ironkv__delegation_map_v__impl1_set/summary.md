# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl1_set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: length_increases_after_set → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** set should preserve length; if the spec entails a length increase the ensures is wrong

### φ2: frame_violation_other_index → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Elements at untouched indices must be preserved; a frame violation means the update spec is too weak

### φ3: target_element_not_k → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After set the element at index i must equal k; if the spec allows otherwise the update semantics are broken

### φ4: single_element_vacuous → `set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single-element vec is a valid boundary case for set at index 0; if the spec is vacuously satisfiable here the preconditions are inconsistent

### φ5: sortedness_broken_in_result → `set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The post-state must be strictly sorted; if the spec entails the existence of an unsorted pair then valid() is not truly maintained

