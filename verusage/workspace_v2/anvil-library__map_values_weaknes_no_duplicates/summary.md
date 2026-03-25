# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/map_values_weaknes_no_duplicates.rs`
**Date:** 2026-03-24T04:48:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives — standard properties of vstd's Seq axiomatization. No spec issues found.

## All Candidates

### φ1: map_injective_preserves_no_dup
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** injective map should preserve no_duplicates — tests the converse direction not proven by the lemma

### φ2: empty_seq_no_duplicates
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty sequence trivially has no duplicates — boundary test for vacuous quantifier handling
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty sequence vacuously has no duplicates. Correct and expected.

### φ3: singleton_no_duplicates
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton mapped sequence trivially has no duplicates — tests minimal nontrivial case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Singleton sequence trivially has no duplicates. Correct.

### φ4: mapped_len_equals_original_len
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** map_values should preserve length — if it doesn't, the index-based proof in the lemma could be reasoning over misaligned indices
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `map_values` preserves length by definition in vstd. Correct and expected.

### φ5: non_injective_map_loses_no_dup
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** constant map on distinct elements should produce duplicates — if this fails, map_values or no_duplicates is broken

