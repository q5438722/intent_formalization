# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_equal_preserved_by_add.rs`
**Date:** 2026-03-24T04:50:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard properties of vstd's Seq axiomatization and direct consequences of the fully verified cancellation lemma. No spec issues found.

## All Candidates

### φ1: concat_empty_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** concatenating empty sequence should be identity — tests basic Seq concat axiom that the lemma relies on
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard vstd axiom. Correct and expected.

### φ2: concat_len_additive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** concat length should be sum of lengths — foundational axiom the proof depends on
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard vstd axiom. Correct and expected.

### φ3: equal_seqs_same_suffix_equal
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests the cancellation direction — if concat equality doesn't imply prefix equality, the biconditional is broken
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct application of the lemma's right-to-left direction. Correct cancellation law.

### φ4: different_len_seqs_different_concat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** sequences of different lengths should produce different concatenations — tests contrapositive of the cancellation law
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of the cancellation law. If s1+suffix == s2+suffix then s1==s2 implies s1.len()==s2.len(). Correct.

### φ5: prefix_not_equal_to_full
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a sequence should not equal itself with a nonempty suffix appended — tests that concat strictly increases length
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s.len() != (s + suffix).len()` when `suffix.len() > 0`, so they can't be equal. Correct.

