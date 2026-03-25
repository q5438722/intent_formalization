# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/multiset_lib/len_is_zero_means_count_for_each_value_is_zero.rs`
**Date:** 2026-03-24T04:46:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard properties of vstd's Multiset axiomatization. No spec issues found.

## All Candidates

### φ1: nonempty_has_positive_len
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tests the reverse direction — a positive count should force nonzero len; if the biconditional is wrong this could fail
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct reverse direction of the biconditional. Correct and expected.

### φ2: empty_multiset_len_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty multiset should have length zero — tests basic consistency of Multiset axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Definitional property of empty multiset in vstd. Correct.

### φ3: singleton_len_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** inserting one element into empty should give len 1 — if len axioms are inconsistent this could produce a different value
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard vstd axiom: insert adds 1 to len. Correct.

### φ4: choose_returns_contained_element
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** choose on nonempty multiset should return an element with positive count — tests that choose's axiomatization is consistent with len
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `choose` is axiomatized to return an element with positive count when len > 0. Correct and expected.

### φ5: add_then_remove_preserves_len
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** insert then remove of same element should preserve length — tests consistency between insert/remove/len axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Insert adds 1 to len, remove subtracts 1. Round-trip preserves len. Correct.

