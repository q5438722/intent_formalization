# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_filter_contains_implies_seq_contains.rs`
**Date:** 2026-03-24T04:54:33Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from the external_body axiom `seq_filter_is_a_subset_of_original_seq`, which asserts filter containment without proof. The property is semantically correct and provable by induction on `Seq::filter`'s definition, making the external_body an unnecessary trust assumption that could be eliminated. Three false positives are standard filter properties proved directly from filter's definition.

## True Positives (Spec Issues)

### external_body_filter_subset
- **Confidence:** high
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. The property is semantically correct and provable from `Seq::filter`'s definition by induction, making the external_body unnecessary but still an unverified axiom.

### filter_index_in_original
- **Confidence:** high
- **Reasoning:** Direct instantiation of the second ensures clause of the same external_body axiom. Same unverified trust surface as φ1 — no independent trust beyond the root axiom.

## All Candidates

### φ1: external_body_filter_subset
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_filter_is_a_subset_of_original_seq is external_body — unverified axiom asserting filter containment without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. The property is semantically correct and provable from `Seq::filter`'s definition by induction, making the external_body unnecessary but still an unverified axiom.

### φ2: filter_index_in_original
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** second ensures clause of external_body axiom — unverified claim that every indexed filter element exists in original
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the second ensures clause of the same external_body axiom. Same unverified trust surface as φ1 — no independent trust beyond the root axiom.

### φ3: filter_pred_true_on_elements
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** filter should only contain elements satisfying pred — tests that filter's definition correctly restricts membership
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved directly from `reveal(Seq::filter)` without the external_body axiom. Correct property of filter's definition.

### φ4: filter_len_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filter should never produce a longer sequence than the original — if this fails the external_body axiom could be inconsistent with filter's definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition without the external_body axiom. Standard property of filter.

### φ5: empty_filter_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filtering an empty sequence should produce empty — tests base case consistency between external_body axiom and filter definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Base case of filter's definition. Correct and proved without the external_body axiom.

