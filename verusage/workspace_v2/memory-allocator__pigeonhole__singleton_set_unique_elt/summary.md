# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__singleton_set_unique_elt.rs`
**Date:** 2026-03-24T11:43:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The `singleton_set_unique_elt` lemma is fully verified (not external_body) using `assert_by_contradiction` with vstd's `remove`/`len` axioms. All tested properties — symmetry, `choose()` determination, concrete singleton membership, removal yielding empty, and contrapositive exclusion — are correct mathematical consequences.

## All Candidates

### φ1: singleton_unique_symmetric
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Symmetry of the uniqueness result — trivially follows from `a == b`, testing the basic consequence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Symmetry of equality is trivial. The lemma is fully verified (not external_body), so `b == a` follows directly from the proved `a == b`.

### φ2: singleton_element_is_choose
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any element in a singleton equals `choose()` — a correct consequence but shows the lemma determines `choose()` for singletons
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `choose()` returns an element contained in a non-empty set. Since the set is a singleton and the lemma proves any two contained elements are equal, `choose() == a` is a correct consequence.

### φ3: singleton_insert_same
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A concrete singleton `{a}` contains only `a` — correct but tests the lemma on a constructed singleton
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `{a}` containing only `a` is a trivially correct set-theoretic fact. The lemma is fully verified, so applying it to a concrete singleton is sound.

### φ4: remove_from_singleton_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Removing the sole element from a singleton yields empty — tests the boundary interaction between `remove` and `len`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Removing the sole element from a singleton yields the empty set — this follows from vstd's set axioms (`remove` decreases `len` by 1 when the element is present). No spec gap.

### φ5: singleton_no_second_element
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A singleton set cannot contain a distinct element — correct contrapositive of uniqueness, but confirms the lemma's full power
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The contrapositive of uniqueness — if `c != a` then `!s.contains(c)` — is a correct logical consequence of the fully verified lemma. No spec gap.

