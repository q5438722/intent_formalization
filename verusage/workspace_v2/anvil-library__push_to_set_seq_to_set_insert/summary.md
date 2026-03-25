# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/push_to_set_seq_to_set_insert.rs`
**Date:** 2026-03-24T04:49:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard properties of sets and direct consequences of the fully verified lemma. No spec issues found.

## All Candidates

### φ1: empty_seq_to_set_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty sequence should convert to empty set — tests base case of to_set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence has no elements, so `to_set()` is empty. Definitional property of vstd.

### φ2: push_preserves_existing_elements
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pushing should not remove existing elements from the set — tests monotonicity of push w.r.t. to_set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the lemma: `push(e).to_set() == to_set().insert(e)`, and `insert` preserves existing elements.

### φ3: push_element_in_result_set
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pushed element must appear in the resulting set — tests basic correctness of the lemma's conclusion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence: `to_set().insert(e).contains(e)` is always true.

### φ4: duplicate_push_to_set_idempotent
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** pushing a duplicate should not change the set — tests idempotence of set insert through the lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s.contains(e)` implies `to_set().contains(e)`, so `to_set().insert(e) == to_set()`. Correct.

### φ5: to_set_subset_of_push_to_set
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** original set should be subset of pushed set — tests that push+to_set is monotone
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `S ⊆ S.insert(e)` is a standard set property. Correct consequence of the lemma.

