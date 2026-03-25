# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_equal_preserved_by_add_prefix.rs`
**Date:** 2026-03-24T04:50:41Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard consequences of vstd's Seq axioms and direct applications of the fully verified prefix cancellation lemma. No spec issues found.

## All Candidates

### φ1: prefix_cancel_left
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests the cancellation direction directly — if this direction is broken the biconditional is unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct application of the lemma's right-to-left direction. Correct cancellation law.

### φ2: different_len_suffix_different_concat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** sequences of different lengths appended to the same prefix should produce different results
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of cancellation: equal concats imply equal suffixes imply equal lengths. Correct.

### φ3: empty_prefix_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty prefix concatenation should be identity — foundational axiom the lemma depends on
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard vstd axiom for sequence concatenation. Correct.

### φ4: prefix_strictly_extends
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a nonempty suffix should make the concatenation differ from the prefix alone
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(prefix + s).len() == prefix.len() + s.len() > prefix.len()` when `s.len() > 0`. Correct.

### φ5: concat_preserves_first_element
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** if suffixes differ at first element then concatenations must differ — tests that element-level inequality propagates
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `s1[0] != s2[0]` then `s1 != s2`, so by the lemma `prefix + s1 != prefix + s2`. Correct.

