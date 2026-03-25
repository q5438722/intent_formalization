# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__set_map_union.rs`
**Date:** 2026-03-24T10:15:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — they verify standard set-algebraic properties (empty identity, union commutativity, union idempotence, and subset monotonicity under map) that follow directly from set definitions and the distributive law proved by `set_map_union`.

## All Candidates

### φ1: map_union_empty_left
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If union with empty on the left changed the map result, the identity element of set union would be broken under mapping
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty set is the identity for union, so mapping distributes correctly. Standard set algebra property.

### φ2: map_union_commutative
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If mapped union wasn't commutative, the set_map_union lemma combined with set union commutativity would yield contradictory equalities
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Set union is commutative by definition. This doesn't even depend on the lemma under test.

### φ3: map_union_idempotent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Mapping over a self-union should equal mapping once — if not, the union distribution would double-count elements in the mapped set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s + s == s` for sets (union is idempotent), so `(s + s).map(f) == s.map(f)` follows directly. Correct property.

### φ4: map_union_subset_left
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If mapping s1 weren't a subset of mapping the union, the distributive law would lose elements from the left operand
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s1 ⊆ s1 ∪ s2` implies `s1.map(f) ⊆ (s1 ∪ s2).map(f)`. Standard monotonicity of set map over union.

### φ5: map_union_constant_fn_singleton
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Mapping a union with a constant function should produce a singleton regardless of input sizes — if it produced more elements, the map operation would fabricate values

