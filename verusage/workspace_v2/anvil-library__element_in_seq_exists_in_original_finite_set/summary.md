# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/set_lib/element_in_seq_exists_in_original_finite_set.rs`
**Date:** 2026-03-24T04:59:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — direct applications or contrapositives of the fully verified lemma. No spec issues found.

## All Candidates

### φ1: empty_set_to_seq_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty set should convert to empty sequence — tests base case of to_seq
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Definitional property of `to_seq` on empty set. Correct.

### φ2: to_seq_contains_implies_set_contains
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** direct restatement of the lemma — tests that to_seq doesn't introduce spurious elements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct restatement of the verified lemma. Correct.

### φ3: singleton_to_seq_no_extra
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton set's to_seq should not contain elements not in the set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive applied to singleton. Correct consequence of the lemma.

### φ4: to_seq_len_equals_set_len
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** to_seq should preserve cardinality — if lengths disagree the bijection is broken

### φ5: not_in_set_not_in_to_seq
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** contrapositive of the lemma — elements not in set should not appear in to_seq
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of the lemma. Correct.

