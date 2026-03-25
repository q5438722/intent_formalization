# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/empty_filter_implies_seq_pred_false_on_all_elements.rs`
**Date:** 2026-03-24T04:47:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard properties of `Seq::filter` from vstd's axiomatization. The lemma under test is a fully verified proof with no external_body axioms, and all candidates confirm expected behavior at boundaries and logical consequences. No spec issues found.

## All Candidates

### φ1: filter_empty_of_empty_seq
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tests that filtering an empty sequence produces empty — if filter is misaxiomatized on empty sequences this could fail
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Filtering an empty sequence produces empty. Definitional base case of `Seq::filter`. Correct and expected.

### φ2: singleton_true_filter_nonempty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a singleton sequence with an element satisfying pred should have nonempty filter — tests contrapositive of the lemma on minimal input
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Singleton with pred(v) true filters to nonempty. Direct consequence of filter's definition on push. Correct.

### φ3: element_satisfying_pred_contradicts_empty_filter
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** directly tests the lemma's conclusion at a specific element — if the forall quantifier is mistriggered this specific instantiation could reveal a gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the lemma's universal postcondition. Correct and expected.

### φ4: push_pred_true_makes_filter_nonempty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** appending an element satisfying pred should always produce nonempty filter — tests that filter interacts correctly with push regardless of original sequence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Filter of `s.push(v)` where `pred(v)` includes `v` in the result by filter's definition. Correct.

### φ5: all_false_pred_implies_filter_len_zero
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always-false predicate should yield empty filter on any sequence — tests the converse direction not proven by the lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Always-false predicate yields empty filter by induction on filter's definition. Correct converse property.

