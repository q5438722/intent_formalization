# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl3__keys_in_index_range_agree.rs`
**Date:** 2026-03-24T08:31:22Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: values_agree_vacuous_empty_range
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When lo == hi, `values_agree` checks only a single element but returns a universal quantifier result — callers may assume a range was validated when only one element was checked

### φ2: delegation_map_ghost_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `DelegationMap`'s ghost field `m` has no invariant linking it to `lows` — two delegation maps with identical `lows` can have completely different abstract maps, making the ghost field semantically disconnected from the implementation

### φ3: sorted_no_transitivity_requirement
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `cmp_spec` is an uninterpreted spec function with no axioms requiring transitivity, antisymmetry, or totality — `sorted` relies on `cmp_spec` being a strict total order but this is never enforced, so the sorted invariant may be vacuously satisfiable or inconsistent

### φ4: map_valid_ghost_desync
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `map_valid` asserts `self.m@.dom() == self.keys@.to_set()` — but since `m` is a `Ghost<Map>` and `valid` is a closed spec, the ghost map's domain is trusted to equal the keys set without runtime enforcement, so external_body functions like `values_agree` could silently violate this invariant

### φ5: values_agree_external_body_unsound
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `values_agree` is external_body and trusted to correctly check value agreement — if the implementation returns wrong results, `keys_in_index_range_agree` propagates the error to the abstract map level, but no proof verifies the implementation matches the spec

