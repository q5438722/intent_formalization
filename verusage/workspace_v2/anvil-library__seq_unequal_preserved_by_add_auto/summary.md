# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_unequal_preserved_by_add_auto.rs`
**Date:** 2026-03-24T04:57:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive from the external_body axiom `seq_unequal_preserved_by_add` (unverified trust assumption that suffix append preserves sequence inequality). Three false positives are direct applications or contrapositives with no independent trust surface.

## True Positives (Spec Issues)

### external_body_suffix_cancellation
- **Confidence:** high
- **Reasoning:** `seq_unequal_preserved_by_add` is external_body — unverified trust assumption. Semantically correct and provable, but remains unverified.

## All Candidates

### φ1: external_body_suffix_cancellation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** seq_unequal_preserved_by_add is external_body — unverified axiom asserting suffix append preserves inequality without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `seq_unequal_preserved_by_add` is external_body — unverified trust assumption. Semantically correct and provable, but remains unverified.

### φ2: concat_equal_implies_prefix_equal
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** contrapositive right cancellation law derived from the external_body axiom — if the axiom is inconsistent this would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of the external_body axiom. No independent trust surface beyond φ1.

### φ3: empty_suffix_preserves_inequality
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty suffix boundary case — tests that the external_body axiom is consistent with s + empty == s
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiation of the axiom with empty suffix. No independent trust surface.

### φ4: different_len_different_concat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** different-length sequences are unequal so the axiom applies — tests the length branch is sound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Different lengths imply inequality, satisfying the axiom's precondition. No independent trust surface.

### φ5: auto_lemma_instantiation
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** tests that the auto wrapper correctly triggers on concrete singleton sequences differing at index 0

