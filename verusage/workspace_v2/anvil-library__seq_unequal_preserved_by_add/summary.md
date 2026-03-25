# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_unequal_preserved_by_add.rs`
**Date:** 2026-03-24T04:57:12Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — direct applications or contrapositives of the fully verified lemma with no external_body axioms. No spec issues found.

## All Candidates

### φ1: unequal_seqs_different_concat
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** direct restatement of the lemma — tests the core claim that suffix append preserves inequality
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct restatement of the verified lemma. Correct.

### φ2: empty_suffix_preserves_inequality
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty suffix case — tests that the lemma doesn't vacuously hold only for nonempty suffixes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiation with empty suffix. Correct — `s + empty == s` by vstd axiom.

### φ3: different_len_seqs_different_concat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** different-length sequences are unequal — tests that the length branch of the proof is sound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Different lengths imply inequality, which satisfies the lemma's precondition. Correct.

### φ4: concat_equal_implies_prefix_equal
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** contrapositive of the lemma — right cancellation law; if the lemma is wrong this would derive false equalities
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of the lemma — right cancellation law. Correct consequence.

### φ5: singleton_diff_preserved
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** minimal same-length case with singletons differing at index 0 — tests the witness branch of the proof

