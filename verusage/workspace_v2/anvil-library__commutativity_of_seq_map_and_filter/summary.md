# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/commutativity_of_seq_map_and_filter.rs`
**Date:** 2026-03-24T04:46:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — standard boundary cases and instantiations of the commutativity lemma with no spec issues. The lemma is a fully verified proof with no external_body axioms; the candidates test expected properties.

## All Candidates

### φ1: empty_seq_commutativity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tests the empty sequence base case — if the base case is mishandled, empty sequences could produce unequal results
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence base case is trivially correct — both sides produce empty. Standard boundary test, no spec gap.

### φ2: map_filter_len_leq_original
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** filtered-then-mapped result should never exceed original length — if filter/map interaction is broken, length could exceed the original
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Filter never increases length. This is a standard property of `Seq::filter` from vstd, not a spec issue.

### φ3: filter_all_true_preserves_map
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** always-true filter should be identity on both sides — tests that trivial predicates don't break the commutativity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Always-true filter is identity, so commutativity holds trivially. Correct and expected.

### φ4: filter_all_false_both_empty
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** always-false filter should produce empty on both sides — tests that the commutativity result is vacuously correct for trivially empty filters

### φ5: singleton_filter_map_commute
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton sequence is the minimal nontrivial case — if the recursive step mishandles length-1 sequences, commutativity could silently break
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Singleton is a direct instantiation of the lemma with compatible predicates (`x > 0` preserved by `x * 2` for positive values). Correct application, no spec gap.

