# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_filter_preserves_no_duplicates.rs`
**Date:** 2026-03-24T04:55:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive from the external_body axiom `seq_filter_is_a_subset_of_original_seq` (unverified trust assumption that filter containment implies original containment). Three false positives are standard filter properties proved from filter's definition. The main lemma `seq_filter_preserves_no_duplicates` is verified but depends on the external_body axiom.

## True Positives (Spec Issues)

### external_body_filter_subset
- **Confidence:** high
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains an unverified axiom.

## All Candidates

### φ1: external_body_filter_subset
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_filter_is_a_subset_of_original_seq is external_body — unverified axiom asserting filter containment without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `seq_filter_is_a_subset_of_original_seq` is external_body — unverified trust assumption. Semantically correct and provable from filter's definition, but remains an unverified axiom.

### φ2: filter_preserves_no_dup
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** directly tests the main lemma — if external_body axiom is inconsistent, no_duplicates preservation could be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct restatement of the verified lemma `seq_filter_preserves_no_duplicates`. The lemma is proved (not external_body); its soundness depends on φ1's axiom but the property itself is correct.

### φ3: filter_len_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filter should never produce a longer sequence — tests basic consistency of filter definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition without the external_body axiom. Standard property.

### φ4: no_dup_filter_len_leq_distinct_count
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** under no_duplicates, filtered length should still be bounded by original — tests interaction of the two lemmas

### φ5: filter_false_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always-false predicate should yield empty filter — tests base consistency of filter and whether external_body axiom interacts correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition. Correct base consistency check.

