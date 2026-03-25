# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl3__set.rs`
**Date:** 2026-03-24T08:34:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Two test external_body proof functions (`gap_means_empty`, `mind_the_gap`) whose specifications are mathematically correct given the ordering axioms. The other two derive standard set-theoretic or validity-invariant consequences without actually depending on external_body implementations. No real spec issues were identified.

## All Candidates

### φ1: gap_means_empty_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `gap_means_empty` is external_body and derives `false` from its preconditions — if any precondition can be fabricated (e.g., via another external_body unsoundness), this becomes an unrestricted `false` oracle
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `gap_means_empty` derives `false` from genuinely contradictory preconditions: `gap(lo, hi)` says no key exists between `lo` and `hi`, yet `contains_key(*k.get())` with `lo < k < hi` contradicts this. The ensures `false` is logically correct — the preconditions are unsatisfiable together. The external_body trust is the standard pattern for this kind of lemma.

### φ2: mind_the_gap_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mind_the_gap` is external_body with no proof — its gap-transitivity ensures clause is asserted without verification, so a bug in the gap merging logic would silently introduce unsoundness
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** Gap transitivity is a correct mathematical property: if no map keys exist in `(w,x)` and no keys exist in `(y,z)`, and `y < x` (the gaps overlap), then no keys exist in `(w,z)`. While external_body means the proof is trusted, the specification itself is mathematically sound given the `cmp_properties` axioms.

### φ3: set_gap_spec_allows_key_removal
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A key inside a gap cannot exist in the map — this is correct but combined with the external_body trust of `gap_means_empty`, could be used to incorrectly derive non-membership of keys that actually exist if the gap predicate is poisoned

### φ4: find_key_none_external_body
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If the map contains key `k`, `find_key`'s spec guarantees an index exists — but `find_key` is external_body, so the implementation could return `None` for an existing key, and the spec-level reasoning would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property that a key in the map domain has a corresponding index in the keys vector follows from `map_valid` which asserts `m@.dom() == keys@.to_set()`. This is a consequence of the validity invariant, not of `find_key`'s external_body. The proof doesn't even call `find_key`.

### φ5: insert_external_body_vec_desync
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `StrictlyOrderedVec::insert` is external_body and ensures `to_set() == old.to_set().insert(k)` — but the set cardinality increase is derived from `!contains(k)` at the spec level without verifying the implementation actually inserts at the right position
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property is a standard set-theoretic fact: inserting an element not in a set increases its cardinality by 1. The proof derives this from `!v@.contains(k)` implying `!v@.to_set().contains(k)`, which is pure spec-level reasoning about sets. It doesn't depend on `StrictlyOrderedVec::insert`'s external_body at all.

