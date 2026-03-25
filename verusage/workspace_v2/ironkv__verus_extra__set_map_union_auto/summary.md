# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__set_map_union_auto.rs`
**Date:** 2026-03-24T10:16:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `set_map_union` is unnecessarily `external_body` — the set map-union distributive law is provable but trusted as an axiom. The remaining four are false positives representing correct set-algebraic properties derived from the distributive law.

## True Positives (Spec Issues)

### set_map_union_external_body_trusted
- **Confidence:** medium
- **Reasoning:** `set_map_union` is `external_body` with `unimplemented!()` — the distributive law is provable via `assert_sets_equal!` but is instead trusted as an axiom, creating an unnecessary soundness dependency.

## All Candidates

### φ1: set_map_union_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `set_map_union` is `external_body` with `unimplemented!()` — this distributive law is provable via `assert_sets_equal!` but is instead trusted, creating an unnecessary soundness dependency
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `set_map_union` is `external_body` with `unimplemented!()` — the distributive law is provable via `assert_sets_equal!` but is instead trusted as an axiom, creating an unnecessary soundness dependency.

### φ2: map_union_empty_left
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If union with empty on the left changed the map result, the identity element of set union would be broken under mapping
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty set is the identity for union, so `(∅ + s).map(f) == s.map(f)` follows from the distributive law plus `∅.map(f) == ∅`. Standard set algebra.

### φ3: map_union_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Mapping over a self-union should equal mapping once — if not, the union distribution would double-count elements in the mapped set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s + s == s` for sets, so `(s + s).map(f) == s.map(f)` is a direct consequence of union idempotence. Correct property.

### φ4: map_union_subset_left
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If mapping s1 weren't a subset of mapping the union, the distributive law would lose elements from the left operand
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard monotonicity: `s1 ⊆ s1 ∪ s2` implies `s1.map(f) ⊆ (s1 ∪ s2).map(f)`. Correct consequence of the distributive law.

### φ5: auto_lemma_universal_instantiation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The auto lemma should compose for three-way unions — if it fails, the universal quantifier doesn't chain correctly through repeated union-map distribution
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Three-way distribution follows from applying the two-way law twice, combined with set union associativity. The auto quantifier correctly chains.

