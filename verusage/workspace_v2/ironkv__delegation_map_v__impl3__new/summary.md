# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl3__new.rs`
**Date:** 2026-03-24T08:32:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Two (cmp_spec_no_irreflexivity, sorted_no_transitivity) are tautologies of the form `P || !P` that verify trivially regardless of the spec. The empty-sorted property is the standard vacuous base case. The map-keys consistency property is a direct and correct consequence of the `map_valid` invariant linking the ghost map domain to the keys set.

## All Candidates

### φ1: sorted_empty_is_valid
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty sequence vacuously satisfies `sorted` — any predicate over pairs of indices holds trivially, which is correct but could mask issues if callers assume sorted implies non-empty
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence vacuously satisfying `sorted` is correct and expected — it's the standard base case for any ordering predicate over pairs. No caller should assume sorted implies non-empty.

### φ2: cmp_spec_no_irreflexivity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `cmp_spec` is uninterpreted with no axiom preventing `a.cmp_spec(a).lt()` from being true — if `cmp_spec` is reflexively less-than, `sorted` allows duplicate-valued elements while `no_duplicates` operates on structural equality, creating an inconsistency between the two validity conditions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property `a.cmp_spec(a).lt() || !a.cmp_spec(a).lt()` is a tautology (law of excluded middle) that holds for any expression. It reveals nothing about whether `cmp_spec` lacks irreflexivity axioms.

### φ3: sorted_no_transitivity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `cmp_spec` has no transitivity axiom — `sorted` checks all pairs directly via the quantifier, but downstream code reasoning about `cmp_spec` outside of sequences cannot derive transitivity, potentially making the ordering relation inconsistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures `a.cmp_spec(c).lt() || !a.cmp_spec(c).lt()` is again a tautology. It does not actually test whether transitivity holds — it would verify regardless of any axioms on `cmp_spec`.

### φ4: map_valid_ghost_unconstrained_outside_keys
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `map_valid` asserts `m@.dom() == keys@.to_set()` — but both `m` and `keys` are ghost/closed fields, and `map_valid` is closed, so external code cannot verify this link holds after external_body operations that might desync the ghost map from the keys vector
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct consequence of `map_valid` which asserts `m@.dom() == keys@.to_set()`. If `k` is not in the map domain, it cannot be in the keys set. This is a desirable and expected property of the validity invariant.

### φ5: new_external_body_sorted_empty_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `StrictlyOrderedVec::new` is external_body and asserts `v.valid()` on the empty vec — the sorted+no_duplicates ensures on empty are trivially true, but the external_body trust means the implementation could return a non-empty vec that violates validity without detection

