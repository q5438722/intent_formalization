# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_pred_false_on_all_elements_is_equivalent_to_empty_filter.rs`
**Date:** 2026-03-24T04:56:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from the two external_body axioms (`empty_filter_implies_seq_pred_false_on_all_elements` and `seq_pred_false_on_all_elements_implies_empty_filter`), both unverified trust assumptions that are semantically correct but could be proved from filter's definition by induction. Three false positives: one is a dependent instantiation of φ1 with no independent trust surface, and two are standard filter properties proved directly from filter's definition.

## True Positives (Spec Issues)

### external_body_forward_direction
- **Confidence:** high
- **Reasoning:** `empty_filter_implies_seq_pred_false_on_all_elements` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains unverified.

### external_body_reverse_direction
- **Confidence:** high
- **Reasoning:** `seq_pred_false_on_all_elements_implies_empty_filter` is external_body — second independent unverified trust assumption. Also provable from filter's definition but not verified.

## All Candidates

### φ1: external_body_forward_direction
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** empty_filter_implies_seq_pred_false_on_all_elements is external_body — unverified axiom asserting empty filter implies pred false on all elements
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `empty_filter_implies_seq_pred_false_on_all_elements` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains unverified.

### φ2: external_body_reverse_direction
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_pred_false_on_all_elements_implies_empty_filter is external_body — unverified axiom asserting pred false on all elements implies empty filter
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `seq_pred_false_on_all_elements_implies_empty_filter` is external_body — second independent unverified trust assumption. Also provable from filter's definition but not verified.

### φ3: biconditional_instantiation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** tests specific element instantiation of the external_body axiom — if the axiom is too strong this could derive contradictions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of φ1's universal quantifier. No independent trust surface beyond the first external_body axiom.

### φ4: singleton_true_filter_nonempty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton with pred true should have nonempty filter — tests that external_body axioms don't contradict filter's definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved from `reveal(Seq::filter)` without any external_body axiom. Correct property of filter's definition.

### φ5: filter_false_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always-false predicate should yield empty filter — tests consistency between external_body axioms and filter's definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition without external_body axioms. Correct.

