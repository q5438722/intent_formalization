# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__map_fold_ok.rs`
**Date:** 2026-03-24T10:13:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: set_fold_empty_returns_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If folding an empty set didn't return the zero element, the base case would inject phantom values

### φ2: set_fold_infinite_returns_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Folding an infinite set silently returns zero instead of being undefined — callers may not realize they get the identity element for non-finite sets

### φ3: map_fold_equals_map
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If map_fold and Set::map diverged, the custom set_fold-based implementation would silently compute a different set than the standard map operation

### φ4: map_fold_singleton
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If mapping a singleton set didn't produce a singleton with the mapped element, set_fold's choose+remove recursion would be broken at the simplest case

### φ5: set_fold_singleton_applies_f
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Folding a singleton should apply f exactly once to (zero, element) — any other result means set_fold mishandles the base-to-single-element transition

