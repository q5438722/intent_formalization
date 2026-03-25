# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__new/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: new_nonempty_domain → `StrictlyOrderedMap::new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A freshly constructed map must have an empty domain; provability would mean the spec allows non-empty initial state

### φ2: new_contains_arbitrary_key → `StrictlyOrderedMap::new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty map must not contain any key; provability would indicate the empty-map postcondition is inconsistent with containment semantics

### φ3: valid_contradicts_empty → `StrictlyOrderedMap::new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** valid() and empty view are co-guaranteed; if both requires lead to !valid(), the ensures of new are mutually contradictory

### φ4: new_domain_off_by_one → `StrictlyOrderedMap::new`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An empty map domain must have length 0 not 1; provability would reveal an off-by-one error in the validity or emptiness specification

### φ5: empty_map_not_finite_domain → `StrictlyOrderedMap::new`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The empty map's domain is trivially finite; provability would mean map_valid's finiteness invariant is contradicted by the view spec

