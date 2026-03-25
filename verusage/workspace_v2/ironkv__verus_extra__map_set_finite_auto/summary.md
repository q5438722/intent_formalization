# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__map_set_finite_auto.rs`
**Date:** 2026-03-24T10:14:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `map_finite` is unnecessarily `external_body` — finiteness preservation under `Set::map` is provable but instead trusted as an axiom. The other two are false positives: empty-set mapping is definitionally correct, and the auto lemma correctly chains to preserve finiteness through composed maps.

## True Positives (Spec Issues)

### map_finite_external_body_trusted
- **Confidence:** medium
- **Reasoning:** `map_finite` is `external_body` with `unimplemented!()` — finiteness preservation of `Set::map` is a provable property (by induction on set cardinality) but is instead trusted as an axiom, creating an unnecessary soundness dependency.

## All Candidates

### φ1: map_finite_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `map_finite` is `external_body` with `unimplemented!()` — the finiteness preservation of `Set::map` is trusted without proof, creating an unnecessary soundness dependency
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `map_finite` is `external_body` with `unimplemented!()` — finiteness preservation of `Set::map` is a provable property (by induction on set cardinality) but is instead trusted as an axiom, creating an unnecessary soundness dependency.

### φ2: map_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Mapping an empty set should produce an empty set — if it didn't, the map operation would fabricate elements from nothing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Mapping an empty set produces an empty set by the definition of `Set::map` — no element exists to satisfy the existential in the mapped set's membership predicate. Standard mathematical property.

### φ3: map_finite_auto_instantiation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The auto lemma should allow chaining map operations while preserving finiteness — if it fails for composed maps, the auto quantifier doesn't propagate correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The auto lemma correctly propagates finiteness through chained maps: `s.finite()` implies `s.map(f).finite()`, which in turn implies `s.map(f).map(g).finite()` by the same universal quantifier. This is the intended behavior of the broadcast lemma.

### φ4: map_singleton_is_singleton
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Mapping a singleton set should produce a singleton with the mapped element — any other result means Set::map mishandles the simplest non-empty case

### φ5: map_constant_fn_produces_singleton
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Mapping with a constant function should collapse all elements to a single value — if the result contained more than one element, Set::map would be duplicating rather than deduplicating

