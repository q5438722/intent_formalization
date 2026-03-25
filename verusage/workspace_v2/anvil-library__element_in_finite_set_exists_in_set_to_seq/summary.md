# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/set_lib/element_in_finite_set_exists_in_set_to_seq.rs`
**Date:** 2026-03-24T04:59:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives — standard properties of `Set::to_seq` proved from the fully verified lemma or vstd definitions. No spec issues found.

## All Candidates

### φ1: empty_set_to_seq_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty set should convert to empty sequence — tests base case of to_seq
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Definitional property of `to_seq` on empty set. Correct and expected.

### φ2: to_seq_len_equals_set_len
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** to_seq should preserve cardinality — if lengths disagree, the bijection between set and sequence is broken

### φ3: singleton_to_seq_contains
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton set's to_seq should contain the single element — minimal nontrivial test
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct application of the lemma on a singleton set. Correct.

### φ4: to_seq_no_extra_elements
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** to_seq should not introduce elements not in the original set — tests the reverse direction not proven by the lemma

### φ5: to_seq_no_duplicates
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** set elements are unique so to_seq should have no duplicates — if it does, cardinality and containment reasoning breaks

