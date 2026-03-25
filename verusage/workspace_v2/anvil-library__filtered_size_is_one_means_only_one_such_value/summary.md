# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/multiset_lib/filtered_size_is_one_means_only_one_such_value.rs`
**Date:** 2026-03-24T04:45:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — standard properties of vstd's Multiset axiomatization. No spec issues found.

## All Candidates

### φ1: len_zero_iff_empty_multiset
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** len_is_zero_means_count_for_each_value_is_zero is external_body — if its axiom is too strong or interacts unexpectedly with Multiset extensionality, it could force equality with empty that bypasses actual multiset reasoning
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A multiset with all counts zero is extensionally equal to the empty multiset. The external_body axiom gives all-zero counts, and extensional equality follows directly. Correct property.

### φ2: filter_true_is_identity
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** if filter with always-true predicate doesn't preserve length, the filter axiomatization would be inconsistent — tests whether filter's len interacts correctly with the identity case

### φ3: filter_false_is_empty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** filter with always-false should yield empty — relies on filtered_size_is_zero external_body axiom being sound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Filtering by always-false means no element satisfies the filter, so the filtered multiset has length 0. Correct consequence of the axiom.

### φ4: filtered_one_unique_witness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests that the uniqueness clause of filtered_size_is_one actually forces all matching elements equal — if the proof has a gap, distinct v and u satisfying the filter could coexist
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The verified lemma `filtered_size_is_one_means_only_one_such_value` explicitly ensures uniqueness of matching elements. This is a direct application — correct and expected.

### φ5: contains_implies_positive_count
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** contains and count must agree — if contains is axiomatized independently of count, a multiset could "contain" an element with zero count, breaking the len_is_zero axiom's foundation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In vstd, `m.contains(v)` is defined as `m.count(v) > 0`. This is definitional — no spec gap.

