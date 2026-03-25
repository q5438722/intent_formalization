# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/true_pred_on_all_element_equal_to_pred_on_all_index.rs`
**Date:** 2026-03-24T04:58:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — direct applications, directions, or contrapositives of the fully verified biconditional lemma. No spec issues found.

## All Candidates

### φ1: empty_seq_vacuously_true
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty sequence vacuously satisfies any predicate — tests that the biconditional handles the empty case correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Vacuous truth on empty sequence. Correct and expected.

### φ2: index_form_implies_contains_form
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests the index-to-contains direction of the biconditional
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** One direction of the verified biconditional. Correct.

### φ3: contains_form_implies_index_form
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tests the contains-to-index direction of the biconditional
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Other direction of the verified biconditional. Correct.

### φ4: singleton_pred_true
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton with pred true should satisfy the contains form — tests minimal nontrivial case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Singleton instantiation — index form trivially holds, so contains form follows. Correct.

### φ5: negated_pred_false_element_not_in_seq
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** element failing pred should not be contained — tests contrapositive of the contains-form universal
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Contrapositive of the contains-form universal. Correct.

