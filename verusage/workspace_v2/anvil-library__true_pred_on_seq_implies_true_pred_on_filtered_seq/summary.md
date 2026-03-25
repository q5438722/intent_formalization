# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/true_pred_on_seq_implies_true_pred_on_filtered_seq.rs`
**Date:** 2026-03-24T04:58:43Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from the two ensures clauses of the external_body axiom `seq_filter_is_a_subset_of_original_seq` (unverified trust assumption that filter elements are contained in the original sequence). Three false positives: one is a dependent application of the verified lemma, and two are standard filter properties proved from filter's definition.

## True Positives (Spec Issues)

### external_body_filter_subset
- **Confidence:** high
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains unverified.

### filter_index_in_original
- **Confidence:** high
- **Reasoning:** Second ensures clause of the same external_body axiom. Same unverified trust surface as φ1.

## All Candidates

### φ1: external_body_filter_subset
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_filter_is_a_subset_of_original_seq is external_body — unverified axiom asserting filter containment without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains unverified.

### φ2: filter_index_in_original
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** second ensures clause of the external_body axiom — unverified claim that indexed filter elements exist in original
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Second ensures clause of the same external_body axiom. Same unverified trust surface as φ1.

### φ3: pred_preserved_through_filter
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** tests the main lemma's conclusion at a specific element — depends on soundness of the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct application of the verified lemma `true_pred_on_seq_implies_true_pred_on_filtered_seq`. No independent trust surface beyond the external_body axiom already flagged.

### φ4: filter_len_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filter should never produce a longer sequence — tests basic consistency of filter definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition without the external_body axiom. Standard property.

### φ5: empty_filter_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filtering empty sequence should produce empty — tests base case consistency with external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Base case of filter's definition. Correct and proved without external_body.

