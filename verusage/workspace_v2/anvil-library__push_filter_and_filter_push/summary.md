# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/push_filter_and_filter_push.rs`
**Date:** 2026-03-24T04:49:08Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — standard properties of `Seq::filter` and direct consequences of the verified lemma. No spec issues found.

## All Candidates

### φ1: filter_preserves_order
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** if filter doesn't preserve element order from the original sequence, push_filter commutativity could silently reorder elements

### φ2: filter_len_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filter should never produce a longer sequence — if it could, the push lemma's ensures clauses would be reasoning about ill-formed sequences
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard property of filter. Correct and expected.

### φ3: filter_empty_seq
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filtering empty sequence should produce empty — tests base case of filter definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence filters to empty. Definitional base case.

### φ4: push_filter_true_increases_len
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pushing an element satisfying pred should increase filtered length by exactly 1 — tests that the equality in the ensures clause is precise
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the lemma's first ensures clause: `s.push(e).filter(pred) == s.filter(pred).push(e)`, so len increases by 1.

### φ5: push_filter_false_preserves_len
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pushing an element not satisfying pred should not change filtered length — tests the second ensures clause is correct
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the lemma's second ensures clause: `s.push(e).filter(pred) == s.filter(pred)`, so len is unchanged.

